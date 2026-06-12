# V0.5 App + Game Platform Deep Dive

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `V0.5 App + Game Platform Deep Dive`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

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

## Contract Proof - 2026-06-03

`packages/parent-domain/src/app-game-control-platform-authority.ts` now records
the shared app/game authority matrix as TypeScript contract proof. The contract
requires platform/action rows to name authority tier, setup state, proof state,
parent-visible state, parent-visible limitation, proof references, and proof
needed to claim. The companion rules reject hard-control execution from
observe-only, manual-required, and not-claimed rows and require:

- Android hide/suspend rows to carry Device Owner or Profile Owner proof;
- iOS shield rows to carry FamilyControls and ManagedSettings proof;
- macOS hard block rows to carry MDM, Endpoint Security, or System Extension
  proof;
- Linux hard block rows to name mechanism, distro, and session proof;
- Windows broad block rows to carry AppLocker or App Control proof before they
  can move out of manual-required.

This is not runtime platform proof. It does not add adapters, enrollment,
rollback execution, cleanup execution, service events, or portal rows.

## Extension Routing Proof - 2026-06-03

WP25 adds
`packages/parent-domain/src/app-game-platform-extension-routing.ts` and
companion rules/data files as the proof-routing layer for platform extension
rows. The matrix covers `MAC-01` through `MAC-12`, `IOS-01` through `IOS-12`,
`ANDROID-01` through `ANDROID-14`, and `LINUX-01` through `LINUX-14`.

The routing contract requires each row to name platform, product scope, action
scope, authority tier, setup state, capability status, promotion state, manual
tags, app/app-game proof packs, and cross-plan handoff. Promotion-ready rows
must attach authority-tier, permission/setup, rollback, manual-platform,
validation, and proof references before a future worker can claim support.

The proof pack is
`output/app-game-plan-proof/25-platform-extension-checklist-and-proof-routing/`.
This is routing and negative-claim proof only; every current platform extension
row remains manual-required or not-claimed until real platform proof exists.
