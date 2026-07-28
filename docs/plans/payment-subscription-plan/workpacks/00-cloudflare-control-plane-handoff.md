# Workpack 00: Cloudflare Control Plane Handoff

## Current status

- Verdict: `blocked / proof-required`
- Expected payment proof root: `output/payment-subscription-plan-proof/00-cloudflare-control-plane-handoff/`
- Expected upstream handoff artifact (absent/not produced in this checkout): `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/payment-handoff-proof.md`
- Runtime effect: payment runtime remains blocked; this packet only records the shared Cloudflare dependency truth and its exact carried blockers.

## Goal

Consume the shared Cloudflare plan handoff before any payment runtime slice starts.

## Ownership boundary

```text
cloudflare-control-plane-plan owns infra/cloudflare, shared Worker/API module, bindings, auth boundary, route manifest, local dev/test shape, deployment promotion, and payment handoff proof.
payment-subscription-plan owns billing runtime semantics only after the Cloudflare handoff is accepted or exactly blocked.
account-identity-family-plan owns account/session authority.
data-custody-storage-plan owns billing-record privacy/retention/export/delete boundaries.
```

## First-touch surface

- `docs/plans/cloudflare-control-plane-plan/PARENT_CLOUDFLARE_MODULE_SPEC.md`
- `docs/plans/cloudflare-control-plane-plan/GAMES_INFRA_PARITY_MAP.md`

## Read inputs

- [PLAN_STATE.md](../PLAN_STATE.md)
- [PLAN_EXECUTION_BLUEPRINT.md](../PLAN_EXECUTION_BLUEPRINT.md)
- [SOURCE_SURFACE_STATUS_MATRIX.md](../SOURCE_SURFACE_STATUS_MATRIX.md)
- [../../cloudflare-control-plane-plan/PLAN_STATE.md](../../cloudflare-control-plane-plan/PLAN_STATE.md)
- [../../cloudflare-control-plane-plan/PARENT_CLOUDFLARE_MODULE_SPEC.md](../../cloudflare-control-plane-plan/PARENT_CLOUDFLARE_MODULE_SPEC.md)
- [../../cloudflare-control-plane-plan/AUTH_BOUNDARY_MODEL.md](../../cloudflare-control-plane-plan/AUTH_BOUNDARY_MODEL.md)
- [../../cloudflare-control-plane-plan/ROUTE_MANIFEST_MODEL.md](../../cloudflare-control-plane-plan/ROUTE_MANIFEST_MODEL.md)
- [../../cloudflare-control-plane-plan/TESTING_STRATEGY.md](../../cloudflare-control-plane-plan/TESTING_STRATEGY.md)
- [../../cloudflare-control-plane-plan/REQUIRED_TEST_ASSERTION_MATRIX.md](../../cloudflare-control-plane-plan/REQUIRED_TEST_ASSERTION_MATRIX.md)
- `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/payment-handoff-proof.md` (expected-only; absent/not produced in this checkout)
- [../REQUIRED_TEST_ASSERTION_MATRIX.md](../REQUIRED_TEST_ASSERTION_MATRIX.md)

## Output files

- [PLAN_STATE.md](../PLAN_STATE.md)
- [NEXT_ACTIONS.md](../NEXT_ACTIONS.md)
- [PLAN_EXECUTION_BLUEPRINT.md](../PLAN_EXECUTION_BLUEPRINT.md)
- [SOURCE_SURFACE_STATUS_MATRIX.md](../SOURCE_SURFACE_STATUS_MATRIX.md)
- `output/payment-subscription-plan-proof/00-cloudflare-control-plane-handoff/`

## Current execution truth

| Field | Current state | Evidence |
| --- | --- | --- |
| `cloudflare_plan_state` | exists / source-backed / proof-required | `docs/plans/cloudflare-control-plane-plan/PLAN_STATE.md`; `docs/plans/cloudflare-control-plane-plan/NEXT_ACTIONS.md` |
| `module_spec_state` | exists / source-backed / consumed | `docs/plans/cloudflare-control-plane-plan/PARENT_CLOUDFLARE_MODULE_SPEC.md` |
| `auth_boundary_state` | exists / consumed / `account-auth-adapter-manual-required` | `docs/plans/cloudflare-control-plane-plan/AUTH_BOUNDARY_MODEL.md`; `docs/plans/payment-subscription-plan/PLAN_STATE.md` |
| `route_manifest_state` | exists / consumed / payment route groups explicit | `docs/plans/cloudflare-control-plane-plan/ROUTE_MANIFEST_MODEL.md` |
| `local_dev_test_state` | runnable / probe-verified; generated billing-contract sidecar resolution is correct and the proof bundle remains absent/not produced | `docs/plans/cloudflare-control-plane-plan/TESTING_STRATEGY.md`; `node --import tsx infra/cloudflare/scripts/local-dev-workflow.ts`; `node --import tsx --test infra/cloudflare/tests/integration/local-dev-seeding-workflow.test.ts` |
| `portal_worker_smoke_state` | blocked / proof-missing | `docs/plans/cloudflare-control-plane-plan/ROUTE_MANIFEST_MODEL.md`; `docs/plans/payment-subscription-plan/PLAN_STATE.md` |
| `payment_route_group_state` | documented / no-runtime-claim | `docs/plans/cloudflare-control-plane-plan/ROUTE_MANIFEST_MODEL.md` |
| `secret_boundary_state` | server-only-secret contract explicit | `docs/plans/cloudflare-control-plane-plan/GAMES_INFRA_PARITY_MAP.md`; module spec |
| `runtime_block_state` | blocked / payment-runtime-must-not-start | expected upstream handoff artifact path only; no tracked payment proof bundle in this checkout |

## Acceptance

- `cloudflare-control-plane-plan` exists.
- `infra/cloudflare/` module spec exists.
- Auth boundary exists or is marked `account-auth-adapter-manual-required` with exact blocker.
- Route manifest includes the required billing route groups.
- Local dev and test runner exist or are blocked with an exact reason.
- Portal-to-worker smoke boundary exists or is blocked with an exact reason.
- No provider secrets are in repo.
- Payment keeps runtime blocked until the handoff proof is explicit.

## Acceptance status

- [x] `cloudflare-control-plane-plan` exists and remains upstream owner of `infra/cloudflare/`.
- [x] `infra/cloudflare/` module spec exists and is consumed rather than redefined in payment.
- [x] Shared auth boundary exists and unresolved authority remains explicit as `manual-required`.
- [x] Shared route manifest defines the billing, webhook, and admin route groups payment must consume.
- [x] Shared local dev and required test family shapes exist, and the blocked-state proof remains absent/not produced in this checkout.
- [x] Portal-to-worker smoke remains visible as `blocked / proof-missing` rather than being silently ignored.
- [x] Secret ownership remains server-only; payment does not claim provider secret or webhook secret exposure.
- [x] Payment runtime remains explicitly blocked until the upstream Cloudflare handoff proof is generated, validated, and downstream-consumed.
- [ ] Shared handoff proof is generated, validated, and downstream-consumed; the current checkout output tree still has no tracked WP00-WP12 proof roots.

## Required proof fields

The selected proof must name, at minimum:

```text
cloudflare_plan_state
module_spec_state
auth_boundary_state
route_manifest_state
local_dev_test_state
portal_worker_smoke_state
payment_route_group_state
secret_boundary_state
runtime_block_state
accepted_proof_roots
missing_proof_roots
carried_blockers
trusted_device_dependency_state
provider_webhook_dependency_state
storage_queue_dependency_state
deployment_promotion_state
data_custody_state
downstream_payment_ack_state
upstream_handoff_artifact
payment_runtime_no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Proof IDs

- `payment-route.cloudflare-prerequisite`
- `payment-route.cloudflare-handoff`
- `payment-route.cloudflare-plan-exists`
- `payment-route.cloudflare-module-spec-exists`
- `payment-route.cloudflare-auth-boundary-consumed`
- `payment-route.cloudflare-route-manifest-consumed`
- `payment-route.cloudflare-test-shape-consumed`
- `payment-route.cloudflare-portal-smoke-blocker-visible`
- `payment-route.payment-remains-blocked-without-handoff`

## Validation

- Docs validation: `npm run format:check`; `npm run lint:schema-boundaries`
- Upstream handoff artifact (expected-only / absent in this checkout): `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/payment-handoff-proof.md`
- Proof bundle: `output/payment-subscription-plan-proof/00-cloudflare-control-plane-handoff/cloudflare-handoff-proof.md`

Current validation truth:

- `cmd /c npm run format:check` -> `fail`; repo-wide Prettier drift outside WP00.
- `cmd /c npm run lint:schema-boundaries` -> `fail`; weak assertion guard failures in `crates/agent-protocol/tests/contract/*` and `crates/agent-service/tests/unit/lan_pairing.rs`, outside payment WP00 ownership.
- Current Cloudflare lint truth: `npm --prefix infra/cloudflare run lint` passed at head `c121ba5eb`; no current Cloudflare module lint blocker is claimed here.

## Exact upstream blocker set

- Current tracked checkout output inventory:
  - no WP00-WP12 proof roots are present in this checkout
  - WP01-WP11 are historical plan references only
  - WP12's generated proof is absent/not produced in this checkout
- Missing Cloudflare proof roots still carried by the upstream handoff:
  - no tracked WP00-WP12 proof roots are present in this checkout; WP00-WP11 are historical plan references only and WP12's generated proof is absent/not produced
- Current blockers still carried upstream:
  - no tracked Cloudflare handoff proof bundle in this checkout
  - the handoff proof path remains absent/not produced: `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/payment-handoff-proof.md`
  - payment runtime remains blocked until the handoff proof is explicit and downstream-consumed
  - `account-auth-adapter-manual-required`
  - trusted-device authority remains owned outside the Cloudflare plan
  - provider-webhook readiness remains `blocked / proof-required`
  - storage and queue operations readiness remains `blocked / proof-required`
  - portal-to-worker smoke proof missing
  - deployment/promotion proof missing
  - data-custody readiness remains `blocked / proof-required`
  - downstream payment acknowledgment remains `blocked / not-recorded`

## Negative cases

- Reject starting checkout or webhook runtime work before the shared handoff is explicit.
- Reject any attempt to move shared auth, bindings, or test-runner ownership into this plan.
- Reject any provider secret or webhook secret in client or repo-tracked code.

## Failure conditions

- Do not mark any runtime payment slice ready while the Cloudflare handoff proof is missing.
- Do not use Cloudflare route presence as payment runtime proof.
- Do not move Cloudflare scaffold ownership into payment-subscription-plan.
