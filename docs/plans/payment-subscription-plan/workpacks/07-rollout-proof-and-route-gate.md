# Workpack 07: Rollout Proof and Route Gate

Goal: define the payment launch gate.

Expected proof:

- Pricing/tier matrix.
- Checkout and billing portal proof.
- Webhook idempotency and lifecycle proof.
- Entitlement gate proof.
- Refund/dispute/cancel proof.
- Privacy/secret/abuse proof.
- Route/index sync.
- Test-mode versus live-mode readiness checklist.
- Support/legal/tax/refund policy decision with Sujan before launch.
- Account/household entitlement binding proof.

Failure: PR_READY without money-critical negative tests and reconciliation evidence.

## Decision Tree

| If rollout claim mentions... | Required proof                                                               |
| ---------------------------- | ---------------------------------------------------------------------------- |
| Price/tier/trial             | decision record, limits, downgrade/upgrade behavior, Sujan approval          |
| Checkout                     | authenticated household context, metadata minimization, success/cancel paths |
| Billing portal               | customer binding, session authorization, no cross-household access           |
| Webhooks                     | signature verification, idempotency, out-of-order/duplicate events           |
| Entitlements                 | account/household/device gate, revocation, grace period, stale cache         |
| Refund/dispute/cancel        | policy, lifecycle states, support handoff, entitlement changes               |
| Live launch                  | Stripe live checklist, secrets custody, monitoring, rollback, support docs   |

## Execution Detail

Minimum context:

- This plan's `PLAN_STATE.md`, `WORKPACK_INDEX.md`, and `TEST_PROOF_EXPECTATIONS.md`.
- Official Stripe go-live checklist before launch claims.

Research required:

- Review existing Parent deployment and secrets path.
- Review games Cloudflare payment test suite structure.
- Discuss launch pricing, refund, and support policy with Sujan.

Required proof pack:

- Pricing/tier decision record.
- Checkout/portal proof.
- Webhook signature/idempotency proof.
- Subscription lifecycle proof.
- Entitlement gate proof.
- Refund/dispute/cancel proof.
- Privacy/security proof.
- Stripe test-mode versus live-mode checklist.
- Route/index sync.

Expected tests/proof names:

- `payment.rollout.pricing-decision`
- `payment.rollout.checkout-proof`
- `payment.rollout.webhook-proof`
- `payment.rollout.entitlement-proof`
- `payment.rollout.refund-dispute-proof`
- `payment.rollout.privacy-security-proof`
- `payment.rollout.route-sync`
- `payment.rollout.testmode-live-boundary`
- `payment.rollout.account-household-binding`
- `payment.rollout.cross-household-denied`
- `payment.rollout.monitoring-alert-proof`

Failure examples:

- Only Checkout happy path.
- No webhook signature test.
- No duplicate event/idempotency proof.
- No entitlement revocation proof.
- No metadata privacy proof.
- Live-mode key used without launch gate approval.
- Entitlement stays active after cancel/refund/dispute without documented grace policy.
- Billing portal opens for wrong household/account.
