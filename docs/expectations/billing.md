# Billing And Subscription Expectations

Billing features should gate paid product value without breaking local child safety irresponsibly.

## Expected Deliverables

- Plan contract.
- Entitlement contract.
- Stripe boundary.
- Billing status sync.
- Trial state.
- Device limit policy.
- Grace/failure behavior.

## Acceptance

- Paid features check entitlements through typed contracts.
- Billing failures are visible.
- Local safety behavior degrades deliberately when billing cannot be checked.
- No billing secret is committed or exposed to the portal.
- Billing state changes are auditable.

## Non-Goals

- Do not put Stripe logic inside capture, journal, or enforcement modules.
- Do not make payment failures silently disable critical local safety behavior.
- Do not add billing provider code before plan and entitlement contracts exist.

## Done Signal

A paid product capability is gated by typed entitlements, failure behavior is explicit, and billing concerns stay outside core child-device evidence and enforcement modules.
