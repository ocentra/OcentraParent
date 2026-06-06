# WP32 Journal SQLite And Read-Model Proof

## Purpose

Store, replay, query, delete, and cite tracking evidence through the shared
journal/SQLite/read-model path before portal, policy, AI, or reports consume it.

## Source Inputs

- `docs/features/evidence-store-query.md`
- `docs/expectations/evidence-storage.md`
- `docs/expectations/data-custody.md`
- `docs/plans/tracking-plan/v0-5-location-test-blueprint.md`

## Target State

Location, status, geofence, check-in, acknowledgement, alert, AI, and retention
events are journaled, replayable, queryable, deletable, and cited by read
models.

## Tests And Proof

Proof root: `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/`

- `03-runtime-location-evidence.json`
- `04-device-status-proof.json`
- `05-geofence-transition-proof.json`
- `10-journal-sqlite-proof.json`
- `14-retention-delete-proof.json`
- `18-service-read-model-proof.json`
- `19-wsl-local-replay-proof.json`
- `20-service-data-ui-proof.json`
- `21-product-surface-summary-proof.json`
- `22-report-policy-consumer-proof.json`
- `23-family-dashboard-rollup-proof.json`
- `24-retention-settings-read-model-proof.json`
- `16-validation-commands.log`
- Pre-device gate:
  `output/tracking-plan-proof/pre-device-gap-closure/proof-summary.json`

## AI Worker Checklist

- [ ] Journal evidence before portal/policy/AI use.
- [x] Add replay/query/delete tests.
- [x] Add tombstone proof.
- [ ] Ensure AI/report/policy cite stored refs.
- [ ] Keep Ocentra-hosted storage off by default.
- [x] Include the P2 service read-model proof in the pre-device gate.
- [x] Record P3 WSL/local replay proof for the read-model proof stack.
- [x] Expose live portal citation rows for service-backed read-model rows and
      retention tombstones.
- [x] Mark and screenshot the hosted service-backed citation detail card so the
      evidence refs and no-product-claim boundary are captured in hosted proof
      output.
- [x] Mark and screenshot the hosted read-only evidence drawer card from the
      selected service-backed citation without adding policy evaluation,
      dispatch, delivery, device, authority, or product-ready claims.
- [x] Expose hosted parent service-data coverage for service-backed read-model
      row counts, kinds, custody, capability, and active/deleted evidence refs.
- [x] Expose active product-surface summary fields for kind/device/capability
      counts and latest active row metadata without reintroducing deleted
      history.
- [x] Expose family dashboard rollup rows for active family summary,
      child-attention summary, and retention-audit summary without claiming
      portal UI completion or child-device/runtime behavior.
- [x] Expose retention settings read-model rows for retention window,
      delete-after-alert, parent export, remote-sync disabled, and remote-AI
      disabled state without claiming service mutation or product UI.
- [x] Render the retention settings read-model rows on the hosted parent route
      without claiming writable settings, service mutation, platform runtime,
      child-device delivery, provider delivery, authority, physical-device
      execution, or production readiness.

## Where We Are

This workpack has P0 contract proof, P1 Rust ActivityStore SQLite ingest proof,
and P2 service-command proof for tracking event kinds from
`codex/tracking-plan-full-scope` under the proof root below. The service proof
adds a narrow `agent.activity.tracking.read-model.get` command that returns
SQLite tracking rows and citation IDs through `trackingReadModel`; the service
proof now also exposes retention-delete rows as tombstone replay rows with
active/tombstone counts, deleted-at metadata, and deleted evidence citation ID
summaries. The same service read model now exposes active product-surface
summary fields for latest active event metadata plus active kind, device, and
capability-status counts; those counts are derived from the same SQLite rows
and explicitly exclude retention tombstones. The parent portal consumes that
event as a narrow live summary plus live citation rows on the `policy-tracking`
route. The hosted parent route now also marks the service-backed citation detail
card and captures it in
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-citation-detail.png`.
The hosted route now also renders a read-only evidence drawer from the selected
service-backed citation and captures it in
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-evidence-drawer.png`.
The hosted parent route now also exposes a service-data coverage panel
backed by the parsed
`trackingReadModel` payload, including active/tombstone counts, latest tombstone
metadata, kind coverage, custody/capability, active evidence references, deleted
evidence references, and `productClaimReady=false`. Full UI, platform replay,
export, broader product read models, and physical-device product claims are not
claimed beyond the proof
state recorded in `proof-summary.json`, `10-journal-sqlite-proof.json`,
`14-retention-delete-proof.json`, `18-service-read-model-proof.json`,
`20-service-data-ui-proof.json`, `21-product-surface-summary-proof.json`,
`22-report-policy-consumer-proof.json`,
`23-family-dashboard-rollup-proof.json`, and the implementation checklist.
The family dashboard rollup proof derives active family summary,
child-attention summary, and retention-audit summary rows from the existing
service read-model/product-surface/report-consumer proof refs while keeping
portal UI, child-device delivery, provider delivery, notification receipt,
physical-device, authority, and product-ready claims false.
The retention settings read-model proof derives retention window,
delete-after-alert, parent export, remote-sync disabled, and remote-AI disabled
rows from the existing WP07 retention/delete/export and WP32 service read-model
proof refs while keeping service mutation, portal UI, platform runtime,
child-device delivery, provider delivery, notification receipt, physical-device,
authority, and product-ready claims false.
The hosted parent route now renders those retention settings read-model rows as
a narrow proof card and captures
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-retention-settings.png`
while keeping writable settings, service mutation, platform runtime,
child-device delivery, provider delivery, physical-device, authority, and
product-ready claims false.
The pre-device proof gate now reruns this service proof and records the
remaining broader read-model, full UI, hosted accessibility, and platform replay
gaps before device work starts.
The WSL/local replay proof now records WSL2/Ubuntu toolchain evidence, the
linked-worktree Git mapping required by this Windows-hosted checkout, contract
build output, the service read-model proof, and the Rust core tracking
read-model test in `19-wsl-local-replay-proof.json` and
`output/tracking-plan-proof/wsl-local-replay/proof.json`. That proof is P3
local-machine evidence only; it does not claim Android/iOS physical behavior,
authority, hosted UI/accessibility, provider delivery, or production readiness.

## Where We Want To Be

This workpack can be assigned independently, implemented against the owning domain boundaries, validated with real contracts or platform proof, and reported without leaving unclear tracking claims behind.

## Scope

- Re-read only the source inputs listed in this workpack and the shared source index.
- Update the owned contracts, runtime paths, UI paths, or docs named by the implementation worker before editing.
- Keep no-claim boundaries explicit for LAN/IP hints, nearby-place ambiguity, AI evidence, background behavior, and remote sync.
- Update the feature doc and product capability checklist only when status, proof, or gaps change.

## Touched Paths

- docs/plans/tracking-plan/workpacks/32-journal-sqlite-and-read-model-proof.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Full UI/report/policy consumers for the active product-surface summary, full
  parent/child UI beyond the hosted parent route, platform physical-device
  replay, export, provider, authority, and production claims remain
  manual-required until the assigned proof artifacts exist. The family
  dashboard rollup rows are read-model consumer proof only; they are not
  rendered portal UI or child-device runtime proof.
- Retention settings read-model rows are not writable product settings, service
  mutation, or live retention UI proof.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [x] Workpack id and branch:
      `codex/tracking-journal-read-model-replay-proof`.
- [x] Touched files: Rust ActivityStore/protocol files, tracking
      contract/runtime files, proof scripts, tracking plan docs, checklist, and
      this workpack doc.
- [x] Validation commands and results:
      focused service proof command
      `node scripts/test/tracking-plan-service-read-model-proof.mjs` passed.
- [x] Proof artifacts under
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/`,
      including `10-journal-sqlite-proof.json`,
      `14-retention-delete-proof.json`, and
      `18-service-read-model-proof.json`.
- [x] Product doc/checklist updates: owning feature doc, implementation
      checklist, and this workpack doc updated for the P2 service-backed
      tombstone replay proof; product capability checklist update remains a
      primary-owned DOC_DELTA.
- [x] Known gaps/manual-required states: hosted portal screenshot/accessibility
      proof, broader read models, full UI, platform replay, export,
      Android/iOS physical proof, provider delivery, and notifications remain
      proof-gated as applicable.
- [x] Workpack id and branch: `codex/tracking-wsl-local-replay-proof`.
- [x] Touched files: WSL proof script, root test script wiring, tracking
      feature doc, tracking README, implementation checklist, WP32, WP33, and
      generated WSL proof artifacts.
- [x] Validation commands and results:
      `npm run test:tracking-plan-wsl-local-proof` passed locally.
- [x] Proof artifacts under
      `output/tracking-plan-proof/wsl-local-replay/` and
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/19-wsl-local-replay-proof.json`.
- [x] Product doc/checklist updates: owning feature doc, tracking checklist,
      README, WP32, and WP33 updated; central capability row delta queued
      through the hub instead of editing `docs/product-capability-checklist.md`.
- [x] Known gaps/manual-required states: richer read models, full UI, hosted
      UI/accessibility, Android/iOS physical-device proof, authority, provider
      delivery, notifications, and production proof remain proof-gated as
      applicable.
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
- [x] Known gaps/manual-required states: broader product read models, full
      parent/child UI, hosted UI/accessibility, Android/iOS physical-device
      proof, authority, provider delivery, notifications, and production proof
      remain proof-gated.
- [x] Workpack id and branch:
      `codex/tracking-service-data-ui-proof`.
- [x] Touched files: portal tracking status renderer/tests, portal text token
      source/tests, service-data UI proof script, root script wiring, tracking
      feature doc, implementation checklist, WP30, WP32, and generated WP30/WP32
      service-data UI proof artifacts.
- [x] Validation commands and results:
      `npm run test:tracking-plan-service-data-ui-proof` passed locally after
      focused text-domain and portal tracking status panel tests.
- [x] Proof artifacts under
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/18-service-data-ui-proof.json`,
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/20-service-data-ui-proof.json`,
      and `test-results/tracking-plan-service-data-ui-proof/proof.json`.
- [x] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP30, and WP32 updated; central capability row delta queued
      through the hub instead of editing `docs/product-capability-checklist.md`.
- [x] Known gaps/manual-required states: broader product read models, full
      parent/child UI beyond the hosted parent route, Android/iOS
      physical-device proof, authority, provider delivery, notifications, and
      production proof remain proof-gated.
- [x] Workpack id and branch:
      `codex/tracking-read-model-product-surface-proof`.
- [x] Touched files: Rust protocol/core/service tracking read-model files and
      tests, TypeScript tracking read-model parser/tests, service proof script,
      tracking feature doc, implementation checklist, WP32, module READMEs, and
      generated WP32 proof artifacts.
- [x] Validation commands and results:
      focused parser and Rust tracking read-model tests passed before proof
      generation; `node scripts/test/tracking-plan-service-read-model-proof.mjs`
      reruns the full focused WP32 proof stack.
- [x] Proof artifacts under
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/21-product-surface-summary-proof.json`
      plus refreshed `14-retention-delete-proof.json`,
      `18-service-read-model-proof.json`, and `proof-summary.json`.
- [x] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP32, and module READMEs updated; central capability row delta
      queued through the hub instead of editing
      `docs/product-capability-checklist.md`.
- [x] Known gaps/manual-required states: full UI/report/policy consumers,
      parent/child UI beyond the hosted parent route, Android/iOS
      physical-device proof, authority, provider delivery, notifications, and
      production proof remain proof-gated.
- [x] Workpack id and branch:
      `codex/tracking-family-dashboard-rollup-proof`.
- [x] Touched files: parent-domain family dashboard rollup proof source/test,
      proof harness, tracking feature doc, implementation checklist, WP32, and
      generated WP32 proof artifacts.
- [x] Validation commands and results:
      `node scripts/test/tracking-family-dashboard-rollup-proof.mjs` passed.
- [x] Proof artifacts under
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/23-family-dashboard-rollup-proof.json`,
      `output/tracking-plan-proof/family-dashboard-rollup-proof/`, and
      `test-results/tracking-family-dashboard-rollup-proof/proof.json`.
- [x] Product doc/checklist updates: owning feature doc, implementation
      checklist, and WP32 updated; central capability checklist row was not
      edited because another lane owns `docs/product-capability-checklist.md`.
- [x] Known gaps/manual-required states: rendered portal dashboard UI,
      child-device delivery/runtime execution, Android/iOS physical-device
      proof, authority, provider delivery, notifications, and production proof
      remain proof-gated.
- [x] Workpack id and branch:
      `codex/tracking-retention-settings-read-model-proof`.
- [x] Touched files: parent-domain retention settings read-model proof
      source/test, proof harness, tracking feature doc, implementation
      checklist, WP07, WP32, and generated WP07/WP32 proof artifacts.
- [x] Validation commands and results:
      `node scripts/test/tracking-retention-settings-read-model-proof.mjs`
      passed locally.
- [x] Proof artifacts under
      `output/tracking-plan-proof/07-retention-and-custody-model/18-retention-settings-read-model-proof.json`,
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/24-retention-settings-read-model-proof.json`,
      and `test-results/tracking-retention-settings-read-model-proof/proof.json`.
- [x] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP07, and WP32 updated; central capability checklist row is
      not edited by this worker while another lane owns it.
- [x] Known gaps/manual-required states: actual writable product settings,
      live service-backed retention UI, service mutation, platform runtime,
      child-device delivery/runtime execution, Android/iOS physical-device
      proof, authority, provider delivery, notification receipts, and
      production proof remain proof-gated.
- [x] Workpack id and branch:
      `codex/tracking-hosted-citation-detail-proof`.
- [x] Touched files: portal renderers, portal-domain proof marker, hosted
      Playwright proof spec, tracking feature doc, implementation checklist,
      WP30, WP32, portal README, and generated hosted proof artifacts.
- [x] Validation commands and results: pending final local hosted proof refresh
      and guard run before PR-ready report.
- [x] Proof artifacts under
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-citation-detail.png`
      and `test-results/tracking-plan-hosted-ui-proof/`.
- [x] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP30, WP32, and portal README updated; central capability row
      delta stays hub-sequenced instead of editing
      `docs/product-capability-checklist.md`.
- [x] Known gaps/manual-required states: broader product read models, full
      parent/child UI beyond the hosted parent route, Android/iOS physical-device
      proof, authority, provider delivery, notifications, and production proof
      remain proof-gated.
- [x] Workpack id and branch:
      `codex/tracking-evidence-drawer-hosted-ui-proof`.
- [x] Touched files: WP32 doc plus hosted portal proof files in WP30; no new
      read-model storage or service contracts were added.
- [x] Validation commands and results: pending final focused hosted proof refresh
      and guard run before PR-ready report.
- [x] Proof artifacts under
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/20-evidence-drawer-hosted-ui-proof.json`
      and `test-results/tracking-plan-hosted-ui-proof/`.
- [x] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP30, and WP32 updated; central capability checklist row stays
      hub-sequenced instead of editing `docs/product-capability-checklist.md`.
- [x] Known gaps/manual-required states: the evidence drawer is hosted
      read-only citation display only; policy evaluation, action dispatch,
      child-device delivery/runtime execution, provider delivery, physical-device
      proof, authority, and product readiness remain proof-gated.
