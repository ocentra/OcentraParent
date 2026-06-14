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
- `docs/proof/cloudflare-control-plane-plan/wp07-local-dev-seeding-and-fixtures/`

## Acceptance

- Local start/seed/teardown path is explicit.
- Fixture families are explicit.
- Missing runtime dependencies are recorded as blockers.

## Proof IDs

- `cloudflare-control.local-dev-start`
- `cloudflare-control.seed-local`

## Validation

- Docs or scaffold validation: `npm run format:check`

## Negative cases

- Reject fake local-start claims without a runnable path or blocker.

## Failure conditions

- Do not let seed placeholders masquerade as populated billing fixtures.
