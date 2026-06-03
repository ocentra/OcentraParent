# V0.5 App + Game Platform Deep Dive

## Correct Mental Model

Platform support is not binary. Every app/game action needs an authority tier,
setup state, capability status, and proof path.

Use these states instead of bare unsupported labels:

- `observe-only`
- `user-approved-helper`
- `accessibility-assisted`
- `managed-profile`
- `device-owner`
- `mdm-enrolled`
- `supervised-device`
- `system-extension`
- `root-or-admin-service`
- `kiosk-or-single-app`
- `manual-required`
- `not-claimed`

## Windows

Evidence sources:

- uninstall registry;
- Start Menu shortcuts;
- Microsoft Store/UWP/AppX/MSIX packages;
- known install locations;
- executable metadata;
- publisher/signature/hash;
- process snapshots;
- process start/exit;
- foreground window;
- launcher manifests and game package ids where available.

Current safe claims:

- observe inventory/runtime/foreground evidence;
- warn and ask through child-facing UX where implemented;
- count running and foreground time from stored evidence;
- scoped owned-process terminate/time-limit where proved.

Manual-required until proved:

- broad installed-app blocking;
- launch prevention;
- system-wide allowlist;
- AppLocker/App Control enforcement;
- rollback/system-app allow proof;
- admin/setup flow;
- game launcher/game process disambiguation beyond stored evidence.

## macOS

Potential evidence sources:

- `/Applications`, `~/Applications`, `/System/Applications`;
- app bundles and `Info.plist`;
- LaunchServices/Spotlight metadata;
- process list and bundle id from process path;
- code signature identity and team id;
- foreground app/window through Accessibility permission;
- launch, terminate, and activation events;
- MDM installed-app query when managed;
- game launcher manifests where available.

Manual-required until proved:

- PPPC profile for Accessibility/Automation;
- MDM/configuration profile path;
- Parental Controls payloads where available;
- System Extensions and Endpoint Security for hard block;
- LaunchAgent/LaunchDaemon or privileged helper setup;
- hard app/game block, allowlist, and rollback.

## Linux

Potential evidence sources:

- desktop entries;
- dpkg/rpm/pacman package database;
- Flatpak;
- Snap;
- AppImage scan in bounded paths;
- procfs process snapshots;
- cgroup and systemd unit identity;
- X11 foreground window where available;
- Wayland compositor/portal-specific foreground states;
- launcher manifests.

Manual-required until proved:

- cgroup/systemd scope enforcement;
- AppArmor or SELinux profile enforcement;
- package-manager restrictions;
- Flatpak/Snap permission policy;
- admin/root service setup;
- distro/session-specific foreground proof.

## Android

Potential evidence sources:

- package inventory with package visibility caveats;
- UsageStats usage summaries;
- UsageEvents timeline;
- Accessibility-assisted foreground/overlay state;
- Device Owner/Profile Owner/DPC package controls;
- Play policy/signing/store constraints.

Manual-required until proved:

- `setApplicationHidden`;
- `setPackagesSuspended`;
- `setUninstallBlocked`;
- lock task/allowlist mode;
- managed configurations;
- package access delegation;
- install restrictions;
- child request/approval UX.

Normal mode must not claim package hide/suspend.

## iOS And iPadOS

Consumer path:

- FamilyControls authorization;
- FamilyActivityPicker token selection;
- DeviceActivity schedules and thresholds;
- ManagedSettings shields;
- shield UI and exceptions.

Managed path:

- MDM installed-app query;
- supervised-device restrictions;
- App Lock payload;
- single-app/kiosk mode;
- store/signing/entitlement proof.

iOS must not claim raw process scanning, arbitrary process killing, or full
installed-app visibility without Apple-approved capability.

## Platform Matrix

| Action                          | Windows                       | macOS                     | Linux                     | Android                    | iOS/iPadOS              |
| ------------------------------- | ----------------------------- | ------------------------- | ------------------------- | -------------------------- | ----------------------- |
| Inventory                       | partial/proved by source      | manual-required           | manual-required           | manual-required            | token/MDM required      |
| Runtime                         | scoped/proved                 | permission-required       | mechanism-specific        | UsageStats required        | DeviceActivity required |
| Foreground                      | scoped/proved                 | Accessibility required    | X11/Wayland-specific      | UsageEvents/Accessibility  | DeviceActivity only     |
| Warn/ask                        | UI required                   | UI required               | UI required               | UI required                | shield/UI required      |
| Time budget                     | evidence-backed dry-run first | manual-required           | manual-required           | permission-required        | DeviceActivity required |
| Terminate owned/current process | scoped Windows proof          | permission/admin required | permission/admin required | not normal-mode            | not claimed             |
| Hide/suspend/shield             | manual-required               | MDM/system path           | mechanism-specific        | Device Owner/Profile Owner | ManagedSettings         |
| Block launch/allowlist          | AppLocker/App Control proof   | Endpoint/MDM proof        | cgroup/AppArmor proof     | DPC/lock task proof        | supervised/MDM proof    |

## Hard Rule

No platform claim moves from manual-required or not-claimed to supported until
the workpack includes authority tier, setup steps, tests, manual proof,
rollback, cleanup, audit refs, and parent-visible capability state.
