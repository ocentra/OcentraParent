# Workpack 01: Product Pricing and Entitlement

## Goal

Define the starter bundle, child-device seat math, referral credit application, and the effective entitlement limit that downstream surfaces consume.

## First-touch surface

- `packages/schema-domain/src/billing-entitlement.ts`
- `packages/schema-domain/src/billing-pricing-matrix-proof.ts`
- `packages/schema-domain/src/billing-entitlement-proof-read-model.ts`
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
- `output/payment-subscription-plan-proof/01-product-pricing-entitlement/`

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
- `payment-pricing.extra-parent-slot`
- `payment-pricing.effective-child-device-limit`
- `payment-pricing.over-limit-grace`
- `payment-pricing.safety-critical-grace`
- `payment-pricing.rejected-game-economy-model`

## Validation

- Docs validation: `npm run format:check`; `npm run lint:schema-boundaries`
- Required proof families: `payment-pricing.free-starter-bundle`, `payment-pricing.base-one-parent-one-child`, `payment-pricing.paid-extra-child-device`, `payment-pricing.extra-parent-slot`, `payment-pricing.effective-child-device-limit`, `payment-pricing.over-limit-grace`, `payment-pricing.safety-critical-grace`, `payment-pricing.rejected-game-economy-model`
- Proof bundle: `output/payment-subscription-plan-proof/01-product-pricing-entitlement/01-free-starter-bundle-proof.md`, `output/payment-subscription-plan-proof/01-product-pricing-entitlement/01-effective-child-device-limit-proof.md`, `output/payment-subscription-plan-proof/01-product-pricing-entitlement/01-safety-critical-grace-proof.md`, `output/payment-subscription-plan-proof/01-product-pricing-entitlement/01-rejected-game-economy-proof.md`

## Negative cases

- Reject negative seat counts.
- Reject overflow or impossible entitlement totals.
- Reject pricing metadata that carries child data or provider secrets.
- Reject any attempt to treat referral credits as cash.

## Failure conditions

- Do not claim entitlement from pricing alone.
- Do not hardcode provider pricing into the UI.
- Do not erase referral history when a credit is lost.

## Execution truth

- Status: `done / proof-present`
- Rust-first override: canonical cross-boundary ownership stays with `crates/schema`; the `@ocentra-parent/schema-domain` edits in this packet are transitional thin edge validation and proof data only, not restored TS business ownership.
- Actual implementation surface stays in `@ocentra-parent/schema-domain` proof and edge-decoder files plus the focused `@ocentra-parent/billing-domain` unit test; `packages/billing-domain/src/billing-pricing-matrix.ts` is not present in this worktree and was not revived as a TS business owner.
- Proven families:
  - `payment-pricing.free-starter-bundle`
  - `payment-pricing.base-one-parent-one-child`
  - `payment-pricing.paid-extra-child-device`
  - `payment-pricing.extra-parent-slot`
  - `payment-pricing.effective-child-device-limit`
  - `payment-pricing.over-limit-grace`
  - `payment-pricing.safety-critical-grace`
  - `payment-pricing.rejected-game-economy-model`
- Proof root: `output/payment-subscription-plan-proof/01-product-pricing-entitlement/`

## Validation run

- `cmd /c npm run test --workspace @ocentra-parent/billing-domain -- tests/unit/billing-pricing-matrix.test.ts`
- `cmd /c npm run build --workspace @ocentra-parent/schema-domain`
- `cmd /c npm run lint:architecture -- --files packages/schema-domain/src/billing-entitlement-values.ts packages/schema-domain/src/billing-entitlement.ts packages/schema-domain/src/billing-pricing-matrix-proof.ts packages/schema-domain/src/billing-entitlement-proof-read-model.ts packages/billing-domain/tests/unit/billing-pricing-matrix.test.ts`
- `cmd /c npx prettier --check packages/schema-domain/src/billing-entitlement-values.ts packages/schema-domain/src/billing-entitlement.ts packages/schema-domain/src/billing-pricing-matrix-proof.ts packages/schema-domain/src/billing-entitlement-proof-read-model.ts packages/billing-domain/tests/unit/billing-pricing-matrix.test.ts docs/plans/payment-subscription-plan/PRODUCT_PRICING_ENTITLEMENT_MODEL.md`
- `cmd /c npm run lint:schema-boundaries`
  - broad gate, failed on existing out-of-scope weak assertions under `crates/agent-protocol/tests/contract/*` and `crates/agent-service/tests/unit/lan_pairing.rs`; not a WP01 file blocker

## No-claim boundary

- This packet does not prove hosted checkout, billing portal, webhook lifecycle, Cloudflare payment runtime readiness, entitlement delivery, device-trust binding, parent dashboard readiness, or support/admin authority.
- The carried WP00 Cloudflare blocker still prevents broader payment runtime readiness claims even though WP01 pricing and entitlement model proof is complete.
