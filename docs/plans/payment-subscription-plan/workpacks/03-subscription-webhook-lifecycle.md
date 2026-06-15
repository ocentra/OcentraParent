# Workpack 03: Subscription Webhook Lifecycle

Purpose: define signature validation, deduplication, lifecycle transitions, and settlement for provider events.

## Owns

- `SUBSCRIPTION_WEBHOOK_LIFECYCLE.md`
- PSP-003 and the webhook part of PSP-005

## Must prove

- Valid signatures are accepted.
- Invalid signatures are rejected.
- Duplicate events do not double-grant entitlement.
- Out-of-order events normalize to one ledger state.
- Reconciliation or retry work is queued when needed.

## Proof path

- Use `docs/proof/payment-subscription-plan/wp03/` or the owning crate's local proof directory.

## Failure conditions

- The workpack fails if an accepted webhook can change access without a ledger entry.
- The workpack fails if dedupe markers are missing.
- The workpack fails if replay or retry creates duplicate entitlement.
