# Workpack 12: Payment Plan Handoff Gate

## Goal

Define exactly what `payment-subscription-plan` may assume from the shared Cloudflare module, what remains blocked, and how to keep current checkout output inventory separate from historical plan references.

## First-touch surfaces

- `docs/proof/cloudflare-control-plane-plan/12-payment-plan-handoff-gate.md`
- `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/payment-handoff-proof.md`

## Read inputs

- [PLAN_STATE.md](../PLAN_STATE.md)
- [PARENT_CLOUDFLARE_MODULE_SPEC.md](../PARENT_CLOUDFLARE_MODULE_SPEC.md)
- [SOURCE_SURFACE_STATUS_MATRIX.md](../SOURCE_SURFACE_STATUS_MATRIX.md)
- [WORKPACK_FAMILIES.md](../WORKPACK_FAMILIES.md)
- [PROOF_INDEX.md](../PROOF_INDEX.md)

## Output files

- `docs/proof/cloudflare-control-plane-plan/12-payment-plan-handoff-gate.md`
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

- `blocked / retained-receipt-present / downstream-ack-required`
- Retained receipt: `docs/proof/cloudflare-control-plane-plan/12-payment-plan-handoff-gate.md`
- Raw/generated proof root: `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/`

## Execution truth

- Raw proof-root custody is recorded separately from Git tracking and acceptance: WP07 and WP12 raw output directories are physically present in this checkout, but `output/` is ignored and neither directory is Git-tracked or accepted as a handoff root.
- A compact tracked WP12 receipt records current-head local validation and the exact blocker handoff without promoting ignored output into source.
- `accepted_proof_roots` for this checkout truth is `none accepted`; physical raw-root presence alone does not satisfy acceptance or downstream custody.
- WP00 through WP06 and WP08 through WP11 remain historical plan references with no physical raw root in this checkout. WP07 is present-but-unaccepted through its raw directory and tracked retained receipt; WP12 is present as raw ignored output plus a tracked blocker receipt. WP00 is parity-extraction history; it is included here as historical-but-absent rather than left ambiguous as never produced.
- WP12 retains its raw/generated output route, but payment handoff truth survives cleanup in the tracked receipt.
- Current Cloudflare lint, unit, contract, real Wrangler integration, generated-artifact lint, and billing-core unit gates are green at `cbb8421875492176bd2a3d5b95eaa7fa0dd8210e`.
- Remaining blockers are absent accepted WP00-WP11 receipts, account/trusted-device/provider/storage/portal/deployment/data-custody states, and no downstream payment acknowledgment.
- This packet records observed validation and blockers; it does not manufacture production runtime proof.

## Current checkout output inventory

- Physically present raw output roots in this checkout, ignored and untracked:
  - `output/cloudflare-control-plane-plan-proof/07-local-dev-seeding-and-fixtures/`
  - `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/`
- Git-tracked raw output roots:
  - none; `output/` is intentionally ignored
- Retained tracked receipts outside generated output:
  - `docs/proof/cloudflare-control-plane-plan/07-local-dev-seeding-and-fixtures.md` (present but unaccepted)
  - `docs/proof/cloudflare-control-plane-plan/12-payment-plan-handoff-gate.md` (blocker receipt, not an accepted handoff)
- Expected only in plan docs:
  - `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/`

## Historical plan references

- `output/cloudflare-control-plane-plan-proof/00-games-infra-parity-extraction/`
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

- Missing accepted proof roots:
  - WP00 through WP06 and WP08 through WP11 have no retained receipt in this checkout
  - WP07 has a retained receipt but remains unaccepted
  - WP12 downstream payment acknowledgment is missing
- Accepted-root carried blockers:
  - the checked-in billing-contract sidecar is consumed by `infra/cloudflare/src/index.ts` and `infra/cloudflare/src/fixtures.ts`, so the remaining blocker is proof-artifact absence in this checkout, not missing source ownership
  - current local Cloudflare and billing-core focused gates passed at `cbb8421875492176bd2a3d5b95eaa7fa0dd8210e`; this does not remove production or dependency blockers
- Dependency and readiness states:
  - account/session authority: `manual-required / blocked`
  - trusted-parent-device authority: `manual-required / blocked`
  - provider webhook readiness: `blocked / proof-required`
  - storage and queue operations readiness: `blocked / proof-required`
  - portal smoke: `blocked / proof-required`
  - deployment promotion: `blocked / proof-required`
  - downstream payment acknowledgment: `blocked / not-recorded`

## Validations run

- `npm run lint --workspace @ocentra-parent/cloudflare-control-plane`
- `npm run test:unit --workspace @ocentra-parent/cloudflare-control-plane`
- `npm run test:contract --workspace @ocentra-parent/cloudflare-control-plane`
- `npm run test:integration --workspace @ocentra-parent/cloudflare-control-plane`
- `npm run lint:generated-artifacts`
- `cargo test -p ocentra-billing-core --test unit`
- `npm run lint:architecture -- --files docs/plans/cloudflare-control-plane-plan docs/proof/cloudflare-control-plane-plan/12-payment-plan-handoff-gate.md`
- `git diff --check`

## No-claim boundary

- This workpack does not unblock `payment-subscription-plan`.
- This workpack does not prove Cloudflare runtime readiness.
- This workpack does not convert historical plan references or the retained blocker receipt into accepted WP00 through WP11 proof roots.
- This workpack does not close account authority, trusted-device authority, deployment readiness, or data-custody ownership.
- This workpack does not convert proof absence into payment readiness.

## Failure conditions

- Reject unblocking payment from docs alone when core Cloudflare blockers remain.
- Reject unblocking payment from historical plan references or ignored raw output alone; physical presence, Git tracking, retained receipt, and acceptance are distinct custody states.
- Reject unblocking payment from the checked-in billing-contract sidecar or the retained blocker receipt while custody/dependency gates or downstream acknowledgment remain unresolved.
- Reject unblocking payment from source presence or route manifest presence alone.
- Reject unblocking payment from local dev proof as production deploy proof.
- Reject unblocking payment from auth adapter proof as production account/trusted-device authority proof.
- Reject unblocking payment from billing handler presence as payment semantics readiness.
- Do not store the handoff proof inside this plan folder. Keep raw/generated evidence under `output/` and the compact cleanup-safe receipt under `docs/proof/cloudflare-control-plane-plan/`.
