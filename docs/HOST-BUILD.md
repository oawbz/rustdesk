# Windows host-only build

This fork's `host-only` Cargo feature builds a locked incoming-only client.
ID and relay server: `home.awbz.cn`. API disabled. Public key:
`sUEp2zK93vGOtk21nS66OOL2CD3ughHpgj3PAzlZ+cQ=`.

The UI shows the device ID, a generated 10-character temporary password, status,
and installation/service notices. Outgoing sessions and settings changes are
blocked. Installation, uninstall, the Windows service, incoming desktop,
clipboard, audio, file transfer, TCP tunneling and restart remain available.
Camera, terminal, remote printing, LAN discovery, direct IP access, WebRTC,
account/API access and official update checks are disabled in this build.
A temporary password is required; there is no baked-in permanent password.
Installed service restart can change the password; this is not an unattended
credential provisioning system.

The original RustDesk controller must use the same server/public key.
Native direct connections remain available; failed direct connections use the
configured relay. Local DNS and OS traffic are outside this client's policy.
Packet capture and real Windows install/UAC/reboot tests are required before
claiming network isolation and production acceptance.

Run `Windows Host Build` in GitHub Actions, push to `host-only`, or push a
`host-v*` tag. The artifact contains the single self-extracting EXE, SHA256,
license and source commit. Run the EXE, then select Install if a persistent
Windows service is wanted. The test executable is unsigned.

Upstream behavior is preserved when `host-only` is disabled. The shared
hbb_common submodule is pinned and unchanged. Source and license are available
at https://github.com/oawbz/rustdesk (AGPL-3.0).
