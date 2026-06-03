# 14 Protected Surface Detector

## Target State

Lock screen, secure desktop, credential prompt, password field, DRM/protected media, OS-protected surface, and unsupported states are skipped or redacted.

## Current State

Expectation docs prohibit sensitive capture. Runtime proof is open.

## Checklist

- [ ] Define protected surface categories.
- [ ] Define skip result state.
- [ ] Define redaction result state.
- [ ] Add capture-side guard where possible.
- [ ] Add OCR-side redaction where needed.
- [ ] Add portal labels for skipped/redacted evidence.

## Proof

- Security tests.
- Manual protected-surface proof.
