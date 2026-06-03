# V0.5 Native Apps Platform Deep Dive

This document records platform authority tiers for native app control. It is a
planning and proof-routing document, not a product claim.

## Correct Mental Model

For each platform, represent separate lanes:

1. Evidence lane: what apps are installed, running, or foreground?
2. Classification lane: what kind of app is it?
3. Soft-control lane: warn, ask parent, show overlay, count time.
4. Hard-control lane: stop, hide, suspend, shield, block launch, allowlist.
5. Enrollment lane: what setup gives enough authority?
6. Proof lane: what artifact proves the claim?

Do not treat normal app permissions as the only route. Parental control needs
multiple authority tiers.

## Authority Tiers

Use these tiers across all platforms:

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

Every platform row must answer:

- Can observe, under which tier?
- Can detect foreground, under which tier?
- Can block launch, under which tier?
- Can hide/suspend/shield app, under which tier?
- Can enforce allowlist, under which tier?
- Can survive reboot/tamper, under which tier?
- What parent-facing setup cost exists?
- What proof moves the state from manual-required to claimed?

## Windows Authority Summary

Windows is the first app-control MVP path.

Evidence paths:

- uninstall registry;
- Start Menu shortcuts;
- Microsoft Store/UWP/AppX packages;
- known install locations;
- executable metadata;
- publisher/signature/hash;
- process snapshots;
- process start/exit;
- foreground window.

Soft controls:

- observe;
- warn;
- ask;
- count running/foreground time;
- scoped owned-process terminate/time-limit where proved.

Hard controls:

- owned/current process terminate where policy and target recheck pass;
- AppLocker audit/enforce proof;
- App Control for Business proof;
- strict allowlist only after rollback/system-app allow proof.

Manual-required until proved:

- broad installed-app blocking;
- launch prevention;
- system-wide allowlist;
- policy rollback;
- safe system-app exceptions;
- admin/setup flow.

## macOS Deep Path

macOS is permission, entitlement, profile, and service driven.

Evidence paths:

- `/Applications`, `~/Applications`, `/System/Applications`;
- app bundles and `Info.plist`;
- bundle id, display name, short version;
- LaunchServices/Spotlight metadata;
- process list, pid, ppid, executable path;
- bundle id from process path/app bundle;
- code signature identity and team id;
- foreground app/window with Accessibility permission;
- launch, terminate, and activation events;
- MDM installed-app query when managed.

Soft controls:

- warn child;
- show overlay/window;
- ask parent;
- count time;
- quit app politely where supported;
- terminate process if permitted;
- manage Ocentra-owned launcher paths;
- hide/disable Ocentra-managed shortcuts.

Hard-control paths:

- MDM/configuration profile path;
- Parental Controls payload research/proof;
- Privacy Preferences Policy Control for Accessibility/Automation permissions;
- Managed Login Items for agent persistence;
- System Extensions payload for system-extension approval;
- Endpoint Security system extension for exec observe/auth where entitled;
- LaunchAgent/LaunchDaemon or privileged helper for persistence and user-session
  events.

macOS capability matrix:

| Tier                 | Evidence                                               | Soft Control           | Hard Control                          | Proof State                                     |
| -------------------- | ------------------------------------------------------ | ---------------------- | ------------------------------------- | ----------------------------------------------- |
| Normal app           | App bundle/process inventory; running state            | Warn/ask UI            | Weak/manual-required                  | Permission and fixture proof required           |
| User-approved helper | Better session and quit/terminate                      | Warn/ask/quit          | Terminate where permitted             | Helper install and audit proof                  |
| MDM/profile          | Installed-app query, restrictions, managed permissions | Setup-visible controls | Profile payload controls              | Device enrollment/profile proof                 |
| System extension     | Exec/process lineage where entitled                    | Policy audit           | Exec auth/block if entitlement allows | Signing, entitlement, approval, extension proof |

macOS extension checklist:

| Step   | Item                                                  | Status | Proof Gate                                 |
| ------ | ----------------------------------------------------- | ------ | ------------------------------------------ |
| MAC-01 | App bundle inventory adapter                          | [ ]    | Fixture and manual host proof              |
| MAC-02 | LaunchServices/Spotlight inventory adapter            | [ ]    | Metadata fixture and privacy proof         |
| MAC-03 | NSWorkspace running app adapter                       | [ ]    | Real macOS host proof                      |
| MAC-04 | Accessibility foreground/window adapter               | [ ]    | Permission prompt and foreground proof     |
| MAC-05 | Code signature/team-id identity adapter               | [ ]    | Signature fixture and invalid-state proof  |
| MAC-06 | LaunchAgent/LaunchDaemon service model                | [ ]    | Service install/restart proof              |
| MAC-07 | PPPC profile for Accessibility/Automation permissions | [ ]    | Profile payload/manual proof               |
| MAC-08 | MDM installed-app query proof                         | [ ]    | MDM query artifact                         |
| MAC-09 | Parental Controls payload proof                       | [ ]    | Managed Mac payload artifact               |
| MAC-10 | Endpoint Security feasibility/proof                   | [ ]    | Entitlement/signing/system-extension proof |
| MAC-11 | macOS terminate/quit adapter                          | [ ]    | Target recheck and rollback proof          |
| MAC-12 | macOS hard-block manual proof gate                    | [ ]    | Explicit no-claim until real proof         |

## iOS And iPadOS Deep Path

iOS is not a Rust-agent process-scanning platform. It is a Screen Time,
FamilyControls, ManagedSettings, DeviceActivity, MDM, supervised-device, and
App Lock platform.

Consumer parental path:

- FamilyControls authorization;
- FamilyActivityPicker app/category/web-domain selection;
- opaque application/category/web-domain tokens;
- DeviceActivity schedules and thresholds;
- ManagedSettings shields;
- shield UI and exceptions.

Managed/school path:

- MDM installed-app query;
- supervised-device restrictions;
- App Lock payload;
- single-app/kiosk mode;
- network/content payloads where adjacent plans own them.

UI rule:

- Do not show all iOS installed apps from private APIs.
- Show selected controlled apps/categories, opaque token status, authorization
  state, shield status, DeviceActivity schedule/threshold state, and
  unavailable/manual-required states.

iOS extension checklist:

| Step   | Item                                        | Status | Proof Gate                                  |
| ------ | ------------------------------------------- | ------ | ------------------------------------------- |
| IOS-01 | FamilyControls authorization UX             | [ ]    | Entitlement and runtime authorization proof |
| IOS-02 | FamilyActivityPicker token selection        | [ ]    | Token selection and privacy proof           |
| IOS-03 | DeviceActivity schedule/threshold monitor   | [ ]    | DeviceActivity callback proof               |
| IOS-04 | ManagedSettings app/category shield adapter | [ ]    | Shield action proof                         |
| IOS-05 | Shield UI and child request flow            | [ ]    | Screenshot and approval proof               |
| IOS-06 | Token-based app identity model              | [ ]    | No raw app inventory proof                  |
| IOS-07 | iOS MDM installed-app query mode            | [ ]    | MDM query artifact                          |
| IOS-08 | Supervised-device restriction matrix        | [ ]    | Supervision/MDM proof                       |
| IOS-09 | App Lock / Single App Mode proof            | [ ]    | App Lock payload proof                      |
| IOS-10 | iOS exception/approval flow                 | [ ]    | Approval expiry and audit proof             |
| IOS-11 | iOS manual-required fallback labels         | [ ]    | UI proof for unavailable states             |
| IOS-12 | iOS store/signing/entitlement proof         | [ ]    | TestFlight/device/App Store proof           |

## Android Deep Path

Android has three different modes.

Normal app plus Usage Access:

- installed package list with package visibility caveats;
- UsageStats usage summaries;
- UsageEvents timeline;
- foreground-ish app transitions;
- screen interactive/noninteractive;
- daily app duration;
- app category from store/catalog/manual/AI.

Soft controls:

- notify;
- warn;
- ask parent;
- launch Ocentra screen;
- guide child away;
- count time.

Accessibility-assisted:

- faster foreground/app-open detection;
- transparent overlay/block screen;
- limited navigation guidance;
- risk-app warnings;
- native social/signup detection only where visible and permissioned.

Device Owner / Profile Owner / DPC:

- hide applications;
- suspend packages;
- block uninstall;
- permitted app lists;
- lock task mode;
- managed profiles;
- managed configurations;
- package access delegation;
- install restrictions;
- cross-profile policies.

Android extension checklist:

| Step       | Item                                              | Status | Proof Gate                              |
| ---------- | ------------------------------------------------- | ------ | --------------------------------------- |
| ANDROID-01 | Package inventory and visibility contract         | [ ]    | Package visibility and permission proof |
| ANDROID-02 | UsageStats permission and usage summary adapter   | [ ]    | Usage access proof                      |
| ANDROID-03 | UsageEvents foreground/session adapter            | [ ]    | UsageEvents replay proof                |
| ANDROID-04 | Accessibility-assisted foreground/overlay adapter | [ ]    | Transparent opt-in proof                |
| ANDROID-05 | VpnService/DNS relation handoff to network plan   | [ ]    | Network-plan no-claim proof             |
| ANDROID-06 | Device Owner provisioning flow                    | [ ]    | Real device-owner proof                 |
| ANDROID-07 | Profile Owner / managed profile flow              | [ ]    | Managed profile proof                   |
| ANDROID-08 | setApplicationHidden adapter proof                | [ ]    | DPM hidden-package proof                |
| ANDROID-09 | setPackagesSuspended adapter proof                | [ ]    | DPM suspension proof                    |
| ANDROID-10 | setUninstallBlocked proof                         | [ ]    | DPM uninstall-block proof               |
| ANDROID-11 | Lock Task / allowlist mode proof                  | [ ]    | Dedicated-device proof                  |
| ANDROID-12 | Managed configurations for app restrictions       | [ ]    | Managed configuration proof             |
| ANDROID-13 | Play policy/signing/store compliance proof        | [ ]    | Store/signing proof                     |
| ANDROID-14 | Android child request/approval UX                 | [ ]    | Device screenshot and audit proof       |

## Linux Deep Path

Linux is messy but powerful. Claims must be distro, desktop, service-manager,
package-format, privilege, and display-server specific.

Evidence paths:

- `/usr/share/applications/*.desktop`;
- `~/.local/share/applications/*.desktop`;
- Flatpak list;
- Snap list;
- dpkg/rpm/pacman package database;
- AppImage scan in bounded known paths;
- known install dirs;
- executable hash/signature where available;
- `/proc` and procfs process snapshots;
- process id/start time/cmdline/exe/cwd;
- cgroup and systemd unit/user unit;
- Flatpak app id from process environment/cgroup;
- X11 foreground window where available;
- Wayland compositor/portal-specific foreground states where available.

Soft controls:

- notify;
- warn;
- ask parent;
- count running/foreground time;
- terminate owned process where permitted.

Hard controls:

- systemd scopes/user units;
- cgroup freezer/kill policies;
- AppArmor or SELinux profiles;
- package-manager restrictions;
- Flatpak/Snap permission policy;
- admin/root service proof;
- desktop-environment integration where proved.

Linux extension checklist:

| Step     | Item                                      | Status | Proof Gate                        |
| -------- | ----------------------------------------- | ------ | --------------------------------- |
| LINUX-01 | Desktop entry inventory adapter           | [ ]    | Desktop-entry fixture proof       |
| LINUX-02 | dpkg/rpm/pacman package inventory adapter | [ ]    | Distro matrix proof               |
| LINUX-03 | Flatpak inventory adapter                 | [ ]    | Flatpak app id proof              |
| LINUX-04 | Snap inventory adapter                    | [ ]    | Snap app id proof                 |
| LINUX-05 | AppImage bounded scan adapter             | [ ]    | Bounded scan/no-secret proof      |
| LINUX-06 | procfs runtime adapter                    | [ ]    | Real host/process proof           |
| LINUX-07 | cgroup/systemd identity adapter           | [ ]    | systemd/cgroup proof              |
| LINUX-08 | X11 foreground adapter                    | [ ]    | X11 foreground proof              |
| LINUX-09 | Wayland compositor capability matrix      | [ ]    | Compositor-specific proof         |
| LINUX-10 | Linux terminate adapter                   | [ ]    | Target recheck and rollback proof |
| LINUX-11 | cgroup/systemd scope enforcement proof    | [ ]    | Service-manager proof             |
| LINUX-12 | AppArmor/SELinux manual proof             | [ ]    | Policy profile proof              |
| LINUX-13 | Package-manager restriction proof         | [ ]    | Distro package proof              |
| LINUX-14 | Flatpak/Snap restriction proof            | [ ]    | Sandbox permission proof          |

## Revised Platform Matrix

| Platform   | Evidence                                | Foreground                         | Soft Control                | Hard Control                             | Authority Path                                     |
| ---------- | --------------------------------------- | ---------------------------------- | --------------------------- | ---------------------------------------- | -------------------------------------------------- |
| Windows    | Excellent                               | Excellent                          | Good                        | Strong with AppLocker/App Control proof  | Service + process + AppLocker/App Control          |
| macOS      | Good/excellent                          | Good with Accessibility            | Good                        | Strong with MDM/Endpoint Security proof  | Agent + PPPC/MDM + ES/System Extension             |
| iOS/iPadOS | Token/MDM based                         | DeviceActivity threshold based     | Good through shield/request | Strong in Screen Time/MDM/App Lock scope | FamilyControls + ManagedSettings + MDM             |
| Android    | Good with UsageStats/package visibility | Good with UsageStats/Accessibility | Good                        | Strong with Device/Profile Owner proof   | UsageStats + Accessibility + DPC                   |
| Linux      | Good but distro-specific                | X11 good, Wayland variable         | Good                        | Strong but distro/admin-specific         | Daemon + procfs + desktop/package + cgroups/policy |

## Hard Rule

Do not write "platform unsupported" unless the plan also lists:

1. normal app capability;
2. permissioned capability;
3. managed-device capability;
4. admin/root/system-extension capability;
5. kiosk/single-app capability if relevant;
6. exact proof needed to move from manual-required to claimed.

The question is not possible or impossible. The question is: under what
authority tier, with what permission, what proof, and what parent-facing setup
cost?

## Contract Proof - 2026-06-03

Native app WP10 is covered by the shared app/game WP11 authority matrix contract
in `packages/parent-domain/src/app-game-control-platform-authority.ts`. The
contract records authority tier, setup state, proof state, capability state,
parent-visible limitation, proof references, and proof needed to claim for each
platform/action row. Tests prove observe-only cannot execute hard-control
adapters, manual-required/not-claimed cannot execute, Android hide/suspend
needs Device Owner or Profile Owner proof, iOS shield needs
FamilyControls/ManagedSettings proof, macOS hard block needs MDM or
Endpoint/System Extension proof, and Linux hard block needs mechanism, distro,
and session proof.

The proof pack is
`output/app-plan-proof/10-cross-platform-authority-matrix/`. It is contract and
test proof only; live platform crawling, app blocking, enrollment, rollback,
service events, portal UI, and product checklist status remain unchanged.

## Extension Routing Proof - 2026-06-03

Native app WP24 is covered by the shared app/game WP25 routing contract in
`packages/parent-domain/src/app-game-platform-extension-routing.ts` with
companion rules/data files. The matrix covers `MAC-01` through `MAC-12`,
`IOS-01` through `IOS-12`, `ANDROID-01` through `ANDROID-14`, and `LINUX-01`
through `LINUX-14`.

Each row names platform, product scope, action scope, authority tier, setup
state, capability status, promotion state, manual tags, app/app-game proof
packs, and cross-plan handoff. Promotion-ready rows must attach
authority-tier, permission/setup, rollback, manual-platform, validation, and
proof references.

The proof pack is
`output/app-plan-proof/24-platform-extension-checklist-and-proof-routing/`.
This is routing and negative-claim proof only; every current platform extension
row remains manual-required or not-claimed until real platform proof exists.
