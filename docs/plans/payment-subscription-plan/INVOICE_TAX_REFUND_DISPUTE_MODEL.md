# Invoice, Tax, Refund, and Dispute Model

Purpose: define invoice, tax, refund, dispute, cancellation, and grace behavior as one coordinated contract.

## Lifecycle

| Event                  | Required ledger effect                                                |
| ---------------------- | --------------------------------------------------------------------- |
| Invoice created        | Record plan, period, currency, and expected tax.                      |
| Invoice finalized      | Record invoice number, tax line, and provider reference.              |
| Invoice paid           | Advance subscription state and entitlement state.                     |
| Invoice payment failed | Enter grace or hold state.                                            |
| Refund issued          | Record full or partial reversal and entitlement impact.               |
| Dispute opened         | Freeze or limit entitlements per policy and retain evidence pointers. |
| Dispute won/lost       | Close the dispute path and apply the final entitlement outcome.       |
| Subscription cancelled | Record end-of-term or immediate cancellation policy.                  |

## Rules

- Tax may be provider-calculated, but the app ledger records the final tax result.
- Refunds and disputes must preserve history; they do not delete the invoice trail.
- Cancellation defaults to end-of-term unless policy or support action says otherwise.
- Grace is time-bound and must be visible in the entitlement ledger and the snapshot.

## Grace model

- Billing grace uses the canonical state names `billingGraceActive` and `billingGraceExpired`.
- Billing grace belongs to WP05 and must not be used to describe referral credit loss.
- Referral grace lives in `REFERRAL_ENTITLEMENT_MODEL.md` as `referralGraceActive` and `referralCreditLost`.

## Failure conditions

- Do not allow a refund or dispute to silently change access without a ledger entry.
- Do not let tax or invoice data contain child telemetry.
- Do not treat a provider invoice event as final until the app ledger records it.
