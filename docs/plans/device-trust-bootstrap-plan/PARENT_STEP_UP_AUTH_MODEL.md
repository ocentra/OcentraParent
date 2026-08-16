# Parent Step-Up Auth Model

This document defines the parent presence proof used for high-risk actions.

## Allowed step-up factors

- WebAuthn / passkey presence proof.
- OS-native biometrics or device unlock prompts, as mediated by the platform.
- Phone approval via the QR bridge when the parent is on desktop.
- Recovery flow only when the action is a restore or reset, not for normal access.

## Not allowed

- Custom face or retina capture stored by the app.
- App-managed biometric templates.
- Repeated permission prompts as the default path after bootstrap.

## High-risk action policy

| Action | Step-up required | Notes |
| --- | --- | --- |
| First parent trust | Yes | Creates the trusted parent device. |
| Child pairing | Yes | Binds the child device to the household. |
| Policy change | Yes | High-risk policy updates need fresh proof. |
| Remote control grant | Yes | Even if remote control is deferred now, the gating model is explicit. |
| Export / delete / restore | Yes | Data custody impact requires fresh proof. |
| Support access | Yes | Support cannot be silent. |
| Revocation / restore | Yes | Parent intent must be explicit. |
| Household transfer | Yes | Transfer is a trust boundary reset. |
| Uninstall authorization | Yes | Child cannot self-authorize this. |

## Step-up behavior

- Step-up is short-lived and action-bound.
- Step-up is household-bound and device-bound.
- Step-up can be satisfied by a local OS prompt, a passkey, or a phone approval challenge depending on the surface.
- Step-up must never return a reusable plaintext secret.

## Negative cases

- A cached session cannot bypass step-up for a high-risk action.
- A child device cannot satisfy parent step-up on its own.
- Expired or replayed step-up assertions must fail.
- Login after idle timeout must not implicitly grant high-risk action authority.