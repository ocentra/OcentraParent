# 19 Sensitive Text And Redaction Model

## Target State

OCR snippet limits, password/credential suppression, PII redaction, and parent-controlled text retention are implemented.

## Current State

Expectation docs prohibit sensitive capture/leakage. The activity-domain
contract now defines parent-controlled OCR text retention, a hard snippet cap,
credential-like suppression, PII-like redaction, disabled OCR text state, and
no raw text/raw image/remote AI retention. Activity Screen read-model rows now
carry redacted OCR snippets and redaction notes, and the Screen Analysis portal
intent renders those redacted fields while proving raw email, phone, credential,
raw image retention, and remote AI are absent. The Windows service WinRT OCR
proof now persists bounded OCR snippets and the structured `redactionNotes`
array from adapter output into the Activity Screen read model while draining the
encrypted queue and deleting the adapter temp image. A real service-emitted
sensitive-redaction portal screenshot remains open because the current service
proof uses a public Wikipedia page with no sensitive text to redact.

## Checklist

- [x] Define OCR snippet limit.
- [x] Define OCR disabled state.
- [x] Define password/credential suppression.
- [x] Define PII redaction mode.
- [x] Define parent-controlled text retention.
- [x] Add security tests.
- [x] Add portal read-model/intent proof for redacted snippets.
- [x] Persist service-emitted OCR snippets and redaction-note shape into the Activity Screen read model.
- [ ] Add real portal screenshot from a service-emitted redaction row.
- [ ] Persist/apply redaction settings in the live service path.

## Proof

- Redaction tests.
- `output/screen-plan-proof/19-sensitive-text-and-redaction-model/proof-summary.json`.
- `output/screen-plan-proof/19-sensitive-text-and-redaction-model/portal-intent-proof-summary.json`.
- `output/screen-ai-pipeline-proof/service-winrt-ocr/proof-summary.json` proves
  real service capture/OCR/read-model persistence of bounded OCR snippets and an
  explicit redaction-note array shape without raw image retention.
- Service-backed portal screenshot showing actually redacted/disabled sensitive
  snippets remains open.
