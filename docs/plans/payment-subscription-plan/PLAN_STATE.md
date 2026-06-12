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
