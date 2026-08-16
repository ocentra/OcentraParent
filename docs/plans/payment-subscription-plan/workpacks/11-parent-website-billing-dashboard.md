# Workpack 11: Parent Website Billing Dashboard

## Goal

Define the parent-facing billing dashboard surface and its self-service actions.

## Ownership boundary

```text
payment-subscription-plan owns billing dashboard state semantics, allowed fields, billing-only visibility, and self-service billing actions.
parent-domain/apps/portal own selected parent projection proof only when the targeted proof file builds and runs.
account-identity-family-plan owns parent account/session/role authority.
data-custody-storage-plan owns privacy, retention, export, and deletion constraints for billing records.
support/admin fields remain hidden unless WP12 proves role, redaction, and audit boundaries.
```

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
- [REQUIRED_TEST_ASSERTION_MATRIX.md](../REQUIRED_TEST_ASSERTION_MATRIX.md)

## Output files

- [PARENT_WEBSITE_BILLING_DASHBOARD.md](../PARENT_WEBSITE_BILLING_DASHBOARD.md)
- [RESEARCH_AND_UI_GUIDANCE.md](../RESEARCH_AND_UI_GUIDANCE.md)
- `output/payment-subscription-plan-proof/11-parent-website-billing-dashboard/`

## Acceptance

- Parent-authenticated users can see their billing state.
- Seat math, invoices, referral credits, paid seats, and grace state are visible.
- The portal handoff is available.
- Child data and support-only fields stay hidden.
- Cancel and change-plan actions reflect ledger state.
- The targeted parent-domain proof file is required and billing-domain tests do not substitute for it.

## Required proof fields

The selected proof must name, at minimum:

```text
parent_account_state
household_authority_state
dashboard_projection_state
seat_math_state
invoice_visibility_state
referral_credit_visibility_state
portal_handoff_state
license_snapshot_visibility_state
wrong_household_state
child_private_data_state
support_only_field_state
cancel_change_plan_state
targeted_parent_proof_state
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

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
- Required proof families: `payment-dashboard.parent-account-visible`, `payment-dashboard.current-plan-visible`, `payment-dashboard.child-device-usage-visible`, `payment-dashboard.referral-credit-visible`, `payment-dashboard.paid-seat-visible`, `payment-dashboard.invoice-visible`, `payment-dashboard.change-plan-visible`, `payment-dashboard.cancel-visible`, `payment-dashboard.billing-portal-link`, `payment-dashboard.license-snapshot-visible`, `payment-dashboard.wrong-household-denied`, `payment-dashboard.no-child-private-data`, `payment-dashboard.targeted-parent-proof-file`
- Proof bundle: `output/payment-subscription-plan-proof/11-parent-website-billing-dashboard/11-parent-website-dashboard-proof.md`, `output/payment-subscription-plan-proof/11-parent-website-billing-dashboard/11-dashboard-wrong-household-negative-proof.md`, `output/payment-subscription-plan-proof/11-parent-website-billing-dashboard/11-dashboard-no-child-private-data-proof.md`
- Execution note: `packages/parent-domain/tests/unit/billing-entitlement-proof.test.ts` is the targeted parent-surface proof file and already exists, but the current blocker is `@ocentra-parent/parent-domain` build/import failure before that focused proof can run.

## Negative cases

- Reject dashboard displays of child telemetry.
- Reject dashboard claims that do not match entitlement state.
- Reject any parent surface that exposes support-only fields.
- Reject portal handoff omissions.
- Reject billing-domain-only tests as parent dashboard proof.

## Failure conditions

- Do not display child telemetry.
- Do not lie about entitlement state.
- Do not omit the portal handoff.
- Do not claim parent-dashboard readiness until the targeted parent-surface proof runs or an explicit blocker is carried.
