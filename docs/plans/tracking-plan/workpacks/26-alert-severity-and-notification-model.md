# WP26 Alert Severity And Notification Model

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
- `23-notification-parent-surface-proof.json`

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
Tracking notification parent-surface proof now derives parent-visible history,
manual-action, unavailable, preference setup, drill-in, audit, and minimal
payload rows from that provider-notification proof through
`node scripts/test/tracking-notification-parent-surface-proof.mjs`. The proof
keeps rendered parent notification UI/history/preferences, parent preference
mutation runtime, provider delivery, receipt ingestion, credentials, adapter
dispatch, child-device delivery, physical-device proof, authority, durable
outbox storage, and product-ready notification behavior unclaimed.

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
- packages/parent-domain/src/tracking-notification-parent-surface-proof.ts
- packages/parent-domain/tests/tracking-notification-parent-surface-proof.test.ts
- scripts/test/tracking-notification-parent-surface-proof.mjs
- `output/tracking-plan-proof/26-alert-severity-and-notification-model/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.
- Rendered parent notification UI/history/preferences, parent preference
  mutation runtime, provider delivery, receipt ingestion, credentials, adapter
  dispatch, child-device delivery, physical-device proof, authority, durable
  outbox storage, and product-ready notification behavior remain
  manual-required.

## Fill This Before Reporting DONE Or PR-ready

- [x] Workpack id and branch: `codex/tracking-provider-notification-proof`.
- [x] Touched files: parent-domain tracking provider-notification proof contract,
      focused tests, proof script, feature docs, checklist, and this workpack doc.
- [x] Validation commands and results:
      `node scripts/test/tracking-provider-notification-proof.mjs` passed.
- [x] Proof artifacts under
      `output/tracking-plan-proof/26-alert-severity-and-notification-model/`
      and `test-results/tracking-provider-notification-proof/proof.json`.
- [x] Product doc/checklist updates: owning feature doc, reports/notifications
      feature doc, implementation checklist, and this workpack doc updated.
      Product capability checklist update is queued through hub doc delta
      because `docs/product-capability-checklist.md` remains sequenced by the
      hub. Package README update is blocked by E-C lock on
      `packages/parent-domain/README.md`.
- [x] Known gaps/manual-required states: Android/iOS, precise desktop, actual
      provider delivery, receipt ingestion, credentials, parent notification UI,
      child-device delivery, physical-device proof, runtime engines,
      retention/delete/export, Rust journal/SQLite, production outbox/runtime,
      and full UI remain proof-gated as applicable.
- [x] Workpack id and branch:
      `codex/tracking-notification-parent-surface-proof`.
- [x] Touched files: parent-domain tracking notification parent-surface proof
      contract, focused tests, proof script, feature doc, implementation
      checklist, this workpack doc, WP33 proof-gate doc, package export, and
      generated proof artifacts.
- [x] Validation commands and results:
      `node scripts/test/tracking-notification-parent-surface-proof.mjs` passed.
- [x] Proof artifacts:
      `output/tracking-plan-proof/26-alert-severity-and-notification-model/23-notification-parent-surface-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/23-notification-parent-surface-proof.json`,
      `output/tracking-plan-proof/tracking-notification-parent-surface-proof/proof.json`,
      and `test-results/tracking-notification-parent-surface-proof/proof.json`.
- [x] Product doc/checklist updates: owning feature doc, implementation
      checklist, and this workpack doc updated. Product capability checklist was
      not edited because E-C currently owns that central checklist lock. Package
      README was not edited because codex-c currently owns the parent-domain
      README lock.
- [x] Known gaps/manual-required states: rendered parent notification
      UI/history/preferences, parent preference mutation runtime, provider
      delivery, receipt ingestion, credentials, adapter dispatch, child-device
      delivery, physical-device proof, authority, durable outbox storage, and
      product-ready notification behavior remain proof-gated.
