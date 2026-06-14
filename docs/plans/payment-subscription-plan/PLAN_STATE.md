# Payment Subscription Plan State

Status: execution-grade Cloudflare billing architecture documented; implementation and proof remain open.

Research status: aligned against the current Parent codebase, Cloudflare storage primitives, Stripe Billing/Checkout/Portal/webhook docs, and the current product pricing direction from Sujan. The prior Cloudflare billing control-plane shape is summarized here as reusable guidance, not an execution dependency. This is the single monetization owner; do not split a separate subscription plan.

Reference control-plane shape summarized in this plan:

- Cloudflare Worker handlers own payment ingress, checkout creation, and webhook ingress.
- Durable Objects store per-payment or per-household state, event history, and idempotency markers.
- Stripe checkout and portal flows are created server-side, not in the browser.
- Stripe webhook handling is signature-verified before settlement.
- Provider normalization exists for Stripe, PayPal, Razorpay, and other adapter-style payment flows; Parent should reuse the adapter boundary, not the game-specific economy.
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
- The historical audit baseline is 46/100 in `PLAN_EXECUTION_SCORECARD_REVIEW.md`; the live audit state is tracked in `PLAN_EXECUTION_SCORECARD.md`.

## Decision records

| Record | Owner | Status | Gap | Closure criteria |
| --- | --- | --- | --- | --- |
| PSP-013 | WP08 + WP09 | closed | Final launch order for regional providers and store billing adapters. | Region matrix and provider order are approved and mirrored in the proof inventory. |
| PSP-014 | WP11 + WP12 | closed | Parent dashboard and support/admin field boundaries. | Allow/deny field lists are present and synchronized across both surfaces. |
| PSP-015 | WP10 | closed | Referral qualification and anti-abuse policy text. | Referral qualification, abuse rejection, and referral-grace naming are closed. |
| PSP-016 | WP05 | closed | Mixed-provider invoice, tax, refund, dispute, and grace behavior. | Billing-grace, refund, dispute, and cancellation rules are explicit and proven. |
| PSP-017 | WP07 | closed | Proof matrix and route sync for rollout readiness. | Proof paths, validation logs, and route docs agree. |

The decision records above are closed at the documentation layer. Implementation, runtime validation, and proof capture remain open when the selected workpack starts.

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
