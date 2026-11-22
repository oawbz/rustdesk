# Windows host-only build

This fork's `host-only` Cargo feature builds an incoming-only client named Klink.
The executable is named `Klink.exe`, with independent Klink installation,
service and configuration names to avoid conflicts with stock RustDesk.
ID and relay server: `home.awbz.cn`. API disabled. Public key:
`sUEp2zK93vGOtk21nS66OOL2CD3ughHpgj3PAzlZ+cQ=`.

The UI shows the device ID, password controls, status, and installation/service
button. Permanent password changes open a dialog directly; settings pages and
outgoing sessions are blocked. Installation, uninstall, the Windows service, incoming desktop,
clipboard, audio, file transfer, TCP tunneling and restart remain available.
Remote permissions default to full access, including terminal, camera and
remote printing where the OS and installed drivers support them. Drivers are
not automatically installed by the permission policy. Windows administrator
rights still require normal UAC authorization/service installation. Portable
startup requests an elevated helper through the upstream pre-elevate-service
flow; it does not bypass UAC. LAN
discovery, direct IP access, WebRTC, account/API access and official update
checks are disabled. Network/permission settings remain locked.
On first hosting startup, a unique 8-character permanent password is generated
if none exists. It persists across restarts. The initial password is stored
encrypted locally (using RustDesk's existing machine-bound storage mechanism;
this does not protect it from a local administrator) and can be revealed/copied from the main window. A manually
changed password is not recoverable from that initial-password view. The main window provides separate ID/password copy buttons and a combined
ID + password copy button. Manually changed passwords cannot be read back. The user
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

被控版仅使用固定密码，主页修改按钮直接弹出改密窗口；不提供设置页面、一次性密码或授权方式切换。隐藏主页支持标语和安装说明文字，保留安装按钮。主程序和交付文件名均为 Klink.exe，使用独立 Klink 安装、服务和配置名称，避免与原版 RustDesk 冲突。

The Klink Windows tray Quit action stops its service and exits Klink processes while retaining the service registration, startup shortcut and installed files. Opening an installed Klink starts its service if stopped, and repairs a missing registration left by an older version. Starting or stopping the service may require UAC approval.

Klink automatically generates an 8-character password for a new device. Existing passwords are preserved. Manual passwords require at least 6 characters, including an ASCII letter and a digit; uppercase/lowercase combinations and special characters are not required. Matching confirmation and the existing storage limit still apply.
