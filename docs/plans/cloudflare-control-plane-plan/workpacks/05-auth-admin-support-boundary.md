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
- `docs/proof/cloudflare-control-plane-plan/wp05-auth-admin-support-boundary/`

## Acceptance

- Auth states are explicit.
- Adapter methods are explicit.
- Unsupported auth-provider assumptions are marked manual-required.

## Proof IDs

- `cloudflare-control.auth-boundary`
- `cloudflare-control.admin-support-boundary`
- `cloudflare-control.provider-webhook-signature-boundary`

## Validation

- Docs or scaffold validation: `npm run format:check`

## Negative cases

- Reject naked private routes.
- Reject admin/support routes without audit rules.

## Failure conditions

- Do not hardcode the account provider before the account plan decides it.
