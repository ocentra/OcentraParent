# WP32 Journal SQLite And Read-Model Proof

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `WP32 Journal SQLite And Read-Model Proof`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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
- `25-retention-settings-writer-boundary-proof.json`
- `26-retention-settings-mutation-proof.json`
- `27-retention-settings-write-command-proof.json`
- `28-report-export-read-model-proof.json`
- `30-ai-stored-ref-consumer-proof.json`
- `31-hosted-storage-default-boundary-proof.json`
- `32-report-policy-consumer-hosted-ui-proof.json`
- `33-retention-local-service-state-proof.json`
- `34-retention-durable-settings-proof.json`
- `35-retention-product-readiness-proof.json`
- `16-validation-commands.log`
- Pre-device gate:
  `output/tracking-plan-proof/pre-device-gap-closure/proof-summary.json`

## AI Worker Checklist

- [ ] Journal evidence before portal/policy/AI use.
- [ ] Add replay/query/delete tests.
- [ ] Add tombstone proof.
- [ ] Ensure AI/report/policy cite stored refs.
- [ ] Keep Ocentra-hosted storage off by default.
- [ ] Include the P2 service read-model proof in the pre-device gate.
- [ ] Record P3 WSL/local replay proof for the read-model proof stack.
- [ ] Expose live portal citation rows for service-backed read-model rows and
      retention tombstones.
- [ ] Mark and screenshot the hosted service-backed citation detail card so the
      evidence refs and no-product-claim boundary are captured in hosted proof
      output.
- [ ] Mark and screenshot the hosted read-only evidence drawer card from the
      selected service-backed citation without adding policy evaluation,
      dispatch, delivery, device, authority, or product-ready claims.
- [ ] Expose hosted parent service-data coverage for service-backed read-model
      row counts, kinds, custody, capability, and active/deleted evidence refs.
- [ ] Expose active product-surface summary fields for kind/device/capability
      counts and latest active row metadata without reintroducing deleted
      history.
- [ ] Expose family dashboard rollup rows for active family summary,
      child-attention summary, and retention-audit summary without claiming
      portal UI completion or child-device/runtime behavior.
- [ ] Expose retention settings read-model rows for retention window,
      delete-after-alert, parent export, remote-sync disabled, and remote-AI
      disabled state without claiming service mutation or product UI.
- [ ] Render the retention settings read-model rows on the hosted parent route
      without claiming writable settings, service mutation, platform runtime,
      child-device delivery, provider delivery, authority, physical-device
      execution, or production readiness.
- [ ] Add retention settings writer-boundary preflight rows for retention
      window, delete-after-alert, parent export, remote-sync disabled, and
      remote-AI disabled write intents without claiming executed service
      mutation, live retention UI, platform runtime, child-device delivery,
      provider delivery, authority, physical-device execution, or production
      readiness.
- [ ] Add local executed service mutation proof for the same retention settings
      rows while preserving remote-sync disabled, remote-AI disabled, and no
      platform/device/product-ready claims.
- [ ] Add typed service transport proof for a retention settings write
      command local-execution result with a local durable settings store ref
      while preserving no writable product UI, platform, device, provider,
      authority, notification receipt, or product-ready claims.
- [ ] Render the service-backed retention write result in hosted portal proof
      from the typed command response, including local service state revision
      evidence and durable store ref, without claiming writable product settings, platform, device,
      provider, authority, notification receipt, or product-ready behavior.
- [ ] Derive retention local service state readback rows from the accepted
      write-command proof, including applied values, service state revision, and
      snapshot ref plus durable store ref, without claiming writable product
      settings, platform, device, provider, authority, notification receipt, or
      product-ready behavior.
- [ ] Add redacted report/export read-model packet rows that compose service
      read-model, report/policy consumer, family dashboard rollup, and
      retention settings proof refs without claiming raw location payload
      export, portal UI, platform runtime, device delivery, authority, provider,
      notification receipt, or product-ready behavior.
- [ ] Require parent report summary, policy drill-in, and retention audit
      export consumer rows to cite stored journal refs and stored read-model row
      refs before report/policy use without claiming AI execution, portal
      completion, platform runtime, device delivery, provider delivery,
      authority, or product-ready behavior.
- [ ] Render the report/policy consumer rows on the hosted parent route without
      claiming AI execution, product policy mutation, platform runtime,
      child-device delivery, provider delivery, notification receipt ingestion,
      physical-device proof, authority, production, or product-ready behavior.
- [ ] Require AI parent-report, policy-drill-in, and metadata-fallback consumer
      rows to cite stored journal refs, stored read-model row refs, provider
      route proof refs, and report/policy consumer proof refs before AI
      report/policy use without claiming model execution, assistant policy
      writes, assistant enforcement, device runtime, provider delivery,
      authority, production behavior, or product-ready behavior.
- [ ] Prove that Ocentra-hosted storage is not the default tracking journal,
      SQLite read-model, parent export, AI context, or remote-sync custody path
      without claiming remote upload, portal UI, service mutation, platform
      runtime, device delivery, authority, production, or product-ready
      behavior.
- [ ] Derive a retention product-readiness blocker row from the durable
      settings proof so local durable settings readiness stays separate from
      writable product settings execution, platform runtime, child-device
      delivery, provider delivery, notification receipt ingestion,
      physical-device proof, authority enrollment, production workers, and
      product-ready behavior.
- [ ] Derive retention product-settings writable execution proof from the
      accepted local service state row, including source proof refs, service
      revision, snapshot ref, durable store ref, applied values, and no-claim
      boundaries.

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
route. That consumer now prefers the additive `latestActive*` metadata when it
is present, falls back to the legacy top-level latest-event fields when it is
absent, and keeps deleted-history evidence refs out of the narrow live summary
while leaving tombstone citation rows visible. The hosted parent route now also
marks the service-backed citation detail card and captures it in
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-citation-detail.png`.
The hosted route now also renders a read-only evidence drawer from the selected
service-backed citation and captures it in
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-evidence-drawer.png`.
The hosted parent route now also exposes a service-data coverage panel
backed by the parsed
`trackingReadModel` payload, including active/tombstone counts, latest tombstone
metadata, kind coverage, custody/capability, active evidence references, deleted
evidence references, and `productClaimReady=false`. That coverage consumer now
uses the additive active kind/device/capability count buckets when they are
present and falls back to the legacy row-derived values when they are absent.
The shared `portal-domain` owner now also has a direct `tests/unit/tracking-status-panel.test.ts`
unit test covering the same active-summary and legacy-fallback mapping without
depending on the blocked `apps/portal` workspace suite. The refreshed
`20-service-data-ui-proof.json` also carries a service-backed citation matrix
that binds the coverage card back to the same service command/event/payload,
live citation fields, active evidence refs, tombstone deleted-evidence refs,
and no-claim boundary set. Full UI, platform replay,
export, broader product read models, and physical-device product claims are not
claimed beyond the proof
state recorded in `proof-summary.json`, `10-journal-sqlite-proof.json`,
`14-retention-delete-proof.json`, `18-service-read-model-proof.json`,
`20-service-data-ui-proof.json`, `21-product-surface-summary-proof.json`,
`22-report-policy-consumer-proof.json`,
`23-family-dashboard-rollup-proof.json`,
`27-retention-settings-write-command-proof.json`,
`28-report-export-read-model-proof.json`,
`33-retention-local-service-state-proof.json`,
`34-retention-durable-settings-proof.json`,
the hosted UI proof output, and the implementation checklist.
The report/policy consumer proof now requires the parent report summary,
policy evidence drill-in, and retention audit export rows to carry stored
journal refs plus stored read-model row refs before report/policy use. The
repeatable `node scripts/test/tracking-report-policy-consumer-proof.mjs`
command records the stored-ref row counts in
`output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/22-report-policy-consumer-proof.json`
while keeping AI execution, portal completion, platform runtime, child-device
delivery, provider delivery, notification receipt, physical-device, authority,
production, and product-ready claims false.
The hosted parent `policy-tracking` route now renders those same report/policy
consumer rows as a read-only proof card, captures
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-report-policy-consumer.png`,
and records
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/25-report-policy-consumer-hosted-ui-proof.json`
plus
`output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/32-report-policy-consumer-hosted-ui-proof.json`
without claiming AI execution, product policy mutation, platform runtime,
child-device delivery, provider delivery, notification receipt ingestion,
physical-device, authority, production, or product-ready report/policy
behavior.
The AI stored-ref consumer proof now requires AI parent-report context,
policy-drill-in context, and metadata-fallback context rows to carry stored
journal refs, stored read-model row refs, provider-route proof refs, and
report/policy consumer proof refs before AI report/policy use. The repeatable
`node scripts/test/tracking-ai-stored-ref-consumer-proof.mjs` command records
the row counts in
`output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/30-ai-stored-ref-consumer-proof.json`
while keeping model execution, assistant policy writes, assistant enforcement,
child-device runtime, provider delivery, notification receipt, physical-device,
authority, production, and product-ready claims false.
The hosted storage default boundary proof now requires tracking journal,
SQLite read-model, parent export, AI context, and remote-sync default rows to
stay local, parent-owned, or remote-disabled by default. The repeatable
`node scripts/test/tracking-hosted-storage-default-boundary-proof.mjs` command
records this in
`output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/31-hosted-storage-default-boundary-proof.json`
while keeping Ocentra-hosted default storage, raw location remote upload,
SQLite snapshot remote upload, remote sync, remote AI, portal UI, service
mutation, platform runtime, device delivery, authority, production, and
product-ready claims false.
The hosted parent route now consumes the typed retention settings local service
write response as a command/result rendering path and captures the accepted
result, local service state revision, local durable settings persistence, plus
mutation proof refs in
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/17-hosted-ui-proof.json`.
It does not claim product-ready service mutation, platform replay,
child-device delivery, provider delivery, authority, notification receipt
ingestion, durable production settings, or product-ready behavior.
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
The retention settings writer-boundary proof derives preflight write intents for
the same five setting rows from the existing WP07/WP32 read-model proof refs,
keeps remote sync and remote AI disabled, and keeps executed service mutation,
live retention UI, platform runtime, child-device delivery, provider delivery,
notification receipt, physical-device, authority, and product-ready claims false.
The retention settings mutation proof applies those five authorized write
intents to local settings rows, keeps remote sync and remote AI disabled, and
keeps live writable UI, platform runtime, child-device delivery, provider
delivery, notification receipt, physical-device, authority, production, and
product-ready claims false.
The retention settings write-command proof adds a typed
`agent.activity.tracking.retention-settings.write` command and matching service
WebSocket response payload, validates the TypeScript request/result parsers,
Rust protocol serialization, and Rust service response, and now carries the
applied local retention values from the typed request plus a local durable
settings store ref. It keeps writable product UI, product-ready service behavior, platform runtime, child-device delivery,
provider delivery, notification receipt, physical-device, authority,
production, and product-ready claims false.
The retention local service state proof derives a parent-domain readback row from
that accepted write-command result, requiring the local service state revision,
snapshot ref, durable store ref, source read-model refs, source mutation refs,
and applied retention values while keeping writable product settings, platform
runtime, child-device delivery, provider delivery, notification receipt,
physical-device, authority, production, and product-ready claims false.
The hosted parent route now renders those retention settings read-model rows as
a narrow proof card and captures
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-retention-settings.png`
while keeping writable settings, service mutation, platform runtime,
child-device delivery, provider delivery, physical-device, authority, and
product-ready claims false.
The report/export read-model proof derives redacted report export, retention
audit export, family dashboard summary, and policy drill-in export packet rows
from the existing service read-model, product-surface summary, report/policy
consumer, family dashboard rollup, and retention settings proof refs. It keeps
raw location payload export, portal UI, service mutation, platform runtime,
child-device delivery, provider delivery, notification receipt ingestion,
physical-device behavior, authority, and product-ready claims false.
The hosted parent `policy-tracking` route now renders those redacted
report/export packet rows, captures
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-report-export.png`,
and records
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/21-report-export-hosted-ui-proof.json`
plus
`output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/29-report-export-hosted-ui-proof.json`
without changing the raw-location export, service mutation, platform runtime,
child-device delivery, provider delivery, notification receipt, authority,
physical-device, production, or product-ready export gaps.
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

- Broader rendered UI/report/policy surfaces for the active product-surface summary, full
  parent/child UI beyond the hosted parent route, platform physical-device
  replay, export, provider, authority, and production claims remain
  manual-required until the assigned proof artifacts exist. The family
  dashboard rollup rows are read-model consumer proof only; they are not
  rendered portal UI or child-device runtime proof.
- Retention settings read-model and mutation rows are not writable product UI or
  live retention UI proof.
- Retention settings write-command proof is local service execution plus local
  service state revision and durable store-ref proof, not writable product UI,
  production hardening, or product-ready service execution proof.
- Retention local service state readback proof is a derived parent-domain
  evidence row from the accepted write command, not platform/device runtime
  proof.
- Report/export read-model proof is redacted evidence-ref packet readiness plus
  hosted packet rendering only, not raw location payload export, service
  mutation, platform runtime, child-device/runtime execution, or product-ready
  export delivery.
- AI stored-ref consumer readiness is now proved for parent-report,
  policy-drill-in, and metadata-fallback contexts, but AI model execution and
  any AI/report/policy product-completion claim remain manual-required.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch:
      `codex/tracking-journal-read-model-replay-proof`.
- [ ] Touched files: Rust ActivityStore/protocol files, tracking
      contract/runtime files, proof scripts, tracking plan docs, checklist, and
      this workpack doc.
- [ ] Validation commands and results:
      focused service proof command
      `node scripts/test/tracking-plan-service-read-model-proof.mjs` passed.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/`,
      including `10-journal-sqlite-proof.json`,
      `14-retention-delete-proof.json`, and
      `18-service-read-model-proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, and this workpack doc updated for the P2 service-backed
      tombstone replay proof; product capability checklist update remains a
      primary-owned DOC_DELTA.
- [ ] Known gaps/manual-required states: hosted portal screenshot/accessibility
      proof, broader read models, full UI, platform replay, export,
      Android/iOS physical proof, provider delivery, and notifications remain
      proof-gated as applicable.
- [ ] Workpack id and branch: `codex/tracking-wsl-local-replay-proof`.
- [ ] Touched files: WSL proof script, root test script wiring, tracking
      feature doc, tracking README, implementation checklist, WP32, WP33, and
      generated WSL proof artifacts.
- [ ] Validation commands and results:
      `npm run test:tracking-plan-wsl-local-proof` passed locally.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/wsl-local-replay/` and
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/19-wsl-local-replay-proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, tracking checklist,
      README, WP32, and WP33 updated; central capability row delta queued
      through the hub instead of editing `docs/product-capability-checklist.md`.
- [ ] Known gaps/manual-required states: richer read models, full UI, hosted
      UI/accessibility, Android/iOS physical-device proof, authority, provider
      delivery, notifications, and production proof remain proof-gated as
      applicable.
- [ ] Workpack id and branch:
      `codex/tracking-live-service-citation-proof`.
- [ ] Touched files: portal tracking status renderer/tests, service proof
      script, tracking feature doc, implementation checklist, WP30, WP32,
      WP33, and generated WP32 proof artifacts.
- [ ] Validation commands and results:
      `node scripts/test/tracking-plan-service-read-model-proof.mjs` passed.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/`,
      including `14-retention-delete-proof.json`,
      `18-service-read-model-proof.json`, and `proof-summary.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP30, WP32, and WP33 updated; central capability row delta
      queued through the hub instead of editing
      `docs/product-capability-checklist.md`.
- [ ] Known gaps/manual-required states: broader product read models, full
      parent/child UI, hosted UI/accessibility, Android/iOS physical-device
      proof, authority, provider delivery, notifications, and production proof
      remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-service-data-ui-proof`.
- [ ] Touched files: portal tracking status renderer/tests, portal text token
      source/tests, service-data UI proof script, root script wiring, tracking
      feature doc, implementation checklist, WP30, WP32, and generated WP30/WP32
      service-data UI proof artifacts.
- [ ] Validation commands and results:
      `npm run test:tracking-plan-service-data-ui-proof` passed locally after
      focused text-domain and portal tracking status panel tests.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/18-service-data-ui-proof.json`,
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/20-service-data-ui-proof.json`,
      and `test-results/tracking-plan-service-data-ui-proof/proof.json`.
- [ ] Service-backed citation matrix recorded in those artifacts for the
      command/event/payload, live citation field set, active evidence refs,
      tombstone deleted-evidence refs, and no-claim boundaries.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP30, and WP32 updated; central capability row delta queued
      through the hub instead of editing `docs/product-capability-checklist.md`.
- [ ] Known gaps/manual-required states: broader product read models, full
      parent/child UI beyond the hosted parent route, Android/iOS
      physical-device proof, authority, provider delivery, notifications, and
      production proof remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-read-model-product-surface-proof`.
- [ ] Touched files: Rust protocol/core/service tracking read-model files and
      tests, TypeScript tracking read-model parser/tests, service proof script,
      tracking feature doc, implementation checklist, WP32, module READMEs, and
      generated WP32 proof artifacts.
- [ ] Validation commands and results:
      focused parser and Rust tracking read-model tests passed before proof
      generation; `node scripts/test/tracking-plan-service-read-model-proof.mjs`
      reruns the full focused WP32 proof stack.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/21-product-surface-summary-proof.json`
      plus refreshed `14-retention-delete-proof.json`,
      `18-service-read-model-proof.json`, and `proof-summary.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP32, and module READMEs updated; central capability row delta
      queued through the hub instead of editing
      `docs/product-capability-checklist.md`.
- [ ] Known gaps/manual-required states: full UI/report/policy consumers,
      parent/child UI beyond the hosted parent route, Android/iOS
      physical-device proof, authority, provider delivery, notifications, and
      production proof remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-family-dashboard-rollup-proof`.
- [ ] Touched files: parent-domain family dashboard rollup proof source/test,
      proof harness, tracking feature doc, implementation checklist, WP32, and
      generated WP32 proof artifacts.
- [ ] Validation commands and results:
      `node scripts/test/tracking-family-dashboard-rollup-proof.mjs` passed.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/23-family-dashboard-rollup-proof.json`,
      `output/tracking-plan-proof/family-dashboard-rollup-proof/`, and
      `test-results/tracking-family-dashboard-rollup-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, and WP32 updated; central capability checklist row was not
      edited because another lane owns `docs/product-capability-checklist.md`.
- [ ] Known gaps/manual-required states: rendered portal dashboard UI,
      child-device delivery/runtime execution, Android/iOS physical-device
      proof, authority, provider delivery, notifications, and production proof
      remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-retention-settings-read-model-proof`.
- [ ] Touched files: parent-domain retention settings read-model proof
      source/test, proof harness, tracking feature doc, implementation
      checklist, WP07, WP32, and generated WP07/WP32 proof artifacts.
- [ ] Validation commands and results:
      `node scripts/test/tracking-retention-settings-read-model-proof.mjs`
      passed locally.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/07-retention-and-custody-model/18-retention-settings-read-model-proof.json`,
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/24-retention-settings-read-model-proof.json`,
      and `test-results/tracking-retention-settings-read-model-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP07, and WP32 updated; central capability checklist row is
      not edited by this worker while another lane owns it.
- [ ] Known gaps/manual-required states: actual writable product settings,
      live service-backed retention UI, service mutation, platform runtime,
      child-device delivery/runtime execution, Android/iOS physical-device
      proof, authority, provider delivery, notification receipts, and
      production proof remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-retention-settings-writer-boundary-proof`.
- [ ] Touched files: parent-domain retention settings writer-boundary proof
      source/test, proof harness, tracking feature doc, implementation
      checklist, WP07, WP32, and generated WP07/WP32/WP33 proof artifacts.
- [ ] Validation commands and results:
      `node scripts/test/tracking-retention-settings-writer-boundary-proof.mjs`
      passed locally.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/07-retention-and-custody-model/19-retention-settings-writer-boundary-proof.json`,
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/25-retention-settings-writer-boundary-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/25-retention-settings-writer-boundary-proof.json`,
      and `test-results/tracking-retention-settings-writer-boundary-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP07, and WP32 updated; central capability checklist row is
      not edited by this worker while another lane owns it.
- [ ] Known gaps/manual-required states: executed service mutation, live
      service-backed retention UI, platform runtime, child-device delivery,
      Android/iOS physical-device proof, authority, provider delivery,
      notification receipts, and production proof remain proof-gated.
- [ ] Workpack id and branch: `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: parent-domain AI stored-ref consumer proof source/test,
      proof harness, tracking feature doc, implementation checklist, WP24,
      WP32, product capability checklist, and generated WP24/WP32/WP33 proof
      artifacts.
- [ ] Validation commands and results:
      `node scripts/test/tracking-ai-stored-ref-consumer-proof.mjs` passed
      locally.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/24-ai-provider-routing/19-ai-stored-ref-consumer-proof.json`,
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/30-ai-stored-ref-consumer-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/30-ai-stored-ref-consumer-proof.json`,
      and `test-results/tracking-ai-stored-ref-consumer-proof/proof.json`.
- [ ] Product doc/checklist updates: owning tracking feature doc, tracking
      implementation checklist, WP24, WP32, and central product capability
      checklist row updated.
- [ ] Known gaps/manual-required states: AI model execution, assistant policy
      writes, assistant enforcement, full rendered UI/report/policy surfaces,
      platform runtime, child-device delivery, Android/iOS physical-device
      proof, authority, provider delivery, notification receipts, production
      behavior, and product-ready tracking remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-hosted-citation-detail-proof`.
- [ ] Touched files: portal renderers, portal-domain proof marker, hosted
      Playwright proof spec, tracking feature doc, implementation checklist,
      WP30, WP32, portal README, and generated hosted proof artifacts.
- [ ] Validation commands and results: pending final local hosted proof refresh
      and guard run before PR-ready report.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-citation-detail.png`
      and `test-results/tracking-plan-hosted-ui-proof/`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP30, WP32, and portal README updated; central capability row
      delta stays hub-sequenced instead of editing
      `docs/product-capability-checklist.md`.
- [ ] Known gaps/manual-required states: broader product read models, full
      parent/child UI beyond the hosted parent route, Android/iOS physical-device
      proof, authority, provider delivery, notifications, and production proof
      remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-evidence-drawer-hosted-ui-proof`.
- [ ] Touched files: WP32 doc plus hosted portal proof files in WP30; no new
      read-model storage or service contracts were added.
- [ ] Validation commands and results: pending final focused hosted proof refresh
      and guard run before PR-ready report.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/20-evidence-drawer-hosted-ui-proof.json`
      and `test-results/tracking-plan-hosted-ui-proof/`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP30, and WP32 updated; central capability checklist row stays
      hub-sequenced instead of editing `docs/product-capability-checklist.md`.
- [ ] Known gaps/manual-required states: the evidence drawer is hosted
      read-only citation display only; policy evaluation, action dispatch,
      child-device delivery/runtime execution, provider delivery, physical-device
      proof, authority, and product readiness remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: parent-domain retention local service state proof source and
      test, proof harness, tracking feature doc, implementation checklist, WP07,
      WP32, and generated WP07/WP32/WP33 proof artifacts.
- [ ] Validation commands and results:
      `node scripts/test/tracking-retention-local-service-state-proof.mjs`
      passed locally.
- [ ] Proof artifacts:
      `output/tracking-plan-proof/07-retention-and-custody-model/22-retention-local-service-state-proof.json`,
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/33-retention-local-service-state-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/40-retention-local-service-state-proof.json`,
      and
      `test-results/tracking-retention-local-service-state-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP07, and WP32 updated; central product capability checklist
      remains hub/primary sequenced.
- [ ] Known gaps/manual-required states: writable product settings, platform
      runtime, child-device delivery, Android/iOS physical proof, authority,
      provider delivery, notification receipts, production workers, and
      product-ready retention behavior remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: parent-domain retention durable settings proof source and
      test, proof harness, tracking feature doc, implementation checklist,
      WP07, WP32, and generated WP07/WP32/WP33 proof artifacts.
- [ ] Validation commands and results:
      `node scripts/test/tracking-retention-durable-settings-proof.mjs`
      passed locally.
- [ ] Proof artifacts:
      `output/tracking-plan-proof/07-retention-and-custody-model/23-retention-durable-settings-proof.json`,
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/34-retention-durable-settings-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/41-retention-durable-settings-proof.json`,
      and
      `test-results/tracking-retention-durable-settings-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP07, and WP32 updated; central product capability checklist
      remains hub/primary sequenced because E-B owns the lock.
- [ ] Known gaps/manual-required states: product-ready writable settings,
      platform runtime, child-device delivery, Android/iOS physical proof,
      authority, provider delivery, notification receipts, production workers,
      and product-ready retention behavior remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: parent-domain retention settings mutation proof source and
      test, proof harness, tracking feature doc, implementation checklist,
      WP07, WP32, and generated WP07/WP32/WP33 proof artifacts.
- [ ] Validation commands and results:
      `node scripts/test/tracking-retention-settings-mutation-proof.mjs`
      passed locally.
- [ ] Proof artifacts:
      `output/tracking-plan-proof/07-retention-and-custody-model/20-retention-settings-mutation-proof.json`,
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/26-retention-settings-mutation-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/32-retention-settings-mutation-proof.json`,
      and `test-results/tracking-retention-settings-mutation-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP07, and WP32 updated; central product capability checklist
      remains hub/primary sequenced.
- [ ] Known gaps/manual-required states: live service-backed writable retention
      UI, platform runtime, child-device delivery, Android/iOS physical proof,
      authority, provider delivery, notification receipts, production workers,
      and product-ready retention behavior remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: agent-protocol-domain retention settings write command
      contract/test, Rust agent-protocol command/event/result types, Rust
      agent-service WebSocket response test, proof harness, tracking feature
      doc, implementation checklist, WP07, WP32, and generated WP07/WP32/WP33
      proof artifacts.
- [ ] Validation commands and results:
      `node scripts/test/tracking-retention-settings-write-command-proof.mjs`
      passed locally.
- [ ] Proof artifacts:
      `output/tracking-plan-proof/07-retention-and-custody-model/21-retention-settings-write-command-proof.json`,
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/27-retention-settings-write-command-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/33-retention-settings-write-command-proof.json`,
      and
      `test-results/tracking-retention-settings-write-command-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP07, and WP32 updated; central product capability checklist
      remains hub/primary sequenced.
- [ ] Known gaps/manual-required states: durable writable retention UI,
      product-ready service execution, platform runtime, child-device
      delivery, Android/iOS physical proof, authority, provider delivery,
      notification receipts, production workers, and product-ready retention
      behavior remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: hosted retention settings UI proof model, live activity
      state, portal event-result routing, React route panel, DOM tracking status
      panel, hosted Playwright proof, portal tests, text-domain tokens,
      portal-domain proof artifact marker, hosted UI proof harness, tracking
      feature doc, implementation checklist, WP07, WP30, WP32, and regenerated
      hosted UI proof screenshots/results.
- [ ] Validation commands and results:
      `cmd /c npm run build --workspace @ocentra-parent/agent-protocol-domain`
      passed; `cmd /c npm run build --workspace @ocentra-parent/text-domain`
      passed; `cmd /c npm run build --workspace @ocentra-parent/portal-domain`
      passed; `cmd /c npm run test --workspace @ocentra-parent/portal --
tracking-status-panel` passed; `cmd /c npm run build --workspace
@ocentra-parent/portal` passed; `cmd /c npm run format:check` passed;
      `cmd /c npm run test:tracking-plan-hosted-ui-proof` passed.
- [ ] Proof artifacts:
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-retention-settings.png`,
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/17-hosted-ui-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/18-hosted-ui-accessibility-proof.json`,
      and `test-results/tracking-plan-hosted-ui-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP07, WP30, and WP32 updated; central product capability
      checklist remains hub/primary sequenced.
- [ ] Known gaps/manual-required states: hosted route consumes and renders the
      typed local service execution response with local service state revision
      and local durable settings persistence. Writable product settings,
      applied product-ready service mutation
      execution, platform replay/runtime, child-device delivery/runtime
      execution, Android/iOS physical-device proof, authority, provider delivery,
      notification receipt ingestion,
      production workers, and full parent/child
      UI beyond the hosted route remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: parent-domain report export read-model proof source and
      test, proof harness, tracking feature doc, implementation checklist,
      WP32, parent-domain README, and generated WP32/WP33/test-results proof
      artifacts.
- [ ] Validation commands and results:
      `node scripts/test/tracking-report-export-read-model-proof.mjs` passed
      locally.
- [ ] Proof artifacts:
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/28-report-export-read-model-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/34-report-export-read-model-proof.json`,
      `output/tracking-plan-proof/tracking-report-export-read-model-proof/proof.json`,
      and `test-results/tracking-report-export-read-model-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP32, and parent-domain README updated; central product
      capability checklist remains hub/primary sequenced.
- [ ] Known gaps/manual-required states: report/export proof is redacted
      evidence-ref packet readiness only. Raw location payload export, rendered
      report UI, service mutation, platform runtime, child-device
      delivery/runtime execution, Android/iOS physical proof, authority,
      provider delivery, notification receipt ingestion, production workers,
      and product-ready export behavior remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: parent-domain hosted storage default boundary proof source
      and test, proof harness, tracking feature doc, implementation checklist,
      WP32, central capability checklist, and generated WP32/WP33/test-results
      proof artifacts.
- [ ] Validation commands and results:
      `node scripts/test/tracking-hosted-storage-default-boundary-proof.mjs`
      passed locally.
- [ ] Proof artifacts:
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/31-hosted-storage-default-boundary-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/38-hosted-storage-default-boundary-proof.json`,
      `test-results/tracking-hosted-storage-default-boundary-proof/proof.json`,
      and
      `test-results/tracking-hosted-storage-default-boundary-proof/hosted-storage-default-boundary-read-model.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP32, and central product capability checklist updated.
- [ ] Known gaps/manual-required states: this proves only that Ocentra-hosted
      storage, raw location remote upload, SQLite snapshot remote upload,
      remote sync, and remote AI are not default tracking custody paths.
      Rendered UI/report/policy surfaces, service mutation, platform runtime,
      child-device delivery/runtime execution, Android/iOS physical proof,
      authority, provider delivery, notification receipt ingestion, production
      workers, and product-ready behavior remain proof-gated.
