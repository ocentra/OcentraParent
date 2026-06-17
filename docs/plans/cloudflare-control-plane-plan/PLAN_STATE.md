# Cloudflare Control Plane Plan State

Status: engineering-grade Cloudflare control-plane spec is complete and the
repo-local module is largely implemented; scoped validation is real, but proof
roots, module docs, and final dependency-gated closure remain open.

Research status: aligned against the current Parent repo and a direct inspection of the reusable games Cloudflare module, summarized in `GAMES_INFRA_PARITY_MAP.md`, including its package scripts, wrangler config, route manifest, auth middleware, payment flows, `PaymentDO`, test runner, and module docs. Parent keeps the module and testing patterns; Parent strips game-only economy, Solana, matchmaking, social, AI proxy, and asset-delivery concerns.

Current Parent direction:

- `infra/cloudflare/` is now a separate repo-local module, not a payment-owned subfolder.
- The module includes package scripts, Wrangler dev/prod configs, concrete env
  and secret names, a real worker entrypoint, a real route manifest, a real
  auth verifier boundary, real local seed helpers, and real unit, integration,
  e2e, contract, security, property, and fuzz suites.
- Several planned source subdirectories remain scaffold-only `README.md`
  surfaces while most runtime behavior currently lives in
  `src/index.ts`, `src/billing-binding-read-model.ts`, and `src/fixtures.ts`.
- The test/proof spec includes an exhaustive required test-file inventory, a
  file-level assertion matrix, explicit redaction and observability boundaries,
  and proof-shape requirements for execution agents that must now match real
  module truth.
- Cloudflare Worker entrypoint owns env validation, CORS fail-fast, request-size limits, emergency kill switch, route-manifest dispatch, safe error handling, redacted logging, and scheduled reconciliation hooks.
- Auth is an adapter boundary with explicit states: `public`, `parent-session-required`, `trusted-parent-device-required`, `admin-required`, `support-required`, `provider-webhook-signature-required`, and `internal-queue-only`.
- Durable Objects coordinate per-account, per-household, and per-ledger idempotent writes.
- D1 owns queryable billing/support/reconciliation ledgers and read models.
- KV owns rate limits, short-lived caches, and idempotency helpers where needed.
- Queues own retry, dead-letter, provider polling, and reconciliation jobs.
- Optional R2 is limited to support-safe audit/export artifacts; it must not become child-data storage.
- Payment work cannot claim runtime readiness until the Cloudflare handoff proof exists.

## Decision records

| Record                    | Status                                                       | Gap                                                                                                                                                | Closure criteria                                                                                                               |
| ------------------------- | ------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| CFCP-001 through CFCP-011 | architecture-closed / runtime-present / proof-open           | The shared module, guard chain, route surface, auth boundary, storage model, and local runtime exist in source, but plan docs and proof roots are stale. | `infra/cloudflare/` source, module docs, plan docs, and output proof roots agree on current runtime truth and remaining no-claim boundaries. |
| CFCP-012                  | architecture-closed / local-runtime-present / proof-open     | Local Wrangler-backed runtime and seed flows are exercised by real tests, but no output proof bundle exists yet for the local start, seed, and teardown path. | Local start, seed, teardown, and environment expectations are captured under `output/cloudflare-control-plane-plan-proof/07-local-dev-seeding-and-fixtures/`. |
| CFCP-013                  | architecture-closed / runtime-mostly-proven / proof-open     | Real runnable unit, contract, security, property, fuzz, e2e, and heavy integration coverage exists, but the proof root is absent and the scoped integration baseline must stay green. | Scoped Cloudflare validation is green and recorded under `output/cloudflare-control-plane-plan-proof/08-testing-runner-and-test-pyramid/` and `.../10-security-fuzz-property-observability/`. |
| CFCP-014                  | architecture-closed / handoff-blocked                        | Payment dependency is explicit, but no accepted WP12 handoff artifact exists and payment must not infer readiness from source or spec alone.      | `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/payment-handoff-proof.md` exists and `payment-subscription-plan` WP00 points at it. |

## HID Execution Guard

- Scope and completion source:
  - follow `PLAN_EXECUTION_BLUEPRINT.md`, then the assigned workpack and `NEXT_ACTIONS.md`.
  - do not treat file presence, empty scaffolds, or stale docs as runtime correctness.
- Before any checked update, attach:
  - exact runtime or blocker state for the touched slice,
  - a proof pointer under `output/cloudflare-control-plane-plan-proof/`.
- Failure rule: no PR-ready claim until the shared module, auth boundary, storage bindings, and test pyramid each have proof or exact blockers.

## Overclaim boundary

This plan is implementation-present but not completion-ready. Cloudflare
correctness remains incomplete until scoped validation stays green, proof
bundles live under `output/cloudflare-control-plane-plan-proof/`, and WP12
records an honest payment handoff with remaining auth and deployment
dependencies called out.
