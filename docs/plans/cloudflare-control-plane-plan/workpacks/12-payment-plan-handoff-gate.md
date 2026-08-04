# Workpack 12: Payment Plan Handoff Gate

> **2026-07-28 correction:** The later missing-private-billing-import blocker text is historical. `infra/cloudflare` now imports module-local generated billing contracts. This workpack remains open because it has no tracked proof bundle; rerun after installing dependencies and record the actual result.

## Goal

Define exactly what `payment-subscription-plan` may assume from the shared Cloudflare module and what remains blocked.

## First-touch surface

- `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/payment-handoff-proof.md`

## Read inputs

- [PLAN_STATE.md](../PLAN_STATE.md)
- [PARENT_CLOUDFLARE_MODULE_SPEC.md](../PARENT_CLOUDFLARE_MODULE_SPEC.md)
- [SOURCE_SURFACE_STATUS_MATRIX.md](../SOURCE_SURFACE_STATUS_MATRIX.md)
- [WORKPACK_FAMILIES.md](../WORKPACK_FAMILIES.md)
- [PROOF_INDEX.md](../PROOF_INDEX.md)

## Output files

- `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/`
- [SOURCE_SURFACE_STATUS_MATRIX.md](../SOURCE_SURFACE_STATUS_MATRIX.md)

## Ownership boundary

```text
cloudflare-control-plane-plan owns shared Worker module readiness and handoff facts.
payment-subscription-plan owns billing semantics, provider behavior, subscription lifecycle, invoice/grace/referral qualification, and payment runtime status.
account-identity-family-plan owns parent session, household, admin/support authority, and account-provider decisions.
device-trust-bootstrap-plan owns trusted-parent-device authority.
portal-ux-household-surfaces-plan owns consumer UI.
data-custody-storage-plan owns retention/export/deletion policy.
```

## Required handoff artifact fields

The handoff artifact must name, at minimum:

```text
handoff_id
cloudflare_module_state
accepted_proof_roots
missing_proof_roots
carried_blockers
payment_may_assume
payment_must_not_assume
auth_account_dependency_state
trusted_device_dependency_state
provider_webhook_dependency_state
storage_queue_dependency_state
portal_smoke_state
deployment_promotion_state
data_custody_state
downstream_payment_ack_state
no_claim
```

These are field requirements for proof routing, not implementation code prescriptions.

## Status

- `blocked / proof-present`
- Proof root: `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/`

## Execution truth

- The handoff artifact now records the current accepted Cloudflare proof roots relevant to payment assumptions from WP01 through WP11.
- No Cloudflare proof roots remain missing in the current handoff inventory.
- Payment remains blocked because the accepted roots still carry external billing-boundary blockers, WP02 lint debt, manual-required authority and deployment states, and no downstream payment acknowledgment is owned or recorded here.

## Accepted proof roots

- `docs/proof/cloudflare-control-plane-plan/01-cloudflare-module-scaffold/`
- `output/cloudflare-control-plane-plan-proof/02-wrangler-env-bindings/`
- `output/cloudflare-control-plane-plan-proof/03-worker-entrypoint-runtime-guards/`
- `output/cloudflare-control-plane-plan-proof/04-route-manifest-and-domain-contracts/`
- `output/cloudflare-control-plane-plan-proof/05-auth-admin-support-boundary/`
- `output/cloudflare-control-plane-plan-proof/06-storage-do-d1-kv-r2-queue-bindings/`
- `output/cloudflare-control-plane-plan-proof/07-local-dev-seeding-and-fixtures/`
- `output/cloudflare-control-plane-plan-proof/08-testing-runner-and-test-pyramid/`
- `output/cloudflare-control-plane-plan-proof/09-portal-to-worker-e2e-smoke/`
- `output/cloudflare-control-plane-plan-proof/10-security-fuzz-property-observability/`
- `output/cloudflare-control-plane-plan-proof/11-deployment-and-environment-promotion/`

## Exact blocker set

- Missing proof roots:
  - none under the current Cloudflare proof inventory
- Accepted-root carried blockers:
  - broader Cloudflare runtime, local-dev, validation, and deploy reruns require their own focused proof; WP01's pinned Wrangler/Workers-types resolver graph is already retained.
  - the worker's generated billing-contract route is module-local, so no private `packages/billing-domain/src/*` import is a carried payment blocker.
  - WP02 also carries module-lint debt from:
    - `infra/cloudflare/src/fixtures.ts` TypeScript return-path errors
- Dependency and readiness states:
  - account/session authority: `manual-required / blocked`
  - trusted-parent-device authority: `manual-required / blocked`
  - provider webhook readiness: `blocked / proof-present`
  - storage and queue operations readiness: `blocked / proof-present`
  - portal smoke: `blocked / proof-present`
  - deployment promotion: `blocked / proof-present`
  - downstream payment acknowledgment: `blocked / not-recorded`

## Validations run

- Powershell proof-root gate check for WP01 through WP11
- `git diff --check -- output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/00-scope-summary.md output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/01-negative-case-proof.md output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/02-rollback-or-teardown-proof.md output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/16-validation-commands.log output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/payment-handoff-proof.md docs/plans/cloudflare-control-plane-plan/workpacks/12-payment-plan-handoff-gate.md docs/plans/cloudflare-control-plane-plan/PLAN_STATE.md docs/plans/cloudflare-control-plane-plan/NEXT_ACTIONS.md docs/plans/cloudflare-control-plane-plan/SOURCE_SURFACE_STATUS_MATRIX.md`
- `npm run lint:architecture -- --files docs/plans/cloudflare-control-plane-plan output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate`

## No-claim boundary

- This workpack does not unblock `payment-subscription-plan`.
- This workpack does not prove Cloudflare runtime readiness.
- This workpack does not convert blocked-state WP01 through WP11 proof into payment readiness.
- This workpack does not close account authority, trusted-device authority, deployment readiness, or data-custody ownership.

## Failure conditions

- Reject unblocking payment from docs alone when core Cloudflare blockers remain.
- Reject unblocking payment from source presence or route manifest presence alone.
- Reject unblocking payment from local dev proof as production deploy proof.
- Reject unblocking payment from auth adapter proof as production account/trusted-device authority proof.
- Reject unblocking payment from billing handler presence as payment semantics readiness.
- Do not store the handoff proof inside this plan folder; use the output proof root.
