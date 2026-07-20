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
- [Retained WP07 proof](../../../proof/cloudflare-control-plane-plan/07-local-dev-seeding-and-fixtures.md)

## Status

- `blocked / proof-required`
- Proof root: `output/cloudflare-control-plane-plan-proof/07-local-dev-seeding-and-fixtures/`
- Retained compact receipt: [07-local-dev-seeding-and-fixtures.md](../../../proof/cloudflare-control-plane-plan/07-local-dev-seeding-and-fixtures.md)
- The workpack remains open until the branch is accepted and the owning checklist/index consumes the current source and retained proof.

## Execution truth

- `node --import tsx infra/cloudflare/scripts/local-dev-workflow.ts` succeeds and reaches the checked-in billing-contract sidecar through the local preflight probe.
- The standalone report separates `preflightStatus = ready`, `importCheckStatus = passed`, and `runtimeBootStatus = unproven`; it does not label an import-only check as a runnable Worker.
- `node --import tsx --test infra/cloudflare/tests/integration/local-dev-seeding-workflow.test.ts` succeeds at 4/4, including real binding persistence, idempotent restart, isolated stores, and five correlated redacted milestones in a test-owned NDJSON store.
- Local seed is runnable only after each command boots bounded Wrangler state and directly reads back positive D1/KV/R2 persistence evidence. The current counts are `3`, `4`, `4`, `2`, `5`, and `2` in the order below.
- `npm --prefix infra/cloudflare run test:integration` succeeds at 62/62. Its separate real Wrangler suite succeeds at 10/10 and proves a bounded local Worker boot/health path independently of the standalone preflight report.
- Teardown remains explicit: stop `wrangler dev --local`, remove harness-created `--persist-to` temp state, and remove `infra/cloudflare/.dev.vars` only when the harness created it.

## Fixture families proved

- `pricing-catalog`: explicit and currently `populated`
- `parent-test-accounts`: explicit and currently `populated`
- `support-admin-test-accounts`: explicit and currently `populated`
- `referral-test-graph`: explicit and currently `populated`
- `webhook-payload-fixtures`: explicit and `test-fixture-backed`
- `queue-replay-fixtures`: explicit and `test-fixture-backed`

## Exact blockers

- The standalone workflow probe does not start Wrangler or issue a health request, so its own `runtimeBootStatus` remains `unproven` even though the separate real-runtime integration suite is green.
- The account auth adapter remains `account-auth-adapter-manual-required`.
- Production deployment proof, the WP12 payment handoff artifact, and downstream payment-plan acceptance remain absent/outside WP07.
- Current module lint truth: `npm --prefix infra/cloudflare run lint` passed on the WP07 validation candidate based at `4573ab436`; no current Cloudflare module lint blocker is claimed here.

## Validations run

- `node --import tsx infra/cloudflare/scripts/local-dev-workflow.ts` -> passed; `preflightStatus = ready`, `importCheckStatus = passed`, `runtimeBootStatus = unproven`, `seed.status = runnable`
- `node --import tsx --test infra/cloudflare/tests/integration/local-dev-seeding-workflow.test.ts` -> passed, 4/4
- `npm --prefix infra/cloudflare run test:integration` -> passed, 62/62; real Wrangler local runtime 10/10
- `npm --prefix infra/cloudflare run test:unit` -> passed, 49/49
- `npm --prefix infra/cloudflare run test:property` -> passed, 9/9
- `npm --prefix infra/cloudflare run lint` -> passed
- Focused `npm run lint:architecture -- --files ...` over all ten touched TypeScript files -> passed with non-empty scope.
- Focused Enforcer `source-shape`, `required-tests`, `no-test-doubles`, and `validation-bypass` checks -> passed.
- Historical raw proof route: `output/cloudflare-control-plane-plan-proof/07-local-dev-seeding-and-fixtures/`; its former tracked files were removed at `cbb8421875492176bd2a3d5b95eaa7fa0dd8210e` and do not count as current retained proof.
- Current retained proof: [07-local-dev-seeding-and-fixtures.md](../../../proof/cloudflare-control-plane-plan/07-local-dev-seeding-and-fixtures.md).

## No-claim boundary

- This workpack does not prove a WP07 packet is accepted downstream from source presence or historical command records.
- This workpack does not prove payment readiness, deploy readiness, or production runtime authority.
- This workpack does not prove the local seed data is the final production dataset.
- The standalone preflight report does not inherit the separate integration suite's Wrangler boot result.

## Failure conditions

- Reject import-only readiness claims and proof-bundle claims without current-head validation plus a retained receipt.
- Do not let fixture population claims imply payment readiness.
