# 06 - LocalProviderCapability Hardening

## Target State

Provider capability reports supported tasks, privacy mode, model formats,
resource class, hardware fit, fallback order, and unavailable/degraded reasons.

## Where We Are

Provider scheduler proof exists. Provider capability must become the shared gate
for deterministic/text/OCR/VLM/embedding/remote routes.

## Checklist

- [ ] Type provider capability by task.
- [ ] Include local-only and remote-disabled states.
- [ ] Include resource class and hardware fit.
- [ ] Include fallback order.
- [ ] Add provider route rejection for unsupported tasks.

## Proof

- Provider capability contract tests.
- Route selection tests.
- Remote disabled-by-default test.
