<!-- agent-capsule -->

> Agent Capsule
> Doc: Cross-Platform Deliverables Checkpoint
> Kind: architecture/reference documentation; read only when selected by plan route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Cross-Platform Deliverables Checkpoint

This checkpoint defines the manual and CI plan that must run before the next AI
or enforcement slice. It turns the platform-deliverables expectations into a
reviewable checklist without claiming support that the current code cannot
prove.

## Scope

The checkpoint covers current V0.7 dry-run and evidence-preview behavior only:

- shared TypeScript contracts, Rust crates, journal, SQLite, protocol, and
  local transport portability;
- Windows local child-device proof on a real PC;
- Linux CI plus WSL or Docker proof for build, package, service, and runtime
  mechanics;
- macOS package, launch, permission, and service-manager proof on the Mac
  system;
- Android emulator and physical-device proof for package, launch, local store,
  foreground service, and permission state;
- iOS simulator, TestFlight, signing, and entitlement notes for parent and
  child-device claims;
- LAN parent-to-child checks with explicit pairing and origin controls;
- installer, autostart, uninstall, update, and reboot-survival checks;
- honest unavailable, disabled, degraded, permission-required, manual-required,
  blocked, or scaffold-only states per platform.

The checkpoint does not implement V0.8 enforcement, blocking, timers, app
control, notification delivery, cloud relay, mobile device-owner policy, iOS
Family Controls, or new AI model behavior. If a capability is not implemented,
the proof must show an honest status rather than a success-shaped placeholder.

## Inputs

Run the checkpoint from a reviewed branch or current `main` after the V0.7
preview-completion merges.

Required source references:

- `docs/product-roadmap.md`
- `docs/expectations/platform-deliverables.md`
- `docs/expectations/platforms.md`
- `docs/expectations/real-evidence-proof.md`
- `docs/expectations/pre-ai-proof-matrix.json`
- `docs/architecture/platform-capabilities.md`
- `docs/architecture/validation-gates.md`

Before recording results, capture:

- commit SHA and branch;
- package or app version;
- OS name, version, architecture, and device model where applicable;
- install path and data/cache path used by the app;
- permission state before the test;
- selected ports for local and LAN service checks;
- whether the run is CI, local manual, emulator, simulator, or physical device.

## Proof State Labels

Use these labels in checkpoint notes, PR bodies, and proof matrix updates:

| Label                 | Meaning                                                                                                                               |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| `implemented`         | The real product path works on the named platform with current code and proof.                                                        |
| `ci-mechanical-proof` | Hosted CI proves build, test, package, install, launch, or transport mechanics, but not privileged OS behavior.                       |
| `manual-required`     | A real host, device, permission grant, network, signing credential, or store capability is required before the claim can be complete. |
| `permission-required` | The OS permission, entitlement, or elevation is missing, denied, or waiting for review.                                               |
| `disabled`            | Parent, system, or build configuration intentionally disables the capability.                                                         |
| `degraded`            | The product reports a reduced capability state and says what fidelity is missing.                                                     |
| `unavailable`         | The capability is not available on this platform or in this environment.                                                              |
| `scaffold-only`       | The repo has package, app, API, or contract structure, but product behavior is not proven.                                            |
| `not-yet-proven`      | Implementation may exist, but the required CI or manual proof has not run.                                                            |
| `blocked`             | A named external dependency, credential, device, entitlement, permission, or unresolved implementation gap prevents proof.            |
| `not-applicable`      | The platform or role should not support that deliverable.                                                                             |

Do not use a green CI job or a rendered portal panel as proof that a privileged
OS capability works. CI can prove repeatable mechanics; real machines and
devices prove permissions, services, LAN, and mobile policy.

## Evidence Record

Every manual proof record should include:

- Git commit SHA and branch;
- package or app version;
- platform, OS version, and device or VM type;
- permission, entitlement, signing, and network state;
- command, installer action, or UI action performed;
- Rust service or app log snippet;
- parent UI screenshot or copied diagnostic output when UI is involved;
- expected result, observed result, proof state label, and follow-up owner.

Sensitive child activity details should be minimized in screenshots and copied
diagnostics. Use synthetic test identities and low-sensitivity activity where
possible while keeping the real product path intact.

Do not count manually inserted database rows, mocked responses, or portal-local
fixtures as runtime proof. Runtime proof must launch the real Rust service or
real package and route through the real local or LAN transport.

## Checkpoint Order

1. Sync the target checkout, record the commit SHA, and verify it is clean.
2. Run the shared local gates that are practical for the machine.
3. Run the pre-AI proof gate against current proof-matrix claims.
4. Review current GitHub Actions state for Windows, Linux, macOS, Android, and
   iOS preview jobs.
5. Build or download package-preview artifacts for each target platform.
6. Run the Windows local PC proof first because Windows is the lead child-agent
   target.
7. Run LAN parent-to-child checks across at least two local machines.
8. Gather Linux proof from CI plus WSL or Docker.
9. Gather macOS proof on the Mac system.
10. Gather Android proof from emulator and one real device when available.
11. Gather iOS proof from simulator plus TestFlight or entitlement notes when
    available.
12. Run installer, autostart, uninstall, update, and reboot-survival checks for
    every platform that has an installable artifact.
13. Record unavailable, degraded, scaffold-only, blocked, and manual-required
    states before deciding whether the V0.7 checkpoint is accepted.
14. Update the proof matrix only after a claim has real CI or manual evidence.

## Shared Baseline Gate

Run from a clean checkout before manual platform work:

```powershell
cmd /c npm run format:check
cmd /c npm run test:pre-ai-proof
cmd /c node scripts/test/platform-os-lan-mobile-proof.mjs
cmd /c node scripts/test/enforcement-lan-mobile-product-proof.mjs
cmd /c npm run validate
```

If full `validate` is too expensive during a manual platform pass, record which
focused commands ran and require full validation before PR-ready handoff.

The shared gate must prove:

- TypeScript contracts build and reject invalid payloads;
- Rust crates compile and test on the host;
- the local Rust service launches;
- the parent surface talks to the real service;
- encrypted journal and SQLite paths use product code;
- unavailable or degraded platform capability states remain explicit.
- owned-process enforcement is real only where the host OS supports it, and
  broad app/domain/browser/mobile states remain manual-required or unavailable
  until real adapters and device artifacts exist.
- product capability proof distinguishes broad enforcement gaps from the
  implemented owned-process/time-limit paths, and separates Android package
  lifecycle plus iOS signing/entitlement gaps from generic mobile scaffold
  states.

## Manual Proof Runbook

Use this runbook as the operator checklist for the pre-AI checkpoint. Create one
evidence record per platform or scenario. Do not upgrade
`docs/expectations/pre-ai-proof-matrix.json` until the matching evidence record
exists.

For every run, record:

- proof owner and date;
- commit SHA, branch, package/app version, and artifact source;
- proof target: CI, Windows PC, Linux WSL/Docker, macOS host, Android emulator,
  Android device, iOS simulator, iOS TestFlight/device, LAN pair, or package
  lifecycle;
- commands or UI actions performed;
- log, screenshot, package, or copied diagnostic artifact path;
- CI proof level and manual proof level;
- final proof state label;
- gap owner and next action for `manual-required`, `permission-required`,
  `scaffold-only`, `not-yet-proven`, or `blocked` rows.

CI can prove only repeatable mechanics:

- contracts, schemas, source-shape, lint, type-check, and tests;
- Rust crate portability and service launch;
- local and single-machine LAN smoke through the real service;
- package preview build, install, or launch smoke where runners support it;
- typed unavailable, degraded, disabled, or scaffold-only capability states.

CI cannot prove privileged or household behavior:

- real foreground window, browser, screen, network, app/game, or service-manager
  behavior that depends on an OS permission or real host state;
- two-device LAN pairing across local router and firewall behavior;
- Windows service autostart after a real install and reboot;
- launchd behavior, macOS permissions, signing, notarization, or stores;
- Android UsageStats, accessibility, VPN/DNS, foreground-service durability,
  device-owner, managed-profile, package lifecycle, or store behavior;
- iOS Family Controls, DeviceActivity, Screen Time, Network Extension,
  notifications, background execution, signing, TestFlight, or entitlement
  review.

Operator checklist:

1. Pull the target commit, confirm the checkout is clean, and record the SHA.
2. Run the shared baseline gate or record exactly why a command was omitted.
3. Capture current CI/package-preview state for Windows, Linux, macOS, Android,
   and iOS.
4. Run the Windows real-PC proof first and record evidence-backed read-model
   results before any platform claim is upgraded.
5. Run LAN proof with a paired request and a failed unpaired request.
6. Run Linux WSL/Docker proof for package and service mechanics, keeping
   capture, network, and enforcement as unavailable or not-yet-proven unless a
   real adapter proof exists.
7. Run macOS host proof for package launch, loopback reachability, launchd or
   permission states, and signing/notarization gaps.
8. Run Android emulator proof, then physical-device proof when available,
   without merging parent-app and child-agent claims.
9. Run iOS simulator proof, then TestFlight/device or entitlement notes when
   available, without treating simulator launch as entitlement proof.
10. Run install, autostart, update, reboot, uninstall, and data-retention checks
    for each installable artifact.
11. For every unsupported or unproved capability, record the honest proof state
    label instead of writing a success-shaped note.
12. Update proof matrix and roadmap checkpoint language only after evidence
    records exist.

Minimum artifact checklist:

- `format:check`, `test:pre-ai-proof`, and `validate` output or omission notes;
- CI/package-preview job links or copied logs per target platform;
- service health/status payload from the real Rust service;
- parent UI screenshot or copied diagnostic output for evidence-backed preview
  state;
- Rust service or app log snippet for each manual runtime claim;
- install, launch, reboot, autostart, update, uninstall, and data-retention
  notes for each package proof;
- permission, entitlement, signing, store, firewall, and pairing notes where
  those states determine the proof label.

## Windows Local PC Proof

Windows is the first production-grade child-agent target. Run this proof on the
local Windows PC after package-preview or local build succeeds.

Required checks:

- install the Windows package or launch the built Rust service;
- start and stop the service from the supported path;
- verify loopback parent portal reachability;
- verify LAN mode only after explicit origin allowlist and pairing setup;
- write real encrypted journal events through product code;
- rebuild/query SQLite from product data;
- prove process/window activity, browser URL/tab state, app/game sessions,
  network-flow summaries, screen-analysis queue summaries, parent-rule context,
  local provider/runtime status, and dry-run policy preview where those claims
  are marked implemented;
- verify unavailable/degraded states for capture or AI capabilities that are not
  enabled on the host;
- reboot and confirm the configured service or autostart state matches the
  documented package claim;
- uninstall and confirm service, process, and data-retention behavior.

Evidence to capture:

- package install output or service launch command;
- `localhost` and LAN URLs used;
- service health/status payload;
- parent UI proof for at least one evidence-backed dry-run preview;
- journal/query proof output;
- reboot and uninstall notes.

Windows failure handling:

- permission or elevation missing: record `permission-required`;
- adapter not implemented: record `scaffold-only` or `not-yet-proven`;
- AI runtime unavailable: record local provider status as unavailable or
  degraded without blocking evidence preview proof;
- LAN blocked by firewall/router: record `manual-required` with firewall state
  and retry steps.

## Linux CI Plus WSL Or Docker Proof

Linux is the portability and package-mechanics gate before broader desktop
claims. CI should run shared Rust/TypeScript tests and package-preview jobs.
WSL or Docker should repeat install/launch smoke where runner privileges are not
enough.

Required checks:

- build and test shared TypeScript and Rust code on Linux;
- build Linux package preview artifacts;
- install or launch-smoke the package in CI when available;
- repeat package install or service launch in WSL or Docker;
- verify service status reports unavailable or scaffold-only for unsupported
  desktop capture, network observation, and enforcement capabilities;
- verify journal and SQLite paths work under Linux filesystem semantics.

Linux non-claims:

- no desktop capture support until Linux adapters name tested APIs;
- no network/domain observation until distro, kernel, permission, and
  attribution assumptions are tested;
- no enforcement support until platform adapters and service-manager behavior
  are implemented and proved.

## macOS Package And Permission Proof

Use CI for shared build/package signal and the Mac system for real OS behavior.

Required checks:

- build or download the macOS package preview;
- install or launch the app/service on the Mac system;
- record launchd service-manager behavior if the package claims it;
- record signing and notarization state honestly;
- check Screen Recording, Accessibility, Network Extension, notifications, and
  background permission states only as claims become relevant;
- verify loopback service reachability and parent UI status rendering;
- verify unavailable/degraded states for Windows-only capture, network, or
  enforcement behavior.

macOS non-claims:

- no process/window, network/domain, screen, or enforcement parity from package
  scaffold alone;
- no notarized production claim until Developer ID, signing, and notarization
  credentials are wired and proved.

## Android Emulator And Device Proof

Track parent Android app and child Android agent claims separately.

Required emulator checks:

- build or install the debug APK;
- launch the app or agent scaffold;
- verify local data path, SQLite compatibility, and journal compatibility where
  wired;
- verify foreground service status if the scaffold claims one;
- verify permission prompts and denied states render as honest capability
  states.

Required physical-device checks when available:

- install on a real Android device;
- verify foreground service behavior after app backgrounding;
- verify notification permission state;
- record whether UsageStats, accessibility, VPN/DNS, device-owner, or
  managed-profile modes are absent, blocked, permission-required, or
  implemented;
- verify parent-device and child-device roles are not conflated.

Android non-claims:

- no desktop-level capture or control;
- no device-owner or managed-profile claim until a real managed-device flow is
  implemented and proved;
- no network visibility claim until VPN/DNS or approved API behavior is named
  and tested.

## iOS Simulator, TestFlight, And Entitlement Notes

Track parent iOS app and child iOS agent claims separately. iOS proof requires
the Mac/Xcode path.

Required checks:

- build and launch the simulator app if the scaffold exists;
- record bundle id, signing team, and provisioning status;
- record TestFlight availability or absence;
- record entitlement state for Family Controls, DeviceActivity, Screen Time,
  Network Extension, notifications, and background execution;
- record unavailable states where Apple-approved APIs or entitlements are not
  present.

iOS non-claims:

- no background monitoring, enforcement, network filtering, or Screen Time
  control without entitlement and review proof;
- no child-agent parity claim if Apple APIs cannot support the capability;
- simulator launch does not prove device entitlement behavior.

## LAN Parent-To-Child Checks

LAN proof must use at least two real devices or a clearly documented substitute
when one is unavailable.

Required checks:

- child service binds only to the intended interface and port;
- allowed origins include only the selected parent portal origin;
- pairing is explicit before remote parent control;
- unauthenticated LAN requests fail;
- parent portal or parent desktop connects through the real LAN transport;
- service replies include child-device identity, capability state, and data
  custody status;
- dry-run preview requests remain evidence-cited and do not execute
  enforcement;
- firewall/router failures are recorded as manual-required or blocked instead
  of hidden.

LAN proof should include:

- parent device OS and IP range;
- child device OS and IP range;
- ports used;
- pairing step performed;
- copied service status or UI screenshot;
- negative check for an unpaired request.

## Installer, Autostart, Update, And Reboot Proof

Run these checks for every platform with an installable artifact.

Required checks:

- install from the package artifact;
- launch from the installed location;
- verify service manager registration when claimed;
- reboot and verify autostart state;
- stop and restart the service or app;
- uninstall and verify process/service cleanup;
- record whether data is retained, removed, or user-controlled;
- verify update manifest behavior only where update scaffolding is wired;
- reject unsigned or unavailable update claims where signing is not implemented.

Package-preview jobs are enough for scaffold proof only. Production installer
claims need signing, notarization, store, or managed-device credentials where
the platform requires them.

## Exit Criteria

The V0.7 checkpoint is ready for review when:

- shared validation has passed or every omitted command has an explicit reason;
- every platform row has CI and manual proof level recorded;
- Windows local PC proof covers the current evidence-preview flow;
- LAN proof has at least one successful paired request and one failed unpaired
  request;
- package/autostart/reboot/uninstall behavior is recorded for installable
  platforms;
- unavailable/degraded states are visible through typed service or package
  status, not hidden in notes only;
- no V0.8 enforcement implementation or unsupported AI behavior was introduced;
- the primary coordinator can update the proof matrix and roadmap checkpoint
  language from concrete evidence records.

## PR Body Outline

When this planning slice is turned into a PR, use this outline:

```text
Scope
- Tightened the cross-platform deliverables checkpoint into an executable
  manual proof runbook for V0.7 review.
- Clarified what CI can prove versus what requires real OS/device proof.
- Linked the roadmap next actions to executing the proof pass and recording
  evidence.
- Kept scope docs-only; no portal, package, proof harness, AI, or enforcement
  implementation.

Touched files
- docs/architecture/cross-platform-deliverables-checkpoint.md
- docs/product-roadmap.md

Validation
- cmd /c npm run format:check
- cmd /c npm run test:pre-ai-proof
- git diff --check

Known gaps and risks
- This is a checkpoint plan, not executed platform proof.
- Real OS/device evidence still has to be gathered on Windows, Linux, macOS,
  Android, iOS, and LAN devices before support claims are upgraded.

Roadmap slice
- V0.7 pre-AI/enforcement cross-platform deliverables manual proof checkpoint.
```
