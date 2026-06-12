# Payment Subscription Plan State

Status: first-pass plan created because roadmap billing and family registration need a dedicated owner.

Research status: incomplete. This plan requires a full follow-up research pass against existing Parent account/portal/domain code, games Cloudflare Stripe implementation, official Stripe Billing/Checkout/webhook docs, and Sujan's pricing/product decisions before implementation claims.

Evidence from `E:\ocentra-games`:

- Cloudflare Worker creates Stripe Checkout sessions and Customer Portal sessions.
- Payment state is persisted behind a Durable Object keyed by user.
- Stripe webhooks are signature-verified and idempotency-checked before fulfillment.
- Turnstile protects checkout creation.
- Firebase is used in games for auth/admin role checks and Firestore role reads, not as the only app platform.
- Game payments include credits, marketplace, multiple providers, and game economy; those are not parent-product defaults.

Current parent direction:

- Cloudflare-first payment API.
- Stripe Billing plus Checkout Sessions for subscriptions.
- Stripe Customer Portal for self-service billing management.
- App-owned entitlement ledger connected to account/household identity.
- Privacy-safe Stripe metadata only.

Open gaps:

- No parent pricing/tier/seat model.
- No household entitlement contract.
- No subscription lifecycle state machine.
- No invoice/tax/refund/dispute policy.
- No payment proof matrix in Parent docs.

## HID Execution Guard (added 2026-06-12)

- Scope and completion source:
  - follow [PLAN_HID_MATRIX.md](../PLAN_HID_MATRIX.md) execution slice, then this plan's assigned WORKPACK_INDEX.md and NEXT_ACTIONS.md.
  - do not mark this plan complete from checklist deltas alone.
- Before any checked update, attach:
  - a real test run log (or explicit known blocker) from the assigned implementation boundary,
  - a proof manifest under docs/proof/payment-subscription-plan/.
- Required proof manifest names:
  - docs/proof/payment-subscription-plan/slice-01-\*.md
  - docs/proof/payment-subscription-plan/slice-02-\*.md
  - docs/proof/payment-subscription-plan/slice-03-\*.md
  - each proof file must include commands, pass/fail,
    negative-cases, and manual-required notes.
- Failure rule: no PR-ready claim until replay/idempotency, authZ/replay, and rollback/teardown proofs are present for the assigned slice.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.
