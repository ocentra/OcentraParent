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

## LAN WP26 dependency routing

WP03 depends on Device Trust WP01's persistent trusted-device/signer-key
registration source. It remains blocked until a real parent ceremony can
authorize the one-time `RegisterLanSignerAnchor` action, verify the signed
assertion, and consume its nonce/receipt exactly once. A typed receipt,
document assertion, or LAN service caller is not that ceremony and must not
advance WP26.

## Implementation-phase routing disposition — 2026-08-17

The default dependency remains blocked until WP01 is DONE. The graph records
only the WP03 -> WP01 edge with the `reviewed-implementation` phase gate, which
permits an implementation query to authorize WP03 source work against reviewed
WP01 implementation evidence. This phase-only route does not provide real
ceremony authority, tests, proof, or completion, and no WP26 edge is opted into
it.

## Negative cases

- Cached login cannot bypass step-up.
- Child devices cannot satisfy parent step-up.
- Expired assertions fail closed.
