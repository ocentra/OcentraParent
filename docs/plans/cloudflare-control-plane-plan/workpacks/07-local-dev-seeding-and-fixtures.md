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

- `node --import tsx infra/cloudflare/scripts/local-dev-workflow.ts` currently fails in this checkout with `ERR_MODULE_NOT_FOUND: Cannot find package 'tsx' imported from C:\\Users\\sujan\\.codex\\worktrees\\4e8a\\OcentraParent\\`, so the workflow cannot reach the generated billing-contract source surface yet.
- `node --import tsx --test infra/cloudflare/tests/integration/local-dev-seeding-workflow.test.ts` currently fails with the same `ERR_MODULE_NOT_FOUND` bootstrap error before it can assert start, seed, teardown, and fixture-family truth.
- Local start remains command-backed but blocked before runtime boot by workspace bootstrap failure (`tsx` missing in this checkout) and the fact that focused reruns have not been refreshed here.
- Local seed remains command-backed but blocked because the workspace bootstrap fails before the seed commands execute and `infra/cloudflare/src/fixtures.ts` still has TypeScript return-path lint debt.
- Teardown remains explicit: stop `wrangler dev --local`, remove harness-created `--persist-to` temp state, and remove `infra/cloudflare/.dev.vars` only when the harness created it.

## Fixture families proved

- `pricing-catalog`: explicit and currently `blocked`
- `parent-test-accounts`: explicit and currently `blocked`
- `support-admin-test-accounts`: explicit and currently `blocked`
- `referral-test-graph`: explicit and currently `blocked`
- `webhook-payload-fixtures`: explicit and `test-fixture-backed`
- `queue-replay-fixtures`: explicit and `test-fixture-backed`

## Exact blockers

- workspace bootstrap failure: `node --import tsx ...` and `node --import tsx --test ...` fail immediately with `ERR_MODULE_NOT_FOUND: Cannot find package 'tsx' imported from C:\\Users\\sujan\\.codex\\worktrees\\4e8a\\OcentraParent\\`
- `infra/cloudflare/src/fixtures.ts` TypeScript return-path errors
- unrun local-start and seed reruns against the live generated billing-contract source surface

## Validations run

- `node --import tsx infra/cloudflare/scripts/local-dev-workflow.ts` -> failed with `ERR_MODULE_NOT_FOUND: Cannot find package 'tsx' imported from C:\\Users\\sujan\\.codex\\worktrees\\4e8a\\OcentraParent\\`
- `node --import tsx --test infra/cloudflare/tests/integration/local-dev-seeding-workflow.test.ts` -> failed with `ERR_MODULE_NOT_FOUND: Cannot find package 'tsx' imported from C:\\Users\\sujan\\.codex\\worktrees\\4e8a\\OcentraParent\\`
- `npm run lint:architecture -- --files infra/cloudflare/scripts/local-dev-workflow.ts infra/cloudflare/tests/integration/local-dev-seeding-workflow.test.ts`
  Result: passed with an honest zero-match report for the focused re-export gate scope.

## No-claim boundary

- This workpack does not prove a runnable interactive local start.
- This workpack does not prove populated billing seed fixtures.
- This workpack does not prove payment readiness, deploy readiness, or production runtime authority.

## Failure conditions

- Reject fake local-start claims without a runnable path or blocker.
- Do not let seed placeholders masquerade as populated billing fixtures.
