# Windows host-only build

This fork's `host-only` Cargo feature builds a locked incoming-only client.
ID and relay server: `home.awbz.cn`. API disabled. Public key:
`sUEp2zK93vGOtk21nS66OOL2CD3ughHpgj3PAzlZ+cQ=`.

The UI shows the device ID, password controls, status, and installation/service
notices. Only the password settings card remains accessible; other settings and
outgoing sessions are blocked. Installation, uninstall, the Windows service, incoming desktop,
clipboard, audio, file transfer, TCP tunneling and restart remain available.
Camera, terminal, remote printing, LAN discovery, direct IP access, WebRTC,
account/API access and official update checks are disabled in this build.
On first hosting startup, a unique 20-character permanent password is generated
if none exists. It persists across restarts. The initial password is stored
encrypted locally and can be revealed/copied from the main window. A manually
changed password is not recoverable from that initial-password view. The user
can change the permanent password, access approval mode, temporary password
length/numeric option, or select either/both password types. There is no shared
password in the source or build logs. Installed security settings retain the
upstream local administrator unlock gate.

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
