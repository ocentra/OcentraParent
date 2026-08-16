<!-- agent-capsule -->

> Agent Capsule
> Plan: `cloudflare-control-plane-plan`
> Doc: `Cloudflare Workpack Families`
> Kind: owner-path classifier for selected workpacks.
> Read when: only after `WORKPACK_INDEX.md` selects or names a workpack and the owner/proof family is unclear.
> Stop rule: classify the selected workpack only; do not use this file as permission to scan every workpack in the family.
> Proves: routing and owner-path classification only.
> Does not prove: Cloudflare runtime readiness, payment readiness, production deployment readiness, or PR readiness.
> Proof rule: if this file changes route/status claims, update `AGENTS.md`, `PLAN_STATE.md`, and any affected selected workpack route.

<!-- /agent-capsule -->

# Cloudflare Workpack Families

Use this file to classify a selected workpack before opening source. This plan proves the shared Cloudflare control-plane module and handoff gates. It does not prove billing business semantics, account authority, trusted-device authority, portal UX, setup journey completion, data custody policy, or child telemetry storage.

## Games-parity reduction family

```text
Workpacks:
WP00 Games Infra Parity Extraction

Owners:
docs/plans/cloudflare-control-plane-plan/GAMES_INFRA_PARITY_MAP.md
infra/cloudflare only for parent-safe kept patterns

Rule:
Keep module, wrangler, route, auth, storage, queue, testing, deployment, and observability patterns. Strip game economy, Solana, matchmaking, presence, social, AI proxy, asset delivery, and other game-only surfaces. A parity map is not runtime proof.
```

## Module scaffold and source-surface family

```text
Workpacks:
WP01 Cloudflare Module Scaffold

Owners:
infra/cloudflare/package.json
infra/cloudflare/src/**
module-local docs and source surface matrices

Rule:
Module tree and scripts prove source shape only. Scaffold directories, README files, and handler stubs are not runtime, auth, storage, payment, deployment, or consumer readiness proof.
```

## Wrangler environment, binding, and secret-custody family

```text
Workpacks:
WP02 Wrangler Env Bindings
WP11 Deployment And Environment Promotion when promotion is selected

Owners:
infra/cloudflare/wrangler.toml
infra/cloudflare/wrangler.production.toml
infra/cloudflare/.dev.vars.example
Cloudflare environment/provisioning proof under the selected output root

Rule:
Binding names and placeholder ids prove configuration shape only. Readiness requires real resource refs, secret custody notes, dev/prod separation, rollback or teardown proof, and no secret values in repo artifacts.
```

## Worker runtime guard family

```text
Workpacks:
WP03 Worker Entrypoint Runtime Guards

Owners:
infra/cloudflare/src/index.ts
infra/cloudflare/src/env.ts
infra/cloudflare/src/security/**

Rule:
Worker guard proof must cover env validation, origin/CORS, request-size handling, kill switch, safe error handling, scheduled hook shape, redacted logging, and negative cases. Handler presence is not consumer or payment readiness.
```

## Route manifest and domain-contract family

```text
Workpacks:
WP04 Route Manifest And Domain Contracts

Owners:
infra/cloudflare/src/routes.ts
domain-owned public request/response contract packages
schema-domain when a route contract is cross-domain or neutral shared shape

Rule:
Route manifest presence is not auth readiness or consumer handoff readiness. Route paths, methods, auth states, request models, response models, audit events, and proof-id families must remain synchronized with public contract owners.
```

## Auth, admin, support, and webhook trust family

```text
Workpacks:
WP05 Auth Admin Support Boundary
WP10 Security Fuzz Property Observability when trust negatives are selected

Owners:
infra/cloudflare/src/auth/**
account-identity-family-plan for parent/session/admin/support authority
device-trust-bootstrap-plan for trusted-parent-device authority
provider-specific proof where webhook verification is claimed

Rule:
Header/mock/local verifier proof is an adapter boundary, not production account authority. Private, admin, support, trusted-device, provider-webhook, and internal-queue routes remain dependency-gated until owning-plan/provider proof exists.
```

## Storage, coordination, queue, and audit family

```text
Workpacks:
WP06 Storage DO D1 KV R2 Queue Bindings
WP10 Security Fuzz Property Observability when queue/dead-letter depth is selected

Owners:
infra/cloudflare/src/env.ts
infra/cloudflare/src/billing-binding-read-model.ts
Cloudflare binding model docs and tests

Rule:
DO, D1, KV, R2, and Queue bindings must keep separate ownership: serialized idempotent writes, queryable ledgers/read models, rate-limit/cache/idempotency helpers, retry/dead-letter/reconciliation jobs, and support-safe audit/export artifacts. Binding presence is not operations readiness.
```

## Local dev, seeding, and fixtures family

```text
Workpacks:
WP07 Local Dev Seeding And Fixtures

Owners:
infra/cloudflare/scripts/seed-*.ts
infra/cloudflare/fixtures and local data only when selected
local start/seed/teardown proof root

Rule:
Local dev proof can prove deterministic local start, seed, fixture, and teardown behavior. It does not prove production resource readiness, payment provider readiness, account authority, or deployment promotion.
```

## Test runner and assertion matrix family

```text
Workpacks:
WP08 Testing Runner And Test Pyramid
WP10 Security Fuzz Property Observability when specialized tests are selected

Owners:
infra/cloudflare/scripts/test-runner.ts
infra/cloudflare/tests/**
docs/plans/cloudflare-control-plane-plan/REQUIRED_TEST_ASSERTION_MATRIX.md

Rule:
Test scripts and assertion matrices are obligations, not proof by themselves. Proof requires focused command output or a precise blocker under the selected output root. Do not shrink test scope to make status green.
```

## Portal-to-worker smoke family

```text
Workpacks:
WP09 Portal To Worker E2E Smoke

Owners:
Cloudflare route/read-model contracts
portal consumer only when the selected smoke proves the UI-side caller
output/cloudflare-control-plane-plan-proof/09-portal-to-worker-e2e-smoke/

Rule:
Portal-to-worker smoke proves only the selected consumer path and redaction-safe request/response boundary. It does not prove payment business readiness, production deployment, or broad portal UX completion.
```

## Security, property, fuzz, and observability family

```text
Workpacks:
WP10 Security Fuzz Property Observability

Owners:
infra/cloudflare/tests/security/**
infra/cloudflare/tests/property/**
infra/cloudflare/tests/fuzz/**
security/privacy/observability docs and proof roots

Rule:
Security proof must cover negative auth states, provider-signature blockers, redaction, request-size abuse, route mismatch, queue/dead-letter states, idempotency, and safe diagnostics. Observability cannot leak secrets, raw child data, provider payloads beyond redacted refs, or private support artifacts.
```

## Deployment, promotion, and rollback family

```text
Workpacks:
WP11 Deployment And Environment Promotion

Owners:
wrangler dev/prod configs
Cloudflare environment/provisioning/deploy proof
secret custody docs and rollback/teardown proof

Rule:
Local green tests are not production promotion. Deployment readiness needs real environment refs, binding/resource existence, secret custody, rollback or teardown proof, route auth state, and no-claim boundaries for payment/account/device-trust dependencies.
```

## Payment handoff gate family

```text
Workpacks:
WP12 Payment Plan Handoff Gate

Owners:
output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/payment-handoff-proof.md
payment-subscription-plan only after consuming the handoff artifact

Rule:
Payment remains blocked until the handoff artifact names accepted proof roots, carried blockers, payment assumptions, non-assumptions, account/device-trust/provider/deployment dependency states, and downstream acknowledgment. Do not unblock payment from docs or source presence alone.
```
