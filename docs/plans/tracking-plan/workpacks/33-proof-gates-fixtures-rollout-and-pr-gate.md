# WP33 Proof Gates Fixtures Rollout And PR Gate

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
- WP30 hosted parent-route proof:
  `../30-parent-and-child-ui-ux-surfaces/17-hosted-ui-proof.json`
- Pre-device aggregate proof:
  `output/tracking-plan-proof/pre-device-gap-closure/proof-summary.json`

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
- [x] Update feature docs and queue the central capability-checklist delta when
      proof changes and the worker is not allowed to edit the central row.
- [x] Include touched files, validation, product-doc updates, known gaps, and
      platform proof state in `DONE`.
- [x] Do not mark product-complete from planning-only docs.

## Where We Are

This workpack has focused contract proof from `codex/tracking-plan-full-scope`
under the proof root below. Tracked `proof-summary.json` records
`minimumSeriousMvpAuditSummary`, and `scripts/test/tracking-plan-runtime-proof.mjs`
writes generated `00-run-metadata.json` with the full
`minimumSeriousMvpAudit` for the first checkpoint. These audits record P1
fixture proof only, including local UI proof artifact references. WP32 now also
has focused P2 service-command proof plus narrow portal summary consumption for
the `trackingReadModel` payload, and WP30 now has P2 hosted parent-route
Playwright/screenshot/accessibility proof for that narrow summary. These proofs
do not upgrade the full UI, child UI, platform physical-device, authority, or
production claims.
This branch adds
`node scripts/test/tracking-plan-pre-device-proof.mjs`, which reruns the
tracking P0/P1/P2 stack, lower-level Android/iOS mobile scaffold proof scripts,
Android debug package artifact gate, and mobile aggregate proof. It writes
`output/tracking-plan-proof/pre-device-gap-closure/` with explicit Android
Studio, iOS simulator, WSL/local, physical-device, and authority proof plans.
Those artifacts close the pre-device accounting gap only; they did not claim
device, authority, full hosted child/parent UI accessibility, or production
readiness.
This branch adds `npm run test:tracking-plan-hosted-ui-proof`, which starts the
real Rust service and Vite portal, runs Playwright on the hosted
`policy-tracking` route, captures desktop/mobile screenshots, records
accessibility summary output, and writes the WP30 hosted proof artifact. It
closes the narrow parent-route hosted proof gap only; it does not claim child
UI, physical Android/iOS behavior, authority, provider delivery, or production
readiness.

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
- `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, full live UI beyond the narrow hosted parent route, or
  runtime claims remain manual-required until the assigned proof artifacts
  exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [x] Workpack id and branch: `codex/tracking-plan-full-scope`.
- [x] Touched files: tracking contract files, proof script, product docs, checklist, and this workpack doc.
- [x] Validation commands and results: `node scripts/test/tracking-plan-contract-proof.mjs`, `node scripts/test/tracking-plan-runtime-proof.mjs`, and `node scripts/test/tracking-plan-service-read-model-proof.mjs` passed locally.
- [x] Proof artifacts under `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/`, including tracked `proof-summary.json` with `minimumSeriousMvpAuditSummary` and generated `00-run-metadata.json` with `minimumSeriousMvpAudit`.
- [x] Product doc/checklist updates: owning feature doc, feature list, capability checklist, implementation checklist, tracking snapshot, and package READMEs updated.
- [x] Known gaps/manual-required states: Android/iOS physical behavior, precise desktop location, provider delivery, full live parent/child UI, broader hosted accessibility, richer live service-backed UI citations, authority proof, production pilot, and full root-gate validation remain proof-gated as applicable.
- [x] Workpack id and branch: `codex/tracking-proof-gap-closure`.
- [x] Touched files: pre-device proof script, root test script wiring, tracking feature doc, tracking README, implementation checklist, WP08-WP12, WP30, WP32, WP33, and generated pre-device proof artifacts.
- [x] Validation commands and results: `npm run test:tracking-plan-pre-device-proof` passed locally; it reran tracking contract/runtime/service proof, child Android device artifact gate, child iOS entitlement proof, and mobile child-agent aggregate proof.
- [x] Proof artifacts under `output/tracking-plan-proof/pre-device-gap-closure/`, including `proof-summary.json`, `android-studio-local-proof-plan.json`, `ios-simulator-local-proof-plan.json`, `wsl-local-proof-plan.json`, `physical-device-manual-proof-plan.json`, and `16-validation-commands.log`.
- [x] Product doc/checklist updates: owning feature doc and tracking checklist updated; the central `docs/product-capability-checklist.md` row update is queued through the hub DOC_DELTA queue with the pre-device proof gate while keeping Android Studio/emulator, iOS simulator, WSL/local, physical-device, authority, full child/parent UI, and production proof as gaps.
- [x] Known gaps/manual-required states: Android Studio/emulator runtime, iOS simulator/local, WSL/local replay, physical Android/iOS behavior, authority-enrolled proof, full hosted child/parent UI accessibility, production pilot, and richer live UI/read-model/product claims remain proof-gated as applicable.
- [x] Workpack id and branch:
      `33-proof-gates-fixtures-rollout-and-pr-gate`,
      `codex/tracking-hosted-ui-accessibility-proof`.
- [x] Touched files: hosted proof script/spec, root package script wiring,
      tracking feature doc, tracking README, implementation checklist, WP30,
      WP32, this workpack, and generated hosted proof artifacts.
- [x] Validation commands and results: `node --check scripts/test/tracking-plan-hosted-ui-proof.mjs`
      passed; `npx prettier --check ...` passed for touched files;
      `git diff --check` passed;
      `npm run test:tracking-plan-hosted-ui-proof` passed locally;
      `npm run format:check` passed; `npm run lint:schema-boundaries` passed
      with existing source-shape advisory warnings only; `npm run lanes:guard`
      passed; `npm run hub:guard` passed.
- [x] Proof artifacts under WP30 include
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/17-hosted-ui-proof.json`,
      `12-playwright-proof.log`, hosted desktop/mobile screenshots, and
      `test-results/tracking-plan-hosted-ui-proof/accessibility-summary.json`.
- [x] Product doc/checklist updates: owning feature doc and tracking checklist
      updated; central product capability checklist update queued through hub
      DOC_DELTA because this worker must not edit that file.
- [x] Known gaps/manual-required states: full child/parent UI beyond the narrow
      hosted parent route, broader accessibility, richer read models/citations,
      Android Studio/emulator runtime, iOS simulator/local, WSL/local replay,
      physical Android/iOS behavior, authority-enrolled proof, production pilot,
      provider delivery, and notifications remain proof-gated.
