# Cloudflare Control Plane Proof And Test Inventory

## Purpose

This document defines required proof and tests for the shared Parent Cloudflare module.

Exact file-level test scope lives in
[`REQUIRED_TEST_ASSERTION_MATRIX.md`](REQUIRED_TEST_ASSERTION_MATRIX.md). Proof
artifacts must map runtime evidence or blockers back to that matrix.

Use canonical proof root:

```text
output/cloudflare-control-plane-plan-proof/
```

No shared-module, auth, binding, queue, test-runner, or payment-handoff claim is DONE or PR_READY without proof, command output or exact blocker, artifact paths, and no-claim boundaries.

## Proof IDs

- `cloudflare-control.module-exists`
- `cloudflare-control.package-scripts`
- `cloudflare-control.wrangler-dev-config`
- `cloudflare-control.wrangler-prod-config`
- `cloudflare-control.dev-vars-example`
- `cloudflare-control.worker-entrypoint`
- `cloudflare-control.env-validation`
- `cloudflare-control.cors-fail-fast`
- `cloudflare-control.request-size-limit`
- `cloudflare-control.kill-switch`
- `cloudflare-control.route-manifest`
- `cloudflare-control.auth-boundary`
- `cloudflare-control.admin-support-boundary`
- `cloudflare-control.provider-webhook-signature-boundary`
- `cloudflare-control.do-bindings`
- `cloudflare-control.d1-bindings`
- `cloudflare-control.queue-bindings`
- `cloudflare-control.kv-bindings`
- `cloudflare-control.r2-audit-binding-manual-required`
- `cloudflare-control.local-dev-preflight`
- `cloudflare-control.local-dev-runtime-boot-integration`
- `cloudflare-control.local-dev-structured-proof-chain`
- `cloudflare-control.seed-local`
- `cloudflare-control.portal-to-worker-smoke`
- `cloudflare-control.no-secrets-in-repo`
- `cloudflare-control.test-runner-unit`
- `cloudflare-control.test-runner-integration`
- `cloudflare-control.test-runner-e2e`
- `cloudflare-control.test-runner-contract`
- `cloudflare-control.test-runner-security`
- `cloudflare-control.property-fuzz-boundary`
- `cloudflare-control.unit-redaction`
- `cloudflare-control.security-redaction`
- `cloudflare-control.billing-idempotency-property`
- `cloudflare-control.test-assertion-matrix`
- `cloudflare-control.payment-plan-handoff`

## Proof minimum contents

Every WP08 or WP10 proof artifact must record:

- the exact test files in scope;
- the assertion IDs executed from `REQUIRED_TEST_ASSERTION_MATRIX.md`;
- the exact command used, or the exact blocker when no command could run;
- for WP08, the runner manifest and any same-directory test files excluded as
  `unexpectedFilesByFamily`;
- at least one negative case covered by that proof slice;
- a rollback or teardown note when the slice mutates runtime state;
- a no-claim boundary stating what remains unproven.

## Spec-only completion rule

- WP08 and WP10 may reach `spec-complete / implementation-present / proof-open`
  when the file inventory, assertion matrix, and proof shape are exhaustive and
  honest.
- WP08 and WP10 are not runtime complete until real commands and artifacts exist
  for the required assertion IDs.
- Payment may not treat spec completeness as handoff completion.

## Validation matrix

| Workpack | Exact validation commands | Proof locations |
| -------- | ------------------------- | --------------- |
| WP00 | `docs-only parity extraction`; record inspected source files and stripped/kept decisions | `output/cloudflare-control-plane-plan-proof/00-games-infra-parity-extraction/` |
| WP01 | `npm --prefix infra/cloudflare run lint`; `npm run lint:architecture -- --files infra/cloudflare` | `output/cloudflare-control-plane-plan-proof/01-cloudflare-module-scaffold/` |
| WP02 | `npm --prefix infra/cloudflare run test:unit`; `npm run lint:architecture -- --files infra/cloudflare` | `output/cloudflare-control-plane-plan-proof/02-wrangler-env-bindings/` |
| WP03 | `npm --prefix infra/cloudflare run test:unit`; `npm --prefix infra/cloudflare run test:integration`; `npm run lint:architecture -- --files infra/cloudflare` | `output/cloudflare-control-plane-plan-proof/03-worker-entrypoint-runtime-guards/` |
| WP04 | `npm --prefix infra/cloudflare run test:unit`; `npm --prefix infra/cloudflare run test:contract`; `npm --prefix infra/cloudflare run test:integration`; `npm run lint:architecture -- --files infra/cloudflare/src/routes.ts infra/cloudflare/tests/contract/billing-api-contract.test.ts` | `output/cloudflare-control-plane-plan-proof/04-route-manifest-and-domain-contracts/` |
| WP05 | `npm --prefix infra/cloudflare run test:unit`; `npm --prefix infra/cloudflare run test:integration`; `npm --prefix infra/cloudflare run test:security`; `npm run lint:architecture -- --files infra/cloudflare/src/auth` | `output/cloudflare-control-plane-plan-proof/05-auth-admin-support-boundary/` |
| WP06 | `npm --prefix infra/cloudflare run test:unit`; `npm --prefix infra/cloudflare run test:integration`; `npm --prefix infra/cloudflare run test:property`; `npm run lint:architecture -- --files infra/cloudflare` | `output/cloudflare-control-plane-plan-proof/06-storage-do-d1-kv-r2-queue-bindings/` |
| WP07 | `node --import tsx infra/cloudflare/scripts/local-dev-workflow.ts`; `node --import tsx --test infra/cloudflare/tests/integration/local-dev-seeding-workflow.test.ts`; `npm --prefix infra/cloudflare run lint`; `npm --prefix infra/cloudflare run test:integration`; focused architecture, source-shape, required-tests, no-test-doubles, and validation-bypass checks over the two owned TS files | `output/cloudflare-control-plane-plan-proof/07-local-dev-seeding-and-fixtures/` |
| WP08 | `node --import tsx infra/cloudflare/scripts/test-runner.ts --list`; `npm --prefix infra/cloudflare run test:unit`; `npm --prefix infra/cloudflare run test:integration`; `npm --prefix infra/cloudflare run test:e2e`; `npm --prefix infra/cloudflare run test:contract`; `npm --prefix infra/cloudflare run test:security`; `npm --prefix infra/cloudflare run test:property`; `npm --prefix infra/cloudflare run test:fuzz`; `npm --prefix infra/cloudflare run lint` | `output/cloudflare-control-plane-plan-proof/08-testing-runner-and-test-pyramid/` |
| WP09 | `npm --prefix infra/cloudflare run test:e2e` | `output/cloudflare-control-plane-plan-proof/09-portal-to-worker-e2e-smoke/` |
| WP10 | `npm --prefix infra/cloudflare run test:security`; `npm --prefix infra/cloudflare run test:property`; `npm --prefix infra/cloudflare run test:fuzz`; `npm --prefix infra/cloudflare run test:integration` | `output/cloudflare-control-plane-plan-proof/10-security-fuzz-property-observability/` |
| WP11 | `npm --prefix infra/cloudflare run deploy:dev`; `npm --prefix infra/cloudflare run deploy`; post-deploy `/health`, `/public/pricing`, and `/auth/billing/status` smoke in the promoted environment | `output/cloudflare-control-plane-plan-proof/11-deployment-and-environment-promotion/` |
| WP12 | Aggregate accepted WP03-WP11 proof roots, then record downstream payment assumptions and blockers | retained receipt: `docs/proof/cloudflare-control-plane-plan/12-payment-plan-handoff-gate.md`; raw/generated root: `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/` |

## Honesty rule

If a command cannot run because a runtime surface or dependency is missing,
record the exact blocker in the proof artifact and
`SOURCE_SURFACE_STATUS_MATRIX.md` instead of implying success.

## Current WP07 state

- WP07 source and focused local behavior remain present.
- The former tracked generated packet under `output/cloudflare-control-plane-plan-proof/07-local-dev-seeding-and-fixtures/` was removed at `cbb8421875492176bd2a3d5b95eaa7fa0dd8210e` because generated output is not source.
- Historical command counts are not accepted current-head proof after that removal.
- Next proof action: rerun the WP07 command family and retain a compact current-head receipt while keeping raw logs/output ignored.
- Open boundary: local behavior does not imply production deployment, WP12 acceptance, or downstream payment acknowledgment.
