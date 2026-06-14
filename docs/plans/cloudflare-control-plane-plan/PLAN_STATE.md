# Cloudflare Control Plane Plan State

Status: cross-cutting Cloudflare control-plane architecture documented and repo-local scaffold created; runtime implementation, validation, and proof remain open.

Research status: aligned against the current Parent repo and a direct inspection of `E:\ocentra-games\infra\cloudflare` package scripts, wrangler config, route manifest, auth middleware, payment flows, `PaymentDO`, test runner, and module docs. Parent keeps the module and testing patterns; Parent strips game-only economy, Solana, matchmaking, social, AI proxy, and asset-delivery concerns.

Current Parent direction:

- `infra/cloudflare/` is now a separate repo-local scaffold, not a payment-owned subfolder.
- The scaffold includes module-local package scripts, Wrangler dev/prod configs, placeholder env docs, a safe worker entrypoint, route manifest, auth verifier interface, seed/test runner placeholders, and test-family placeholders.
- Cloudflare Worker entrypoint owns env validation, CORS fail-fast, request-size limits, emergency kill switch, route-manifest dispatch, safe error handling, redacted logging, and scheduled reconciliation hooks.
- Auth is an adapter boundary with explicit states: `public`, `parent-session-required`, `trusted-parent-device-required`, `admin-required`, `support-required`, `provider-webhook-signature-required`, and `internal-queue-only`.
- Durable Objects coordinate per-account, per-household, and per-ledger idempotent writes.
- D1 owns queryable billing/support/reconciliation ledgers and read models.
- KV owns rate limits, short-lived caches, and idempotency helpers where needed.
- Queues own retry, dead-letter, provider polling, and reconciliation jobs.
- Optional R2 is limited to support-safe audit/export artifacts; it must not become child-data storage.
- Payment work cannot claim runtime readiness until the Cloudflare handoff proof exists.

## Decision records

| Record | Status | Gap | Closure criteria |
| --- | --- | --- | --- |
| CFCP-001 through CFCP-011 | architecture-closed / scaffold-present / implementation-open | Shared module, runtime guards, route/auth/storage ownership now have repo-local files, but no runtime proof exists yet. | `infra/cloudflare/` scaffold, bindings, route/docs, and future proof stay synchronized. |
| CFCP-012 | architecture-closed / scaffold-present / manual-required | Local Wrangler and seed flow are specified and scaffolded, but no validated dev loop is captured yet. | Local start, seed, and teardown proof exists. |
| CFCP-013 | architecture-closed / scaffold-present / proof-open | Test pyramid is named and placeholder files exist, but real runnable coverage is not proven. | Unit/integration/e2e/contract/security/property/fuzz proof exists or exact blocker is recorded. |
| CFCP-014 | architecture-closed / proof-open | Payment dependency is explicit, but the handoff proof is not yet satisfied. | `payment-subscription-plan` WP00 proof points to this plan's accepted handoff artifact. |

## HID Execution Guard

- Scope and completion source:
  - follow `PLAN_EXECUTION_BLUEPRINT.md`, then the assigned workpack and `NEXT_ACTIONS.md`.
  - do not treat scaffold existence as runtime correctness.
- Before any checked update, attach:
  - exact scaffold state or blocker for the touched slice,
  - a proof pointer under `docs/proof/cloudflare-control-plane-plan/`.
- Failure rule: no PR-ready claim until the shared module, auth boundary, storage bindings, and test pyramid each have proof or exact blockers.

## Overclaim boundary

This plan is architecture-route ready with a repo-local scaffold, not runtime complete. Cloudflare correctness remains unproven until `infra/cloudflare/` has real handler code, negative cases, proof bundles, rollback notes, and validation logs under `docs/proof/cloudflare-control-plane-plan/`.
