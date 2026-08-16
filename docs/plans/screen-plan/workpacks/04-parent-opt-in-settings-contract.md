# 04 Parent Opt-In Settings Contract

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `04 Parent Opt-In Settings Contract`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Disabled-by-default settings exist for enablement, cadence, triggers, scope, OCR snippets, redaction, TTL, retention, policy use, and audit fields.

## Current State

`ScreenAnalysisParentSettingSchema` validates disabled, observe-only, strict
dry-run, cadence, trigger, OCR/redaction, TTL/retry, deletion, policy-use, and
audit fields. `screenEvidenceSettingsWritableUiProof` now renders those setting
intents in the real Settings route and proves disabled, observe-only, and strict
dry-run drafts through Playwright. The Rust protocol now mirrors the parent
setting shape, and `screen-settings-service-persistence-proof` proves a local
JSON-backed service settings runtime returns a disabled default, persists a
parent strict dry-run setting across a reload, and rejects raw image retention,
observe-only policy use, stale base versions, and unsafe inconsistent settings
before persistence.

## Checklist

- [ ] Define disabled-by-default setting.
- [ ] Define child/device/schedule scope.
- [ ] Define analysis mode.
- [ ] Define cadence/trigger settings.
- [ ] Define capture scope.
- [ ] Define OCR/redaction settings.
- [ ] Define TTL/retry/deletion settings.
- [ ] Define parent audit fields.
- [ ] Persist parent setting changes in a local child-device service store.
- [ ] Reject raw screenshot retention and unsafe policy/capture combinations
      before service persistence.

## Proof

- Contract tests.
- `output/screen-plan-proof/settings-writable-controls/proof-summary.json`.
- `output/screen-plan-proof/settings-writable-controls/parent-settings-writable-controls.png`.
- `output/screen-plan-proof/screen-settings-service-persistence/proof-summary.json`.

## Remaining Gap

The service persistence proof is backend/runtime proof. It does not yet wire the
parent portal Settings route to a WebSocket command, ship product-complete
retention-control UI, enable raw screenshot retention, enable live view, or
claim privacy/legal approval.
