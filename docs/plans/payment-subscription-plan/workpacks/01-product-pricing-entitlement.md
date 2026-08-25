# Workpack 01: Product Pricing and Entitlement

## Goal

Define the starter bundle, child-device seat math, referral credit application, and the effective entitlement limit that downstream surfaces consume.

## First-touch surface

- `crates/entitlement-core/src/entitlement_snapshot.rs`
- `crates/entitlement-core/src/entitlement_snapshot_derivation.rs`
- `crates/schema/src/billing_entitlement_proof.rs`
- `packages/schema-domain/src/generated-billing-entitlement-proof.ts`
- expected test root (absent):
  `crates/entitlement-core/tests/unit/entitlement_snapshot_derivation.rs`

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

- Status: `source present / production caller and expected tests open / proof deferred`
- The Rust owner implements checked
  `base_child_device_limit + active_referral_credits + paid_extra_child_device_seats`
  derivation, rejects a zero base and arithmetic overflow, and produces only an
  unsigned projection. Provider values remain input-only; the projection has
  no signature, key identity, or capability authority.
- No non-test production caller consumes this derivation. No durable
  provider-owned billing/referral ledger, Account-authority composition,
  Device-Trust binding, or real issuer/signer bridge reaches it. Paid access
  therefore remains manual-required.
- `packages/billing-domain` is absent and must not be recreated as a parallel
  TypeScript business owner. Generated `schema-domain` files remain edge
  contracts only.
- The named pricing-matrix/read-model TypeScript files, focused Rust derivation
  test, and current proof root are absent. Historical `done / proof-present`
  wording is withdrawn; proof is regenerated only after source ownership,
  expected tests, and focused validation close.

## Source-pass validation (2026-08-24)

- Focused Rust library checks for `entitlement-core` and `schema` passed.
- Focused architecture, re-export, source-shape, no-test-doubles,
  validation-bypass, Enforcer lane/hub guards, and `git diff --check` passed.
- No tests, proof, precommit, CI, PR, or merge were run in this source audit.

## No-claim boundary

- This packet does not prove hosted checkout, billing portal, webhook lifecycle, Cloudflare payment runtime readiness, entitlement delivery, device-trust binding, parent dashboard readiness, or support/admin authority.
- The carried WP00 Cloudflare blocker and the missing provider/Account/Device
  Trust/issuer composition prevent runtime entitlement claims. The arithmetic
  source alone does not prove pricing authority or paid access.
