# Workpack 04: Phone QR Approval Bridge

Purpose: define desktop QR approval, phone approval, and replay-resistant action binding.

## Owns

- QR challenge shape.
- Action, household, parent, desktop, phone, and target binding.
- Short-lived approval response.
- Audit recording and replay rejection.
- No-login-only and no-generic-approval boundaries.

## Ownership boundary

```text
device-trust-bootstrap-plan owns the QR approval contract, replay boundary, and trust handoff proof.
account-identity-family-plan owns parent account, household, role, and session authority.
portal plans own UI rendering only.
LAN/remote/setup plans own their transport or setup surfaces only through typed handoff when selected.
```

## Required proof fields

The selected proof must name, at minimum:

```text
challenge_id
action_ref
household_ref
parent_account_ref
approving_device_ref
desktop_device_ref
target_ref
issued_at
expires_at
nonce_or_challenge_ref
approval_result
audit_ref
replay_state
wrong_household_state
wrong_target_state
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Exit condition

- Desktop approval can be completed on a phone with a standards-aligned flow.
- The challenge is one-time and expires quickly.
- The approval is tied to the specific action, not generic login.
- Approval is bound to household, parent account, approving device, desktop device, target, and audit refs.
- Replayed, expired, wrong-household, wrong-target, and wrong-action responses are rejected.
- The proof root records whether this is contract-only, local integration, or real end-to-end approval behavior.

## Proof target

- `output/device-trust-bootstrap-plan-proof/04-phone-qr-approval-bridge/`

## Required proof files

```text
output/device-trust-bootstrap-plan-proof/04-phone-qr-approval-bridge/00-scope-summary.md
output/device-trust-bootstrap-plan-proof/04-phone-qr-approval-bridge/01-negative-case-proof.md
output/device-trust-bootstrap-plan-proof/04-phone-qr-approval-bridge/02-no-claim-boundary.md
output/device-trust-bootstrap-plan-proof/04-phone-qr-approval-bridge/03-platform-proof-status.md
output/device-trust-bootstrap-plan-proof/04-phone-qr-approval-bridge/16-validation-commands.log
output/device-trust-bootstrap-plan-proof/04-phone-qr-approval-bridge/17-blockers.md
```

## Current audit state

- No proof root currently exists on disk for this workpack.
- The current plan-local tests for this slice are document assertions, not QR approval runtime proof.
- The Rust production boundary now owns typed challenge/response shapes and
  rejects mismatched bindings, expired or overlong challenges, and non-fresh
  response state before consulting an authority verifier. The verifier remains
  unavailable/manual-required until a real issuer, phone ceremony, signature
  check, nonce consume, and transport owner exist.
- The response's `Fresh` replay field is an untrusted input claim; only the
  authority verifier may establish freshness by consuming the nonce or
  challenge reference.

## Negative cases

- Replayed QR or response is rejected.
- Wrong household or wrong target fails.
- Wrong action fails.
- Expired approval fails.
- Login/session alone cannot approve the action.
- QR shape alone cannot prove phone approval bridge behavior.
- Approval cannot be reused for a different desktop, target, child device, or high-risk action.
