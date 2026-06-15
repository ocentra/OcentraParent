# Workpack 04: Phone QR Approval Bridge

Purpose: define desktop QR approval, phone approval, and replay-resistant action binding.

## Owns

- QR challenge shape.
- Action, household, parent, desktop, and target binding.
- Short-lived approval response.
- Audit recording and replay rejection.

## Exit condition

- Desktop approval can be completed on a phone with a standards-aligned flow.
- The challenge is one-time and expires quickly.
- The approval is tied to the specific action, not generic login.

## Proof target

- `docs/proof/device-trust-bootstrap-plan/04-*`

## Negative cases

- Replayed QR or response is rejected.
- Wrong household or wrong target fails.
- Expired approval fails.