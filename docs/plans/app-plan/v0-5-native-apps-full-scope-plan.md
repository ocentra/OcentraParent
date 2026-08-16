# V0.5 Native Apps Full Scope Plan

This plan organizes native app evidence and control work. It does not implement
new product behavior by itself.

## Product Rule

```text
Installed app inventory is evidence, not current use.
Running process evidence proves process use, not foreground use.
Foreground evidence proves active use, not content.
AI may classify unknown apps from stored evidence, but cannot scan the OS directly.
Parent policy decides allow, observe, warn, ask, limit, block, or manual-required.
Enforcement requires platform adapter proof and authority-tier proof.
```

## Scope

This app plan covers native, installed, package, desktop, and mobile apps.

Included:

- productivity apps;
- school apps;
- chat apps;
- social apps as native apps;
- video apps as native apps;
- music apps;
- utility apps;
- VPN/proxy/tunnel apps;
- download/torrent apps;
- remote desktop apps;
- AI/chatbot apps;
- developer tools;
- unknown apps;
- portable apps;
- store apps;
- system apps;
- background helper apps;
- accessibility-sensitive apps;
- installer/updater apps.

Not included here:

- browser URL/page/video logic;
- browser games;
- browser social pages;
- native games and game launchers as game-specific product slice;
- network/domain filtering;
- screen content analysis;
- message content capture;
- full install/purchase approval product flow.

Native games may still be detected as apps at the evidence layer. Game-specific
classification, launchers, play time, game ratings, UGC/multiplayer, purchases,
and game rules should route to a future `docs/plans/game-plan/` or the existing
app/game source docs until that folder exists.

## Existing Repo State

The repo already has:

- app/game session contracts and read-model proof;
- package/process identity direction;
- scoped owned-process time-limit proof;
- V0.8 runtime read models for app time-limit and scoped owned-process states;
- policy-dispatch proof with parent actor, target device, schedule, evidence
  refs, timer state, approval state, and audit refs;
- supported-adapter runtime proof narrowed to Windows owned-process time-limit;
- broad installed-app blocking still manual-required.

The existing architecture already lists the important evidence sources:

- process/window evidence;
- installed app inventory;
- Microsoft Store/UWP package inventory;
- executable metadata/signature/hash;
- known install folders;
- Start Menu shortcuts;
- launcher/library hints;
- session summaries;
- policy/enforcement handoff.

The new app plan should not replace that. It should become the organized
implementation lane for all apps except browser/web apps and game-specific
native game scope.

## Authority-Tier Model

Platform control is not binary. Every platform capability must name its
authority tier:

```text
observe-only
user-approved-helper
accessibility-assisted
managed-profile
device-owner
mdm-enrolled
supervised-device
system-extension
root-or-admin-service
kiosk-or-single-app
manual-required
not-claimed
```

Every app-control claim must answer:

- Can observe, under which tier?
- Can detect foreground, under which tier?
- Can warn or ask, under which tier?
- Can count time, under which tier?
- Can terminate, hide, suspend, shield, block launch, or allowlist, under which
  tier?
- Can survive reboot or tamper, under which tier?
- What exact proof moves it from manual-required to claimed?

## Evidence Model

### App Identity

Do not use display name as identity by itself. Identity can come from:

- package id;
- bundle id;
- AppUserModelId;
- desktop entry id;
- application token reference;
- executable path reference;
- publisher signature reference;
- file hash reference;
- display name;
- parent label.

Identity rows must include strength, confidence, source evidence refs, and
reason codes.

### App Inventory Evidence

Inventory can show that an app appears installed or detectable. It does not
prove the child used it, that it is currently running, that it is safe, or that
it can be blocked.

Inventory sources include:

- Windows uninstall registry;
- Windows Start Menu shortcut;
- Windows Store/UWP/AppX package;
- known install path;
- executable metadata;
- macOS app bundle;
- Linux desktop entry;
- Linux package manager;
- Flatpak;
- Snap;
- Android package manager;
- iOS FamilyControls/Screen Time token or MDM installed-app query;
- parent catalog.

### Runtime App Evidence

Runtime evidence can come from:

- process snapshot;
- process start;
- process exit;
- foreground window;
- Android usage stats;
- Android usage events;
- iOS DeviceActivity;
- accessibility state;
- managed-device state.

Runtime evidence can prove running and sometimes foreground/activity state. It
does not prove app content, messages, account details, or private documents.

### App Session Summary

Sessions are derived read models backed by raw evidence ids. They are not portal
state and not AI output.

Session summaries must include:

- session id;
- device/local user refs;
- app identity ref where known;
- classification state;
- start/last observed/end timestamps;
- running duration;
- foreground duration;
- background duration;
- observation gap;
- evidence refs;
- policy decision refs where present;
- enforcement result refs where present;
- confidence.

## App Categories

Apps need a broad category model, separate from game categories:

- school;
- productivity;
- browser;
- social;
- messaging;
- video;
- music;
- AI chatbot;
- developer tool;
- creative;
- office;
- email;
- remote desktop;
- VPN/proxy;
- download/torrent;
- file sharing;
- store/installer;
- system;
- security;
- settings;
- unknown.

Rules:

- deterministic catalog first;
- AI only for unknown or ambiguous app classification;
- parent label overrides display, not raw evidence;
- category labels are policy inputs, not automatic decisions.

## App Policy Targets

Supported app policy targets should include:

- specific app identity;
- package id;
- bundle id;
- AppUserModelId;
- desktop entry id;
- executable hash;
- publisher signature;
- category;
- unknown apps;
- newly installed apps;
- portable apps;
- VPN/proxy apps;
- remote desktop apps;
- download/torrent apps;
- AI/chatbot apps;
- all non-system apps.

Supported app policy actions should include:

- allow;
- observe;
- warn;
- ask parent;
- time limit;
- bonus time;
- block launch;
- terminate running;
- hide app;
- suspend app;
- shield app;
- require parent approval;
- manual required;
- unavailable.

## App Control Modes

Mode A: observe only

- Record app inventory and usage.
- No enforcement.
- Parent sees reports.

Mode B: warn

- Child opens restricted app.
- Show warning.
- Continue or ask depending on parent setting.

Mode C: ask parent

- Unknown, new, or restricted app requires approval.
- Child request is created.
- Parent approves once, for session, for schedule, or permanently.

Mode D: time limit

- Track running/foreground duration.
- Warn near limit.
- Terminate, block, hide, or shield only where adapter proof exists.

Mode E: strict allowlist

- Only approved, school, or system apps are allowed.
- Unknown/new apps require parent approval.
- Portable apps are blocked or manual-required where proof exists.

Mode F: risk app guard

- VPN/proxy, remote desktop, torrent/download, unknown installer, and AI/chatbot
  apps receive special rules.

## Risk App Types

High-priority native app risk types:

- VPN, proxy, tunneling, DNS changer, Tor, approved/unapproved mesh VPN;
- remote desktop and screen sharing;
- download manager, torrent, package installer, portable launcher;
- AI chatbot and local LLM apps;
- messaging, social, and video native apps.

The app layer detects installed/running/foreground/duration/known-unknown
states. It must not claim message, feed, video, or content understanding.

## Install And New-App Detection

New app triggers:

- new inventory row;
- new executable observed;
- new package installed;
- new Start Menu shortcut;
- new desktop entry;
- new app bundle;
- new Android package;
- new iOS token or MDM app query row;
- portable executable launched;
- installer/updater ran.

Parent actions:

- allow once;
- allow this app;
- allow this category;
- ask child why;
- block if supported;
- ignore/report only.

Install and purchase product flows remain adjacent scope unless this app plan
only emits evidence and approval handoff refs.

## Enforcement Reality

Windows:

- Level 0: observe/report only.
- Level 1: warn child.
- Level 2: terminate owned/current process.
- Level 3: block launch using AppLocker/App Control proof.
- Level 4: strict allowlist with lab-tested App Control policy.

macOS:

- observe through app bundle/process/session evidence;
- permissioned foreground through Accessibility/NSWorkspace-style paths;
- soft control through warn, ask, overlay, quit/terminate where permitted;
- strong control through MDM/profile payloads, PPPC, system extension, Endpoint
  Security, or launchd service only when proved.

iOS/iPadOS:

- consumer parental path through FamilyControls, DeviceActivity, and
  ManagedSettings tokens/shields;
- managed/school path through MDM, supervised device, App Lock, restrictions,
  and installed-app query;
- no raw Rust daemon or arbitrary process-kill claim.

Android:

- normal mode through UsageStats/UsageEvents and warnings;
- accessibility-assisted mode through opt-in foreground/overlay help;
- strong mode through Device Owner/Profile Owner/DPC package hide, suspend,
  uninstall block, managed configuration, and lock task allowlist proof.

Linux:

- evidence through procfs, desktop entries, package managers, Flatpak/Snap,
  AppImage scans, cgroups, systemd, and desktop/window APIs;
- soft control through warn, ask, and terminate where permitted;
- hard control through cgroups/systemd scopes, AppArmor/SELinux, package
  policy, Flatpak/Snap restrictions, or admin service proof.

## AI And Classifier Boundary

AI may classify unknown apps only from stored evidence/digests. Inputs must be
evidence refs, inventory refs, runtime refs, structured digest fields, parent
rule refs, and custody labels.

AI output must include category candidates, risk signals, recommended policy
input, confidence, uncertainty reason codes, model/runtime refs, prompt/template
version, and analyzed timestamp.

AI classification cannot block directly. Parent policy decides. Enforcement
adapters execute only if capability and authority-tier proof exists.

## Parent UI Requirements

Main native app pages or sections:

- Apps overview;
- Installed apps;
- Running now;
- Foreground now;
- Recent app sessions;
- New/unknown apps;
- Risk apps;
- Approval requests;
- App rules;
- Evidence details;
- Capability/platform status;
- Audit timeline.

The UI must visibly separate inventory, running, foreground, category,
decision, capability, action result, and proof.

## Child UX Requirements

Child-facing copy should be calm and action-oriented:

- "This app is limited by your family rules."
- "This new app needs parent approval before you can use it."
- "Your app time is almost finished."
- "This app is blocked right now. You can ask your parent for more time."

Avoid blame, surveillance language, and AI authority claims.

## Minimum MVP

The first strong app MVP:

- Windows app inventory;
- Windows process/runtime evidence;
- Windows foreground evidence;
- app identity merge;
- unknown app handling;
- session summaries;
- daily app rollups;
- risk app candidates;
- parent app dashboard;
- observe/warn/ask/time-limit policy dry-run;
- scoped owned-process time-limit/terminate proof;
- manual-required broad blocking labels;
- journal/SQLite replay;
- Playwright UI proof;
- security negative tests.

## Done Signal

The app subsystem is credible when a child device records real app
inventory/runtime/foreground evidence, evidence is journaled and replayed into
SQLite, sessions and durations derive from stored evidence, unknown apps stay
unknown until classified with evidence, parent policy can make deterministic
decisions, child UX explains warnings/limits/approval requests, and platform
adapters expose exactly what they can and cannot enforce.
