<!-- agent-capsule -->

> Agent Capsule
> Plan: `cloudflare-control-plane-plan`
> Doc: AGENTS
> Kind: plan route and local agent contract.
> Read when: First file inside this plan after a global route selects it.
> Stop rule: Choose one workpack; do not inspect sibling plans unless the selected workpack names a handoff.
> Proves: local routing and ownership only.
> Does not prove: runtime readiness, payment readiness, production deployment readiness, or PR readiness.
> Proof rule: Route changes require PLAN_STATE.md, WORKPACK_INDEX.md, TEST_PROOF_EXPECTATIONS.md, PLAN_INDEX.md, and FEATURE_ROUTE_INDEX.md to stay aligned.

<!-- /agent-capsule -->

# Cloudflare Control Plane Plan Agent Route

Task: define the repo-local Cloudflare backend module for Ocentra Parent so payment, portal, support/admin, entitlement, and future cloud edges share one control-plane foundation.

Context: this plan is derived from the reusable control-plane patterns captured in `GAMES_INFRA_PARITY_MAP.md` from the games Cloudflare module. Parent keeps the module boundary, wrangler shape, runtime guards, manifest routing, auth middleware, Durable Object coordination, storage bindings, queue/reconciliation model, and heavy test boundary. Parent explicitly strips game economy, Solana, matchmaking, presence, tournaments, leaderboard, marketplace, AI proxy, asset delivery, and other game-only surfaces.

Scope: `infra/cloudflare/` module ownership, wrangler config, environment and secret custody, Worker entrypoint, auth/admin/support boundary, route manifest ownership, Durable Object and D1/KV/R2/Queue binding model, local dev and seeding, testing pyramid, deployment promotion, security/privacy/observability, and the handoff gate to payment.

Out of scope: billing product math, referral qualification policy, invoice/grace semantics, family identity authority, device trust, portal shell UX, setup journey ownership, data custody policy, and child telemetry. Consumer plans depend on this module only after their named handoff gates pass.

## High-Density Execution Contract

- Route first from `PLAN_STATE.md`; this plan owns the shared Cloudflare module and must not absorb payment semantics or account/device-trust ownership.
- Work only the selected workpack plus its proof rows and route-sync docs.
- Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear.
- For each claimed row, capture failure state, evidence path, negative case, rollback or teardown path, and consumer handoff before marking progress.
- Stop condition: no payment runtime work starts until this plan's WP12 handoff gate is explicit and consumed by the payment plan.

## Ownership, Import, And Boundary Contract

This plan owns the shared Cloudflare control-plane module. It does not own the business semantics of the domains that call it.

Module roles:

```text
infra/cloudflare: Worker module, runtime guard chain, wrangler environment model, route manifest, auth adapter boundary, DO/D1/KV/R2/Queue bindings, local seeding/dev runner, test pyramid, deployment promotion proof, observability/redaction boundary, and consumer handoff gates.
billing-domain and payment-subscription-plan: billing request/response contracts, product math, provider semantics, subscription lifecycle, invoice/grace/referral qualification, and payment runtime readiness.
account-identity-family-plan: parent session, household, guardian/admin/support role authority, and account-provider selection.
device-trust-bootstrap-plan: trusted parent device proof and device trust material used by sensitive Cloudflare routes.
portal-ux-household-surfaces-plan: parent shell and consumer UI; it may call Cloudflare routes but does not own Worker route behavior.
setup-install-provisioning-plan: setup/bootstrap callers and install journey entrypoints; it does not own Cloudflare module internals.
data-custody-storage-plan: retention, export, deletion, and sensitive-data custody policy.
schema-domain or domain public packages: canonical shared contracts when Cloudflare route shapes cross package or plan boundaries.
```

Direct imports are allowed only for neutral/shared infrastructure or explicit public domain contract surfaces:

```text
Cloudflare module-local files under infra/cloudflare when the selected workpack owns the slice
public domain package exports for request/response schemas and route contracts
schema-domain shared shapes when a route contract is cross-domain or cross-plan
neutral logging/redaction/protocol/test helpers
wrangler config, manifest, fixture, seed, and output proof artifacts when the selected workpack names them
```

Migration-sensitive imports and forbidden claims:

```text
private source imports from domain packages are compatibility debt unless the package explicitly exposes them as public contract surfaces
Cloudflare handlers must not define payment product math, account authority, trusted-device authority, setup journey state, or data custody policy
route manifest presence is not auth readiness
header/mock auth proof is not production account or trusted-device authority
billing handler presence is not payment runtime readiness
wrangler placeholder IDs are not environment readiness
local dev/miniflare proof is not production deployment proof
D1/KV/R2/Queue binding presence is not operations readiness
R2 must stay support-safe audit/export storage and must not become child telemetry or raw child-data storage
```

If Cloudflare work needs payment, account, device trust, portal, setup, data custody, support, or admin behavior, it must use typed contracts, route manifests, commands, events, read models, storage bindings, queues, proof roots, and explicit handoffs. Do not solve cross-plan behavior by importing another feature owner's runtime internals.

## Research Gate

Before implementation, DONE, or PR_READY, the assigned agent must inspect only the selected slice of the parent repo, this plan's docs, the games Cloudflare source files summarized in `GAMES_INFRA_PARITY_MAP.md`, and the current Parent plan boundaries. Do not copy game-only runtime concerns into Parent.

## Decision Tree

| If the task is about... | Open |
| --- | --- |
| Extracting the games module pattern and reducing it for Parent | `workpacks/00-games-infra-parity-extraction.md` |
| Module scaffold and file tree under `infra/cloudflare/` | `workpacks/01-cloudflare-module-scaffold.md` |
| Wrangler environments, bindings, secrets, and env docs | `workpacks/02-wrangler-env-bindings.md` |
| Worker entrypoint, runtime guards, and scheduled hooks | `workpacks/03-worker-entrypoint-runtime-guards.md` |
| Route manifest and contract-owned endpoint surfaces | `workpacks/04-route-manifest-and-domain-contracts.md` |
| Auth, admin, support, and webhook trust boundary | `workpacks/05-auth-admin-support-boundary.md` |
| Durable Object, D1, KV, R2, and Queue ownership | `workpacks/06-storage-do-d1-kv-r2-queue-bindings.md` |
| Local development, fixtures, and seed flows | `workpacks/07-local-dev-seeding-and-fixtures.md` |
| Test runner shape and test pyramid | `workpacks/08-testing-runner-and-test-pyramid.md` |
| Portal-to-worker smoke and first shared consumer gate | `workpacks/09-portal-to-worker-e2e-smoke.md` |
| Security, property, fuzz, and observability coverage | `workpacks/10-security-fuzz-property-observability.md` |
| Deployment promotion and environment rollout | `workpacks/11-deployment-and-environment-promotion.md` |
| Payment handoff gate | `workpacks/12-payment-plan-handoff-gate.md` |

## Architecture Decisions

- `infra/cloudflare/` is a separate repo-local module.
- Cloudflare control plane is cross-cutting; payment consumes it but does not own it.
- Worker entrypoint must fail fast on env, origin, request-size, and kill-switch checks before dispatch.
- Route paths and request/response contracts must be owned by domain packages, schema-domain, or explicit route-manifest docs, not ad hoc handler strings.
- Auth is an adapter boundary; the account plan decides the concrete provider.
- Durable Objects coordinate serialized state and idempotency; D1 owns queryable ledgers; KV owns rate-limit/cache/idempotency helpers; Queues own retry and dead-letter flow; optional R2 is limited to support-safe audit/export artifacts.
- Payment-subscription-plan cannot execute runtime payment slices until this plan's handoff gate is explicit.

## Handoffs

- `payment-subscription-plan` owns billing semantics on top of this module.
- `account-identity-family-plan` owns parent session and household authority.
- `device-trust-bootstrap-plan` owns trusted-parent-device proof for sensitive billing routes.
- `portal-ux-household-surfaces-plan` owns the parent shell and its consumer-side views.
- `setup-install-provisioning-plan` owns install/bootstrap entrypoints that later call this module.
- `data-custody-storage-plan` owns retention/export/deletion policy for any support-safe Cloudflare artifacts.

## Failure Conditions

- Do not let payment own the shared Cloudflare scaffold.
- Do not expose private routes without auth or verified provider signatures.
- Do not copy game-only economy, match, social, AI proxy, or asset-delivery code into Parent.
- Do not store child telemetry or raw child data in D1, KV, or R2.
- Do not claim payment, account, trusted-device, production deployment, or operations readiness from Cloudflare source presence alone.
