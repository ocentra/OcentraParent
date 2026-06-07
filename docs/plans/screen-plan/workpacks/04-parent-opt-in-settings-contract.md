# 04 Parent Opt-In Settings Contract

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

- [x] Define disabled-by-default setting.
- [x] Define child/device/schedule scope.
- [x] Define analysis mode.
- [x] Define cadence/trigger settings.
- [x] Define capture scope.
- [x] Define OCR/redaction settings.
- [x] Define TTL/retry/deletion settings.
- [x] Define parent audit fields.
- [x] Persist parent setting changes in a local child-device service store.
- [x] Reject raw screenshot retention and unsafe policy/capture combinations
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
