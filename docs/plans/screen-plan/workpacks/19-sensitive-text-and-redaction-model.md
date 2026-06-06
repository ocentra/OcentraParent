# 19 Sensitive Text And Redaction Model

## Target State

OCR snippet limits, password/credential suppression, PII redaction, and parent-controlled text retention are implemented.

## Current State

Expectation docs prohibit sensitive capture/leakage. The activity-domain
contract now defines parent-controlled OCR text retention, a hard snippet cap,
credential-like suppression, PII-like redaction, disabled OCR text state, and
no raw text/raw image/remote AI retention. This is contract proof only; service
persistence and portal screenshot rendering remain open.

## Checklist

- [x] Define OCR snippet limit.
- [x] Define OCR disabled state.
- [x] Define password/credential suppression.
- [x] Define PII redaction mode.
- [x] Define parent-controlled text retention.
- [x] Add security tests.
- [ ] Add portal screenshot showing redacted/disabled snippets.
- [ ] Persist/apply redaction settings in the live service path.

## Proof

- Redaction tests.
- `output/screen-plan-proof/19-sensitive-text-and-redaction-model/proof-summary.json`.
- Portal screenshot showing redacted/disabled snippets remains open.
