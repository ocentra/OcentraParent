# Workpack 08: Testing Runner And Test Pyramid

> **2026-07-28 correction:** The later missing-private-billing-import blocker text is historical. `infra/cloudflare` now imports module-local generated billing contracts. This workpack remains open because it has no tracked proof bundle; rerun after installing dependencies and record the actual result.

## Goal

Define the Cloudflare-specific test command family and the reduced Parent test pyramid.

## First-touch surface

- `infra/cloudflare/scripts/test-runner.ts`

## Read inputs

- [TESTING_STRATEGY.md](../TESTING_STRATEGY.md)
- [REQUIRED_TEST_ASSERTION_MATRIX.md](../REQUIRED_TEST_ASSERTION_MATRIX.md)
- [PROOF_AND_TEST_INVENTORY.md](../PROOF_AND_TEST_INVENTORY.md)
- `workpacks/06-storage-do-d1-kv-r2-queue-bindings.md` for the selected storage binding/migration proof route
- `docs/plans/account-identity-family-plan/workpacks/08-rust-schema-workers-d1-runtime-migration.md` for the consumed Rust account/family contract handoff only

## Output files

- `infra/cloudflare/scripts/test-runner.ts`
- `infra/cloudflare/tests/`
- [TESTING_STRATEGY.md](../TESTING_STRATEGY.md)
- [REQUIRED_TEST_ASSERTION_MATRIX.md](../REQUIRED_TEST_ASSERTION_MATRIX.md)
- `output/cloudflare-control-plane-plan-proof/08-testing-runner-and-test-pyramid/`

## Acceptance

- Unit, integration, e2e, contract, security, property, and fuzz families are explicit.
- Every required test file and exact assertion ID is explicit.
- Real test families, any remaining blockers, and no-claim boundaries are explicit.
- The runner contract is explicit.
- Proof output can map executed or blocked assertion IDs back to the matrix.
- Account-identity storage-facing runner proof is sequenced after Cloudflare WP06 and is handed to Account WP06 without taking ownership of account/family authority.

## Proof IDs

- `cloudflare-control.test-runner-unit`
- `cloudflare-control.test-runner-integration`
- `cloudflare-control.test-runner-e2e`
- `cloudflare-control.test-runner-contract`
- `cloudflare-control.test-runner-security`
- `cloudflare-control.property-fuzz-boundary`

## Current execution truth

- Status: `blocked / proof-present`
- Proof root: `output/cloudflare-control-plane-plan-proof/08-testing-runner-and-test-pyramid/`
- The runner in `infra/cloudflare/scripts/test-runner.ts` now owns the WP08 family-to-file contract and emits the selected proof IDs, assertion IDs, and any same-directory exclusions with `--list`.
- Same-directory extras are explicit and excluded from the WP08 runner contract until the strategy and matrix adopt them first:
  - unit: `tests/unit/billing-binding-read-model.test.ts`
  - integration: `tests/integration/checkout-portal-hosted.test.ts`, `local-dev-seeding-workflow.test.ts`, `payment-routes-real.test.ts`, `provider-webhooks.test.ts`, `reconciliation-auth-boundary.test.ts`, `worker-runtime-real.test.ts`
  - security: `tests/security/interactive-billing-boundary.test.ts`

## Families proved vs blocked

- Proved at runner-contract surface: `unit`, `integration`, `e2e`, `contract`, `security`, `property`, and `fuzz` are explicit, file-scoped, and mapped to exact assertion IDs.
- Runtime-blocked today: `unit`, `integration`, `e2e`, `contract`, `security`, `property`, and `fuzz`.
- Partial file-level passes inside blocked family runs:
  - unit: `auth-boundary`, `route-manifest`, `env-bindings`, `redaction`
  - security: `redaction`
  - property: `route-auth-state`
- Blocked file-level execution remains explicit:
  - unit: `request-limits`, `kill-switch`
  - integration: `worker-health`, `pricing-public`, `billing-status-auth`, `webhook-signature-rejection`, `admin-auth-rejection`
  - e2e: `portal-to-worker-billing-status`
  - contract: `billing-api-contract`
  - security: `no-provider-secrets-in-client`, `cors-origin-rejection`, `request-smuggling`
  - property: `billing-idempotency`
  - fuzz: `provider-webhook-payload`

## Exact blockers

- WP01's module dependency preflight is retained and pinned: `npm --prefix infra/cloudflare ls wrangler @cloudflare/workers-types` resolves `wrangler@4.118.0` with deduped `@cloudflare/workers-types@5.20260804.1`. It does not prove the WP08 runner family or replace WP06's storage proof.
- Account WP08's Rust contract and Cloudflare WP06's account-D1 binding, adapter, migration, and integration proof are not yet retained. WP08 therefore cannot produce its account-storage runner handoff to Account WP06.
- `infra/cloudflare/src/index.ts` imports the current module-local generated route `./generated/billing-contracts.js`; the old `packages/billing-domain/src/*` import paths are not WP08 blockers and must not be revived.

## Validation

- `node --import tsx infra/cloudflare/scripts/test-runner.ts --list`
- `npm --prefix infra/cloudflare run test:unit`
- `npm --prefix infra/cloudflare run test:integration`
- `npm --prefix infra/cloudflare run test:e2e`
- `npm --prefix infra/cloudflare run test:contract`
- `npm --prefix infra/cloudflare run test:security`
- `npm --prefix infra/cloudflare run test:property`
- `npm --prefix infra/cloudflare run test:fuzz`
- `npm --prefix infra/cloudflare run lint`
- `npm run lint:architecture -- --files infra/cloudflare/scripts/test-runner.ts`

For the account-identity storage handoff, WP06 records the migration command
`cd infra/cloudflare && npm exec -c "wrangler d1 migrations apply <account-identity-d1-database> --local"`; WP08 runs the integration family through the module script `npm --prefix infra/cloudflare run test:integration`, not a raw unprefixed Node invocation. Retain either the mapped result or its exact blocker for Account WP06.

Validation truth from the current checkout:

- Historical `--list` and family results are runner-inventory evidence only, not a current Account-storage handoff.
- WP01's resolver graph is available; do not report the graph as a WP08 family result. WP06 still must supply its required storage proof.
- After the remaining WP06 prerequisite exists, rerun the selected module script and retain its mapped result or a new exact blocker for Account WP06.

## Negative cases

- Reject pretending coverage or mutation exists from docs alone.
- Reject collapsing several required files into one broad umbrella suite without
  updating the matrix first.
- Reject treating the narrowed runner inventory as runtime-green while the current module dependency environment or the required WP06 storage handoff is absent.

## Failure conditions

- Do not treat placeholder tests as passing tests.
- Do not mark WP08 runtime-complete from assertion inventory alone.

## No-claim boundary

- This workpack does not prove Cloudflare runtime readiness.
- This workpack does not prove billing readiness, payment handoff readiness, or portal completion.
- This workpack does not prove the excluded same-directory tests belong to the WP08 contract.
- This workpack consumes but does not redefine the Account WP08 Rust authority contract; its account-identity result is not a whole-account or runtime-ready claim.
- WP08 stays open/blocked until WP06 retains the account-storage proof required for the WP08 runner handoff; WP01's resolver graph is already restored and is not a substitute.
