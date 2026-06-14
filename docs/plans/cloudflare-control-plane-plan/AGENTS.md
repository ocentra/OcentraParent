<!-- agent-capsule -->

> Agent Capsule
> Plan: `cloudflare-control-plane-plan`
> Doc: AGENTS
> Kind: plan route and local agent contract.
> Read when: First file inside this plan after a global route selects it.
> Stop rule: Choose one workpack; do not inspect sibling plans unless the selected workpack names a handoff.
> Proves: local routing and ownership only.

<!-- /agent-capsule -->

# Cloudflare Control Plane Plan Agent Route

Task: define the repo-local Cloudflare backend module for Ocentra Parent so payment, portal, support/admin, entitlement, and future cloud edges share one control-plane foundation.

Context: this plan is derived from the reusable control-plane patterns found in `E:\ocentra-games\infra\cloudflare`. Parent keeps the module boundary, wrangler shape, runtime guards, manifest routing, auth middleware, Durable Object coordination, storage bindings, queue/reconciliation model, and heavy test boundary. Parent explicitly strips game economy, Solana, matchmaking, presence, tournaments, leaderboard, marketplace, AI proxy, asset delivery, and other game-only surfaces.

Scope: `infra/cloudflare/` module ownership, wrangler config, environment and secret custody, Worker entrypoint, auth/admin/support boundary, route manifest ownership, Durable Object and D1/KV/R2/Queue binding model, local dev and seeding, testing pyramid, deployment promotion, security/privacy/observability, and the handoff gate to payment.

Out of scope: billing product math, referral qualification policy, invoice/grace semantics, family identity authority, device trust, portal shell UX, and child telemetry. Consumer plans depend on this module after the handoff gate passes.

## High-Density Execution Contract

- Route first from `PLAN_STATE.md`; this plan owns the shared Cloudflare module and must not absorb payment semantics or account/device-trust ownership.
- Work only the selected workpack plus its proof rows and route-sync docs.
- For each claimed row, capture failure state, evidence path, negative case, rollback path, and consumer handoff before marking progress.
- Stop condition: no payment runtime work starts until this plan's WP12 handoff gate is explicit.

## Research Gate

Before implementation, DONE, or PR_READY, the assigned agent must inspect the parent repo, this plan's docs, the named `ocentra-games/infra/cloudflare` source files, and the current Parent plan boundaries. Do not copy game-only runtime concerns into Parent.

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
- Route paths and request/response contracts must be owned by domain packages or explicit route-manifest docs, not ad hoc handler strings.
- Auth is an adapter boundary; the account plan decides the concrete provider.
- Durable Objects coordinate serialized state and idempotency; D1 owns queryable ledgers; KV owns rate-limit/cache/idempotency helpers; Queues own retry and dead-letter flow; optional R2 is limited to support-safe audit/export artifacts.
- Payment-subscription-plan cannot execute runtime payment slices until this plan's handoff gate is explicit.

## Handoffs

- `payment-subscription-plan` owns billing semantics on top of this module.
- `account-identity-family-plan` owns parent session and household authority.
- `device-trust-bootstrap-plan` owns trusted-parent-device proof for sensitive billing routes.
- `portal-ux-household-surfaces-plan` owns the parent shell and its consumer-side views.
- `setup-install-provisioning-plan` owns install/bootstrap entrypoints that later call this module.

## Failure Conditions

- Do not let payment own the shared Cloudflare scaffold.
- Do not expose private routes without auth or verified provider signatures.
- Do not copy game-only economy, match, social, AI proxy, or asset-delivery code into Parent.
- Do not store child telemetry or raw child data in D1, KV, or R2.
