# Research and UI Guidance

Purpose: define the user-facing and operator-facing surfaces this plan must support without inventing a new auth model.

## Parent billing UI

- Show current plan, starter bundle, child-device seats, referral credits, next renewal, grace state, invoice history, and provider mode.
- Show parent and co-parent invite usage, referral invite links, earned credits, and paid extra child-device seats.
- Show a clear entry into the hosted portal for payment method updates, cancellations, and invoice downloads.
- Show region and provider availability when the user is in a market with more than one supported route.
- Show support-safe labels only; do not surface child telemetry or provider internals.
- Show paid extra child-device seats, license and device entitlement state, and manual invoice state when relevant.

## Support/admin UI

- Show account lookup, billing timeline, provider refs, refund and dispute state, manual adjustments, and reconciliation status.
- Show over-limit households, webhook failures, and referral abuse state.
- Redact child data by default.
- Require audit logging for every support mutation.
- Keep the support dashboard separate from the parent dashboard.

## Checkout and portal copy

- The user should understand what is being purchased: starter bundle, paid child-device seats, or referral-credited seats.
- Failure states must explain provider availability, missing configuration, or auth requirements without leaking secrets.
- If device trust or account identity is missing, hand off to the owning plan instead of inventing a payment-specific auth flow.
- Portal copy must be explicit that the hosted portal is the place for payment method, cancel, and invoice actions.

## Research notes

- The exact device trust flow belongs to `device-trust-bootstrap-plan`.
- The exact household identity flow belongs to `account-identity-family-plan`.
- The billing plan owns the billing semantics, not the generic portal shell.
- The dashboard plan owns surface copy and state framing, not provider-specific policy.
