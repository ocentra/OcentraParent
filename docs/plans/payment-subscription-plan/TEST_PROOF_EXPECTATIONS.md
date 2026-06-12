# Test and Proof Expectations

| Risk surface           | Expected proof                                                                                                             |
| ---------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| Pricing/tier model     | Product/price fixture proof, trial/grace/seat boundary, downgrade/upgrade matrix.                                          |
| Checkout               | authenticated checkout request, Turnstile/abuse gate, invalid product rejection, return/cancel state, no secret in client. |
| Webhooks               | signature verification, raw-body handling, idempotency, duplicate event, stale event, unknown event, replay, retry.        |
| Subscription lifecycle | created, active, trialing, past_due, canceled, unpaid, resumed, upgraded, downgraded, payment_failed.                      |
| Entitlement            | household/device/role gating, stale entitlement rejection, grace period, revoke after cancellation/refund/dispute.         |
| Privacy                | forbidden child data metadata check, redacted logs, export/delete boundary, support diagnostics.                           |
| Security/abuse         | brute force, rate limit, CSRF/origin/header, open redirect, webhook smuggling, DoS/backpressure.                           |
| Observability          | metrics, traces, alerts, reconciliation report, admin/support audit.                                                       |
| PR gate                | proof artifacts, route sync, skipped-risk notes, remaining gaps.                                                           |

## Where tests should live

- When implementation exists, place unit/integration tests in the billing/AI/payment package test trees used by the assigned workpack.
- Capture screenshot/log artifact and proof-output files in the plan proof folder or the owning package proof folder.
- Use real contract fixtures and protocol boundaries; avoid mock-only coverage for webhook and session entitlement boundaries.

## Expected test/proof inventory

- `payment-subscription.billing.webhook-idempotency`: duplicate and replayed webhooks are replay-safe and do not duplicate entitlement writes.
- `payment-subscription.billing.entitlement-state-boundary`: trial/grace/downgrade/upgrade/cancel states enforce role/device/family boundaries without over-privilege.
- `payment-subscription.billing.subscription-lifecycle`: lifecycle transitions include unpaid/past_due/resume flows with stale/replay handling.
- `payment-subscription.billing.refund-dispute-abuse`: refund/dispute/reverse events are idempotent and non-privilege escalating.
- `payment-subscription.billing.security-rate-limit`: abuse, brute force, CSRF/origin/header, and webhook smuggling/fraud tests remain negative-first.

## Failure conditions

- No billing DONE/PR_READY if entitlement writes are validated only on happy path.
- No billing DONE/PR_READY if webhook idempotency/replay/backpressure proofs are missing.
- No billing DONE/PR_READY if privacy, parent consent, and abuse/fraud controls lack negative proof or proof artifacts.
