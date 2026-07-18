# Workpack 04: Route Manifest And Domain Contracts

## Status

`blocked / proof-required`

## Goal

Define the shared route groups and the rule that handlers consume manifest-owned paths and contracts without inventing a second route list or over-claiming runtime contract readiness.

## First-touch surface

- `infra/cloudflare/src/routes.ts`

## Read inputs

- [ROUTE_MANIFEST_MODEL.md](../ROUTE_MANIFEST_MODEL.md)
- [AUTH_BOUNDARY_MODEL.md](../AUTH_BOUNDARY_MODEL.md)

## Output files

- `infra/cloudflare/src/routes.ts`
- [ROUTE_MANIFEST_MODEL.md](../ROUTE_MANIFEST_MODEL.md)
- [AUTH_BOUNDARY_MODEL.md](../AUTH_BOUNDARY_MODEL.md)
- `output/cloudflare-control-plane-plan-proof/04-route-manifest-and-domain-contracts/`

## Acceptance

- Required route groups are explicit.
- Auth state and proof families can attach per route group.
- Consumer plans do not re-invent a second route list.

## Execution truth

- `infra/cloudflare/src/routes.ts` remains the single shared route manifest for the Cloudflare worker surface.
- `infra/cloudflare/tests/unit/route-manifest.test.ts` now proves the manifest route-key list is exact, not merely pattern-valid.
- `infra/cloudflare/tests/property/route-auth-state.property.test.ts` now proves `/auth/billing/manual-invoice` is the only elevated support-owned exception inside `/auth/billing/*`.
- `ROUTE_MANIFEST_MODEL.md` and `AUTH_BOUNDARY_MODEL.md` now make that support exception explicit so consumer plans do not infer a second support-only API surface.
- Contract and integration proof remain proof-required because the current checkout still lacks the physical proof bundle, so WP04 cannot claim request/response contract readiness or live worker dispatch readiness.

## Proof IDs

- `cloudflare-control.route-manifest`

## Families proved vs blocked

- Proved:
  - manifest key ownership and exact route list
  - auth-state ownership for public, parent-session, trusted-device, support, admin, webhook, and internal-queue rows
  - explicit support-only exception for `/auth/billing/manual-invoice`
- Blocked:
  - contract-family execution through `tests/contract/billing-api-contract.test.ts`
  - integration-family execution through booted worker routes
  - boot-dependent unit/property siblings that import `infra/cloudflare/src/index.ts`

## Exact blocker set

- `infra/cloudflare/tests/contract/billing-api-contract.test.ts` and the boot-dependent integration suites remain proof-required until the current execution record is captured in the proof bundle.
- `infra/cloudflare/src/index.ts` and `infra/cloudflare/tests/contract/billing-api-contract.test.ts` are the route surfaces that need the refreshed rerun before dispatch-proof can be claimed.

## Validation

- Scoped validation: `npm --prefix infra/cloudflare run test:unit`
- Scoped validation: `npm --prefix infra/cloudflare run test:property`
- Scoped validation: `npm --prefix infra/cloudflare run test:contract`
- Scoped validation: `npm --prefix infra/cloudflare run test:integration`
- Architecture validation: `npm run lint:architecture -- --files infra/cloudflare/src/routes.ts infra/cloudflare/tests/contract/billing-api-contract.test.ts`

## Negative cases

- Reject raw handler-only route strings.
- Reject schema-readiness claims from path ownership alone.
- Reject a second support-only route list for `/auth/billing/manual-invoice`.

## No-claim boundary

- Route manifest proof does not prove billing schema readiness.
- Route manifest proof does not prove worker runtime readiness, auth provider readiness, payment runtime readiness, or consumer handoff readiness.
- This workpack does not take ownership of the shared billing contract boundary surface or payment semantics.

## Proof root

- `output/cloudflare-control-plane-plan-proof/04-route-manifest-and-domain-contracts/`

## Failure conditions

- Do not imply request/response schema readiness from path lists alone.
