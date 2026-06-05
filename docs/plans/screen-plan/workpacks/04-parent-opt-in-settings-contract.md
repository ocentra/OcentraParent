# 04 Parent Opt-In Settings Contract

## Target State

Disabled-by-default settings exist for enablement, cadence, triggers, scope, OCR snippets, redaction, TTL, retention, policy use, and audit fields.

## Current State

`ScreenAnalysisParentSettingSchema` validates disabled, observe-only, strict
dry-run, cadence, trigger, OCR/redaction, TTL/retry, deletion, policy-use, and
audit fields. `screenEvidenceSettingsWritableUiProof` now renders those setting
intents in the real Settings route and proves disabled, observe-only, and strict
dry-run drafts through Playwright.

## Checklist

- [x] Define disabled-by-default setting.
- [x] Define child/device/schedule scope.
- [x] Define analysis mode.
- [x] Define cadence/trigger settings.
- [x] Define capture scope.
- [x] Define OCR/redaction settings.
- [x] Define TTL/retry/deletion settings.
- [x] Define parent audit fields.

## Proof

- Contract tests.
- `output/screen-plan-proof/settings-writable-controls/proof-summary.json`.
- `output/screen-plan-proof/settings-writable-controls/parent-settings-writable-controls.png`.
