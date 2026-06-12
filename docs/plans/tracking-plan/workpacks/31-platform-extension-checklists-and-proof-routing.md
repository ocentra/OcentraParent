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

## AI Worker Checklist

- [ ] Maintain Android extension rows.
- [ ] Maintain iOS extension rows.
- [ ] Maintain desktop extension rows.
- [ ] Add managed-device proof only when real enrollment/control exists.
- [ ] Keep CI/package proof separate from real device capability proof.
- [x] Route iOS simulator package build/install/launch proof separately from
      iOS Core Location, background, entitlement, and physical-device claims.

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

- [x] Workpack id and branch: `codex/tracking-plan-full-scope`.
- [x] Touched files: tracking contract files, proof script, product docs, checklist, and this workpack doc.
- [x] Validation commands and results: `node scripts/test/tracking-plan-contract-proof.mjs` passed.
- [x] Proof artifacts under `output/tracking-plan-proof/31-platform-extension-checklists-and-proof-routing/`.
- [x] Product doc/checklist updates: owning feature doc, feature list, capability checklist, implementation checklist, tracking snapshot, and package READMEs updated.
- [x] Known gaps/manual-required states: Android/iOS, precise desktop, provider delivery, runtime engines, retention/delete/export, Rust journal/SQLite, notifications, and UI remain proof-gated as applicable.
- [x] Workpack id and branch: `codex/tracking-ios-simulator-proof`.
- [x] Touched files: iOS simulator proof script, root script wiring,
      package-preview CI artifact wiring, tracking feature doc, implementation
      checklist, WP11, WP12, WP31, and generated tracking iOS proof artifacts.
- [x] Validation commands and results: `npm run test:tracking-plan-ios-simulator-proof`
      writes local proof; macOS package-preview runs the same proof with
      `--require-simulator` after the real iOS simulator smoke.
- [x] Proof artifacts under `output/tracking-plan-proof/31-platform-extension-checklists-and-proof-routing/`.
- [x] Product doc/checklist updates: owning feature doc and tracking checklist
      updated; central capability checklist delta remains primary-owned through
      hub.
- [x] Known gaps/manual-required states: Core Location, background/region,
      notifications, entitlements, signing/TestFlight/App Store, physical-device,
      authority, and production proof remain unclaimed.
- [x] Workpack id and branch:
      `codex/tracking-unsupported-manual-hosted-ui-proof`.
- [x] Touched files: hosted tracking portal proof renderer, text/artifact
      domain constants, Playwright hosted proof, proof script, portal README,
      feature doc, tracking implementation checklist, and this workpack doc.
- [x] Validation commands and results: `npm run build:contracts` passed; focused
      portal tracking-status-panel test passed before hosted proof validation.
- [x] Proof artifacts under
      `output/tracking-plan-proof/31-platform-extension-checklists-and-proof-routing/`.
- [x] Product doc/checklist updates: owning feature doc and tracking
      implementation checklist updated; central product capability checklist
      remains E-C/primary-owned and should receive the same row delta through
      hub.
- [x] Known gaps/manual-required states: physical-device execution, authority
      enrollment, provider delivery, production worker, and product-ready
      tracking remain unclaimed.
