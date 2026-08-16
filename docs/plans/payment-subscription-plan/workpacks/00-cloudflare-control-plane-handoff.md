# Workpack 00: Cloudflare Control Plane Handoff

## Current status

- Verdict: `blocked / proof-present`
- Payment proof root: `output/payment-subscription-plan-proof/00-cloudflare-control-plane-handoff/`
- Upstream handoff artifact: `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/payment-handoff-proof.md`
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
- `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/payment-handoff-proof.md`
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
| `cloudflare_plan_state` | exists / consumed / blocked-upstream | `docs/plans/cloudflare-control-plane-plan/PLAN_STATE.md`; `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/payment-handoff-proof.md` |
| `module_spec_state` | exists / source-backed / consumed | `docs/plans/cloudflare-control-plane-plan/PARENT_CLOUDFLARE_MODULE_SPEC.md` |
| `auth_boundary_state` | exists / consumed / `account-auth-adapter-manual-required` | `docs/plans/cloudflare-control-plane-plan/AUTH_BOUNDARY_MODEL.md`; upstream handoff artifact |
| `route_manifest_state` | exists / consumed / payment route groups explicit | `docs/plans/cloudflare-control-plane-plan/ROUTE_MANIFEST_MODEL.md` |
| `local_dev_test_state` | blocked / proof-present | `docs/plans/cloudflare-control-plane-plan/TESTING_STRATEGY.md`; upstream handoff artifact |
| `portal_worker_smoke_state` | blocked / proof-missing | upstream handoff artifact |
| `payment_route_group_state` | documented / no-runtime-claim | `docs/plans/cloudflare-control-plane-plan/ROUTE_MANIFEST_MODEL.md` |
| `secret_boundary_state` | server-only-secret contract explicit | `docs/plans/cloudflare-control-plane-plan/GAMES_INFRA_PARITY_MAP.md`; module spec |
| `runtime_block_state` | blocked / payment-runtime-must-not-start | upstream handoff artifact and payment proof root |

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
- [x] Shared local dev and required test family shapes exist, with blocked-state proof carried forward instead of fake green.
- [x] Portal-to-worker smoke remains visible as `blocked / proof-missing` rather than being silently ignored.
- [x] Secret ownership remains server-only; payment does not claim provider secret or webhook secret exposure.
- [x] Payment runtime remains explicitly blocked until the upstream Cloudflare blocker set is cleared.

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
- Upstream handoff artifact: `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/payment-handoff-proof.md`
- Proof bundle: `output/payment-subscription-plan-proof/00-cloudflare-control-plane-handoff/cloudflare-handoff-proof.md`

Current validation truth:

- `cmd /c npm run format:check` -> `fail`; repo-wide Prettier drift outside WP00.
- `cmd /c npm run lint:schema-boundaries` -> `fail`; weak assertion guard failures in `crates/agent-protocol/tests/contract/*` and `crates/agent-service/tests/unit/lan_pairing.rs`, outside payment WP00 ownership.

## Exact upstream blocker set

- Missing Cloudflare proof roots still carried by the upstream handoff:
  - `output/cloudflare-control-plane-plan-proof/03-worker-entrypoint-runtime-guards/`
  - `output/cloudflare-control-plane-plan-proof/05-auth-admin-support-boundary/`
  - `output/cloudflare-control-plane-plan-proof/09-portal-to-worker-e2e-smoke/`
  - `output/cloudflare-control-plane-plan-proof/11-deployment-and-environment-promotion/`
- Missing billing-domain runtime boundary modules still blocking the upstream Cloudflare packet:
  - `packages/billing-domain/src/billing-checkout-portal-boundary.js`
  - `packages/billing-domain/src/billing-referral-boundary.js`
  - `packages/billing-domain/src/billing-support-admin-api-boundary.js`
  - `packages/billing-domain/src/billing-account-runtime-boundary.js`
  - `packages/billing-domain/src/billing-support-admin-runtime-boundary.js`
- Dependency-gated authority and runtime blockers still carried upstream:
  - `account-auth-adapter-manual-required`
  - trusted-device authority remains owned outside the Cloudflare plan
  - portal-to-worker smoke proof missing
  - deployment/promotion proof missing

## Negative cases

- Reject starting checkout or webhook runtime work before the shared handoff is explicit.
- Reject any attempt to move shared auth, bindings, or test-runner ownership into this plan.
- Reject any provider secret or webhook secret in client or repo-tracked code.

## Failure conditions

- Do not mark any runtime payment slice ready while the Cloudflare handoff proof is missing.
- Do not use Cloudflare route presence as payment runtime proof.
- Do not move Cloudflare scaffold ownership into payment-subscription-plan.
