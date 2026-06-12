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

Failure: no billing DONE without webhook, idempotency, entitlement, refund/dispute, privacy, and abuse proof.
