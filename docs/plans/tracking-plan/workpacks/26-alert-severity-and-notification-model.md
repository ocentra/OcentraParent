# WP26 Alert Severity And Notification Model

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `WP26 Alert Severity And Notification Model`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Purpose

Model info, watch, warning, urgent, and critical alerts with evidence refs,
policy refs, safe copy, acknowledgement state, notification intent, and audit
refs.

## Source Inputs

- `docs/expectations/notifications.md`
- `docs/features/reports-notifications-sync.md`
- `docs/plans/tracking-plan/ui-ux-requirements-guide.md`

## Target State

Location alerts are evidence-backed, provider-minimized, authenticated-drill-in
friendly, quiet-hours aware, and not emitted from raw unclassified noise.

## Tests And Proof

Proof root: `output/tracking-plan-proof/26-alert-severity-and-notification-model/`

- `01-contract-proof.log`
- `09-policy-alert-proof.json`
- `11-ui-snapshots/`
- `13-security-negative-proof.log`
- `16-validation-commands.log`
- `22-notification-receipt-boundary-proof.json`
- `24-notification-preference-preflight-proof.json`
- `26-notification-parent-surface-history-proof.json`
- `27-notification-parent-surface-hosted-ui-proof.json`
- `28-notification-local-outbox-readiness-proof.json`
- `29-provider-delivery-artifact-gate-proof.json`
- `31-notification-preference-status-handoff-proof.json`

## AI Worker Checklist

- [ ] Require evidence refs and policy refs for every alert.
- [ ] Keep provider payload minimal.
- [ ] Test info/watch/warning/urgent/critical severity.
- [ ] Test safe parent copy and no sensitive provider preview.
- [ ] Keep notification delivery separate from evidence custody.

## Where We Are

This workpack has focused contract proof from `codex/tracking-plan-full-scope` under the proof root below. Runtime, platform, provider, and UI behavior is not claimed beyond the proof state recorded in `proof-summary.json` and the implementation checklist.
Tracking provider-notification proof now maps tracking alert intents from the
tracking policy read model into existing V0.8 notification provider-status
boundary rows through `node scripts/test/tracking-provider-notification-proof.mjs`.
The proof preserves evidence refs, policy decision refs, notification status
refs, reason refs, and minimal/authenticated-drill-in provider payload
boundaries while keeping provider delivery, receipts, credentials, parent
notification UI, child-device delivery, physical-device proof, production
runtime, durable outbox storage, and adapter dispatch unclaimed.
Tracking notification receipt boundary proof now derives
receipt-ingestion-required, manual-receipt-required, and provider-unavailable
rows from those provider-notification proof rows through
`node scripts/test/tracking-notification-receipt-boundary-proof.mjs`. The proof
preserves provider proof refs, evidence refs, policy decision refs,
notification status refs, reason refs, provider attempt refs, and audit refs,
cites the V0.8 delivered-provider receipt-required contract, and still does not
claim webhook receipt ingestion runtime, provider delivery, credentials,
adapter dispatch, child-device delivery, authority, physical-device proof, or
durable outbox storage.
Tracking notification local outbox readiness proof now maps those tracking
receipt boundary rows to the existing notification local outbox adapter and
scheduler proof rows through
`node scripts/test/tracking-notification-local-outbox-readiness-proof.mjs`.
The proof preserves tracking evidence, policy decision, notification status,
receipt requirement, local outbox file/data-path, and scheduler artifact refs
while keeping provider delivery, receipt ingestion runtime, credentials, cloud
routing, parent notification UI, retry/quiet-hours runtime, child-device
delivery, physical-device proof, authority proof, production durable outbox
storage, adapter dispatch, and product-ready notification behavior unclaimed.
Tracking provider-delivery artifact gate proof now checks
`output/tracking-plan-proof/notification-provider-delivery/` for the exact
real-runtime artifact names required before provider delivery can be claimed:
run metadata, redacted runtime config, credential-presence attestation,
minimal payload snapshot, provider attempt/response, receipt webhook event,
receipt ingestion result, retry/quiet-hours worker log, parent notification UI
screenshot, and result summary. It writes WP26/WP33 artifacts through
`node scripts/test/tracking-provider-delivery-artifact-gate-proof.mjs` and
keeps provider delivery runtime, webhook receipt ingestion runtime,
credentials, adapter dispatch, retry/quiet-hours runtime, parent notification
UI runtime, production durable outbox storage, child-device delivery,
physical-device behavior, authority, and product-ready notification behavior
unclaimed until those artifacts exist.
Tracking provider-runtime readiness blocker proof now consumes that artifact
gate plus provider notification, receipt-boundary, and local outbox readiness
proofs, preserving exact required, present, and missing provider runtime
artifact refs while keeping zero provider runtime artifacts present and all
provider/product claims false.
Tracking notification preference preflight proof now derives
parent-preference-required, source-manual-required, and source-unavailable rows
from those provider-notification proof rows through
`node scripts/test/tracking-notification-preference-preflight-proof.mjs`. The
proof preserves provider attempt refs, provider preference refs, evidence refs,
policy decision refs, notification status refs, reason refs, parent preference
requirement refs, quiet-hours requirement refs, and manual proof requirements
while keeping parent notification preference UI/history UI, frequency controls,
quiet-hours timer runtime, provider delivery, receipt runtime, credentials,
adapter dispatch, child-device delivery, physical-device proof, and durable
outbox storage unclaimed.
Tracking notification preference status handoff proof now maps those preference
preflight rows into V3 notification preference and quiet-hours status entries
through
`node scripts/test/tracking-notification-preference-status-handoff-proof.mjs`.
The proof preserves provider attempt, provider preference, evidence, policy
decision, notification status, reason, and quiet-hours refs while keeping parent
notification preference UI/history UI, preference mutation runtime,
quiet-hours timer runtime, provider delivery, receipt runtime, credentials,
cloud routing, child-device delivery, physical-device proof, authority proof,
retry workers, production durable outbox storage, adapter dispatch, and
product-ready notification behavior unclaimed.
Tracking notification parent-surface history intent proof now joins the
provider-notification, receipt boundary, and preference preflight proof rows
through
`node scripts/test/tracking-notification-parent-surface-history-proof.mjs`.
The proof produces redacted parent history/preference intent rows with
provider, receipt, preference, quiet-hours, evidence, policy, notification
status, reason, audit, manual-proof, and authenticated drill-in refs preserved.
The hosted parent route now renders those rows as read-only notification
history/preference-intent proof, captures a Playwright screenshot, and records
accessibility proof. It is not writable preference mutation runtime, provider
delivery, receipt ingestion runtime, child-device delivery, physical-device
proof, authority proof, production durable history/outbox storage, retry
workers, or adapter dispatch.

## Where We Want To Be

This workpack can be assigned independently, implemented against the owning domain boundaries, validated with real contracts or platform proof, and reported without leaving unclear tracking claims behind.

## Scope

- Re-read only the source inputs listed in this workpack and the shared source index.
- Update the owned contracts, runtime paths, UI paths, or docs named by the implementation worker before editing.
- Keep no-claim boundaries explicit for LAN/IP hints, nearby-place ambiguity, AI evidence, background behavior, and remote sync.
- Update the feature doc and product capability checklist only when status, proof, or gaps change.

## Touched Paths

- docs/plans/tracking-plan/workpacks/26-alert-severity-and-notification-model.md
- docs/plans/tracking-plan/implementation-checklist.md
- packages/parent-domain/src/tracking-notification-receipt-boundary-proof.ts
- packages/parent-domain/tests/tracking-notification-receipt-boundary-proof.test.ts
- scripts/test/tracking-notification-receipt-boundary-proof.mjs
- packages/parent-domain/src/tracking-notification-preference-preflight-proof.ts
- packages/parent-domain/tests/tracking-notification-preference-preflight-proof.test.ts
- scripts/test/tracking-notification-preference-preflight-proof.mjs
- packages/parent-domain/src/tracking-notification-preference-status-handoff.ts
- packages/parent-domain/tests/tracking-notification-preference-status-handoff.test.ts
- scripts/test/tracking-notification-preference-status-handoff-proof.mjs
- `output/tracking-plan-proof/26-alert-severity-and-notification-model/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.
- Actual provider delivery, webhook/provider receipt ingestion runtime,
  provider credentials, retry/quiet-hours workers, parent notification UI,
  child-device delivery, physical-device proof, authority proof, production
  durable outbox storage, and adapter dispatch remain manual-required.
- Hosted parent-surface notification history/preference-intent rendering is
  proved read-only. Parent preference mutation runtime, writable frequency
  controls, and quiet-hours timer runtime remain manual-required.
- Parent-surface history intent rows are hosted read-model rendering proof only;
  provider delivery, receipt ingestion runtime, production durable storage,
  child delivery, physical-device proof, authority, and adapter dispatch remain
  manual-required.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch: `codex/tracking-provider-notification-proof`.
- [ ] Touched files: parent-domain tracking provider-notification proof contract,
      focused tests, proof script, feature docs, checklist, and this workpack doc.
- [ ] Validation commands and results:
      `node scripts/test/tracking-provider-notification-proof.mjs` passed.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/26-alert-severity-and-notification-model/`
      and `test-results/tracking-provider-notification-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, reports/notifications
      feature doc, implementation checklist, and this workpack doc updated.
      Product capability checklist update is queued through hub doc delta
      because `docs/product-capability-checklist.md` remains sequenced by the
      hub. Package README update is blocked by E-C lock on
      `packages/parent-domain/README.md`.
- [ ] Known gaps/manual-required states: Android/iOS, precise desktop, actual
      provider delivery, receipt ingestion, credentials, parent notification UI,
      child-device delivery, physical-device proof, runtime engines,
      retention/delete/export, Rust journal/SQLite, production outbox/runtime,
      and full UI remain proof-gated as applicable.
- [ ] Workpack id and branch:
      `codex/tracking-notification-receipt-boundary-proof`.
- [ ] Touched files: parent-domain tracking notification receipt boundary proof
      contract, focused tests, proof script, owning tracking feature doc,
      implementation checklist, this workpack doc, and WP33 proof-gate doc.
- [ ] Validation commands and results:
      `node scripts/test/tracking-notification-receipt-boundary-proof.mjs`
      passed.
- [ ] Proof artifacts:
      `output/tracking-plan-proof/26-alert-severity-and-notification-model/22-notification-receipt-boundary-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/22-notification-receipt-boundary-proof.json`,
      `output/tracking-plan-proof/tracking-notification-receipt-boundary-proof/proof.json`,
      and
      `test-results/tracking-notification-receipt-boundary-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, this workpack doc, and WP33 proof-gate doc updated. Central
      `docs/product-capability-checklist.md` was not edited because E-C
      currently owns that lock.
- [ ] Known gaps/manual-required states: actual webhook/provider receipt
      ingestion runtime, provider delivery, credentials, adapter dispatch,
      retry/quiet-hours workers, parent notification UI, child-device delivery,
      physical-device proof, authority proof, durable outbox storage, and
      product-ready notification behavior remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-notification-preference-preflight-proof`.
- [ ] Touched files: parent-domain tracking notification preference preflight
      proof contract, focused tests, proof script, owning tracking feature doc,
      implementation checklist, this workpack doc, and WP33 proof-gate doc.
- [ ] Validation commands and results:
      `node scripts/test/tracking-notification-preference-preflight-proof.mjs`
      passed.
- [ ] Proof artifacts:
      `output/tracking-plan-proof/26-alert-severity-and-notification-model/24-notification-preference-preflight-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/24-notification-preference-preflight-proof.json`,
      `output/tracking-plan-proof/tracking-notification-preference-preflight-proof/proof.json`,
      and
      `test-results/tracking-notification-preference-preflight-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, this workpack doc, and WP33 proof-gate doc updated. Central
      `docs/product-capability-checklist.md` was not edited because E-C
      currently owns that lock. Package export/README was not edited because
      codex-d currently owns `packages/parent-domain/package.json`.
- [ ] Known gaps/manual-required states: parent notification preference
      UI/history UI, preference mutation runtime, frequency-control UI,
      quiet-hours timer runtime, receipt ingestion runtime, provider delivery,
      credentials, adapter dispatch, child-device delivery, physical-device
      proof, authority proof, durable outbox storage, and product-ready
      notification behavior remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: parent-domain tracking notification parent-surface history
      proof/test, proof script, owning tracking feature docs, implementation
      checklist, this workpack doc, WP33 proof-gate doc, and generated WP26/WP33
      proof artifacts.
- [ ] Validation commands and results:
      `node scripts/test/tracking-notification-parent-surface-history-proof.mjs`
      passed.
- [ ] Proof artifacts:
      `output/tracking-plan-proof/26-alert-severity-and-notification-model/26-notification-parent-surface-history-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/26-notification-parent-surface-history-proof.json`,
      `output/tracking-plan-proof/tracking-notification-parent-surface-history-proof/proof.json`,
      and
      `test-results/tracking-notification-parent-surface-history-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature docs, implementation
      checklist, this workpack doc, and WP33 proof-gate doc updated. Central
      `docs/product-capability-checklist.md` update is queued through the hub
      doc delta.
- [ ] Known gaps/manual-required states: rendered parent notification UI,
      parent preference mutation runtime, frequency-control UI, quiet-hours
      timer runtime, provider delivery, receipt ingestion runtime, credentials,
      cloud routing, child-device delivery, physical-device proof, authority
      proof, retry workers, production durable history/outbox storage, adapter
      dispatch, and product-ready notification behavior remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: hosted notification parent-surface proof model, portal
      tracking status route renderer, portal hosted Playwright proof spec,
      portal tests, hosted proof script, portal/text/domain constants, owning
      tracking feature doc, implementation checklist, WP26/WP30/WP33 docs, and
      generated hosted proof artifacts.
- [ ] Validation commands and results:
      `cmd /c npm run build --workspace @ocentra-parent/text-domain` passed;
      `cmd /c npm run build --workspace @ocentra-parent/portal-domain` passed;
      `cmd /c npm run test --workspace @ocentra-parent/portal --
tracking-status-panel` passed; `cmd /c npm run lint --workspace
@ocentra-parent/portal` passed; `cmd /c npm run
test:tracking-plan-hosted-ui-proof` passed.
- [ ] Proof artifacts:
      `output/tracking-plan-proof/26-alert-severity-and-notification-model/27-notification-parent-surface-hosted-ui-proof.json`,
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/22-notification-parent-surface-hosted-ui-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/35-notification-parent-surface-hosted-ui-proof.json`,
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-notification-parent-surface.png`,
      and `test-results/tracking-plan-hosted-ui-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, this workpack doc, WP30, and WP33 updated. Central
      `docs/product-capability-checklist.md` update remains hub-sequenced
      because E-B owns that lock.
- [ ] Known gaps/manual-required states: hosted rendering is read-only;
      writable notification preferences, parent mutation runtime, quiet-hours
      runtime, provider delivery, receipt ingestion runtime, credentials, cloud
      routing, child-device delivery, physical-device proof, authority proof,
      retry workers, production durable history/outbox storage, adapter
      dispatch, and product-ready notification behavior remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: parent-domain tracking notification local outbox
      readiness proof/test, proof script, owning tracking feature doc,
      implementation checklist, this workpack doc, WP33 proof-gate doc, and
      generated WP26/WP33 proof artifacts.
- [ ] Validation commands and results:
      `node scripts/test/tracking-notification-local-outbox-readiness-proof.mjs`
      passed.
- [ ] Proof artifacts:
      `output/tracking-plan-proof/26-alert-severity-and-notification-model/28-notification-local-outbox-readiness-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/42-notification-local-outbox-readiness-proof.json`,
      `output/tracking-plan-proof/tracking-notification-local-outbox-readiness-proof/proof.json`,
      and
      `test-results/tracking-notification-local-outbox-readiness-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, this workpack doc, and WP33 proof-gate doc updated. Central
      `docs/product-capability-checklist.md` was not edited because E-C
      currently owns that lock.
- [ ] Known gaps/manual-required states: actual provider delivery, receipt
      ingestion runtime, credentials, cloud routing, parent notification UI,
      retry/quiet-hours worker runtime, child-device delivery,
      physical-device proof, authority proof, production durable outbox
      storage, adapter dispatch, and product-ready notification behavior
      remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: parent-domain tracking provider runtime readiness blocker
      proof/test, proof script, owning tracking feature doc, implementation
      checklist, this workpack doc, WP33 proof-gate doc, and generated WP26/WP33
      proof artifacts.
- [ ] Validation commands and results:
      `node scripts/test/tracking-provider-runtime-readiness-blocker-proof.mjs`
      passed.
- [ ] Proof artifacts:
      `output/tracking-plan-proof/26-alert-severity-and-notification-model/30-provider-runtime-readiness-blocker-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/52-provider-runtime-readiness-blocker-proof.json`,
      `output/tracking-plan-proof/tracking-provider-runtime-readiness-blocker-proof/proof.json`,
      and
      `test-results/tracking-provider-runtime-readiness-blocker-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, this workpack doc, and WP33 proof-gate doc updated. Central
      `docs/product-capability-checklist.md` update is queued through the hub
      doc delta instead of editing the shared checklist directly.
- [ ] Known gaps/manual-required states: provider delivery runtime, webhook
      receipt ingestion runtime, credentials, adapter dispatch,
      retry/quiet-hours runtime, parent notification UI runtime, production
      durable outbox storage, child-device delivery, physical-device proof,
      authority proof, and product-ready tracking remain proof-gated until real
      provider-runtime artifacts exist.
- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: parent-domain tracking notification preference status
      handoff proof/test, proof script, owning tracking feature doc,
      implementation checklist, this workpack doc, WP33 proof-gate doc,
      generated WP26/WP33 status handoff proof artifacts, and refreshed closure
      proof artifacts.
- [ ] Validation commands and results:
      `node scripts/test/tracking-notification-preference-status-handoff-proof.mjs`
      passed; `node scripts/test/tracking-product-readiness-closure-proof.mjs`
      passed.
- [ ] Proof artifacts:
      `output/tracking-plan-proof/26-alert-severity-and-notification-model/31-notification-preference-status-handoff-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/54-notification-preference-status-handoff-proof.json`,
      `output/tracking-plan-proof/tracking-notification-preference-status-handoff-proof/proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/46-product-readiness-closure-proof.json`,
      `output/tracking-plan-proof/tracking-product-readiness-closure-proof/proof.json`,
      `test-results/tracking-notification-preference-status-handoff-proof/proof.json`,
      and `test-results/tracking-product-readiness-closure-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, this workpack doc, and WP33 proof-gate doc updated. Central
      `docs/product-capability-checklist.md` update is queued through the hub
      doc delta instead of editing the shared checklist directly.
- [ ] Known gaps/manual-required states: parent notification preference UI,
      parent notification history UI, preference mutation runtime,
      quiet-hours timer runtime, provider delivery, receipt runtime,
      credentials, cloud routing, child-device delivery, physical-device proof,
      authority proof, retry workers, production durable outbox storage,
      adapter dispatch, and product-ready notification behavior remain
      proof-gated.
