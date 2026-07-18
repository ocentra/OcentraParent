# Workpack 12: Payment Plan Handoff Gate

## Goal

Define exactly what `payment-subscription-plan` may assume from the shared Cloudflare module, what remains blocked, and how to keep current checkout output inventory separate from historical plan references.

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

- `blocked / proof-required`
- Proof root: `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/`

## Execution truth

- The current checkout output tree contains no tracked WP00-WP12 proof roots.
- `accepted_proof_roots` for this checkout truth is `none-present`.
- WP01 through WP11 remain historical plan references in the plan docs and are not present artifacts in this checkout's output tree.
- WP12 has a defined expected output path in the plan docs, but the generated proof is absent/not produced in this checkout.
- Current Cloudflare execution is green across the local dev, test, deploy dry-run, and workflow probe surface; the remaining blockers are the absent proof artifact, account/trusted-device/deployment custody states, and no downstream payment acknowledgment being owned or recorded here.
- current module lint truth passed at head `c121ba5eb`, so no current Cloudflare module lint blocker is claimed here.
- This packet is docs-only, so no runtime proof is being manufactured here.

## Current checkout output inventory

- Present in this checkout's output tree:
  - none
- Expected only in plan docs:
  - `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/`

## Historical plan references

- `output/cloudflare-control-plane-plan-proof/01-cloudflare-module-scaffold/`
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
  - WP00 through WP12 are missing from the current checkout output tree
- Accepted-root carried blockers:
  - the checked-in billing-contract sidecar is consumed by `infra/cloudflare/src/index.ts` and `infra/cloudflare/src/fixtures.ts`, so the remaining blocker is proof-artifact absence in this checkout, not missing source ownership
  - current module lint truth passed at head `c121ba5eb`, so no current Cloudflare module lint blocker is claimed here
- Dependency and readiness states:
  - account/session authority: `manual-required / blocked`
  - trusted-parent-device authority: `manual-required / blocked`
  - provider webhook readiness: `blocked / proof-required`
  - storage and queue operations readiness: `blocked / proof-required`
  - portal smoke: `blocked / proof-required`
  - deployment promotion: `blocked / proof-required`
  - downstream payment acknowledgment: `blocked / not-recorded`

## Validations run

- `npm run lint:architecture -- --files docs/plans/cloudflare-control-plane-plan output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate`
- `git diff --check`

## No-claim boundary

- This workpack does not unblock `payment-subscription-plan`.
- This workpack does not prove Cloudflare runtime readiness.
- This workpack does not convert historical plan references for WP01 through WP11 into current checkout proof roots.
- This workpack does not close account authority, trusted-device authority, deployment readiness, or data-custody ownership.
- This workpack does not convert proof absence into payment readiness.

## Failure conditions

- Reject unblocking payment from docs alone when core Cloudflare blockers remain.
- Reject unblocking payment from historical plan references alone when the current checkout output tree does not contain WP01 through WP11.
- Reject unblocking payment from the checked-in billing-contract sidecar alone when the proof artifact is absent or the custody gates remain unresolved.
- Reject unblocking payment from source presence or route manifest presence alone.
- Reject unblocking payment from local dev proof as production deploy proof.
- Reject unblocking payment from auth adapter proof as production account/trusted-device authority proof.
- Reject unblocking payment from billing handler presence as payment semantics readiness.
- Do not store the handoff proof inside this plan folder; use the output proof root.
