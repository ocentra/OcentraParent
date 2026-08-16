# Workpack 06: Security, Privacy, and Observability

## Goal

Define the non-negotiable controls around monetization data, secret handling, privacy, observability, and test/live separation.

## First-touch surface

- `packages/billing-domain/src/billing-security-privacy-observability.ts`
- `packages/billing-domain/tests/unit/billing-security-privacy-observability.test.ts`

## Read inputs

- [PLAN_STATE.md](../PLAN_STATE.md)
- [PLAN_EXECUTION_BLUEPRINT.md](../PLAN_EXECUTION_BLUEPRINT.md)
- [SECURITY_PRIVACY_OBSERVABILITY.md](../SECURITY_PRIVACY_OBSERVABILITY.md)
- [BILLING_API_BOUNDARY.md](../BILLING_API_BOUNDARY.md)
- [PROOF_AND_TEST_INVENTORY.md](../PROOF_AND_TEST_INVENTORY.md)
- [REQUIRED_TEST_ASSERTION_MATRIX.md](../REQUIRED_TEST_ASSERTION_MATRIX.md)
- [DECISIONS.md](../DECISIONS.md)

## Output files

- [SECURITY_PRIVACY_OBSERVABILITY.md](../SECURITY_PRIVACY_OBSERVABILITY.md)
- [PROOF_AND_TEST_INVENTORY.md](../PROOF_AND_TEST_INVENTORY.md)
- `output/payment-subscription-plan-proof/06-security-privacy-observability/`

## Acceptance

- Provider and webhook secrets stay server-side.
- Support and admin actions are authenticated, audited, and redacted by default.
- Child data never appears in payment telemetry or provider metadata.
- Test and live mode separation is visible in the ledger and dashboard.
- Queue retries and dead-letter events are observable.
- Provider metadata allow/deny boundaries are explicit.

## Proof IDs

- `payment-security.no-child-data-metadata`
- `payment-security.secret-scan`
- `payment-security.support-view-minimized`
- `payment-security.test-live-separated`
- `payment-security.redacted-logs`

## Validation

- Docs validation: `npm run format:check`; `npm run lint:schema-boundaries`
- Required proof families: `payment-security.provider-metadata-allow-deny`, `payment-security.no-child-data-metadata`, `payment-security.secret-scan`, `payment-security.webhook-smuggling-negative`, `payment-security.webhook-replay`, `payment-security.rate-limit`, `payment-security.bot-abuse-gate`, `payment-security.open-redirect-negative`, `payment-security.redacted-logs`, `payment-security.support-view-minimized`, `payment-security.pci-hosted-checkout-boundary`, `payment-security.referral-abuse-signals`, `payment-security.admin-audit-required`
- Proof bundle: `output/payment-subscription-plan-proof/06-security-privacy-observability/06-metadata-privacy-proof.md`, `output/payment-subscription-plan-proof/06-security-privacy-observability/06-secret-scan-proof.md`, `output/payment-subscription-plan-proof/06-security-privacy-observability/06-referral-abuse-proof.md`, `output/payment-subscription-plan-proof/06-security-privacy-observability/06-support-view-minimized-proof.md`, `output/payment-subscription-plan-proof/06-security-privacy-observability/06-pci-hosted-boundary-proof.md`

## Negative cases

- Reject raw secret logging.
- Reject child data in payment telemetry.
- Reject admin-only timelines on parent surfaces.
- Reject support views that are not redacted by default.

## Failure conditions

- Do not leak secrets.
- Do not log child data.
- Do not expose admin-only timelines to parent surfaces.
- Do not let provider metadata carry child content or policy details.
