# Product Pricing Entitlement Model

Purpose: define the units the billing system sells and the entitlement math the rest of the product consumes.

## Starter household

- 1 parent portal.
- 1 child device.
- Trial/free starter service.
- Extra parent access is not counted as a child-device seat; it remains a separate manual-required add-on state until a later workpack proves otherwise.

## Referral

- A parent can referral-invite other parents to create their own households.
- Each active qualifying referred parent or household grants +1 child-device credit.
- Referral credit lasts only while the referred household remains active and qualified.
- Lost referral enters grace or over-limit state, not data deletion.

## Paid add-on

- Extra child device seat.
- Suggested initial price is $1/month/device.
- Price must live in the product catalog or provider mapping, not hardcoded logic.
- Optional later add-ons may include extra co-parent or guardian invite slots, remote access, advanced history or reporting retention, priority support, and manual or enterprise invoice plans.

## Effective entitlement

```text
effectiveChildDeviceLimit =
  baseChildDeviceLimit
  + activeReferralChildDeviceCredits
  + paidExtraChildDeviceSeats
  - revokedCreditAdjustments
```

The effective limit is derived from the app-owned entitlement ledger. Hosted checkout redirects, provider UI success states, and portal return states must not be treated as entitlement truth on their own.

## Units

| Unit | Meaning | Source | Revocable |
| --- | --- | --- | --- |
| Parent portal | One billing parent account | Base bundle | No, unless account is closed or revoked. |
| Child device seat | One active child device entitlement | Base bundle, paid add-on, referral credit, or support override | Yes. |
| Referral credit | Temporary additional child-device entitlement | Qualified referral | Yes. |
| Grace seat | Temporary continuation during a billing failure or policy hold | Billing or support action | Yes. |
| Support override | Manual courtesy entitlement | Support/admin action | Yes. |

## Rules

- Referral credits and paid seats are not cash.
- A lost referral must reduce future entitlement, not erase history.
- The billing system must be able to explain every seat in the household from the ledger alone.
- If the household exceeds entitlement, enter over-limit grace instead of deleting data or uninstalling child service.
- Over-limit grace must not silently disable safety-critical local protection.
- The system must block adding new child devices while showing upgrade, invite-more, or remove-device options.
- Pricing references must reject game-economy tokens, currencies, or marketplace semantics; household billing is not a coin, gem, loot, crate, or battle-pass system.
