# Workpack 04: Route Manifest And Domain Contracts

> **2026-07-28 correction:** The later missing-private-billing-import blocker text is historical. `infra/cloudflare` now imports module-local generated billing contracts. This workpack remains open because it has no tracked proof bundle; rerun after installing dependencies and record the actual result.

## Status

`source-integrated / tests-stale / dependency-blocked / proof-deferred`

## Current source checkpoint — 2026-08-25

The accepted route-contract source is present in the canonical tree through
`1abe4dfc9`. The owner surface is `src/routes.ts`, `src/auth/model.ts`,
`src/auth/verifier.ts`, and `src/index.ts`, with generated billing contracts
remaining an edge input. One frozen handler-keyed tuple owns method/path,
auth, route class, audit rule/event, and provider; duplicate route and webhook
keys fail before handler readiness. This is source evidence only.

## Goal

Define the shared route groups and the rule that handlers consume manifest-owned paths and contracts without inventing a second route list or over-claiming runtime contract readiness.

## First-touch surface

- `infra/cloudflare/src/routes.ts`
- `infra/cloudflare/src/auth/model.ts`
- `infra/cloudflare/src/auth/verifier.ts`
- `infra/cloudflare/src/index.ts`

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
- The existing unit/property/contract test sources are stale against the current model and must be repaired; they do not currently prove the integrated source.
- `ROUTE_MANIFEST_MODEL.md` and `AUTH_BOUNDARY_MODEL.md` remain the route/auth contract inputs; they do not by themselves prove Worker dispatch.
- Contract and integration proof remain deferred while WP01's module dependency graph is empty, so WP04 cannot claim request/response contract readiness or live worker dispatch readiness.

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

- `npm --prefix infra/cloudflare ls wrangler @cloudflare/workers-types` currently reports an empty module dependency tree.
- `infra/cloudflare/tests/unit/route-manifest.test.ts` still references removed `requestModel`/`responseModel` members and must be reconciled with the frozen tuple model.
- `infra/cloudflare/src/index.ts` consumes `./generated/billing-contracts.js`; after dependencies and tests are repaired, rerun boot-dependent and contract suites and record any then-current route/contract failure.

## Validation

- Scoped validation: `npm --prefix infra/cloudflare run test:unit`
- Scoped validation: `npm --prefix infra/cloudflare run test:property`
- Scoped validation: `npm --prefix infra/cloudflare run test:contract`
- Scoped validation: `npm --prefix infra/cloudflare run test:integration`
- Architecture validation: `npm run lint:architecture -- --files infra/cloudflare/src/routes.ts infra/cloudflare/src/generated/billing-contracts.ts`

## Negative cases

- Reject raw handler-only route strings.
- Reject schema-readiness claims from path ownership alone.
- Reject a second support-only route list for `/auth/billing/manual-invoice`.

## No-claim boundary

- Route manifest proof does not prove billing schema readiness.
- Route manifest proof does not prove worker runtime readiness, auth provider readiness, payment runtime readiness, or consumer handoff readiness.
- This workpack does not take ownership of billing-domain boundary modules or payment semantics.

## Proof root

- `output/cloudflare-control-plane-plan-proof/04-route-manifest-and-domain-contracts/`

## Failure conditions

- Do not imply request/response schema readiness from path lists alone.
