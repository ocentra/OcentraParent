# Workpack 07: Local Dev Seeding And Fixtures

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

- `blocked / proof-required`
- Proof root: `output/cloudflare-control-plane-plan-proof/07-local-dev-seeding-and-fixtures/`

## Execution truth

- `node --import tsx infra/cloudflare/scripts/local-dev-workflow.ts` now succeeds in this checkout and reaches the generated billing-contract source surface through the local probe.
- `node --import tsx --test infra/cloudflare/tests/integration/local-dev-seeding-workflow.test.ts` now succeeds and asserts start, seed, teardown, and fixture-family truth.
- Local start is import-check passed; runtime boot remains unproven.
- Local seed is command-backed and runnable.
- Teardown remains explicit: stop `wrangler dev --local`, remove harness-created `--persist-to` temp state, and remove `infra/cloudflare/.dev.vars` only when the harness created it.

## Fixture families proved

- `pricing-catalog`: explicit and currently `populated`
- `parent-test-accounts`: explicit and currently `populated`
- `support-admin-test-accounts`: explicit and currently `populated`
- `referral-test-graph`: explicit and currently `populated`
- `webhook-payload-fixtures`: explicit and `test-fixture-backed`
- `queue-replay-fixtures`: explicit and `test-fixture-backed`

## Exact blockers

- Proof artifact absent/not produced in this checkout: `output/cloudflare-control-plane-plan-proof/07-local-dev-seeding-and-fixtures/`
- Current module lint truth: `npm --prefix infra/cloudflare run lint` passed at head `c121ba5eb`; no current Cloudflare module lint blocker is claimed here.

## Validations run

- `node --import tsx infra/cloudflare/scripts/local-dev-workflow.ts` -> passed; `start.status = runnable`, `seed.status = runnable`
- `node --import tsx --test infra/cloudflare/tests/integration/local-dev-seeding-workflow.test.ts` -> passed
- `npm --prefix infra/cloudflare run lint` -> passed at head `c121ba5eb`
- `npm run lint:architecture -- --files infra/cloudflare/scripts/local-dev-workflow.ts infra/cloudflare/tests/integration/local-dev-seeding-workflow.test.ts`
  Result: passed with an honest zero-match report for the focused re-export gate scope.

## No-claim boundary

- This workpack does not prove a published proof bundle.
- This workpack does not prove payment readiness, deploy readiness, or production runtime authority.
- This workpack does not prove the local seed data is the final production dataset.

## Failure conditions

- Reject fake proof-bundle claims without the output artifact.
- Do not let fixture population claims imply payment readiness.
