# 39 - Device Hardware Model Fit Lane

## Target State

Parents and developers can see whether the child device can run the selected
local model tasks.

## Where We Are

AI UI notes mention hardware fit. Runtime acceleration config and device details
exist, but model fit must be explicit for text/OCR/VLM/embedding tasks.

## Checklist

- [ ] Capture CPU/RAM/GPU capability refs.
- [ ] Map model/task requirements.
- [ ] Add fit states: fits, maybe, too large, unsupported, unknown.
- [ ] Include acceleration settings.
- [ ] Expose status in portal.

## Proof

- Hardware/model fit tests.
- Runtime status proof.
- Portal screenshot if UI changes.
