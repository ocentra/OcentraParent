# Workpack 08: Testing Runner And Test Pyramid

## Goal

Define the Cloudflare-specific test command family and the reduced Parent test pyramid.

## First-touch surface

- `infra/cloudflare/scripts/test-runner.ts`

## Read inputs

- [TESTING_STRATEGY.md](../TESTING_STRATEGY.md)
- [PROOF_AND_TEST_INVENTORY.md](../PROOF_AND_TEST_INVENTORY.md)

## Output files

- `infra/cloudflare/scripts/test-runner.ts`
- `infra/cloudflare/tests/`
- [TESTING_STRATEGY.md](../TESTING_STRATEGY.md)
- `docs/proof/cloudflare-control-plane-plan/wp08-testing-runner-and-test-pyramid/`

## Acceptance

- Unit, integration, e2e, contract, security, property, and fuzz families are explicit.
- Placeholder files or blockers are explicit.
- The runner contract is explicit.

## Proof IDs

- `cloudflare-control.test-runner-unit`
- `cloudflare-control.test-runner-integration`
- `cloudflare-control.test-runner-e2e`
- `cloudflare-control.test-runner-contract`
- `cloudflare-control.test-runner-security`
- `cloudflare-control.property-fuzz-boundary`

## Validation

- Docs or scaffold validation: `npm run format:check`

## Negative cases

- Reject pretending coverage or mutation exists from docs alone.

## Failure conditions

- Do not treat placeholder tests as passing tests.
