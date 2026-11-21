use base::config::keys;
use hbb_common::config;

pub const SERVER: &str = "home.awbz.cn";
pub const PUBLIC_KEY: &str = "sUEp2zK93vGOtk21nS66OOL2CD3ughHpgj3PAzlZ+cQ=";

pub fn apply() {
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
        if ![keys::OPTION_PROXY_URL, keys::OPTION_PROXY_USERNAME, keys::OPTION_PROXY_PASSWORD].contains(key) {
            settings.insert((*key).to_owned(), String::new());
        }
    }
    for (key, value) in [
        ("custom-rendezvous-server", SERVER),
        ("rendezvous-servers", SERVER),
        ("relay-server", SERVER),
        ("api-server", ""),
        ("key", PUBLIC_KEY),
        ("verification-method", "use-temporary-password"),
        ("temporary-password-length", "10"),
        ("approve-mode", "password"),
        ("enable-keyboard", "Y"),
        ("enable-clipboard", "Y"),
        ("enable-file-transfer", "Y"),
        ("enable-tunnel", "Y"),
        ("enable-remote-restart", "Y"),
        ("enable-audio", "Y"),
        ("enable-camera", "N"),
        ("enable-terminal", "N"),
        ("enable-remote-printer", "N"),
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
    for key in ["hide-general-settings", "hide-security-settings", "hide-network-settings",
        "hide-help-cards", "hide-server-settings", "hide-proxy-settings",
        "hide-remote-printer-settings", "hide-stop-service", "disable-change-id",
        "disable-change-permanent-password", "disable-unlock-pin"] {
        builtin.insert(key.to_owned(), "Y".to_owned());
    }
    builtin.insert("register-device".to_owned(), "N".to_owned());
    drop(builtin);
    let mut local = config::OVERWRITE_LOCAL_SETTINGS.write().unwrap();
    local.insert("enable-check-update".to_owned(), "N".to_owned());
    local.insert("enable-webrtc".to_owned(), "N".to_owned());
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
        apply();
        config::Config::set_option("custom-rendezvous-server".into(), "other.example".into());
        config::Config::set_option("key".into(), "wrong-key".into());
        assert_eq!(config::Config::get_rendezvous_servers(), vec![SERVER.to_owned()]);
        assert_eq!(config::Config::get_option("key"), PUBLIC_KEY);
        assert!(config::is_incoming_only());
        assert!(config::is_disable_settings());
        assert!(!config::is_disable_installation());
        assert_eq!(config::Config::get_option("verification-method"), "use-temporary-password");
        assert_eq!(config::Config::get_option("api-server"), "");
    }
    #[test]
    fn installation_and_service_commands_are_preserved() {
        for arg in ["--install", "--uninstall", "--service", "--server", "--cm", "--elevate", "--silent-install"] {
            assert!(allows_args([arg.to_owned()]));
        }
    }
}
