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

- `blocked / proof-absent`
- Expected proof root: `output/cloudflare-control-plane-plan-proof/07-local-dev-seeding-and-fixtures/`

## Execution truth

- `node --import tsx infra/cloudflare/scripts/local-dev-workflow.ts` exited `0` and emitted an explicit blocked local-start state plus an explicit runnable seed state.
- `node --import tsx --test infra/cloudflare/tests/integration/local-dev-seeding-workflow.test.ts` exited `0` and proved that start, seed, teardown, fixture families, and blocker reporting stay explicit.
- Local start is command-backed and reports the precise module-local Wrangler
  blocker from the current module layout.
- Local seed is command-backed and currently reports populated pricing, parent
  account, support/admin account, and referral fixture families.
- Teardown remains explicit: stop `wrangler dev --local`, remove harness-created `--persist-to` temp state, and remove `infra/cloudflare/.dev.vars` only when the harness created it.

## Fixture families observed

- `pricing-catalog`: explicit, observed `populated` with `3` items; not retained proof
- `parent-test-accounts`: explicit, observed `populated` with `4` items; not retained proof
- `support-admin-test-accounts`: explicit, observed `populated` with `4` items; not retained proof
- `referral-test-graph`: explicit, observed `populated` with `2` items; not retained proof
- `webhook-payload-fixtures`: explicit and `test-fixture-backed`; not retained proof
- `queue-replay-fixtures`: explicit and `test-fixture-backed`; not retained proof

## Exact blockers

- The expected proof root is absent, so the prior workflow stdout/stderr is not
  recoverable from tracked artifacts.
- In the current checkout the generated billing contract exists at
  `infra/cloudflare/src/generated/billing-contracts.ts`, but module-local
  Wrangler is not installed at `infra/cloudflare/node_modules/wrangler`.
- `npm --prefix infra/cloudflare install --ignore-scripts --no-audit
  --no-fund --no-package-lock` cannot resolve `wrangler@4.118.0` because its
  optional peer requires `@cloudflare/workers-types ^5.20260730.1` while this
  module declares `^4.20260601.0`; rerun local-start proof after that dependency
  boundary is reconciled.

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
