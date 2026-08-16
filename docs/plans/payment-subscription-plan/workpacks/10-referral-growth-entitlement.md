# Workpack 10: Referral Growth and Entitlement

## Goal

Define referral qualification, anti-abuse, credit grant and revoke behavior, and grace handling.

## First-touch surface

- `packages/billing-domain/src/billing-entitlement.ts`
- `packages/billing-domain/src/billing-entitlement-runtime-proof.ts`

## Read inputs

- [PLAN_STATE.md](../PLAN_STATE.md)
- [PLAN_EXECUTION_BLUEPRINT.md](../PLAN_EXECUTION_BLUEPRINT.md)
- [REFERRAL_ENTITLEMENT_MODEL.md](../REFERRAL_ENTITLEMENT_MODEL.md)
- [PRODUCT_PRICING_ENTITLEMENT_MODEL.md](../PRODUCT_PRICING_ENTITLEMENT_MODEL.md)
- [APP_OWNED_REFERRAL_LEDGER.md](../APP_OWNED_REFERRAL_LEDGER.md)
- [APP_OWNED_ENTITLEMENT_LEDGER.md](../APP_OWNED_ENTITLEMENT_LEDGER.md)

## Output files

- [REFERRAL_ENTITLEMENT_MODEL.md](../REFERRAL_ENTITLEMENT_MODEL.md)
- [APP_OWNED_REFERRAL_LEDGER.md](../APP_OWNED_REFERRAL_LEDGER.md)
- [APP_OWNED_ENTITLEMENT_LEDGER.md](../APP_OWNED_ENTITLEMENT_LEDGER.md)
- `output/payment-subscription-plan-proof/10-referral-growth-entitlement/`

## Acceptance

- Referral qualification is explicit.
- Self-referral and duplicate referral cases are rejected.
- Qualified referrals grant child-device credit.
- Revocation changes entitlement without deleting history.
- Over-limit grace state is visible when referral credits are lost.

## Proof IDs

- `payment-referral.invite-created`
- `payment-referral.qualified-credit-granted`
- `payment-referral.active-referred-parent-required`
- `payment-referral.lost-referral-credit-removed`
- `payment-referral.self-referral-rejected`
- `payment-referral.same-household-rejected`
- `payment-referral.same-device-farm-rejected`
- `payment-referral.entitlement-recalculated`
- `payment-referral.over-limit-grace-visible`

## Validation

- Docs validation: `npm run format:check`; `npm run lint:schema-boundaries`
- Required proof families: `payment-referral.invite-created`, `payment-referral.invite-opened`, `payment-referral.signup-started`, `payment-referral.account-created`, `payment-referral.household-created`, `payment-referral.setup-activated`, `payment-referral.qualified-credit-granted`, `payment-referral.active-referred-parent-required`, `payment-referral.lost-referral-credit-removed`, `payment-referral.referral-grace`, `payment-referral.self-referral-rejected`, `payment-referral.same-household-rejected`, `payment-referral.same-device-farm-rejected`, `payment-referral.same-payment-method-manual-review`, `payment-referral.fraud-review`, `payment-referral.entitlement-recalculated`, `payment-referral.over-limit-grace-visible`, `payment-referral.no-data-delete-on-lost-referral`
- Proof bundle: `output/payment-subscription-plan-proof/10-referral-growth-entitlement/10-referral-state-machine-proof.md`, `output/payment-subscription-plan-proof/10-referral-growth-entitlement/10-referral-qualification-proof.md`, `output/payment-subscription-plan-proof/10-referral-growth-entitlement/10-referral-abuse-negative-proof.md`, `output/payment-subscription-plan-proof/10-referral-growth-entitlement/10-referral-loss-entitlement-proof.md`, `output/payment-subscription-plan-proof/10-referral-growth-entitlement/10-over-limit-grace-proof.md`

## Negative cases

- Reject referral and household invites being conflated.
- Reject referral credit becoming cash-like by accident.
- Reject abuse signals that are not auditable.
- Reject credit grants that do not trace back to a qualified referral.

## Failure conditions

- Do not conflate referral and household invites.
- Do not let referral credit become cash-like by accident.
- Do not lose auditability for abuse signals or revocations.
