# Support/Admin Billing Dashboard

Purpose: define the operator surface for refunds, disputes, adjustments, and reconciliation.

## Required sections

- Account search and billing timeline.
- Payment attempt history and provider refs.
- Invoice, refund, and dispute state.
- Manual credit, debit, freeze, or restore actions.
- Reconciliation and dead-letter queue visibility.

## Rules

- Every support action must be audited.
- Every support view must be redacted by default.
- The dashboard must not expose child telemetry or child content.
- Support can change billing state, but not device trust or account ownership.

## Required behaviors

| Action               | Expected result                                     |
| -------------------- | --------------------------------------------------- |
| Search account       | Return a redacted billing timeline.                 |
| Issue refund         | Write a ledger entry and update entitlement impact. |
| Open dispute review  | Surface provider refs and evidence pointers.        |
| Apply manual credit  | Record the adjustment and actor.                    |
| Retry reconciliation | Re-run the queued or dead-lettered billing work.    |

## Failure conditions

- Do not let support bypass audit logging.
- Do not let support read child data.
- Do not let support mutate provider history without a ledger entry.
