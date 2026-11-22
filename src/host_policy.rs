use base::config::keys;
use hbb_common::config;

pub const SERVER: &str = "home.awbz.cn";
pub const PUBLIC_KEY: &str = "sUEp2zK93vGOtk21nS66OOL2CD3ughHpgj3PAzlZ+cQ=";

pub fn apply() {
    #[cfg(not(test))]
    { *config::APP_NAME.write().unwrap() = "Klink".to_owned(); }
    *config::PROD_RENDEZVOUS_SERVER.write().unwrap() = SERVER.to_owned();
    *config::EXE_RENDEZVOUS_SERVER.write().unwrap() = SERVER.to_owned();
    let mut hard = config::HARD_SETTINGS.write().unwrap();
    hard.clear();
    for (key, value) in [
        ("conn-type", "incoming"),
        ("disable-settings", "Y"),
        ("disable-account", "Y"),
        ("disable-ab", "Y"),
        ("disable-installation", "N"),
    ] {
        hard.insert(key.to_owned(), value.to_owned());
    }
    drop(hard);
    let mut settings = config::OVERWRITE_SETTINGS.write().unwrap();
    for key in keys::KEYS_SETTINGS {
        if !is_password_option(key) && ![keys::OPTION_PROXY_URL, keys::OPTION_PROXY_USERNAME, keys::OPTION_PROXY_PASSWORD].contains(key) {
            settings.insert((*key).to_owned(), String::new());
        }
    }
    for (key, value) in [
        ("custom-rendezvous-server", SERVER),
        ("rendezvous-servers", SERVER),
        ("relay-server", SERVER),
        ("api-server", ""),
        ("key", PUBLIC_KEY),
        ("access-mode", "full"),
        ("verification-method", "use-permanent-password"),
        ("approve-mode", "password"),
        ("enable-keyboard", "Y"),
        ("enable-clipboard", "Y"),
        ("enable-file-transfer", "Y"),
        ("enable-tunnel", "Y"),
        ("enable-remote-restart", "Y"),
        ("enable-audio", "Y"),
        ("enable-camera", "Y"),
        ("enable-terminal", "Y"),
        ("enable-remote-printer", "Y"),
        ("enable-lan-discovery", "N"),
        ("enable-trusted-devices", "N"),
        ("allow-remote-config-modification", "N"),
        ("enable-perm-change-in-accept-window", "N"),
        ("direct-server", "N"),
        ("stop-service", ""),
        ("allow-auto-update", "N"),
        ("allow-websocket", "N"),
    ] {
        settings.insert(key.to_owned(), value.to_owned());
    }
    drop(settings);
    config::Config::set_socks(None);
    let mut builtin = config::BUILTIN_SETTINGS.write().unwrap();
    for key in ["host-only", "hide-general-settings", "hide-security-settings", "hide-network-settings",
        "hide-help-cards", "hide-server-settings", "hide-proxy-settings",
        "hide-remote-printer-settings", "disable-change-id",
        "disable-unlock-pin"] {
        builtin.insert(key.to_owned(), "Y".to_owned());
    }
    builtin.insert("register-device".to_owned(), "N".to_owned());
    drop(builtin);
    let mut defaults = config::DEFAULT_SETTINGS.write().unwrap();
    defaults.insert("verification-method".to_owned(), "use-permanent-password".to_owned());
    defaults.insert("temporary-password-length".to_owned(), "10".to_owned());
    defaults.insert("approve-mode".to_owned(), "password".to_owned());
    drop(defaults);
    let mut local = config::OVERWRITE_LOCAL_SETTINGS.write().unwrap();
    local.insert("pre-elevate-service".to_owned(), "Y".to_owned());
    local.insert("enable-check-update".to_owned(), "N".to_owned());
    local.insert("enable-webrtc".to_owned(), "N".to_owned());
}

pub const GENERATED_PASSWORD_OPTION: &str = "host-generated-password";

pub fn is_password_option(key: &str) -> bool {
    matches!(key, "verification-method" | "temporary-password-length" |
        "approve-mode" | "allow-numeric-one-time-password")
}

pub fn allows_password_value(key: &str, value: &str) -> bool {
    match key {
        "verification-method" => value == "use-permanent-password",
        "temporary-password-length" => matches!(value, "6" | "8" | "10"),
        "approve-mode" => value == "password",
        "allow-numeric-one-time-password" => matches!(value, "" | "Y" | "N"),
        _ => false,
    }
}

pub fn provision_password() -> hbb_common::ResultType<()> {
    use hbb_common::{bail, config::Config, password_security};
    if Config::has_permanent_password() {
        return Ok(());
    }
    let password = Config::get_auto_password(20);
    let encrypted = password_security::encrypt_str_or_original(&password, "00", 1024);
    if encrypted == password || encrypted.is_empty() {
        bail!("Cannot securely store the generated host password");
    }
    if !Config::set_permanent_password(&password) {
        bail!("Cannot provision the host password");
    }
    let (storage, _) = Config::get_local_permanent_password_storage_and_salt();
    let record = serde_json::to_string(&(encrypted, storage))?;
    Config::set_option(GENERATED_PASSWORD_OPTION.to_owned(), record);
    Ok(())
}

pub fn preserve_generated_password(mut options: std::collections::HashMap<String, String>) -> std::collections::HashMap<String, String> {
    options.remove(GENERATED_PASSWORD_OPTION);
    let record = config::Config::get_option(GENERATED_PASSWORD_OPTION);
    if !record.is_empty() {
        options.insert(GENERATED_PASSWORD_OPTION.to_owned(), record);
    }
    options
}

pub fn generated_password() -> String {
    use hbb_common::{config::Config, password_security};
    let record = Config::get_option(GENERATED_PASSWORD_OPTION);
    let Ok((encrypted, expected_storage)) = serde_json::from_str::<(String, String)>(&record) else {
        return String::new();
    };
    if Config::get_local_permanent_password_storage_and_salt().0 != expected_storage {
        return String::new();
    }
    let (password, decrypted, _) = password_security::decrypt_str_or_original(&encrypted, "00");
    if decrypted { password } else { String::new() }
}

pub fn allows_args(args: impl IntoIterator<Item = String>) -> bool {
    !args.into_iter().any(|arg| {
        matches!(arg.split('=').next().unwrap_or(""),
            "--connect" | "--play" | "--file-transfer" | "--view-camera" |
            "--port-forward" | "--terminal" | "--terminal-admin" | "--rdp" |
            "--config" | "--import-config" | "--option" | "--password" |
            "--set-id" | "--set-unlock-pin") || arg.contains("://")
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn outgoing_and_configuration_commands_are_rejected() {
        for arg in ["--connect", "--port-forward=123", "--option", "--password", "rustdesk://123"] {
            assert!(!allows_args([arg.to_owned()]));
        }
    }
    #[test]
    fn fixed_policy_overrides_old_network_options_and_keeps_installation() {
        *config::APP_NAME.write().unwrap() = "RustDeskHostPolicyTests".to_owned();
        crate::common::init_sodium();
        apply();
        config::Config::set_option("custom-rendezvous-server".into(), "other.example".into());
        config::Config::set_option("key".into(), "wrong-key".into());
        assert_eq!(config::Config::get_rendezvous_servers(), vec![SERVER.to_owned()]);
        assert_eq!(config::Config::get_option("key"), PUBLIC_KEY);
        assert!(config::is_incoming_only());
        assert!(config::is_disable_settings());
        assert!(!config::is_disable_installation());
        assert_eq!(config::Config::get_option("access-mode"), "full");
        assert_eq!(config::Config::get_option("verification-method"), "use-permanent-password");
        assert_eq!(config::Config::get_option("api-server"), "");
        assert!(!config::Config::is_disable_change_permanent_password());
        config::Config::set_option("verification-method".into(), "use-both-passwords".into());
        apply();
        assert_eq!(config::Config::get_option("verification-method"), "use-permanent-password");
        assert!(config::Config::set_permanent_password(""));
        assert!(provision_password().is_ok());
        let password = generated_password();
        assert_eq!(password.len(), 20);
        assert!(!config::Config::get_option(GENERATED_PASSWORD_OPTION).contains(&password));
        assert!(provision_password().is_ok());
        assert!(generated_password() == password);
        let options = preserve_generated_password(Default::default());
        assert!(!options.get(GENERATED_PASSWORD_OPTION).unwrap().is_empty());
        assert!(config::Config::set_permanent_password("test-user-selected-password"));
        assert!(generated_password().is_empty());
    }
    #[test]
    fn only_password_settings_accept_changes() {
        assert!(allows_password_value("verification-method", "use-permanent-password"));
        assert!(!allows_password_value("approve-mode", "password-click"));
        assert!(!allows_password_value("verification-method", "use-both-passwords"));
        assert!(allows_password_value("temporary-password-length", "8"));
        assert!(!allows_password_value("temporary-password-length", "1"));
        assert!(!allows_password_value("relay-server", "other.example"));
    }
    #[test]
    fn installation_and_service_commands_are_preserved() {
        for arg in ["--install", "--uninstall", "--service", "--server", "--cm", "--elevate", "--silent-install"] {
            assert!(allows_args([arg.to_owned()]));
        }
    }
}
