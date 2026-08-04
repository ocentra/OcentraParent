<!-- agent-capsule -->

> Agent Capsule
> Plan: `cloudflare-control-plane-plan`
> Doc: `Cloudflare Control Plane Workpack Index`
> Kind: workpack selector.
> Read when: after PLAN_STATE.md and NEXT_ACTIONS.md.
> Stop rule: open exactly one selected workpack; do not read every workpack.
> Proves: workpack routing only.
> Does not prove: Cloudflare runtime readiness, payment readiness, account readiness, or PR readiness.
> Proof rule: update counts/status only after matching proof artifacts exist.

<!-- /agent-capsule -->

# Cloudflare Control Plane Workpack Index

Choose one workpack. Do not open all workpacks.

Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear. Do not use it as permission to scan multiple workpacks.

| Status | Workpack | Boxes | Primary source docs | Proof root |
| --- | --- | ---: | --- | --- |
| source-present / retained-proof-absent | [WP00 Games Infra Parity Extraction](workpacks/00-games-infra-parity-extraction.md) | 0/8 | `GAMES_INFRA_PARITY_MAP.md` | no tracked root |
| source-present / dependency-reconciliation-proof-absent | [WP01 Cloudflare Module Scaffold](workpacks/01-cloudflare-module-scaffold.md) | 0/10 | `PARENT_CLOUDFLARE_MODULE_SPEC.md`; `SOURCE_SURFACE_STATUS_MATRIX.md` | `03-package-dependency-graph.md` required |
| source-present / retained-proof-absent | [WP02 Wrangler Env Bindings](workpacks/02-wrangler-env-bindings.md) | 0/10 | `STORAGE_BINDING_MODEL.md`; `DEPLOYMENT_MODEL.md` | no tracked root |
| source-present / retained-proof-absent | [WP03 Worker Entrypoint Runtime Guards](workpacks/03-worker-entrypoint-runtime-guards.md) | 0/11 | `SECURITY_PRIVACY_OBSERVABILITY.md`; `DEPLOYMENT_MODEL.md` | no tracked root |
| source-present / retained-proof-absent | [WP04 Route Manifest And Domain Contracts](workpacks/04-route-manifest-and-domain-contracts.md) | 0/11 | `ROUTE_MANIFEST_MODEL.md`; `AUTH_BOUNDARY_MODEL.md` | no tracked root |
| source-present / retained-proof-absent | [WP05 Auth Admin Support Boundary](workpacks/05-auth-admin-support-boundary.md) | 0/11 | `AUTH_BOUNDARY_MODEL.md`; `ROUTE_MANIFEST_MODEL.md` | no tracked root |
| source-present / retained-proof-absent | [WP06 Storage DO D1 KV R2 Queue Bindings](workpacks/06-storage-do-d1-kv-r2-queue-bindings.md) | 0/11 | `STORAGE_BINDING_MODEL.md`; `SECURITY_PRIVACY_OBSERVABILITY.md` | no tracked root |
| source-present / retained-proof-absent | [WP07 Local Dev Seeding And Fixtures](workpacks/07-local-dev-seeding-and-fixtures.md) | 0/10 | `LOCAL_DEV_AND_SEEDING_MODEL.md`; `TESTING_STRATEGY.md` | no tracked root |
| source-present / retained-proof-absent | [WP08 Testing Runner And Test Pyramid](workpacks/08-testing-runner-and-test-pyramid.md) | 0/12 | `REQUIRED_TEST_ASSERTION_MATRIX.md` | no tracked root |
| source-present / retained-proof-absent | [WP09 Portal To Worker E2E Smoke](workpacks/09-portal-to-worker-e2e-smoke.md) | 0/10 | `TESTING_STRATEGY.md`; `REQUIRED_TEST_ASSERTION_MATRIX.md` | no tracked root |
| source-present / retained-proof-absent | [WP10 Security Fuzz Property Observability](workpacks/10-security-fuzz-property-observability.md) | 0/12 | `SECURITY_PRIVACY_OBSERVABILITY.md`; `REQUIRED_TEST_ASSERTION_MATRIX.md` | no tracked root |
| source-present / retained-proof-absent | [WP11 Deployment And Environment Promotion](workpacks/11-deployment-and-environment-promotion.md) | 0/10 | `DEPLOYMENT_MODEL.md` | no tracked root |
| source-present / retained-proof-absent | [WP12 Payment Plan Handoff Gate](workpacks/12-payment-plan-handoff-gate.md) | 0/10 | prior proof roots | no tracked root |

Several rows already have real source or test implementations. Boxes stay `0/x`
until matching output proof artifacts exist.

## Default execution order

```text
WP00 -> WP01 -> WP02 -> WP03 -> WP04 -> WP05 -> WP06 -> WP07 -> WP08 -> WP09 -> WP10 -> WP11 -> WP12
```

## Dependency rules

```text
WP00 prevents copying game-only concerns into Parent.
WP01 establishes the module scaffold and must retain a clean Wrangler/Workers-types dependency graph before WP07 can be selected; WP02 establishes environment/binding scaffold.
WP03/WP04 establish entrypoint and routes.
WP05 blocks private/admin/support/webhook readiness claims.
WP06 blocks storage/coordination/queue claims and owns the account-identity D1/DO/KV binding, adapter, and migration packet after Account WP08 supplies the Rust contract handoff.
WP08 establishes the Cloudflare test-runner/pyramid proof after WP06; it uses module-scoped scripts and does not redefine the account/family contract.
WP09 is the first consumer smoke.
WP10 hardens negative/security/observability coverage.
WP11 establishes deployment promotion.
WP12 is last and gates payment runtime assumptions.
```

## Do not select

Do not implement payment semantics, account provider decisions, device trust, setup UI, portal shell UX, or data custody policy in this plan.

Do not raise status/counts from source presence, scaffold directories, route manifest presence, wrangler placeholder IDs, local dev proof, empty proof directories, stale legacy proof paths, or payment-domain handler presence.
