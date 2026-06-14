# Workpack 01: Product Pricing and Entitlement

## Goal

Define the starter bundle, child-device seat math, referral credit application, and the effective entitlement limit that downstream surfaces consume.

## First-touch surface

- `packages/billing-domain/src/billing-pricing-matrix.ts`
- `packages/billing-domain/tests/unit/billing-pricing-matrix.test.ts`

## Read inputs

- [PLAN_STATE.md](../PLAN_STATE.md)
- [PLAN_EXECUTION_BLUEPRINT.md](../PLAN_EXECUTION_BLUEPRINT.md)
- [PRODUCT_PRICING_ENTITLEMENT_MODEL.md](../PRODUCT_PRICING_ENTITLEMENT_MODEL.md)
- [REFERRAL_ENTITLEMENT_MODEL.md](../REFERRAL_ENTITLEMENT_MODEL.md)
- [APP_OWNED_ENTITLEMENT_LEDGER.md](../APP_OWNED_ENTITLEMENT_LEDGER.md)
- [DECISIONS.md](../DECISIONS.md)

## Output files

- [PRODUCT_PRICING_ENTITLEMENT_MODEL.md](../PRODUCT_PRICING_ENTITLEMENT_MODEL.md)
- [REFERRAL_ENTITLEMENT_MODEL.md](../REFERRAL_ENTITLEMENT_MODEL.md)
- [APP_OWNED_ENTITLEMENT_LEDGER.md](../APP_OWNED_ENTITLEMENT_LEDGER.md)
- `docs/proof/payment-subscription-plan/wp01-product-pricing-entitlement/`

## Acceptance

- Starter bundle is defined as one parent portal and one child device.
- Paid extra child-device seats and active referral credits roll into the same effective limit.
- Lost referral credit reduces future entitlement without deleting billing history.
- Pricing rules reject negative or overflow inputs.
- No pricing text or metadata leaks child telemetry or provider internals.

## Proof IDs

- `payment-pricing.free-starter-bundle`
- `payment-pricing.base-one-parent-one-child`
- `payment-pricing.paid-extra-child-device`
- `payment-pricing.effective-child-device-limit`

## Validation

- Docs validation: `npm run format:check`; `npm run lint:schema-boundaries`
- Required proof families: `payment-pricing.free-starter-bundle`, `payment-pricing.base-one-parent-one-child`, `payment-pricing.paid-extra-child-device`, `payment-pricing.extra-parent-slot`, `payment-pricing.effective-child-device-limit`, `payment-pricing.over-limit-grace`, `payment-pricing.safety-critical-grace`, `payment-pricing.rejected-game-economy-model`
- Proof bundle: `docs/proof/payment-subscription-plan/01-free-starter-bundle-proof.md`, `docs/proof/payment-subscription-plan/01-effective-child-device-limit-proof.md`, `docs/proof/payment-subscription-plan/01-safety-critical-grace-proof.md`, `docs/proof/payment-subscription-plan/01-rejected-game-economy-proof.md`

## Negative cases

- Reject negative seat counts.
- Reject overflow or impossible entitlement totals.
- Reject pricing metadata that carries child data or provider secrets.
- Reject any attempt to treat referral credits as cash.

## Failure conditions

- Do not claim entitlement from pricing alone.
- Do not hardcode provider pricing into the UI.
- Do not erase referral history when a credit is lost.
