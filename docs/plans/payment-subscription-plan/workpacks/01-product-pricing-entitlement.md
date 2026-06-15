# Workpack 01: Product Pricing and Entitlement

Purpose: define the starter bundle, paid child-device seats, referral credits, and the entitlement math the rest of the route consumes.

## Owns

- `PRODUCT_PRICING_ENTITLEMENT_MODEL.md`
- `REFERRAL_ENTITLEMENT_MODEL.md`
- PSP-007 through PSP-010

## Must prove

- Starter bundle size is explicit.
- Paid child-seat pricing is explicit.
- Effective seat math is explainable from the ledger.
- Referral credits are separate from household invites.
- Lost referral behavior reduces entitlement without deleting history.

## Proof path

- Use `docs/proof/payment-subscription-plan/wp01/` or the owning crate's local proof directory.

## Failure conditions

- The workpack fails if the effective child-seat count cannot be derived from the ledger alone.
- The workpack fails if referral and household invites are conflated.
- The workpack fails if child data appears in pricing or referral payloads.
