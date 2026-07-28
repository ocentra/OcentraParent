# Workpack 07: Local Dev Seeding And Fixtures

> **2026-07-28 correction:** The later missing-private-billing-import blocker text is historical. `infra/cloudflare` now imports module-local generated billing contracts. This workpack remains open because it has no tracked proof bundle; rerun after installing dependencies and record the actual result.

## Goal

Define the local Wrangler workflow, seed scripts, and required fixture families.

## First-touch surface

- `LOCAL_DEV_AND_SEEDING_MODEL.md`

## Read inputs

- [LOCAL_DEV_AND_SEEDING_MODEL.md](../LOCAL_DEV_AND_SEEDING_MODEL.md)
- [TESTING_STRATEGY.md](../TESTING_STRATEGY.md)

## Output files

- `infra/cloudflare/scripts/`
- [LOCAL_DEV_AND_SEEDING_MODEL.md](../LOCAL_DEV_AND_SEEDING_MODEL.md)
- `output/cloudflare-control-plane-plan-proof/07-local-dev-seeding-and-fixtures/`

## Status

- `blocked / proof-present`
- Proof root: `output/cloudflare-control-plane-plan-proof/07-local-dev-seeding-and-fixtures/`

## Execution truth

- `node --import tsx infra/cloudflare/scripts/local-dev-workflow.ts` exited `0` and emitted an explicit blocked local-start state plus an explicit blocked seed state.
- `node --import tsx --test infra/cloudflare/tests/integration/local-dev-seeding-workflow.test.ts` exited `0` and proved that start, seed, teardown, fixture families, and blocker reporting stay explicit.
- Local start remains command-backed but blocked before runtime boot by missing billing-domain boundary imports.
- Local seed remains command-backed but blocked because `infra/cloudflare/src/fixtures.ts` cannot import `packages/billing-domain/src/billing-account-runtime-boundary.js`.
- Teardown remains explicit: stop `wrangler dev --local`, remove harness-created `--persist-to` temp state, and remove `infra/cloudflare/.dev.vars` only when the harness created it.

## Fixture families proved

- `pricing-catalog`: explicit and currently `blocked`
- `parent-test-accounts`: explicit and currently `blocked`
- `support-admin-test-accounts`: explicit and currently `blocked`
- `referral-test-graph`: explicit and currently `blocked`
- `webhook-payload-fixtures`: explicit and `test-fixture-backed`
- `queue-replay-fixtures`: explicit and `test-fixture-backed`

## Exact blockers

- `packages/billing-domain/src/billing-checkout-portal-boundary.js`
- `packages/billing-domain/src/billing-referral-boundary.js`
- `packages/billing-domain/src/billing-support-admin-api-boundary.js`
- `packages/billing-domain/src/billing-account-runtime-boundary.js`
- `packages/billing-domain/src/billing-support-admin-runtime-boundary.js`

## Validations run

- `node --import tsx infra/cloudflare/scripts/local-dev-workflow.ts`
- `node --import tsx --test infra/cloudflare/tests/integration/local-dev-seeding-workflow.test.ts`
- `npm run lint:architecture -- --files infra/cloudflare/scripts/local-dev-workflow.ts infra/cloudflare/tests/integration/local-dev-seeding-workflow.test.ts`
  Result: passed with an honest zero-match report for the focused re-export gate scope.

## No-claim boundary

- This workpack does not prove a runnable interactive local start.
- This workpack does not prove populated billing seed fixtures.
- This workpack does not prove payment readiness, deploy readiness, or production runtime authority.

## Failure conditions

- Reject fake local-start claims without a runnable path or blocker.
- Do not let seed placeholders masquerade as populated billing fixtures.
