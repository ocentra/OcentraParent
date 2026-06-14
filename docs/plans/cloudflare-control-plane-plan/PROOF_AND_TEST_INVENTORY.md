# Cloudflare Control Plane Proof And Test Inventory

## Purpose

This document defines required proof and tests for the shared Parent Cloudflare module.

Use proof root:

```text
docs/proof/cloudflare-control-plane-plan/
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
- `cloudflare-control.local-dev-start`
- `cloudflare-control.seed-local`
- `cloudflare-control.portal-to-worker-smoke`
- `cloudflare-control.no-secrets-in-repo`
- `cloudflare-control.test-runner-unit`
- `cloudflare-control.test-runner-integration`
- `cloudflare-control.test-runner-e2e`
- `cloudflare-control.test-runner-contract`
- `cloudflare-control.test-runner-security`
- `cloudflare-control.property-fuzz-boundary`
- `cloudflare-control.payment-plan-handoff`

## Validation matrix

| Workpack | Exact validation commands | Proof locations |
| --- | --- | --- |
| WP00 | `docs-only parity extraction`; record inspected source files | `docs/proof/cloudflare-control-plane-plan/wp00-games-infra-parity-extraction/parity-extraction-proof.md` |
| WP01 | `npm run format:check` | `docs/proof/cloudflare-control-plane-plan/wp01-cloudflare-module-scaffold/module-scaffold-proof.md` |
| WP02 | `npm run format:check` | `docs/proof/cloudflare-control-plane-plan/wp02-wrangler-env-bindings/wrangler-binding-proof.md` |
| WP03 | `npm run format:check` | `docs/proof/cloudflare-control-plane-plan/wp03-worker-entrypoint-runtime-guards/entrypoint-guard-proof.md` |
| WP04 | `npm run format:check` | `docs/proof/cloudflare-control-plane-plan/wp04-route-manifest-and-domain-contracts/route-manifest-proof.md` |
| WP05 | `npm run format:check` | `docs/proof/cloudflare-control-plane-plan/wp05-auth-admin-support-boundary/auth-boundary-proof.md` |
| WP06 | `npm run format:check` | `docs/proof/cloudflare-control-plane-plan/wp06-storage-do-d1-kv-r2-queue-bindings/storage-binding-proof.md` |
| WP07 | `npm run format:check` | `docs/proof/cloudflare-control-plane-plan/wp07-local-dev-seeding-and-fixtures/local-dev-proof.md` |
| WP08 | `npm run format:check`; scaffold command family now exists: `npm run test:cloudflare:unit`, `test:cloudflare:integration`, `test:cloudflare:e2e`, `test:cloudflare:contract`, `test:cloudflare:security`, `test:cloudflare:property`, `test:cloudflare:fuzz`; current runner still returns exact blockers | `docs/proof/cloudflare-control-plane-plan/wp08-testing-runner-and-test-pyramid/test-pyramid-proof.md` |
| WP09 | `npm run format:check`; scaffold command family now exists: `npm run test:cloudflare:e2e`; current runner still returns exact blockers | `docs/proof/cloudflare-control-plane-plan/wp09-portal-to-worker-e2e-smoke/portal-smoke-proof.md` |
| WP10 | `npm run format:check`; scaffold command family now exists: `npm run test:cloudflare:security`; `npm run test:cloudflare:property`; `npm run test:cloudflare:fuzz`; current runner still returns exact blockers | `docs/proof/cloudflare-control-plane-plan/wp10-security-fuzz-property-observability/security-baseline-proof.md` |
| WP11 | `npm run format:check` | `docs/proof/cloudflare-control-plane-plan/wp11-deployment-and-environment-promotion/deployment-model-proof.md` |
| WP12 | `npm run format:check` | `docs/proof/cloudflare-control-plane-plan/wp12-payment-plan-handoff-gate/payment-handoff-proof.md` |

## Honesty rule

If a command cannot run because the scaffold or dependency is missing, record the exact blocker in the proof artifact and `SOURCE_SURFACE_STATUS_MATRIX.md` instead of implying success.
