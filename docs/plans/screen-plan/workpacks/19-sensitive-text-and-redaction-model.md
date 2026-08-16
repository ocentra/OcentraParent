# 19 Sensitive Text And Redaction Model

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `19 Sensitive Text And Redaction Model`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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
encrypted queue and deleting the adapter temp image. The service also applies a
local OCR redaction pass before event persistence: a real local Chrome text
surface is captured as pixels, WinRT OCR reads sensitive text markers, the
service row stores `[redacted-email]`/`[redacted-phone]` snippets plus
redaction notes, and the real `#/screen-analysis` portal route screenshot shows
the redacted service row without raw email or phone text. The proof now writes a
persisted parent-selected redaction policy file and passes it into the service
analysis runtime so the service consumes explicit OCR text retention,
credential suppression, PII redaction, and snippet limit settings.

## Checklist

- [ ] Define OCR snippet limit.
- [ ] Define OCR disabled state.
- [ ] Define password/credential suppression.
- [ ] Define PII redaction mode.
- [ ] Define parent-controlled text retention.
- [ ] Add security tests.
- [ ] Add portal read-model/intent proof for redacted snippets.
- [ ] Persist service-emitted OCR snippets and redaction-note shape into the Activity Screen read model.
- [ ] Add real portal screenshot from a service-emitted redaction row.
- [ ] Apply local redaction in the live service path from a persisted parent-selected OCR redaction policy.

## Proof

- Redaction tests.
- `output/screen-plan-proof/19-sensitive-text-and-redaction-model/proof-summary.json`.
- `output/screen-plan-proof/19-sensitive-text-and-redaction-model/portal-intent-proof-summary.json`.
- `output/screen-ai-pipeline-proof/service-winrt-ocr/proof-summary.json` proves
  real service capture/OCR/read-model persistence of bounded OCR snippets and an
  explicit redaction-note array shape without raw image retention.
- `output/screen-ai-pipeline-proof/service-winrt-ocr-redaction/proof-summary.json`
  and `portal-screen-analysis-redaction.png` prove service-emitted redacted OCR
  snippets render on the real Screen Analysis portal route without raw sensitive
  text.
- `output/screen-ai-pipeline-proof/service-winrt-ocr-redaction/parent-redaction-policy.json`
  is the parent-selected OCR text retention/redaction policy consumed by the
  service proof.
