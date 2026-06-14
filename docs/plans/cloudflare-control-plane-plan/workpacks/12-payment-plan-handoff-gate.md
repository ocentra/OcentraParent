# Workpack 12: Payment Plan Handoff Gate

## Goal

Define exactly what `payment-subscription-plan` may assume from the shared Cloudflare module and what remains blocked.

## First-touch surface

- `docs/proof/cloudflare-control-plane-plan/wp12-payment-plan-handoff-gate/payment-handoff-proof.md`

## Read inputs

- [PLAN_STATE.md](../PLAN_STATE.md)
- [PARENT_CLOUDFLARE_MODULE_SPEC.md](../PARENT_CLOUDFLARE_MODULE_SPEC.md)
- [SOURCE_SURFACE_STATUS_MATRIX.md](../SOURCE_SURFACE_STATUS_MATRIX.md)

## Output files

- `docs/proof/cloudflare-control-plane-plan/wp12-payment-plan-handoff-gate/`
- [SOURCE_SURFACE_STATUS_MATRIX.md](../SOURCE_SURFACE_STATUS_MATRIX.md)

## Acceptance

- Payment knows whether the module exists.
- Payment knows whether auth boundary exists or is blocked.
- Payment knows whether local dev/test runner exists or is blocked.
- Payment knows whether no-provider-secrets and portal-to-worker boundaries are explicit.

## Proof IDs

- `cloudflare-control.payment-plan-handoff`

## Validation

- Docs or scaffold validation: `npm run format:check`

## Negative cases

- Reject unblocking payment from docs alone when core Cloudflare blockers remain.

## Failure conditions

- Do not mark payment unblocked without explicit handoff proof.
