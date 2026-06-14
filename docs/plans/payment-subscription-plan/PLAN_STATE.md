# Payment Subscription Plan State

Status: execution-grade Cloudflare billing architecture documented; implementation and proof remain open.

Research status: aligned against the current Parent codebase, the `E:\ocentra-games\infra\cloudflare` payment blueprint, Cloudflare storage primitives, Stripe Billing/Checkout/Portal/webhook docs, and the current product pricing direction from Sujan. This is the single monetization owner; do not split a separate subscription plan.

Evidence from `E:\ocentra-games\infra\cloudflare`:

- Cloudflare Worker handlers own payment ingress, checkout creation, and webhook ingress.
- `PaymentDO` stores per-payment state, event history, and idempotency markers.
- Stripe checkout and portal flows are created server-side, not in the browser.
- Stripe webhook handling is signature-verified before settlement.
- Provider normalization exists for Stripe, PayPal, Razorpay, and Solana-style flows; Parent should reuse the adapter boundary, not the game-specific economy.
- Integration tests already cover webhook idempotency, checkout settlement, malformed events, and refund/reconciliation-style flows.
- Wrangler config and test scripts already express a Cloudflare-first deploy/test boundary.

Current Parent direction:

- Cloudflare Worker/API is the billing control plane.
- D1 or equivalent SQL-backed storage owns queryable ledgers and dashboards.
- Durable Objects serialize per-household or per-account billing writes.
- Queues own retries, dead-letter handling, and reconciliation jobs.
- Stripe Checkout, Billing, Portal, invoices, entitlements, and webhooks are the default web control-plane path.
- Razorpay is the India-native adapter.
- PayPal is the secondary wallet/subscription adapter.
- Apple and Google store billing are channel adapters, not the root billing authority.
- App-owned billing, referral, and entitlement ledgers decide access.
- Signed entitlement snapshots are derived artifacts consumed by trusted devices, not the root of trust.

Open gaps:

- Final launch order for regional providers and store billing adapters.
- Dashboard surface detail for parent billing and support/admin operations.
- Exact referral qualification and anti-abuse policy text.
- Invoice/tax/refund/dispute/grace behavior under mixed provider and regional setups.
- Proof matrix and route sync for the new workpack set.

## HID Execution Guard

- Scope and completion source:
  - follow `PLAN_EXECUTION_BLUEPRINT.md`, then this plan's assigned `WORKPACK_INDEX.md` and `NEXT_ACTIONS.md`.
  - do not mark this plan complete from checklist deltas alone.
- Before any checked update, attach:
  - a real test run log or explicit blocker from the assigned implementation boundary,
  - a proof manifest under the designated local artifact path outside this plan folder.
- Failure rule: no PR-ready claim until replay/idempotency, authZ/replay, and rollback/teardown proofs are present for the assigned slice.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.
