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

- `docs/proof/device-trust-bootstrap-plan/03-*`

## Negative cases

- Cached login cannot bypass step-up.
- Child devices cannot satisfy parent step-up.
- Expired assertions fail closed.