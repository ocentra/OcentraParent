# Product Pricing Entitlement Model

Purpose: define the units the billing system sells and the entitlement math the rest of the product consumes.

## Units

| Unit              | Meaning                                                        | Source                                                         | Revocable                                |
| ----------------- | -------------------------------------------------------------- | -------------------------------------------------------------- | ---------------------------------------- |
| Parent portal     | One billing parent account                                     | Base bundle                                                    | No, unless account is closed or revoked. |
| Child device seat | One active child device entitlement                            | Base bundle, paid add-on, referral credit, or support override | Yes.                                     |
| Referral credit   | Temporary additional child-device entitlement                  | Qualified referral                                             | Yes.                                     |
| Grace seat        | Temporary continuation during a billing failure or policy hold | Billing or support action                                      | Yes.                                     |
| Support override  | Manual courtesy entitlement                                    | Support/admin action                                           | Yes.                                     |

## Default bundle

- The starter bundle includes 1 parent portal and 1 child device.
- The base starter bundle may include a free or trial service window, but the bundle size itself is not optional.
- Additional child-device seats are monthly add-ons.
- Default target price for an additional child-device seat is $1/month unless market or legal validation changes it.

## Entitlement formula

- `effectiveChildSeats = baseChildSeat + paidChildSeats + activeReferralCredits + approvedSupportSeats - revokedOrExpiredSeats`
- `baseChildSeat = 1`
- `parentSeat = 1`
- The product may show marketing plan names, but the ledger must track units.

## Rules

- Referral credits and paid seats are not cash.
- A lost referral must reduce future entitlement, not erase history.
- The billing system must be able to explain every seat in the household from the ledger alone.
