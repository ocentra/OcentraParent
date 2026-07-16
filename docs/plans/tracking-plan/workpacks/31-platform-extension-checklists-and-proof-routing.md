# WP31 Platform Extension Checklists And Proof Routing

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `WP31 Platform Extension Checklists And Proof Routing`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Purpose

Route Android, iOS, desktop, managed-device, store/privacy, and manual platform
proof without bloating base contracts.

## Source Inputs

- `docs/plans/tracking-plan/v0-5-location-platform-deep-dive.md`
- `docs/expectations/platforms.md`
- `docs/expectations/platform-deliverables.md`

## Target State

Each platform extension has a proof checklist, manual-required state, output
path, and no-claim rule before product status changes.

## Tests And Proof

Proof root: `output/tracking-plan-proof/31-platform-extension-checklists-and-proof-routing/`

- `02-platform-permission-proof.md`
- `15-manual-platform-proof.md`
- `16-validation-commands.log`
- `18-ios-simulator-proof.json`
- `19-unsupported-manual-hosted-ui-proof.json`
- `19-unsupported-manual-hosted-ui.png`
- `20-platform-extension-inventory-proof.json`
- `20-platform-extension-inventory-source-snapshot.md`
- `21-authority-enrollment-manual-required-proof.json`
- `21-authority-enrollment-source-snapshot.md`
- `21-authority-enrollment-validation-commands.log`
- `22-authority-runtime-readiness-blocker-proof.json`

## AI Worker Checklist

- [ ] Maintain Android extension rows.
- [ ] Maintain iOS extension rows.
- [ ] Maintain desktop extension rows.
- [ ] Add managed-device proof only when real enrollment/control exists.
- [ ] Keep CI/package proof separate from real device capability proof.
- [ ] Route iOS simulator package build/install/launch proof separately from
      iOS Core Location, background, entitlement, and physical-device claims.
- [ ] Record authority enrollment/manual-required evidence requirements without
      claiming hard-control runtime.

## Where We Are

This workpack has focused contract proof from `codex/tracking-plan-full-scope` under the proof root below. Runtime, platform, provider, and UI behavior is not claimed beyond the proof state recorded in `proof-summary.json` and the implementation checklist.

Android emulator package/service/status proof routing now exists through
`npm run test:tracking-plan-android-emulator-proof`. It writes raw adb evidence
to `test-results/tracking-plan-android-emulator-proof/`, fills WP08/WP09/WP10
proof roots, and preserves no-claim states for foreground location samples,
background/geofence transitions, notification delivery, physical-device proof,
and authority proof.

iOS simulator package proof routing now exists through
`npm run test:tracking-plan-ios-simulator-proof`. It validates the existing
Xcode build and simctl smoke scripts, writes local manual-required proof on
non-macOS hosts, and is wired into the macOS package-preview job after the real
iOS simulator build/install/launch smoke. It keeps Core Location,
background/region, notification, entitlement, signing, TestFlight/App Store,
physical-device, and authority behavior as separate manual-required claims.

Hosted unsupported/manual platform UI proof now exists through
`npm run test:tracking-plan-hosted-ui-proof`. The hosted parent
`policy-tracking` route renders the existing unsupported-platform manual proof
rows for Android/iOS background and geofence gaps, desktop OS location, web
child-agent unavailability, and authority-required hard control. The proof
captures `19-unsupported-manual-hosted-ui.png` and writes
`19-unsupported-manual-hosted-ui-proof.json` under this workpack root while
keeping physical-device execution, authority enrollment, provider delivery, and
product-ready tracking unclaimed.

WP31 platform extension inventory proof now exists through
`node scripts/test/tracking-platform-extension-inventory-proof.mjs`. It verifies
the existing Android emulator package/service/status proof, Android
foreground/background manual-required rows, Android status manual-required rows,
iOS simulator routing, iOS Core Location manual-required rows, desktop
hint-only proof, and hosted unsupported/manual UI proof. It writes
`20-platform-extension-inventory-proof.json` under this workpack root and keeps
Android/iOS physical-device behavior, background runtime, precise desktop
location, authority enrollment, provider delivery, production upload workers,
and product-ready tracking unclaimed.

Authority enrollment manual-required proof now exists through
`node scripts/test/tracking-authority-enrollment-manual-required-proof.mjs`.
It records Android device-owner, Android managed-profile, iOS Family Controls
entitlement, iOS App Review approval, and desktop managed-policy evidence rows
under this workpack root plus the WP33 companion gate. It does not claim
authority enrollment, hard-control runtime, physical-device behavior, provider
delivery, production workers, or product-ready tracking.
Cross-platform runtime capability proof now refreshes Windows host, WSL/Linux,
Docker, Android SDK/Gradle, Android emulator, Android physical status, and
macOS/iOS CI/manual routing in one parent-domain proof. The harness normalizes
WSL probe output before storing artifacts, records Docker runtime only when both
CLI and daemon are reachable, and keeps macOS/iOS runtime proof CI/manual-routed
on this Windows host without claiming physical-device behavior, authority,
production, or product readiness.
The local platform proof batch now consumes that cross-platform proof plus the
Samsung S9 Android physical package/service/status proof as an explicit local
status row while preserving physical behavior, authority, production, and
product-ready claims as false.
Authority-runtime readiness blocker proof now exists through
`node scripts/test/tracking-authority-runtime-readiness-blocker-proof.mjs`. It
consumes the existing authority enrollment manual-required rows, writes
`22-authority-runtime-readiness-blocker-proof.json` under this workpack root
plus the WP33 companion gate, and records authority-required blockers for
enrollment, hard-control runtime, parent-visible authority status,
physical-device authority proof, production authority workers, and
product-ready authority behavior. It does not claim enrolled authority,
hard-control runtime, physical-device behavior, production workers, or
product-ready tracking.

## Where We Want To Be

This workpack can be assigned independently, implemented against the owning domain boundaries, validated with real contracts or platform proof, and reported without leaving unclear tracking claims behind.

## Scope

- Re-read only the source inputs listed in this workpack and the shared source index.
- Update the owned contracts, runtime paths, UI paths, or docs named by the implementation worker before editing.
- Keep no-claim boundaries explicit for LAN/IP hints, nearby-place ambiguity, AI evidence, background behavior, and remote sync.
- Update the feature doc and product capability checklist only when status, proof, or gaps change.

## Touched Paths

- docs/plans/tracking-plan/workpacks/31-platform-extension-checklists-and-proof-routing.md
- docs/plans/tracking-plan/implementation-checklist.md
- scripts/test/tracking-plan-ios-simulator-proof.mjs
- scripts/test/tracking-platform-extension-inventory-proof.mjs
- scripts/test/tracking-authority-enrollment-manual-required-proof.mjs
- scripts/test/tracking-authority-runtime-readiness-blocker-proof.mjs
- `output/tracking-plan-proof/31-platform-extension-checklists-and-proof-routing/`
- `apps/portal/src/tracking-status-panel.ts`
- `apps/portal/src/TrackingStatusRoutePanel.tsx`
- `apps/portal/e2e/tracking-hosted-ui-proof.spec.ts`
- `scripts/test/tracking-plan-hosted-ui-proof.mjs`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch: `codex/tracking-plan-full-scope`.
- [ ] Touched files: tracking contract files, proof script, product docs, checklist, and this workpack doc.
- [ ] Validation commands and results: `node scripts/test/tracking-plan-contract-proof.mjs` passed.
- [ ] Proof artifacts under `output/tracking-plan-proof/31-platform-extension-checklists-and-proof-routing/`.
- [ ] Product doc/checklist updates: owning feature doc, feature list, capability checklist, implementation checklist, tracking snapshot, and package READMEs updated.
- [ ] Known gaps/manual-required states: Android/iOS, precise desktop, provider delivery, runtime engines, retention/delete/export, Rust journal/SQLite, notifications, and UI remain proof-gated as applicable.
- [ ] Workpack id and branch: `codex/tracking-ios-simulator-proof`.
- [ ] Touched files: iOS simulator proof script, root script wiring,
      package-preview CI artifact wiring, tracking feature doc, implementation
      checklist, WP11, WP12, WP31, and generated tracking iOS proof artifacts.
- [ ] Validation commands and results: `npm run test:tracking-plan-ios-simulator-proof`
      writes local proof; macOS package-preview runs the same proof with
      `--require-simulator` after the real iOS simulator smoke.
- [ ] Proof artifacts under `output/tracking-plan-proof/31-platform-extension-checklists-and-proof-routing/`.
- [ ] Product doc/checklist updates: owning feature doc and tracking checklist
      updated; central capability checklist delta remains primary-owned through
      hub.
- [ ] Known gaps/manual-required states: Core Location, background/region,
      notifications, entitlements, signing/TestFlight/App Store, physical-device,
      authority, and production proof remain unclaimed.
- [ ] Workpack id and branch:
      `codex/tracking-unsupported-manual-hosted-ui-proof`.
- [ ] Touched files: hosted tracking portal proof renderer, text/artifact
      domain constants, Playwright hosted proof, proof script, portal README,
      feature doc, tracking implementation checklist, and this workpack doc.
- [ ] Validation commands and results: `npm run build:contracts` passed; focused
      portal tracking-status-panel test passed before hosted proof validation.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/31-platform-extension-checklists-and-proof-routing/`.
- [ ] Product doc/checklist updates: owning feature doc and tracking
      implementation checklist updated; central product capability checklist
      remains E-C/primary-owned and should receive the same row delta through
      hub.
- [ ] Known gaps/manual-required states: physical-device execution, authority
      enrollment, provider delivery, production worker, and product-ready
      tracking remain unclaimed.
- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: WP31 platform extension inventory proof script, owning
      tracking feature doc, implementation checklist, this workpack doc, and
      generated WP31/test-results proof artifacts.
- [ ] Validation commands and results:
      `node scripts/test/tracking-platform-extension-inventory-proof.mjs`
      passed.
- [ ] Proof artifacts:
      `output/tracking-plan-proof/31-platform-extension-checklists-and-proof-routing/20-platform-extension-inventory-proof.json`,
      `output/tracking-plan-proof/31-platform-extension-checklists-and-proof-routing/20-platform-extension-inventory-source-snapshot.md`,
      and
      `test-results/tracking-platform-extension-inventory-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, and WP31 updated; central product capability checklist remains
      hub/primary sequenced.
- [ ] Known gaps/manual-required states: managed-device enrollment/control,
      Android/iOS foreground/background physical-device behavior, precise
      desktop location, authority enrollment, provider delivery, production
      upload workers, and product-ready tracking remain unclaimed.
- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: parent-domain authority enrollment proof/test, focused
      proof script, owning tracking feature doc, implementation checklist,
      WP31, WP33, generated WP31/WP33 proof artifacts, and hub doc delta queue.
- [ ] Validation commands and results:
      `node scripts/test/tracking-authority-enrollment-manual-required-proof.mjs`
      passed.
- [ ] Proof artifacts:
      `output/tracking-plan-proof/31-platform-extension-checklists-and-proof-routing/21-authority-enrollment-manual-required-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/48-authority-enrollment-manual-required-proof.json`,
      and `test-results/tracking-authority-enrollment-manual-required-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP31, and WP33 updated. Central
      `docs/product-capability-checklist.md` update is queued through the hub
      doc delta instead of editing the shared checklist directly.
- [ ] Known gaps/manual-required states: real enrolled-device authority state,
      hard-control runtime, physical-device behavior, provider delivery,
      production workers, and product-ready tracking remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: parent-domain authority-runtime readiness blocker
      proof/test, focused proof script, product-readiness closure proof model,
      closure harness, owning tracking feature doc, implementation checklist,
      WP31, WP33, generated WP31/WP33 authority-runtime blocker artifacts, and
      refreshed closure proof artifacts.
- [ ] Validation commands and results:
      `node scripts/test/tracking-authority-runtime-readiness-blocker-proof.mjs`
      passed; `node scripts/test/tracking-product-readiness-closure-proof.mjs`
      passed.
- [ ] Proof artifacts:
      `output/tracking-plan-proof/31-platform-extension-checklists-and-proof-routing/22-authority-runtime-readiness-blocker-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/55-authority-runtime-readiness-blocker-proof.json`,
      `output/tracking-plan-proof/tracking-authority-runtime-readiness-blocker-proof/proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/46-product-readiness-closure-proof.json`,
      `output/tracking-plan-proof/tracking-product-readiness-closure-proof/proof.json`,
      `test-results/tracking-authority-runtime-readiness-blocker-proof/proof.json`,
      and `test-results/tracking-product-readiness-closure-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP31, and WP33 updated. Central
      `docs/product-capability-checklist.md` update is queued through the hub
      doc delta instead of editing the shared checklist directly.
- [ ] Known gaps/manual-required states: real enrolled-device authority state,
      hard-control runtime, parent-visible authority runtime,
      physical-device behavior, provider delivery, production workers, and
      product-ready tracking remain proof-gated.

- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: parent-domain authority runtime artifact gate proof/test,
      focused proof script, product-readiness closure proof model, closure
      harness, owning tracking feature doc, implementation checklist, WP31,
      WP33, generated authority runtime artifact gate proof artifacts, and
      refreshed closure proof artifacts.
- [ ] Validation commands and results:
      `node scripts/test/tracking-authority-runtime-artifact-gate-proof.mjs`
      passed; `node scripts/test/tracking-product-readiness-closure-proof.mjs`
      passed.
- [ ] Proof artifacts:
      `output/tracking-plan-proof/31-platform-extension-checklists-and-proof-routing/23-authority-runtime-artifact-gate-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/61-authority-runtime-artifact-gate-proof.json`,
      `output/tracking-plan-proof/tracking-authority-runtime-artifact-gate-proof/proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/46-product-readiness-closure-proof.json`,
      `output/tracking-plan-proof/tracking-product-readiness-closure-proof/proof.json`,
      `test-results/tracking-authority-runtime-artifact-gate-proof/proof.json`,
      and `test-results/tracking-product-readiness-closure-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP31, and WP33 updated. Central
      `docs/product-capability-checklist.md` update is queued through the hub
      doc delta instead of editing the shared checklist directly.
- [ ] Known gaps/manual-required states: real enrolled-device authority state,
      hard-control runtime, parent-visible authority runtime,
      physical-device behavior, provider delivery, production workers, and
      product-ready tracking remain proof-gated; this gate only classifies the
      authority runtime artifact inventory.
