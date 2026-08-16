# V0.5 Native Apps Test Blueprint

This blueprint defines the test and proof contract for native installed app
work. It is not implementation proof by itself. It exists to stop weak evidence
from being promoted into stronger product claims.

## 1. Core Proof Rule

```text
Installed app inventory proves app presence only.
Runtime evidence proves process/package activity only.
Foreground evidence proves active app focus only.
Session summaries prove duration only from stored evidence.
AI classification is evidence, not authority.
Parent policy decides action.
Platform adapter proof decides whether enforcement is real.
```

Never allow tests, docs, UI, or product copy to upgrade weak evidence into
stronger claims.

Bad claims:

```text
App is installed, so child used it.
Process is running, so child was actively using it.
Foreground app means we know what content was inside it.
AI classified app as risky, so block it directly.
Android normal app can suspend packages.
iOS app can enumerate or kill apps like Windows.
```

Good claims:

```text
App appears installed.
Process was observed running.
App was foreground for 12 minutes.
AI suggested category: vpn_proxy, confidence 0.82.
Policy decision: ask parent.
Enforcement: manual-required on this platform.
```

## 2. Required Test Layers

Every app-plan workpack must use the relevant layers from this set:

```text
Unit tests
Integration tests
Contract tests
Platform adapter fixture tests
Security-negative tests
Persistence/journal/SQLite tests
Policy compiler tests
Enforcement adapter tests
E2E tests
Playwright UI tests
Performance tests
Manual platform proof
CI merge gates
```

Default CI must not require privileged OS controls, MDM, Device Owner, Screen
Time authorization, Endpoint Security, AppLocker, App Control, or real device
enrollment unless the test is explicitly marked manual/platform-required.

## 3. Recommended Test Layout

```text
tests/app/
  unit/
    app_identity_model.test.ts
    app_inventory_model.test.ts
    app_runtime_evidence.test.ts
    app_session_model.test.ts
    app_category_taxonomy.test.ts
    app_policy_target.test.ts
    app_authority_tier.test.ts
    app_ai_classification_contract.test.ts
    app_unknown_handling.test.ts

  integration/
    windows_inventory_adapter.test.ts
    windows_process_runtime_adapter.test.ts
    windows_foreground_adapter.test.ts
    app_identity_merge.test.ts
    app_sessionization.test.ts
    app_journal_sqlite_ingest.test.ts
    app_read_model.test.ts
    app_policy_compile.test.ts
    app_time_budget_dry_run.test.ts
    app_new_unknown_approval_flow.test.ts
    app_risk_category_detection.test.ts

  contract/
    native_app_identity_contract.test.ts
    native_app_inventory_evidence_contract.test.ts
    native_app_runtime_evidence_contract.test.ts
    native_app_session_summary_contract.test.ts
    native_app_policy_target_contract.test.ts
    native_app_policy_decision_contract.test.ts
    native_app_enforcement_result_contract.test.ts
    new_app_approval_request_contract.test.ts
    app_capability_status_contract.test.ts

  security/
    weak_evidence_no_upgrade.test.ts
    app_ai_no_direct_enforcement.test.ts
    path_redaction_security.test.ts
    unknown_app_security.test.ts
    manual_required_enforcement_guard.test.ts
    platform_authority_guard.test.ts
    tamper_and_stale_state.test.ts
    malicious_app_metadata_ui.test.ts

  platform/
    windows/
      windows_inventory_real.manual.test.ts
      windows_process_real.manual.test.ts
      windows_foreground_real.manual.test.ts
      windows_terminate_owned_process.manual.test.ts
      windows_applocker_audit.manual.test.ts
      windows_app_control.manual.test.ts

    macos/
      macos_app_bundle_inventory.manual.test.ts
      macos_nsworkspace_runtime.manual.test.ts
      macos_accessibility_foreground.manual.test.ts
      macos_codesign_identity.manual.test.ts
      macos_launchdaemon_launchagent.manual.test.ts
      macos_pppc_profile.manual.test.ts
      macos_endpoint_security.manual.test.ts
      macos_mdm_installed_app_query.manual.test.ts

    linux/
      linux_desktop_entry_inventory.manual.test.ts
      linux_package_inventory.manual.test.ts
      linux_flatpak_snap_inventory.manual.test.ts
      linux_procfs_runtime.manual.test.ts
      linux_x11_foreground.manual.test.ts
      linux_wayland_capability.manual.test.ts
      linux_systemd_cgroup_enforcement.manual.test.ts
      linux_apparmor_selinux.manual.test.ts

    android/
      android_package_inventory.manual.test.ts
      android_usage_stats_permission.manual.test.ts
      android_usage_events_foreground.manual.test.ts
      android_accessibility_overlay.manual.test.ts
      android_device_owner_provisioning.manual.test.ts
      android_hide_package.manual.test.ts
      android_suspend_package.manual.test.ts
      android_lock_task_allowlist.manual.test.ts

    ios/
      ios_familycontrols_authorization.manual.test.ts
      ios_activity_picker_tokens.manual.test.ts
      ios_device_activity_monitor.manual.test.ts
      ios_managedsettings_shield.manual.test.ts
      ios_mdm_installed_app_query.manual.test.ts
      ios_app_lock_single_app_mode.manual.test.ts

  e2e/
    windows_app_inventory_to_portal.e2e.ts
    windows_app_runtime_session.e2e.ts
    windows_foreground_duration.e2e.ts
    unknown_app_approval.e2e.ts
    risk_app_detection.e2e.ts
    app_time_limit_dry_run.e2e.ts
    app_time_limit_enforced_owned_process.e2e.ts
    manual_required_broad_block.e2e.ts
    journal_replay_after_restart.e2e.ts

playwright/
  app-dashboard.spec.ts
  app-inventory.spec.ts
  app-running-now.spec.ts
  app-session-evidence.spec.ts
  app-unknown-approval.spec.ts
  app-risk-categories.spec.ts
  app-policy-authoring.spec.ts
  app-capability-status.spec.ts
  app-child-request.spec.ts
  app-platform-matrix.spec.ts
```

## 4. Core Invariants

### 4.1 Evidence Invariants

```text
Inventory evidence must not imply use.
Runtime evidence must not imply foreground.
Foreground evidence must not imply content knowledge.
Window title is optional and permission-gated.
Path/hash/signature evidence must be redacted where needed.
Every parent-visible app claim must cite evidence refs.
Every session summary must cite raw evidence refs.
Unknown apps must remain unknown until evidence/classifier confidence improves.
```

### 4.2 Session Invariants

```text
Running duration comes from process/package evidence.
Foreground duration comes from foreground evidence only.
Portal refresh interval must not create duration.
AI must not invent duration.
Restart/replay must reconstruct the same summary.
Stale gaps must be explicit.
Process exit must close session if observed.
Long observation gaps must close or degrade session.
```

The existing architecture already says running time must come from process
observations, foreground time only when foreground/window evidence proves active
focus, and session summaries must carry source evidence IDs.

### 4.3 Policy Invariants

```text
Policy consumes evidence/session summaries.
Policy does not scan OS.
Policy does not run AI.
Policy can output observe/warn/ask/limit/block/manual-required.
Policy action must include policyDecisionId, target refs, evidence refs,
schedule, actor, and audit ref.
Dry-run must not enforce.
Manual-required must not execute enforcement adapter.
Unavailable must not execute enforcement adapter.
```

### 4.4 Enforcement Invariants

```text
Terminate current owned process is not broad app blocking.
Time-limit proof is not install/package blocking proof.
AppLocker/App Control proof is Windows-specific.
Android package suspend/hide requires Device Owner/Profile Owner/delegated capability.
iOS shielding requires FamilyControls/ManagedSettings authorization.
macOS hard block requires MDM/profile/Endpoint Security/System Extension proof.
Linux hard block is distro/mechanism-specific.
```

### 4.5 AI Invariants

```text
AI consumes stored evidence/digest only.
AI does not enumerate processes.
AI does not scan filesystem.
AI does not count time.
AI does not enforce.
AI output must include evidence refs, confidence, uncertainty, model/runtime,
prompt version, and fallback.
Invalid AI output is rejected.
```

The repo already sets this boundary: AI may consume stored evidence references,
app/game session digest, unknown/ambiguous candidate digest, and parent rule
context. It may classify, but parent-controlled rules decide what to do.

## 5. Unit Tests

### 5.1 App Identity Model

Test:

- identity can represent Windows executable path;
- identity can represent Windows AppUserModelId;
- identity can represent Android package id;
- identity can represent macOS bundle id;
- identity can represent Linux desktop entry id;
- identity can represent iOS Screen Time token ref;
- display name alone produces weak identity;
- same display name with different hash does not merge;
- same package id with different user profile is scoped correctly;
- publisher signature improves confidence;
- file hash improves confidence;
- parent label changes display only, not raw identity.

Proof: no platform uses display name as the only durable app identity.

### 5.2 Inventory Evidence

Test:

- inventory row requires `evidenceId`, `sourceId`, `adapterId`, and
  `observedAt`;
- inventory row supports installed, possibly_installed, stale,
  permission_limited, unsupported, and adapter_error states;
- inventory evidence cannot set `running=true`;
- inventory evidence cannot set `foreground=true`;
- inventory source is preserved.

Proof: installed app catalog cannot be confused with current usage.

### 5.3 Runtime Evidence

Test:

- process snapshot, process start, and process exit create runtime evidence;
- foreground window creates foreground runtime evidence;
- background process does not create foreground evidence;
- runtime evidence can exist without inventory match;
- runtime evidence can be `unknown_process`;
- window title can be omitted;
- permission-limited foreground state is preserved;
- adapter_error state is preserved.

Proof: running/foreground state is evidence-backed and can represent missing
permission.

### 5.4 App Session Model

Test:

- session starts on first process observation;
- session continues within gap window;
- session closes on process exit or stale timeout;
- running duration increments from process observations;
- foreground duration increments only from foreground observations;
- background duration derives from running minus foreground where valid;
- foreground duration never exceeds running duration;
- title changes do not create a new session alone;
- restart/replay reconstructs the same session.

Proof: session time is deterministic and replayable.

### 5.5 App Category Taxonomy

Test categories:

```text
school
productivity
social
messaging
video
music
ai_chatbot
vpn_proxy
remote_desktop
download_torrent
system
unknown
```

Also test that category candidates require source and confidence, and
confidence is in `0..1`.

Proof: parent rules can target real app categories without mixing games or
browser pages into native app semantics.

### 5.6 Authority Tier

Test:

- observe-only cannot block;
- user-approved-helper can warn/ask if UI exists;
- accessibility-assisted can overlay only where the platform supports it;
- device-owner can hide/suspend only on Android;
- managed-profile can support managed controls where the platform supports it;
- supervised-device is required for stronger iOS controls;
- system-extension is required for macOS exec authorization claims;
- root/admin-service is required for privileged desktop control;
- manual-required cannot execute adapter;
- not-claimed cannot execute adapter.

Proof: capability is tied to authority tier, not product copy.

### 5.7 App Policy Target

Test:

- `specific_app` target requires app identity ref;
- `package_id` target requires package id;
- `bundle_id` target requires bundle id;
- `app_user_model_id` target requires AppUserModelId;
- `desktop_entry_id` target requires desktop entry id;
- `executable_hash` target requires hash ref;
- `publisher` target requires signature ref;
- `category` target requires category;
- `unknown_apps` target compiles from unknown state;
- `newly_installed_apps` target compiles from inventory delta;
- `portable_apps` target requires portable evidence;
- `vpn_proxy_apps` target requires risk/category candidate;
- `all_non_system_apps` excludes system apps.

Proof: policy cannot target fields that the platform has not proved.

### 5.8 AI Classification Contract

Test:

- valid app classification result accepted;
- missing evidence refs rejected;
- confidence outside `0..1` rejected;
- missing model/runtime ref rejected;
- missing prompt template version rejected;
- invalid category rejected;
- block action in AI output rejected;
- duration field in AI output rejected;
- raw process scan result in AI output rejected.

Proof: AI output stays classification/evidence only.

## 6. Integration Tests

### 6.1 Windows Inventory Adapter

Fixture inputs:

```text
registry uninstall fixtures
Start Menu shortcut fixtures
AppX/MSIX package fixtures
known path fixtures
executable metadata fixtures
signature/hash fixtures
```

Test:

- detects registry-installed app;
- detects Start Menu app;
- detects Microsoft Store app;
- deduplicates same app from registry and shortcut;
- keeps separate apps with same display name;
- records publisher/signature/hash when present;
- marks incomplete records permission_limited or weak;
- does not mark app as used.

Proof: Windows installed app inventory is useful but not overclaimed.

### 6.2 Windows Runtime Adapter

Use process snapshot fixtures.

Test:

- process appears -> runtime evidence;
- same process persists -> session continues;
- process exits -> session closes;
- new pid same executable after gap -> new session or continuation by rule;
- unknown process creates unknown_process evidence;
- unavailable path maps permission_limited;
- unavailable publisher/hash lowers confidence.

Proof: process/runtime evidence survives real-world partial metadata.

### 6.3 Windows Foreground Adapter

Use foreground snapshot fixtures.

Test:

- foreground app updates foreground session;
- background apps do not gain foreground time;
- foreground switch closes previous foreground interval;
- permission denied maps permission_limited;
- title disabled maps title omitted;
- title enabled stores permitted title/ref.

Proof: foreground duration is not guessed from the running process list.

### 6.4 App Identity Merge

Test:

- registry app and running process merge by executable path/hash;
- Store package and process merge by AppUserModelId/package family;
- shortcut and process merge by target path;
- same display name alone does not merge;
- same publisher alone does not merge;
- parent manual label does not merge records;
- AI category does not merge identities.

Proof: app identity is evidence-based.

### 6.5 Journal And SQLite Ingest

Test:

- inventory evidence writes to journal;
- runtime evidence writes to journal;
- foreground evidence writes to journal;
- SQLite replay creates inventory, running-now, foreground-now, and daily rollup
  read models;
- restart/replay produces the same session summary;
- invalid evidence is rejected before SQLite.

Proof: portal and policy read from replayable local evidence, not memory-only
state.

### 6.6 Policy Compile

Test:

- observe policy compiles for all evidence states;
- warn policy requires child-facing warning capability;
- ask_parent policy creates approval request;
- time_limit policy requires session summary;
- terminate_running requires current target process proof;
- block_launch returns manual_required unless platform proof exists;
- Android suspend requires Device Owner/Profile Owner capability;
- iOS shield requires ManagedSettings capability;
- macOS exec deny requires Endpoint Security/System Extension capability;
- Linux cgroup/app block requires platform mechanism proof.

Proof: policy cannot compile into impossible enforcement.

### 6.7 New And Unknown App Approval

Test:

- new inventory app creates new_app_detected candidate;
- new runtime unknown process creates new_app_launched candidate;
- portable executable creates portable_app_launched candidate;
- installer process creates installer_detected candidate;
- parent approval request includes evidence refs;
- parent allow once expires;
- parent allow app persists;
- parent block remains manual_required if no adapter proof;
- approval survives restart.

Proof: new apps can be reviewed without pretending every platform can hard block
them.

### 6.8 Risk App Detection

Test:

- known VPN app classified vpn_proxy;
- known remote desktop app classified remote_desktop;
- known torrent app classified download_torrent;
- known AI chatbot app classified ai_chatbot;
- unknown executable with VPN-like name becomes candidate, not fact;
- unknown publisher lowers confidence;
- parent label can override category display;
- risk category creates policy candidate.

Proof: risk apps are first-class, explainable, and confidence-bounded.

## 7. Contract Tests

### 7.1 NativeAppIdentity

Required:

```text
appIdentityId
platform
identityFields
identityStrength
sourceEvidenceRefs
confidence
```

Forbidden:

```text
raw unredacted private path in portal DTO unless explicitly allowed
credentials
tokens
private app data
```

Test valid Windows, Android, macOS, and iOS identities; reject missing evidence
refs and out-of-range confidence; mark display-name-only identity weak.

### 7.2 NativeAppInventoryEvidence

Required:

```text
evidenceId
observedAt
sourceId
adapterId
deviceId
inventorySource
appIdentity
installedState
confidence
reasonCodes
```

Test valid registry, macOS app bundle, Android package, and iOS token inventory;
reject runtime fields, foreground fields, and invalid sources.

### 7.3 NativeAppRuntimeEvidence

Required:

```text
evidenceId
observedAt
sourceId
adapterId
deviceId
runtimeSource
foregroundState
confidence
reasonCodes
```

Test valid process snapshot, foreground window evidence, Android usage event,
and iOS device activity event; reject inventory-only fields and
content/message fields.

### 7.4 NativeAppSessionSummary

Required:

```text
sessionId
deviceId
classificationState
startedAt
lastObservedAt
runningDurationMs
foregroundDurationMs
backgroundDurationMs
sourceEvidenceRefs
confidence
```

Test valid known_app and unknown_process sessions; reject foreground duration
greater than running duration, missing evidence refs, negative duration, and
AI-created duration.

### 7.5 NativeAppPolicyDecision

Required:

```text
policyDecisionId
actorRef
targetDeviceRef
targetRefs
sourceEvidenceRefs
scheduleRef
decision
enforcementMode
auditRef
```

Test observe, warn, ask_parent, time_limit, manual_required, and dry-run states.
Reject or degrade block_launch without capability.

### 7.6 NativeAppEnforcementResult

Required:

```text
enforcementResultId
policyDecisionId
targetRefs
action
result
attemptedAt
adapterCapabilityRef
auditRef
```

Allowed results include:

```text
notEnabled
observeOnly
warned
askParentCreated
timeLimitStarted
timeLimitReached
terminated
alreadyExited
permissionLimited
targetChanged
failed
blockedLaunch
manualRequired
unavailable
```

Test that terminated requires a target process ref, blockedLaunch requires a
platform proof ref, manualRequired cannot include `executed=true`, and
unavailable cannot include `success=true`.

## 8. Security-Negative Tests

### 8.1 Weak Evidence No-Upgrade

Test:

- installed app does not become used app;
- running app does not become foreground app;
- foreground app does not become content analysis;
- window title does not become message/content proof;
- AI category does not become deterministic app identity;
- unknown app does not become known app from weak name.

### 8.2 Path And Metadata Safety

Test:

- raw user home path redacted in portal DTO;
- executable path stored as pathRef where required;
- malicious display name escaped;
- script tag in app name escaped;
- very long app name truncates safely;
- invalid UTF-8 handled;
- path traversal in shortcut target rejected/degraded;
- symlink/junction escaping monitored root flagged.

### 8.3 Manual-Required Enforcement Guard

Test:

- manual_required block_launch cannot call adapter;
- unavailable action cannot call adapter;
- dry_run cannot terminate process;
- Android suspend without Device Owner cannot execute;
- iOS shield without authorization cannot execute;
- macOS endpoint block without entitlement cannot execute;
- Linux cgroup block without proof cannot execute.

### 8.4 Tamper And Stale State

Test:

- stale process evidence not shown as running;
- stale foreground evidence not shown as current foreground;
- agent restart marks gaps;
- device sleep creates observation gap;
- clock skew handled;
- reused process id does not corrupt old session;
- policy decision with stale evidence rejected;
- wrong device decision rejected;
- wrong local user decision rejected.

## 9. Manual Platform Proof Tests

Manual/platform-specific proof must generate artifacts under:

```text
output/app-plan-proof/<platform>/<capability>/
```

### 9.1 Windows Proof Matrix

Required proofs:

```text
installed app inventory from registry
installed app inventory from Start Menu shortcut
installed app inventory from Microsoft Store package
process runtime evidence
foreground window evidence
session duration
foreground duration
new unknown portable executable detection
owned-process time-limit dry-run
owned-process terminate proof
broad blocking manual-required label
AppLocker audit-only proof
AppLocker enforce proof only if lab-safe
App Control proof only if lab-safe
rollback proof
```

Artifact examples:

```text
output/app-plan-proof/windows/inventory-registry.json
output/app-plan-proof/windows/runtime-process.json
output/app-plan-proof/windows/foreground-window.json
output/app-plan-proof/windows/session-rollup.json
output/app-plan-proof/windows/owned-process-terminate.json
output/app-plan-proof/windows/applocker-audit.md
output/app-plan-proof/windows/manual-required-broad-block.png
```

### 9.2 macOS Proof Matrix

Required proofs:

```text
.app bundle inventory from /Applications
user app inventory from ~/Applications
Info.plist bundle id/name/version extraction
code signature/team id extraction
NSWorkspace running app list
NSWorkspace launch/terminate/activate events
Accessibility foreground/window proof
permission denied -> permission_limited proof
LaunchAgent proof
LaunchDaemon/helper proof if used
PPPC profile proof if used
MDM installed-app query proof if used
Parental Controls/restriction payload proof if used
Endpoint Security observe proof if used
Endpoint Security block/auth proof only if entitlement/lab proof exists
```

Artifact examples:

```text
output/app-plan-proof/macos/app-bundle-inventory.json
output/app-plan-proof/macos/nsworkspace-running.json
output/app-plan-proof/macos/accessibility-foreground.json
output/app-plan-proof/macos/codesign-identity.json
output/app-plan-proof/macos/pppc-profile-proof.md
output/app-plan-proof/macos/mdm-installed-app-query.md
output/app-plan-proof/macos/endpoint-security-manual-required.md
```

Hard gate: no macOS hard-block claim without Endpoint Security, MDM, or profile
proof.

### 9.3 Linux Proof Matrix

Required proofs:

```text
.desktop inventory
dpkg/rpm/pacman inventory depending distro
Flatpak inventory
Snap inventory
AppImage/portable detection
procfs process evidence
X11 foreground evidence
Wayland unsupported/permission-limited proof
systemd service proof
process-tree terminate proof
cgroup/systemd scope proof if used
AppArmor/SELinux proof if used
Flatpak/Snap restriction proof if used
```

Artifact examples:

```text
output/app-plan-proof/linux/desktop-entry-inventory.json
output/app-plan-proof/linux/package-inventory.json
output/app-plan-proof/linux/flatpak-inventory.json
output/app-plan-proof/linux/procfs-runtime.json
output/app-plan-proof/linux/x11-foreground.json
output/app-plan-proof/linux/wayland-manual-required.md
output/app-plan-proof/linux/systemd-cgroup-proof.md
```

Hard gate: no universal Linux blocking claim. Every Linux block claim must name
distro, session type, and mechanism.

### 9.4 Android Proof Matrix

Required proofs:

```text
package inventory with package visibility notes
UsageStats permission grant UX
queryUsageStats proof
queryEvents proof
foreground/session reconstruction from usage events
permission denied state
Accessibility service enabled proof if used
Accessibility overlay/warn proof if used
Device Owner provisioning proof
Profile Owner/managed profile proof
setApplicationHidden proof
setPackagesSuspended proof
setUninstallBlocked proof
Lock Task allowlist proof
normal-app mode cannot hide/suspend proof
```

Artifact examples:

```text
output/app-plan-proof/android/package-inventory.json
output/app-plan-proof/android/usage-stats-events.json
output/app-plan-proof/android/accessibility-overlay.png
output/app-plan-proof/android/device-owner-provisioning.md
output/app-plan-proof/android/package-hidden-result.json
output/app-plan-proof/android/package-suspended-result.json
output/app-plan-proof/android/normal-mode-manual-required.json
```

Hard gate: no Android app block/hide/suspend claim without Device Owner, Profile
Owner, or delegation proof.

### 9.5 iOS/iPadOS Proof Matrix

Required proofs:

```text
FamilyControls authorization UX
FamilyActivityPicker app/category token selection
DeviceActivity schedule monitor proof
DeviceActivity threshold/event proof
ManagedSettings shield proof
shield removed/exception proof
app/category token read-model proof
MDM installed-app query proof if managed
supervised device restriction proof if used
App Lock / Single App Mode proof if used
normal iOS process scanning not-claimed proof
```

Artifact examples:

```text
output/app-plan-proof/ios/familycontrols-authorization.png
output/app-plan-proof/ios/activity-picker-token-selection.png
output/app-plan-proof/ios/device-activity-monitor.json
output/app-plan-proof/ios/managedsettings-shield.png
output/app-plan-proof/ios/mdm-installed-app-query.md
output/app-plan-proof/ios/app-lock-single-app-mode.md
output/app-plan-proof/ios/no-process-scanning-claim.md
```

Hard gate: no iOS process enumeration/kill claim. iOS app control must be Screen
Time, ManagedSettings, MDM, or App Lock based.

## 10. E2E Tests

### 10.1 Windows App Inventory To Portal

Scenario: agent scans installed apps, journal writes inventory evidence, SQLite
read model updates, and portal displays installed apps.

Expected: apps appear as inventory, not usage; evidence refs and source/custody
are visible; unknown/permission-limited states are visible.

### 10.2 Windows Runtime Session

Scenario: start Notepad or test app, observe process and foreground state, record
duration, close app, then replay journal.

Expected: session starts, foreground time increments only while active, session
closes on exit, journal replay reconstructs session, and portal shows duration
with evidence refs.

### 10.3 Unknown App Approval

Scenario: launch unknown portable test executable, record unknown_process, apply
unknown-apps ask-parent policy, create approval request, and parent allows once.

Expected: unknown remains unknown, approval request has evidence refs, allow once
expires, and audit is recorded.

### 10.4 Risk App Detection

Scenario: launch known VPN/proxy/remote/torrent test fixture, classifier marks
risk category, policy says ask parent.

Expected: risk category shown with confidence, policy decision refs shown, and
no content inspection claim.

### 10.5 Time-Limit Dry-Run

Scenario: app session reaches configured limit while policy is in dry-run mode.

Expected: would-limit decision recorded, no process terminated, portal shows
dry-run, and child UX is not interrupted unless warn dry-run is explicitly
configured.

### 10.6 Owned-Process Enforcement

Scenario: launch Ocentra-owned test app and apply terminate-after-limit policy
with adapter proof enabled.

Expected: terminate action attempted, result is terminated/alreadyExited/failed,
rollback/audit refs recorded, and portal shows action proof.

### 10.7 Broad Blocking Manual-Required

Scenario: parent configures block_launch for arbitrary app with no platform
app-control proof.

Expected: decision is manual_required, no adapter is called, and portal explains
that block launch requires AppLocker/App Control or platform setup.

## 11. Playwright UI Tests

### 11.1 App Dashboard

States:

```text
empty state
installed apps visible
running now visible
foreground now visible
recent sessions visible
unknown apps visible
risk apps visible
manual-required states visible
```

Assertions:

```text
inventory not labeled as usage
running not labeled as foreground
foreground duration has evidence link
manual-required displayed honestly
```

### 11.2 App Evidence Drawer

Must show:

```text
identity fields
inventory evidence
runtime evidence
foreground evidence
session summary
source/custody
confidence
reason codes
policy decisions
enforcement results
```

### 11.3 New App Approval UI

Test unknown approval request, allow once, allow always, block/manual-required,
ask-child-why, and request expiry.

### 11.4 Risk App UI

Test categories:

```text
VPN/proxy
remote desktop
torrent/download
AI chatbot
messaging/social native app
unknown installer
```

Assertions: risk explanation, confidence, and source evidence are visible, and
no content claim is shown.

### 11.5 Platform Matrix UI

Must show per platform:

```text
observe available
foreground permission required
warn available
time-limit available
terminate available
block launch manual-required
strong control authority required
```

Specific labels:

```text
Windows: AppLocker/App Control proof required for broad blocking.
macOS: MDM/PPPC/Endpoint Security proof required for hard block.
Linux: distro/mechanism-specific proof required.
Android: Device Owner/Profile Owner required for hide/suspend.
iOS: FamilyControls/ManagedSettings/MDM required.
```

### 11.6 Malicious Metadata UI

Fixtures:

```text
app name with script tag
very long app name
unicode/punycode-like app name
invalid UTF-8 app metadata
path-looking display name
fake system app name
```

Assertions: no XSS, long names truncate, fake system app is not auto-trusted,
and raw private paths are not exposed.

## 12. Required Fixtures

### 12.1 App Inventory Fixtures

```text
fixtures/app/inventory/windows_registry_apps.json
fixtures/app/inventory/windows_start_menu_shortcuts.json
fixtures/app/inventory/windows_store_packages.json
fixtures/app/inventory/macos_app_bundles.json
fixtures/app/inventory/linux_desktop_entries.json
fixtures/app/inventory/linux_flatpak_apps.json
fixtures/app/inventory/android_packages.json
fixtures/app/inventory/ios_activity_tokens.json
```

### 12.2 Runtime Fixtures

```text
fixtures/app/runtime/windows_process_snapshot.json
fixtures/app/runtime/windows_process_start_exit.json
fixtures/app/runtime/windows_foreground_window.json
fixtures/app/runtime/macos_nsworkspace_running.json
fixtures/app/runtime/linux_procfs_snapshot.json
fixtures/app/runtime/android_usage_events.json
fixtures/app/runtime/ios_device_activity.json
```

### 12.3 Session Fixtures

```text
fixtures/app/sessions/simple_foreground_session.json
fixtures/app/sessions/background_only_session.json
fixtures/app/sessions/process_exit_session.json
fixtures/app/sessions/stale_gap_session.json
fixtures/app/sessions/replayed_session.json
fixtures/app/sessions/unknown_process_session.json
```

### 12.4 Policy Fixtures

```text
fixtures/app/policy/observe_all_apps.json
fixtures/app/policy/warn_social_apps.json
fixtures/app/policy/ask_unknown_apps.json
fixtures/app/policy/time_limit_ai_apps.json
fixtures/app/policy/block_vpn_manual_required.json
fixtures/app/policy/android_suspend_device_owner_required.json
fixtures/app/policy/ios_shield_managedsettings_required.json
fixtures/app/policy/macos_endpoint_security_required.json
```

### 12.5 UI Fixtures

```text
fixtures/app/ui/app_dashboard_empty.json
fixtures/app/ui/app_dashboard_mixed.json
fixtures/app/ui/app_unknown_approval.json
fixtures/app/ui/app_risk_vpn.json
fixtures/app/ui/app_manual_required.json
fixtures/app/ui/app_platform_matrix.json
fixtures/app/ui/app_malicious_metadata.json
```

## 13. Performance Tests

Targets:

```text
inventory normalize 1,000 apps under target threshold
runtime process snapshot normalize 500 processes under target threshold
session replay 100,000 observations under acceptable threshold
portal render 500 apps without freezing
policy compile 1,000 app rules under acceptable threshold
journal write latency bounded
SQLite daily rollup query bounded
```

Performance proof:

```text
output/app-plan-proof/performance/inventory-1000.log
output/app-plan-proof/performance/runtime-500.log
output/app-plan-proof/performance/replay-100k.log
output/app-plan-proof/performance/portal-500-screenshot.png
```

## 14. CI Gates

Minimum gates:

```bash
pnpm lint
pnpm test
pnpm test:contracts
pnpm test:app
pnpm playwright test app-dashboard.spec.ts
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

App-specific gates:

```text
contract tests pass
evidence no-upgrade tests pass
manual-required guard tests pass
journal/SQLite replay tests pass
policy dry-run tests pass
security-negative tests pass
Playwright mocked UI tests pass
```

Manual tests must be tagged:

```text
@manual
@requires-windows
@requires-macos
@requires-linux
@requires-android-device-owner
@requires-ios-familycontrols
@requires-mdm
@requires-endpoint-security
@requires-applocker
@requires-app-control
```

No ignored/manual test is allowed without reason.

## 15. Merge-Blocking Failures

Block merge if:

```text
inventory evidence is displayed as app usage
running evidence is displayed as foreground usage
foreground evidence is displayed as content knowledge
AI output can directly enforce
dry-run terminates or blocks app
manual-required action calls adapter
Android normal mode claims package suspend/hide
iOS claims process scanning/killing
macOS hard block claimed without entitlement/profile proof
Linux universal block claimed without mechanism/distro proof
session duration changes after journal replay
portal hides stale/unsupported/permission-limited state
raw private executable paths leak into parent UI
malicious app metadata causes XSS
```

## 16. Required Proof Pack

Each workpack proof root:

```text
output/app-plan-proof/<workpack-id>/
```

Required files:

```text
00-source-snapshot.md
01-contract-proof.log
02-rust-protocol-proof.log
03-runtime-evidence.json
04-journal-sqlite-proof.json
05-policy-action-proof.json
06-ui-snapshots/
07-playwright-ui-proof.log
08-security-negative-proof.log
09-manual-platform-proof.md
10-validation-commands.log
```

For platform workpacks, add:

```text
11-authority-tier-proof.md
12-permission-setup-proof.md
13-rollback-proof.md
```

## 17. Workpack Done Signal

A workpack is done only when:

```text
typed contracts exist
runtime behavior exists or manual-required state exists
journal/read-model path exists if evidence-facing
portal/UI state exists if parent-facing
unit tests exist
integration tests exist
contract tests exist
security-negative tests exist
Playwright tests exist if UI-facing
manual platform proof exists if platform claim is stronger than observe-only
docs/checklists updated
```

Example done criteria for Android Device Owner package suspend:

```text
Device Owner authority tier contract exists.
package suspend capability status exists.
normal-app mode returns manual_required.
Device Owner proof artifact exists.
suspend result proof exists.
rollback/unsuspend proof exists.
UI shows Device Owner required when missing.
tests prove no suspend call in normal mode.
```

## 18. Platform-Specific No-Claim Gates

Windows:

```text
Terminate proof is not broad block proof.
AppLocker audit proof is not AppLocker enforce proof.
App Control unavailable must show manual-required.
System app allowlist rollback must be proved before strict mode.
```

macOS:

```text
Accessibility foreground proof is not app block proof.
Quit/terminate proof is not launch block proof.
LaunchDaemon persistence is not Endpoint Security proof.
MDM profile installed is not Endpoint Security proof.
Endpoint Security observe is not Endpoint Security deny/auth proof.
```

Linux:

```text
procfs process proof is not foreground proof.
X11 foreground proof is not Wayland proof.
process kill proof is not broad app block proof.
cgroup proof is distro/mechanism-specific.
Flatpak inventory proof is not Snap proof.
```

Android:

```text
UsageStats proof is not foreground real-time block proof.
Accessibility overlay proof is not package suspend proof.
Device Owner provisioning proof is required for hide/suspend claims.
Profile Owner proof is scoped to managed profile.
Lock Task proof is kiosk/allowlist-specific.
```

iOS:

```text
FamilyControls authorization is not installed app inventory.
Activity token selection is not raw app list.
DeviceActivity monitoring is not arbitrary process monitoring.
ManagedSettings shield proof is required for blocking.
MDM installed-app query is managed-device-only.
App Lock is supervised/managed/kiosk-specific.
```

## 19. Minimum Serious MVP Test Set

Do not go below this:

```text
Unit:
- app identity
- inventory evidence
- runtime evidence
- foreground evidence
- session model
- category taxonomy
- policy target compiler
- authority tier
- AI no-direct-enforcement

Integration:
- Windows inventory fixtures
- Windows process fixtures
- Windows foreground fixtures
- identity merge
- sessionization
- journal/SQLite replay
- policy dry-run
- unknown app approval
- risk app detection

Contract:
- NativeAppIdentity
- NativeAppInventoryEvidence
- NativeAppRuntimeEvidence
- NativeAppSessionSummary
- NativeAppPolicyDecision
- NativeAppEnforcementResult
- NewAppApprovalRequest
- AppCapabilityStatus

Security:
- weak evidence no-upgrade
- manual-required guard
- platform authority guard
- path redaction
- malicious metadata escaping
- stale evidence rejection

E2E:
- Windows app inventory to portal
- Windows runtime session
- foreground duration
- unknown app approval
- risk app detection
- time-limit dry-run
- owned-process enforcement where already scoped
- broad block manual-required

Playwright:
- app dashboard
- inventory details
- running/foreground states
- evidence drawer
- unknown approval
- risk categories
- policy authoring
- platform matrix
- manual-required labels
```

## 20. Worker Instruction Template

```text
Build the V0.5 Native Apps test/proof suite.

Use docs/features/app-game-control.md, docs/expectations/app-game-evidence.md,
docs/architecture/app-game-evidence-sessions.md,
docs/app-control-settings-inventory.md, and docs/plans/app-plan as source
inputs.

Rules:
- Apps plan covers native/installed apps, not browser pages and not
  game-specific product semantics.
- Installed inventory is evidence only, not current use.
- Runtime process/package evidence is activity only, not foreground.
- Foreground evidence is active app only, not content.
- Session duration must replay from journal/SQLite evidence.
- AI may classify unknown apps only from stored evidence/digests and cannot
  enforce.
- Parent policy is the action authority.
- Platform adapters must expose authority tier and capability status.
- Manual-required/unavailable states must never call enforcement adapters.
- Android hide/suspend requires Device Owner/Profile Owner/delegation proof.
- iOS app shielding requires FamilyControls/ManagedSettings proof.
- macOS hard app blocking requires MDM/Endpoint Security/System Extension proof.
- Linux blocking must name mechanism and distro/session proof.
- Windows broad blocking needs AppLocker/App Control proof and rollback proof.

Required proof:
- Unit, integration, contract, security, E2E, Playwright, performance, and
  manual-platform proof matrices.
- Fixture-backed tests for inventory, runtime, foreground, sessions, policy,
  enforcement, UI, malicious metadata, and stale/manual-required states.
- Proof artifacts under output/app-plan-proof/<workpack-id>/.
```

## 21. Final Quality Bar

The native apps subsystem is solid only when:

```text
A parent can see installed apps without mistaking inventory for use.
A parent can see running apps without mistaking running for foreground.
A parent can see foreground duration backed by evidence.
Unknown apps remain unknown until classified with evidence.
Risk apps are first-class, confidence-scored candidates.
App sessions replay from journal and SQLite.
Policy decisions cite evidence, schedule, actor, and target.
Child app warnings/requests are calm and auditable.
Enforcement only runs where platform authority is proved.
Every platform limitation is shown as capability status, not hidden.
```

Final rule:

```text
Do not use bare "unsupported" as a product claim.
Say observe-only, permission-required, managed-device-required,
admin/root-required, system-extension-required, supervised-device-required,
manual-required, or not-claimed, with exact proof needed to move up.
```
