# Workpack 11: Parent Website Billing Dashboard

## Goal

Define the parent-facing billing dashboard surface and its self-service actions.

## First-touch surface

- `packages/parent-domain/src/billing-entitlement.ts`
- `packages/parent-domain/src/billing-entitlement-proof.ts`

## Read inputs

- [PLAN_STATE.md](../PLAN_STATE.md)
- [PLAN_EXECUTION_BLUEPRINT.md](../PLAN_EXECUTION_BLUEPRINT.md)
- [PARENT_WEBSITE_BILLING_DASHBOARD.md](../PARENT_WEBSITE_BILLING_DASHBOARD.md)
- [RESEARCH_AND_UI_GUIDANCE.md](../RESEARCH_AND_UI_GUIDANCE.md)
- [BILLING_API_BOUNDARY.md](../BILLING_API_BOUNDARY.md)
- [APP_OWNED_ENTITLEMENT_LEDGER.md](../APP_OWNED_ENTITLEMENT_LEDGER.md)

## Output files

- [PARENT_WEBSITE_BILLING_DASHBOARD.md](../PARENT_WEBSITE_BILLING_DASHBOARD.md)
- [RESEARCH_AND_UI_GUIDANCE.md](../RESEARCH_AND_UI_GUIDANCE.md)
- `docs/proof/payment-subscription-plan/wp11-parent-website-billing-dashboard/`

## Acceptance

- Parent-authenticated users can see their billing state.
- Seat math, invoices, referral credits, paid seats, and grace state are visible.
- The portal handoff is available.
- Child data and support-only fields stay hidden.
- Cancel and change-plan actions reflect ledger state.

## Proof IDs

- `payment-dashboard.parent-account-visible`
- `payment-dashboard.current-plan-visible`
- `payment-dashboard.child-device-usage-visible`
- `payment-dashboard.referral-credit-visible`
- `payment-dashboard.invoice-visible`
- `payment-dashboard.change-plan-visible`
- `payment-dashboard.wrong-household-denied`

## Validation

- Docs validation: `npm run format:check`; `npm run lint:schema-boundaries`
- Required proof families: `payment-dashboard.parent-account-visible`, `payment-dashboard.current-plan-visible`, `payment-dashboard.child-device-usage-visible`, `payment-dashboard.referral-credit-visible`, `payment-dashboard.paid-seat-visible`, `payment-dashboard.invoice-visible`, `payment-dashboard.change-plan-visible`, `payment-dashboard.cancel-visible`, `payment-dashboard.billing-portal-link`, `payment-dashboard.license-snapshot-visible`, `payment-dashboard.wrong-household-denied`, `payment-dashboard.no-child-private-data`
- Proof bundle: `docs/proof/payment-subscription-plan/11-parent-website-dashboard-proof.md`, `docs/proof/payment-subscription-plan/11-dashboard-wrong-household-negative-proof.md`, `docs/proof/payment-subscription-plan/11-dashboard-no-child-private-data-proof.md`

## Negative cases

- Reject dashboard displays of child telemetry.
- Reject dashboard claims that do not match entitlement state.
- Reject any parent surface that exposes support-only fields.
- Reject portal handoff omissions.

## Failure conditions

- Do not display child telemetry.
- Do not lie about entitlement state.
- Do not omit the portal handoff.
