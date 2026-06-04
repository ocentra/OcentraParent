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
- WP30 hosted parent route proof:
  `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/17-accessibility-proof.json`
- WP32 companion proof:
  `../32-journal-sqlite-and-read-model-proof/18-service-read-model-proof.json`
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
- [ ] Update feature docs and capability checklist when proof changes.
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
CI/a11y, full live UI, child UI, platform physical-device proof, authority
proof, and production-pilot proof remain unclaimed. WP32 now also has focused
P2 service-command proof plus narrow portal summary/latest-row detail/evidence
drawer consumption for the `trackingReadModel` payload; that proof does not
upgrade the full UI, platform, authority, or production claims.
WP30 now has narrow P2 hosted browser proof for the parent portal
`policy-tracking` route through `node scripts/test/tracking-plan-hosted-ui-proof.mjs`.
The proof writes
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/17-accessibility-proof.json`,
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/12-playwright-proof.log`,
and
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/policy-tracking-hosted-ui-proof.png`;
it proves first-target parent route browser rendering, service-detail labels,
service-row evidence drawer labels, and accessibility basics only, not
child-device UI, browser-to-service live-data screenshots, full live UI,
physical device behavior, or authority.
This branch adds
`node scripts/test/tracking-plan-pre-device-proof.mjs`, which reruns the
tracking P0/P1/P2 stack, hosted parent route UI proof, lower-level Android/iOS
mobile scaffold proof scripts, Android debug package artifact gate, and mobile
aggregate proof. It writes
`output/tracking-plan-proof/pre-device-gap-closure/` with explicit Android
Studio, iOS simulator, WSL/local, physical-device, and authority proof plans.
Those artifacts close the pre-device accounting gap only; they do not claim
device, authority, full parent/child UI accessibility, or production readiness.

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
- scripts/test/tracking-plan-hosted-ui-proof.mjs
- scripts/test/tracking-plan-pre-device-proof.mjs
- `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/`
- `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/17-accessibility-proof.json`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, full live UI, or runtime claims remain
  manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [x] Workpack id and branch: `codex/tracking-plan-full-scope`.
- [x] Touched files: tracking contract files, proof script, product docs, checklist, and this workpack doc.
- [x] Validation commands and results: `node scripts/test/tracking-plan-contract-proof.mjs`, `node scripts/test/tracking-plan-runtime-proof.mjs`, and `node scripts/test/tracking-plan-service-read-model-proof.mjs` passed locally.
- [x] Proof artifacts under `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/`, including tracked `proof-summary.json` with `minimumSeriousMvpAuditSummary` and generated `00-run-metadata.json` with `minimumSeriousMvpAudit`.
- [x] Product doc/checklist updates: owning feature doc, feature list, capability checklist, implementation checklist, tracking snapshot, and package READMEs updated.
- [x] Known gaps/manual-required states: Android/iOS physical behavior, precise desktop location, provider delivery, full live parent/child UI, browser-to-service live-data screenshots, full product evidence drawer UI, authority proof, production pilot, and full root-gate validation remain proof-gated as applicable.
- [x] Workpack id and branch: `codex/tracking-proof-gap-closure`.
- [x] Touched files: pre-device proof script, root test script wiring, tracking feature doc, tracking README, implementation checklist, WP08-WP12, WP30, WP32, WP33, and generated pre-device proof artifacts.
- [x] Validation commands and results: `npm run test:tracking-plan-pre-device-proof` passed locally; it reran tracking contract/runtime/service proof, child Android device artifact gate, child iOS entitlement proof, and mobile child-agent aggregate proof.
- [x] Proof artifacts under `output/tracking-plan-proof/pre-device-gap-closure/`, including `proof-summary.json`, `android-studio-local-proof-plan.json`, `ios-simulator-local-proof-plan.json`, `wsl-local-proof-plan.json`, `physical-device-manual-proof-plan.json`, and `16-validation-commands.log`.
- [x] Product doc/checklist updates: owning feature doc and tracking checklist updated; the central `docs/product-capability-checklist.md` row update is queued through the hub DOC_DELTA queue with the pre-device proof gate while keeping Android Studio/emulator, iOS simulator, WSL/local, physical-device, authority, hosted UI, and production proof as gaps.
- [x] Known gaps/manual-required states: Android Studio/emulator runtime, iOS simulator/local, WSL/local replay, physical Android/iOS behavior, authority-enrolled proof, full hosted UI/accessibility, production pilot, and richer live UI/read-model/product claims remain proof-gated as applicable.
- [x] Workpack id and branch: `codex/tracking-read-model-portal-proof`.
- [x] Touched files: `scripts/test/tracking-plan-hosted-ui-proof.mjs`,
      `scripts/test/tracking-plan-pre-device-proof.mjs`, `package.json`,
      WP30/WP33/checklist docs, feature doc, tracking README, and generated
      WP30/pre-device proof artifacts.
- [x] Validation commands and results:
      `cmd /c npm run test:tracking-plan-hosted-ui-proof` passed;
      `cmd /c npm run test:tracking-plan-pre-device-proof` passed.
- [x] Proof artifacts:
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/17-accessibility-proof.json`;
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/policy-tracking-hosted-ui-proof.png`;
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/12-playwright-proof.log`;
      refreshed `output/tracking-plan-proof/pre-device-gap-closure/proof-summary.json`.
- [x] Product doc/checklist updates: owning feature doc, tracking README,
      implementation checklist, WP30, and WP33 updated. Central product
      checklist remains primary-owned and was not edited in this branch.
- [x] Known gaps/manual-required states: child-device UI, browser-to-service
      live-data screenshots, full product evidence drawer UI, Android Studio/emulator
      runtime proof, iOS simulator/local proof, WSL/local replay, physical
      Android/iOS behavior, authority-enrolled proof, and production pilot
      remain proof-gated.
