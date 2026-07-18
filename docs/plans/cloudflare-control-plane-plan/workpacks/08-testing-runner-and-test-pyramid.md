# Workpack 08: Testing Runner And Test Pyramid

## Goal

Define the Cloudflare-specific test command family and the reduced Parent test pyramid.

## First-touch surface

- `infra/cloudflare/scripts/test-runner.ts`

## Read inputs

- [TESTING_STRATEGY.md](../TESTING_STRATEGY.md)
- [REQUIRED_TEST_ASSERTION_MATRIX.md](../REQUIRED_TEST_ASSERTION_MATRIX.md)
- [PROOF_AND_TEST_INVENTORY.md](../PROOF_AND_TEST_INVENTORY.md)

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

## Proof IDs

- `cloudflare-control.test-runner-unit`
- `cloudflare-control.test-runner-integration`
- `cloudflare-control.test-runner-e2e`
- `cloudflare-control.test-runner-contract`
- `cloudflare-control.test-runner-security`
- `cloudflare-control.property-fuzz-boundary`

## Current execution truth

- Status: `proof-required`
- Proof root: expected output path `output/cloudflare-control-plane-plan-proof/08-testing-runner-and-test-pyramid/`
- The runner in `infra/cloudflare/scripts/test-runner.ts` now owns the WP08 family-to-file contract and emits the selected proof IDs, assertion IDs, and any same-directory exclusions with `--list`.
- Same-directory extras are explicit and excluded from the WP08 runner contract until the strategy and matrix adopt them first:
  - unit: `tests/unit/billing-binding-read-model.test.ts`
  - integration: `tests/integration/checkout-portal-hosted.test.ts`, `local-dev-seeding-workflow.test.ts`, `payment-routes-real.test.ts`, `provider-webhooks.test.ts`, `reconciliation-auth-boundary.test.ts`, `worker-runtime-real.test.ts`
  - security: `tests/security/interactive-billing-boundary.test.ts`

## Families proved vs blocked

- Proved at runner-contract surface: `unit`, `integration`, `e2e`, `contract`, `security`, `property`, and `fuzz` are explicit, file-scoped, and mapped to exact assertion IDs.
- Current execution green today: `unit`, `integration`, `e2e`, `contract`, `security`, `property`, and `fuzz`.
- Partial file-level passes inside the current green family runs:
  - unit: `auth-boundary`, `route-manifest`, `env-bindings`, `redaction`
  - security: `redaction`
  - property: `route-auth-state`
- Representative file-level coverage remains explicit:
  - unit: `request-limits`, `kill-switch`
  - integration: `worker-health`, `pricing-public`, `billing-status-auth`, `webhook-signature-rejection`, `admin-auth-rejection`
  - e2e: `portal-to-worker-billing-status`
  - contract: `billing-api-contract`
  - security: `no-provider-secrets-in-client`, `cors-origin-rejection`, `request-smuggling`
  - property: `billing-idempotency`
  - fuzz: `provider-webhook-payload`

## Exact blockers

- proof artifact absent/not produced in this checkout: `output/cloudflare-control-plane-plan-proof/08-testing-runner-and-test-pyramid/`
- Current execution truth is green across the owned unit, integration, e2e, contract, security, property, and fuzz families; this packet no longer claims a runtime blocker.
- `npm --prefix infra/cloudflare run lint` passed at head `c121ba5eb`; no current Cloudflare module lint blocker is claimed here.
- `npm --prefix infra/cloudflare run lint` also stays outside the narrowed WP08 inventory:
  - `infra/cloudflare/tests/integration/local-dev-seeding-workflow.test.ts`
  - `infra/cloudflare/tests/integration/payment-routes-real.test.ts`
  - `infra/cloudflare/tests/integration/provider-webhooks.test.ts`

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

Validation truth from the current checkout:

- `--list` passed.
- `npm --prefix infra/cloudflare run lint` passed at head `c121ba5eb`.
- `test:unit`, `test:integration`, `test:e2e`, `test:contract`, `test:security`, `test:property`, and `test:fuzz` are currently green in the execution record for this checkout; the remaining open item is the proof artifact.
- focused architecture lint passed with a non-empty scoped run under the current gate surface.

## Negative cases

- Reject pretending coverage or mutation exists from docs alone.
- Reject collapsing several required files into one broad umbrella suite without
  updating the matrix first.
- Reject treating the proof artifact absence as a runtime failure when execution is already green.

## Failure conditions

- Do not treat placeholder tests as passing tests.
- Do not mark WP08 runtime-complete from assertion inventory alone.

## No-claim boundary

- This workpack does not prove Cloudflare runtime readiness.
- This workpack does not prove billing readiness, payment handoff readiness, or portal completion.
- This workpack does not prove the excluded same-directory tests belong to the WP08 contract.
- WP08 stays open until the proof artifact is produced and the remaining custody gates are closed.
