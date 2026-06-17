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

- `output/device-trust-bootstrap-plan-proof/04-phone-qr-approval-bridge/`

## Current audit state

- No proof root currently exists on disk for this workpack.
- The current plan-local tests for this slice are document assertions, not QR approval runtime proof.

## Negative cases

- Replayed QR or response is rejected.
- Wrong household or wrong target fails.
- Expired approval fails.
