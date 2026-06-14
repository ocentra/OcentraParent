# Workpack 00: Cloudflare Control Plane Handoff

## Goal

Consume the shared Cloudflare plan handoff before any payment runtime slice starts.

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

## Output files

- [PLAN_STATE.md](../PLAN_STATE.md)
- [NEXT_ACTIONS.md](../NEXT_ACTIONS.md)
- [PLAN_EXECUTION_BLUEPRINT.md](../PLAN_EXECUTION_BLUEPRINT.md)
- [SOURCE_SURFACE_STATUS_MATRIX.md](../SOURCE_SURFACE_STATUS_MATRIX.md)
- `docs/proof/payment-subscription-plan/wp00-cloudflare-control-plane-handoff/`

## Acceptance

- `cloudflare-control-plane-plan` exists.
- `infra/cloudflare/` module spec exists.
- Auth boundary exists or is marked `account-auth-adapter-manual-required` with exact blocker.
- Route manifest includes the required billing route groups.
- Local dev and test runner exist or are blocked with an exact reason.
- Portal-to-worker smoke boundary exists or is blocked with an exact reason.
- No provider secrets are in repo.

## Proof IDs

- `payment-route.cloudflare-prerequisite`
- `payment-route.cloudflare-handoff`

## Validation

- Docs validation: `npm run format:check`; `npm run lint:schema-boundaries`
- Proof bundle: `docs/proof/payment-subscription-plan/wp00-cloudflare-control-plane-handoff/cloudflare-handoff-proof.md`

## Negative cases

- Reject starting checkout or webhook runtime work before the shared handoff is explicit.
- Reject any attempt to move shared auth, bindings, or test-runner ownership into this plan.

## Failure conditions

- Do not mark any runtime payment slice ready while the Cloudflare handoff proof is missing.
