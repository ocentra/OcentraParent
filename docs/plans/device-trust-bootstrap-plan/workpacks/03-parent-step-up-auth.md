# Workpack 03: Parent Step-Up Auth

Purpose: define parent step-up auth with passkeys, biometrics, and OS-native approval.

## Owns

- High-risk action list.
- Passkey / WebAuthn step-up.
- Native biometric or device-unlock prompts.
- Step-up expiry and replay rules.

## Exit condition

- Each high-risk action has an explicit step-up rule.
- No custom biometric storage appears anywhere in the model.
- Step-up is short-lived, action-bound, and household-bound.

## Proof target

- `output/device-trust-bootstrap-plan-proof/03-parent-step-up-auth/`

## Test layout

- `tests/device-trust-bootstrap-plan/contract/parent-step-up-auth.test.mjs`
- `tests/device-trust-bootstrap-plan/unit/local-key-sealing.test.mjs`
- `tests/device-trust-bootstrap-plan/integration/recovery-re-pair-boundary.test.mjs`

## Current audit state

- No proof root currently exists on disk for this workpack.
- Current plan-local tests for this slice are document assertions; passkey/WebAuthn and OS-native step-up runtime proof are still missing.
- The Rust receipt-shape boundary now rejects receipts whose issued-to-expiry
  lifetime exceeds five minutes before consulting an authority verifier. The
  verifier remains unavailable/manual-required until a real passkey or
  OS-native authority owns signature verification and one-time nonce consume.

## Negative cases

- Cached login cannot bypass step-up.
- Child devices cannot satisfy parent step-up.
- Expired assertions fail closed.
