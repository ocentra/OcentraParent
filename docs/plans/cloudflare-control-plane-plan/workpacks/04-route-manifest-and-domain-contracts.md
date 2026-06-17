# Workpack 04: Route Manifest And Domain Contracts

## Goal

Define the shared route groups and the rule that handlers consume manifest-owned paths and contracts.

## First-touch surface

- `infra/cloudflare/src/routes.ts`

## Read inputs

- [ROUTE_MANIFEST_MODEL.md](../ROUTE_MANIFEST_MODEL.md)
- [AUTH_BOUNDARY_MODEL.md](../AUTH_BOUNDARY_MODEL.md)

## Output files

- `infra/cloudflare/src/routes.ts`
- [ROUTE_MANIFEST_MODEL.md](../ROUTE_MANIFEST_MODEL.md)
- `output/cloudflare-control-plane-plan-proof/04-route-manifest-and-domain-contracts/`

## Acceptance

- Required route groups are explicit.
- Auth state and proof families can attach per route group.
- Consumer plans do not re-invent a second route list.

## Proof IDs

- `cloudflare-control.route-manifest`

## Validation

- Scoped validation: `npm --prefix infra/cloudflare run test:unit`
- Scoped validation: `npm --prefix infra/cloudflare run test:contract`
- Scoped validation: `npm --prefix infra/cloudflare run test:integration`
- Architecture validation: `npm run lint:architecture -- --files infra/cloudflare/src/routes.ts packages/billing-domain/src`

## Negative cases

- Reject raw handler-only route strings.

## Failure conditions

- Do not imply request/response schema readiness from path lists alone.
