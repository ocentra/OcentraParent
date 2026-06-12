# WP33 Proof Gates Fixtures Rollout And PR Gate

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `WP33 Proof Gates Fixtures Rollout And PR Gate`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Purpose

Define final test fixtures, proof packs, Playwright/manual proof, docs updates,
and merge blockers for tracking work.

## Source Inputs

- `docs/plans/tracking-plan/v0-5-location-test-blueprint.md`
- `docs/plans/tracking-plan/implementation-checklist.md`
- `docs/plans/tracking-plan/pasted-content-coverage-audit.md`
- `.ocentra-ai/rules/ocentra-parent-validation.mdc`

## Target State

No tracking implementation can report `DONE` or PR-ready without proof packs,
validation commands, docs/checklist updates, and explicit known gaps.

## Tests And Proof

Proof root: `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/`

- `00-source-snapshot.md`
- `01-contract-proof.log`
- `02-platform-permission-proof.md`
- `03-runtime-location-evidence.json`
- `04-device-status-proof.json`
- `05-geofence-transition-proof.json`
- `06-expected-place-proof.json`
- `07-nearby-place-proof.json`
- `08-ai-analysis-proof.json`
- `09-policy-alert-proof.json`
- `10-journal-sqlite-proof.json`
- `11-ui-snapshots/`
- `12-playwright-proof.log`
- `13-security-negative-proof.log`
- `14-retention-delete-proof.json`
- `15-manual-platform-proof.md`
- `16-validation-commands.log`
- WP32 companion proof:
  `../32-journal-sqlite-and-read-model-proof/18-service-read-model-proof.json`
- WP32 WSL/local companion proof:
  `../32-journal-sqlite-and-read-model-proof/19-wsl-local-replay-proof.json`
- Pre-device aggregate proof:
  `output/tracking-plan-proof/pre-device-gap-closure/proof-summary.json`
- WSL/local replay proof:
  `17-wsl-local-proof.json`
- iOS simulator package proof:
  `test-results/tracking-plan-ios-simulator-proof/proof.json`
- Hosted parent route screenshot/accessibility proof:
  `18-hosted-ui-accessibility-proof.json`
- Evidence quality gate proof:
  `19-evidence-quality-gate-proof.json`
- Evidence quality gate validation log:
  `20-evidence-quality-gate-validation.log`
- Tracking notification receipt boundary proof:
  `22-notification-receipt-boundary-proof.json`
- Hosted child-safe check-in screenshot:
  `../30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-child-check-in.png`
- Notification preference preflight proof:
  `24-notification-preference-preflight-proof.json`

## Merge Blockers

- LAN/IP displayed as GPS.
- Location missing accuracy/source/timestamp/freshness.
- Stale displayed as live.
- Nearby POI displayed as exact place with low accuracy.
- AI triggers notification without policy decision.
- Critical alert suppressed by generic exception.
- Parent acknowledgement ignored.
- Retention delete fails.
- Remote sync runs by default.
- Remote AI runs by default.
- Background tracking claimed without Android/iOS proof.

## AI Worker Checklist

- [x] Run the smallest useful validation while working.
- [x] Run requested focused tests before handoff.
- [x] Update feature docs and queue central capability checklist delta when
      proof changes.
- [x] Include touched files, validation, product-doc updates, known gaps, and
      platform proof state in `DONE`.
- [x] Do not mark product-complete from planning-only docs.

## Where We Are

This workpack has focused contract proof from `codex/tracking-plan-full-scope`
under the proof root below. Tracked `proof-summary.json` records
`minimumSeriousMvpAuditSummary`, and `scripts/test/tracking-plan-runtime-proof.mjs`
writes generated `00-run-metadata.json` with the full
`minimumSeriousMvpAudit` for the first checkpoint. These audits record P1
fixture proof only, including local UI proof artifact references; hosted
CI/a11y, full live UI, child-device runtime UI, platform physical-device proof,
authority proof, and production-pilot proof remain unclaimed. WP32 now also has focused
P2 service-command proof, retention-delete tombstone replay, and narrow portal
summary consumption for the `trackingReadModel` payload; that proof does not
upgrade the full UI, platform, authority, or production claims.
The pre-device proof gate uses
`node scripts/test/tracking-plan-pre-device-proof.mjs`, which reruns the
tracking P0/P1/P2 stack, lower-level Android/iOS mobile scaffold proof scripts,
Android debug package artifact gate, and mobile aggregate proof. It writes
`output/tracking-plan-proof/pre-device-gap-closure/` with explicit Android
Studio, iOS simulator, WSL/local, physical-device, and authority proof plans.
Those artifacts close the pre-device accounting gap only; they do not claim
device, authority, full hosted UI/accessibility, or production readiness.
This branch adds `npm run test:tracking-plan-wsl-local-proof`, which records
P3 WSL/local replay proof for the narrow tracking read-model proof stack. It
writes `output/tracking-plan-proof/wsl-local-replay/` plus WP32/WP33 companion
artifacts and captures the WSL2/Ubuntu toolchain, linked-worktree Git mapping,
contract build output, service read-model proof, and Rust core tracking
read-model test. It does not claim Android/iOS physical behavior, authority,
full hosted UI/accessibility, provider delivery, or production readiness.

This branch adds `npm run test:tracking-plan-ios-simulator-proof`, which writes
tracking-specific iOS simulator package proof artifacts. On macOS
package-preview CI it runs after the existing iOS simulator build and simctl
install/launch smoke with `--require-simulator`; on non-macOS hosts it writes
manual-required output. It is a package-mechanics proof gate only and does not
upgrade Core Location, background/region, entitlement, physical-device, or
authority claims.

This branch adds `npm run test:tracking-plan-hosted-ui-proof`, which starts the
real Rust service against a seeded temporary ActivityStore SQLite database and
drives the hosted parent `policy-tracking` route through Playwright. It writes
desktop/mobile screenshots, a hosted child-safe check-in screenshot, an
accessibility summary, WP30 hosted UI proof, and WP33 hosted UI gate proof while
keeping `productClaimReady=false`. It is a hosted route proof only and does not
claim child-device delivery/runtime UI, full service-data UI, Android/iOS
physical-device behavior, authority, provider delivery, or production
readiness.

This branch adds `npm run test:tracking-plan-evidence-quality-gate-proof`,
which validates location UI evidence refs, geofence rule/source refs,
nearby-place provider/radius/category/distance/confidence/ambiguity fields, AI
source refs with no final action, alert policy-decision refs, and retention
delete/export before/after proof through parser-backed fixtures, retention
helpers, parent-domain contracts, and the portal citation test. It writes
`19-evidence-quality-gate-proof.json`,
`20-evidence-quality-gate-validation.log`, and
`test-results/tracking-plan-evidence-quality-gate-proof/proof.json` while
keeping live device/provider delivery and production behavior unclaimed.
Tracking notification preference preflight proof now writes
`24-notification-preference-preflight-proof.json` through
`node scripts/test/tracking-notification-preference-preflight-proof.mjs`. It
adds parent-preference-required, source-manual-required, and source-unavailable
boundary proof rows while keeping parent notification UI/history, preference
mutation runtime, quiet-hours timer runtime, provider delivery/receipt runtime,
child-device delivery, physical-device proof, authority, and production
notification behavior unclaimed.

This branch adds `node scripts/test/tracking-notification-receipt-boundary-proof.mjs`,
which derives tracking notification receipt boundary rows from the WP26
provider-notification proof and writes WP26/WP33 companion proof artifacts. It
preserves provider proof refs, evidence refs, policy decision refs,
notification status refs, reason refs, provider attempt refs, and audit refs
while keeping actual webhook/provider receipt ingestion, provider delivery,
credentials, adapter dispatch, child-device delivery, authority, physical-device
proof, and production durable outbox storage unclaimed.

## Where We Want To Be

This workpack can be assigned independently, implemented against the owning domain boundaries, validated with real contracts or platform proof, and reported without leaving unclear tracking claims behind.

## Scope

- Re-read only the source inputs listed in this workpack and the shared source index.
- Update the owned contracts, runtime paths, UI paths, or docs named by the implementation worker before editing.
- Keep no-claim boundaries explicit for LAN/IP hints, nearby-place ambiguity, AI evidence, background behavior, and remote sync.
- Update the feature doc and product capability checklist only when status, proof, or gaps change.

## Touched Paths

- docs/plans/tracking-plan/workpacks/33-proof-gates-fixtures-rollout-and-pr-gate.md
- docs/plans/tracking-plan/implementation-checklist.md
- apps/portal/e2e/tracking-hosted-ui-proof.spec.ts
- scripts/test/tracking-plan-hosted-ui-proof.mjs
- scripts/test/tracking-plan-ios-simulator-proof.mjs
- scripts/test/tracking-plan-evidence-quality-gate-proof.mjs
- scripts/test/tracking-notification-preference-preflight-proof.mjs
- packages/activity-domain/src/tracking-evidence-quality-gate.ts
- packages/activity-domain/tests/tracking-evidence-quality-gate.test.ts
- scripts/test/tracking-notification-receipt-boundary-proof.mjs
- `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, live UI, or runtime claims remain
  manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.
- Actual webhook/provider receipt ingestion, provider delivery, credentials,
  adapter dispatch, child-device delivery, authority, physical-device proof,
  and production durable outbox storage remain manual-required until matching
  proof exists.
- Parent notification preferences and quiet-hours must remain manual-required
  until runtime/UI mutation proof exists.

## Fill This Before Reporting DONE Or PR-ready

- [x] Workpack id and branch: `codex/tracking-plan-full-scope`.
- [x] Touched files: tracking contract files, proof script, product docs, checklist, and this workpack doc.
- [x] Validation commands and results: `node scripts/test/tracking-plan-contract-proof.mjs`, `node scripts/test/tracking-plan-runtime-proof.mjs`, and `node scripts/test/tracking-plan-service-read-model-proof.mjs` passed locally.
- [x] Proof artifacts under `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/`, including tracked `proof-summary.json` with `minimumSeriousMvpAuditSummary` and generated `00-run-metadata.json` with `minimumSeriousMvpAudit`.
- [x] Product doc/checklist updates: owning feature doc, feature list, capability checklist, implementation checklist, tracking snapshot, and package READMEs updated.
- [x] Known gaps/manual-required states: Android/iOS physical behavior, precise desktop location, provider delivery, full live parent/child UI, hosted a11y, richer live service-backed UI citations, authority proof, production pilot, and full root-gate validation remain proof-gated as applicable.
- [x] Workpack id and branch: `codex/tracking-proof-gap-closure`.
- [x] Touched files: pre-device proof script, root test script wiring, tracking feature doc, tracking README, implementation checklist, WP08-WP12, WP30, WP32, WP33, and generated pre-device proof artifacts.
- [x] Validation commands and results: `npm run test:tracking-plan-pre-device-proof` passed locally; it reran tracking contract/runtime/service proof, child Android device artifact gate, child iOS entitlement proof, and mobile child-agent aggregate proof.
- [x] Proof artifacts under `output/tracking-plan-proof/pre-device-gap-closure/`, including `proof-summary.json`, `android-studio-local-proof-plan.json`, `ios-simulator-local-proof-plan.json`, `wsl-local-proof-plan.json`, `physical-device-manual-proof-plan.json`, and `16-validation-commands.log`.
- [x] Product doc/checklist updates: owning feature doc and tracking checklist updated; the central `docs/product-capability-checklist.md` row update is queued through the hub DOC_DELTA queue with the pre-device proof gate while keeping Android Studio/emulator, iOS simulator, WSL/local, physical-device, authority, hosted UI, and production proof as gaps.
- [x] Known gaps/manual-required states: Android Studio/emulator runtime, iOS simulator/local, WSL/local replay, physical Android/iOS behavior, authority-enrolled proof, full hosted UI/accessibility, production pilot, and richer live UI/read-model/product claims remain proof-gated as applicable.
- [x] Workpack id and branch: `codex/tracking-wsl-local-replay-proof`.
- [x] Touched files: WSL proof script, root test script wiring, tracking
      feature doc, tracking README, implementation checklist, WP32, WP33, and
      generated WSL proof artifacts.
- [x] Validation commands and results:
      `npm run test:tracking-plan-wsl-local-proof` passed locally.
- [x] Proof artifacts under `output/tracking-plan-proof/wsl-local-replay/`,
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/19-wsl-local-replay-proof.json`,
      and
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/17-wsl-local-proof.json`.
- [x] Product doc/checklist updates: owning feature doc, tracking README,
      implementation checklist, WP32, and WP33 updated; the central
      `docs/product-capability-checklist.md` row update is queued through the
      hub DOC_DELTA queue with WSL/local replay proof while keeping
      Android/iOS physical-device, authority, hosted UI, provider-delivery, and
      production proof as gaps.
- [x] Known gaps/manual-required states: Android Studio/emulator runtime, iOS
      simulator/local, physical Android/iOS behavior, authority-enrolled proof,
      full hosted UI/accessibility, production pilot, provider delivery, and
      richer live UI/read-model/product claims remain proof-gated as
      applicable.
- [x] Workpack id and branch: `codex/tracking-ios-simulator-proof`.
- [x] Touched files: iOS simulator proof script, root script wiring,
      package-preview CI artifact wiring, tracking feature doc, implementation
      checklist, WP11, WP12, WP31, WP33, and generated tracking iOS proof
      artifacts.
- [x] Validation commands and results: `npm run test:tracking-plan-ios-simulator-proof`
      writes local proof; macOS package-preview runs the proof with
      `--require-simulator` after the real iOS simulator smoke.
- [x] Proof artifacts under `test-results/tracking-plan-ios-simulator-proof/`,
      WP11, WP12, and WP31 proof roots.
- [x] Product doc/checklist updates: owning feature doc and tracking checklist
      updated; central capability checklist delta remains primary-owned through
      hub.
- [x] Known gaps/manual-required states: Core Location authorization/sample,
      background/region, notifications, entitlements, signing/TestFlight/App
      Store, physical-device, authority, hosted full UI/accessibility, and
      production proof remain unclaimed.
- [x] Workpack id and branch:
      `codex/tracking-live-service-citation-proof`.
- [x] Touched files: portal tracking status renderer/tests, service proof
      script, tracking feature doc, implementation checklist, WP30, WP32,
      WP33, and generated WP32 proof artifacts.
- [x] Validation commands and results:
      `node scripts/test/tracking-plan-service-read-model-proof.mjs` passed.
- [x] Proof artifacts under
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/`,
      including `14-retention-delete-proof.json`,
      `18-service-read-model-proof.json`, and `proof-summary.json`.
- [x] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP30, WP32, and WP33 updated; central capability row delta
      queued through the hub instead of editing
      `docs/product-capability-checklist.md`.
- [x] Known gaps/manual-required states: hosted full UI/accessibility, full
      parent/child UI, broader product read models, Android/iOS physical-device
      proof, authority, provider delivery, notifications, and production proof
      remain proof-gated.
- [x] Workpack id and branch:
      `codex/tracking-hosted-ui-accessibility-proof-v2`.
- [x] Touched files: hosted Playwright proof spec, hosted proof script, root
      script wiring, parent route tracking CSS, tracking feature doc,
      implementation checklist, WP30, WP33, and generated hosted proof
      artifacts.
- [x] Validation commands and results:
      `npm run test:tracking-plan-hosted-ui-proof` passed locally.
- [x] Proof artifacts under
      `test-results/tracking-plan-hosted-ui-proof/`,
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/`, and
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/`,
      including `18-hosted-ui-accessibility-proof.json`.
- [x] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP30, and WP33 updated; central capability row delta queued
      through the hub instead of editing `docs/product-capability-checklist.md`.
- [x] Known gaps/manual-required states: child-device delivery/runtime UI, full
      service-data UI beyond the hosted parent route, Android/iOS
      physical-device proof, authority, provider delivery, notifications, and
      production proof remain proof-gated.
- [x] Workpack id and branch:
      `codex/tracking-child-check-in-ui-proof`.
- [x] Touched files: hosted child check-in proof model, portal tracking status
      renderer/tests, hosted Playwright proof spec, hosted proof script,
      portal/text domain constants, tracking feature doc, implementation
      checklist, WP30, WP33, and generated hosted proof artifacts.
- [x] Validation commands and results: `npm run test:tracking-plan-hosted-ui-proof`
      passes locally after focused portal/text tests.
- [x] Proof artifacts under
      `test-results/tracking-plan-hosted-ui-proof/`,
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/`, and
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/`,
      including
      `../30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-child-check-in.png`.
- [x] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP30, and WP33 updated; central capability row delta queued
      through the hub instead of editing `docs/product-capability-checklist.md`.
- [x] Known gaps/manual-required states: child-device delivery/runtime UI, full
      parent/child UI beyond the hosted route, Android/iOS physical-device
      proof, authority, provider delivery, notifications, and production proof
      remain proof-gated.
- [x] Workpack id and branch:
      `codex/tracking-evidence-quality-gate-proof`.
- [x] Touched files: activity-domain evidence quality gate helper/test, root
      script wiring, evidence-quality proof script, tracking feature doc,
      implementation checklist, WP33, generated WP33 proof artifacts, and hub
      doc delta queue.
- [x] Validation commands and results:
      `npm run test:tracking-plan-evidence-quality-gate-proof` is the focused
      proof command for this slice and reruns activity-domain, parent-domain,
      and portal citation checks.
- [x] Proof artifacts under
      `test-results/tracking-plan-evidence-quality-gate-proof/` and
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/`,
      including `19-evidence-quality-gate-proof.json` and
      `20-evidence-quality-gate-validation.log`.
- [x] Product doc/checklist updates: owning feature doc, implementation
      checklist, and WP33 updated; central capability row delta queued through
      the hub instead of editing `docs/product-capability-checklist.md`.
- [x] Known gaps/manual-required states: hosted CI for this gate, live
      device/provider behavior, child-device delivery/runtime UI, full
      parent/child UI beyond the hosted route, Android/iOS physical-device
      proof, authority, provider delivery, notifications, and production proof
      remain proof-gated.
- [x] Workpack id and branch:
      `codex/tracking-notification-receipt-boundary-proof`.
- [x] Touched files: parent-domain receipt boundary proof contract, focused
      tests, proof script, owning tracking feature doc, implementation
      checklist, WP26, WP33, and generated receipt boundary proof artifacts.
- [x] Validation commands and results:
      `node scripts/test/tracking-notification-receipt-boundary-proof.mjs`
      passed.
- [x] Proof artifacts:
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/22-notification-receipt-boundary-proof.json`,
      `output/tracking-plan-proof/26-alert-severity-and-notification-model/22-notification-receipt-boundary-proof.json`,
      `output/tracking-plan-proof/tracking-notification-receipt-boundary-proof/proof.json`,
      and
      `test-results/tracking-notification-receipt-boundary-proof/proof.json`.
- [x] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP26, and WP33 updated. Central
      `docs/product-capability-checklist.md` was not edited because E-C
      currently owns that lock.
- [x] Known gaps/manual-required states: actual webhook/provider receipt
      ingestion runtime, provider delivery, credentials, adapter dispatch,
      retry/quiet-hours workers, parent notification UI, child-device delivery,
      physical-device proof, authority proof, durable outbox storage, and
      product-ready notification behavior remain proof-gated.
- [x] Workpack id and branch:
      `codex/tracking-notification-preference-preflight-proof`.
- [x] Touched files: parent-domain tracking notification preference preflight
      proof/test, proof script, tracking feature doc, implementation checklist,
      WP26, WP33, and generated WP26/WP33 proof artifacts.
- [x] Validation commands and results:
      `node scripts/test/tracking-notification-preference-preflight-proof.mjs`
      passed.
- [x] Proof artifacts:
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/24-notification-preference-preflight-proof.json`
      plus the WP26 companion proof and test-results proof.
- [x] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP26, and WP33 updated; central
      `docs/product-capability-checklist.md` was not edited because E-C owns
      that lock.
- [x] Known gaps/manual-required states: parent notification UI/history,
      preference mutation runtime, frequency-control UI, quiet-hours timer
      runtime, provider delivery/receipt runtime, credentials, child-device
      delivery, physical-device proof, authority, durable outbox storage, and
      product-ready notification behavior remain proof-gated.
