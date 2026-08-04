# Workpack 05: Auth Admin Support Boundary

> **2026-07-28 correction:** The later missing-private-billing-import blocker text is historical. `infra/cloudflare` now imports module-local generated billing contracts. This workpack remains open because it has no tracked proof bundle; rerun after installing dependencies and record the actual result.

## Goal

Define the auth-state model and adapter interface for parent, admin, support, webhook, and queue-only routes.

## First-touch surface

- `infra/cloudflare/src/auth/verifier.ts`

## Read inputs

- [AUTH_BOUNDARY_MODEL.md](../AUTH_BOUNDARY_MODEL.md)
- [ROUTE_MANIFEST_MODEL.md](../ROUTE_MANIFEST_MODEL.md)

## Output files

- `infra/cloudflare/src/auth/`
- [AUTH_BOUNDARY_MODEL.md](../AUTH_BOUNDARY_MODEL.md)
- `output/cloudflare-control-plane-plan-proof/05-auth-admin-support-boundary/`

## Acceptance

- Auth states are explicit.
- Adapter methods are explicit.
- Unsupported auth-provider assumptions are marked manual-required.

## Proof IDs

- `cloudflare-control.auth-boundary`
- `cloudflare-control.admin-support-boundary`
- `cloudflare-control.provider-webhook-signature-boundary`

## Validation

- Scoped validation: `npm --prefix infra/cloudflare run test:unit`
- Scoped validation: `npm --prefix infra/cloudflare run test:security`
- Scoped validation: `npm --prefix infra/cloudflare run test:integration`
- Architecture validation: `npm run lint:architecture -- --files infra/cloudflare/src/auth/verifier.ts infra/cloudflare/tests/unit/auth-boundary.test.ts infra/cloudflare/tests/integration/webhook-signature-rejection.test.ts`

## Negative cases

- Reject naked private routes.
- Reject admin/support routes without audit rules.

## Failure conditions

- Do not hardcode the account provider before the account plan decides it.

## Execution truth

Status: `blocked / proof-present`.

Implemented packet:

- `infra/cloudflare/src/auth/verifier.ts`
- `infra/cloudflare/tests/unit/auth-boundary.test.ts`
- `infra/cloudflare/tests/integration/webhook-signature-rejection.test.ts`

Focused outcome:

- Webhook auth now treats unresolved and unknown auth-adapter modes as `manual-required` instead of falling through to provider-signature evaluation.
- Unit auth coverage now proves the same `manual-required` boundary for provider webhooks.
- Integration webhook rejection coverage now proves the same `manual-required` boundary at the Worker request surface.

External blocker:

- `npm --prefix infra/cloudflare run test:unit`, `test:security`, and `test:integration` need WP05-owned focused results; WP01's pinned module dependency tree is now resolved.
- `infra/cloudflare/src/index.ts` uses the module-local generated billing-contract route; rerun the selected families and retain only their then-current exact blockers.
- This blocker is outside the WP05 owner surface, so WP05 is not green; it is proof-present with an exact carried dependency blocker.

Proof root:

- `output/cloudflare-control-plane-plan-proof/05-auth-admin-support-boundary/`
- validation log: `output/cloudflare-control-plane-plan-proof/05-auth-admin-support-boundary/16-validation-commands.log`
