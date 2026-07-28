# Workpack 05: Auth Admin Support Boundary

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

Status: `blocked / proof-required`.

Implemented packet:

- `infra/cloudflare/src/auth/verifier.ts`
- `infra/cloudflare/tests/unit/auth-boundary.test.ts`
- `infra/cloudflare/tests/integration/webhook-signature-rejection.test.ts`

Focused outcome:

- Webhook auth now treats unresolved and unknown auth-adapter modes as `manual-required` instead of falling through to provider-signature evaluation.
- Unit auth coverage now proves the same `manual-required` boundary for provider webhooks.
- Integration webhook rejection coverage now proves the same `manual-required` boundary at the Worker request surface.

External blocker:

- `npm --prefix infra/cloudflare run test:unit`, `test:security`, and `test:integration` are green in the execution record; the remaining gap is the absent proof artifact.
- The account-provider decision remains manual-required, so WP05 is not green; it is blocked / proof-required with an exact carried dependency gap.

Proof root:

- `output/cloudflare-control-plane-plan-proof/05-auth-admin-support-boundary/`
- validation log: `output/cloudflare-control-plane-plan-proof/05-auth-admin-support-boundary/16-validation-commands.log`
