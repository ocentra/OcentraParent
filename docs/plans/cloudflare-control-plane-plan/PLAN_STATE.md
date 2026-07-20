# Cloudflare Control Plane Plan State

Status: engineering-grade Cloudflare control-plane spec is complete. At branch commit `cbb8421875492176bd2a3d5b95eaa7fa0dd8210e`, Cloudflare lint, unit, contract, real Wrangler integration, generated-artifact lint, and billing-core unit gates are green. Generated WP07 files were removed from source control because `output/` is build evidence, not retained source. WP00-WP11 still lack accepted current-head retained receipts. WP12 now has a compact tracked blocker receipt at `docs/proof/cloudflare-control-plane-plan/12-payment-plan-handoff-gate.md`; it preserves validation and handoff blockers but does not unblock payment. The checked-in TS sidecar remains the operational billing-contract source consumed by Cloudflare, while Rust-first canonical generation/ownership, account/trusted-device authority, provider/storage/portal/deployment/data-custody proof, and downstream payment acknowledgment remain open.

Research status: aligned against the current Parent repo and a direct inspection of the reusable games Cloudflare module, summarized in `GAMES_INFRA_PARITY_MAP.md`, including its package scripts, wrangler config, route manifest, auth middleware, payment flows, `PaymentDO`, test runner, and module docs. Parent keeps the module and testing patterns; Parent strips game-only economy, Solana, matchmaking, social, AI proxy, and asset-delivery concerns.

## Current ownership interpretation

```text
infra/cloudflare:
  Shared Worker module, env guards, route manifest, auth adapter boundary, DO/D1/KV/R2/Queue bindings, local dev/seeding, test runner, deployment proof, observability/redaction boundary, and consumer handoff gates.

infra/cloudflare/src/generated/billing-contracts:
  Checked-in TS sidecar consumed by Cloudflare; current operational billing-contract source surface.

billing-core:
  Rust webhook intake, provider lifecycle classification, event/idempotency, dispute/manual-review, and downstream entitlement update helpers.

crates/schema:
  Reader/contract-test bridge around the checked-in TS sidecar; canonical Rust billing-contract generation and ownership remain an open migration gap/blocker.

payment-subscription-plan:
  Billing/payment semantics, provider semantics, subscription lifecycle, invoice/grace/referral qualification, and payment runtime readiness.

account-identity-family-plan:
  Parent session, household, guardian/admin/support role authority, and account-provider selection.

device-trust-bootstrap-plan:
  Trusted parent device proof and device trust material for sensitive routes.

portal-ux-household-surfaces-plan:
  Parent shell and Cloudflare consumer UI only.

setup-install-provisioning-plan:
  Setup/bootstrap callers and install journey entrypoints.

data-custody-storage-plan:
  Retention, export, deletion, and sensitive-data custody policy.

schema-domain or domain public packages:
  Canonical shared contracts when Cloudflare route shapes cross package or plan boundaries.
```

## Current coupling risks

```text
- Implementation-present is not completion-ready.
- Billing handler presence is not payment semantics readiness.
- Auth verifier presence is not production account/session or trusted-device authority.
- Wrangler placeholder config is not deploy readiness.
- Local tests and local dev proof are not production promotion proof.
- Route manifest presence is not consumer handoff readiness.
- Binding presence is not operations readiness.
- Optional R2 remains support-safe audit/export only and must not become child telemetry or raw child-data storage.
- Private source imports from domain packages are migration-sensitive unless the package intentionally exposes them as public contract surfaces.
- Payment remains blocked until WP12 handoff is proven and downstream payment-plan consumption acknowledges what is accepted and what remains blocked.
```

## Current proof interpretation

```text
Source presence is not runtime proof.
Scaffold directory presence is not module readiness.
Route manifest proof is not auth proof.
Auth adapter proof is not account or trusted-device authority proof.
Wrangler config proof is not deployed environment proof.
Local dev/seed proof is not production promotion proof.
Storage binding proof is not queue/dead-letter/reconciliation operations proof.
Portal smoke proof is not portal UX completion or payment readiness.
WP12 can aggregate only accepted proof roots plus exact carried blockers.
```

Current Parent direction:

- Current tracked checkout output inventory: no generated `output/cloudflare-control-plane-plan-proof/` packet is tracked. Raw/generated evidence stays ignored. Compact current-head receipts may be retained under `docs/proof/cloudflare-control-plane-plan/`; WP12 is the first receipt using that cleanup-safe contract.
- Historical plan records without accepted current-head receipts: WP00-WP11. Source and tests may exist, but plan history or ignored output alone is not retained completion proof.
- WP00 historical record: parity extraction history; closure remains blocked by repo-wide `npm run format:check` drift outside that historical packet.
- WP01 historical record: module scaffold source exists; no accepted current-head WP01 receipt exists.
- WP02 historical record: wrangler/binding source exists and module lint is green at `cbb842187`, but environment/deployment behavior lacks an accepted current-head WP02 receipt.
- WP03 historical record: worker-entrypoint source exists and the current unit/integration suites are green, but exact guard coverage lacks an accepted WP03 receipt.
- WP04 historical record: route-manifest source exists and current contract/integration suites are green, but exact route/auth coverage lacks an accepted WP04 receipt.
- WP05 historical record: auth/admin/support boundary source exists, while production authority inputs and an accepted current-head WP05 receipt remain missing.
- WP06 historical record: storage binding source exists, while queue/retry/dead-letter/operations proof and an accepted current-head WP06 receipt remain missing.
- WP07 current record: local source and focused behavior remain present, but its former tracked generated output files were removed to satisfy generated-artifact policy. WP07 needs a current-head retained receipt before acceptance; production deployment, WP12 handoff, and downstream acceptance remain open.
- WP08 historical record: unit, contract, and integration families are green at `cbb842187`; e2e/security/property/fuzz families were not freshly rerun for this receipt, so WP08 remains unverified and receipt-required.
- WP09 historical record: portal-to-worker e2e was not freshly rerun for this receipt; portal smoke remains proof-required.
- WP10 historical record: security/property/fuzz families were not freshly rerun for this receipt; security and observability closure remains unverified.
- WP11 historical record: deploy dry-run, promoted-environment smoke, real resource identifiers, and rollback were not freshly rerun for this receipt; deployment remains manual-required/proof-required.
- WP12 current record: the retained receipt at `docs/proof/cloudflare-control-plane-plan/12-payment-plan-handoff-gate.md` records current local green gates and exact blockers. Raw output remains absent/ignored. Account, trusted device, provider, storage/queue, portal, deployment, data custody, accepted upstream receipts, and downstream payment acknowledgment remain blocked or proof-required.
- `infra/cloudflare/` is now a separate repo-local module, not a payment-owned subfolder.
- The module includes package scripts, Wrangler dev/prod configs, concrete env and secret names, a real worker entrypoint, a real route manifest, a real auth verifier boundary, real local seed helpers, and real unit, integration, e2e, contract, security, property, and fuzz suites.
- Several planned source subdirectories remain scaffold-only `README.md` surfaces while most runtime behavior currently lives in `src/index.ts`, `src/billing-binding-read-model.ts`, and `src/fixtures.ts`.
- The test/proof spec includes an exhaustive required test-file inventory, a file-level assertion matrix, explicit redaction and observability boundaries, and proof-shape requirements for execution agents that must now match real module truth.
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
| CFCP-001 through CFCP-011 | architecture-closed / runtime-present / proof-required | The shared module, guard chain, route surface, auth boundary, storage model, and local runtime exist in source. WP00-WP11 lack accepted current-head retained receipts; WP07 source and local tests remain but its generated packet is no longer tracked. | `infra/cloudflare/` source, module docs, retained receipts, and ignored raw-output expectations agree on current runtime truth and remaining no-claim boundaries. |
| CFCP-021                  | architecture-closed / lint-green / receipt-required | WP02 config exists and module lint is green, but environment binding/deployment behavior is not freshly proven. | Run the exact WP02 family and retain environment-safe proof without claiming placeholder resources as ready. |
| CFCP-018                  | architecture-closed / local-subset-green / receipt-required | Current unit/integration suites cover worker behavior broadly, but no exact current-head WP03 receipt maps the guard assertions. | Rerun/map the WP03 guard family and retain exact negative-path evidence. |
| CFCP-020                  | architecture-closed / local-subset-green / receipt-required | Current contract/integration suites cover route behavior broadly, but no exact current-head WP04 receipt maps route/auth assertions. | Rerun/map the WP04 family without widening into payment semantics. |
| CFCP-019                  | architecture-closed / source-present / operations-proof-required | Storage source exists, but queue/retry/dead-letter/operations behavior lacks accepted current-head proof. | Run the exact WP06 family and retain operations-safe proof. |
| CFCP-012                  | architecture-closed / source-present / receipt-required | WP07 source and focused behavior exist, but generated output is no longer tracked and the prior packet cannot serve as retained current-head proof. | Regenerate current gates and add a compact retained receipt without tracking raw output or promoting local behavior into production/payment readiness. |
| CFCP-013                  | architecture-closed / partial-family-green / receipt-required | Unit, contract, and integration are current; e2e/security/property/fuzz were not rerun for this receipt. | Run all WP08 families and retain assertion-mapped results. |
| CFCP-015                  | architecture-closed / current-proof-missing | WP10 security/property/fuzz/observability families were not freshly rerun. | `test:security`, `test:property`, `test:fuzz`, and `test:integration` rerun green with retained negative/observability evidence. |
| CFCP-017                  | architecture-closed / current-proof-missing | WP09 e2e was not freshly rerun; portal smoke remains unproven. | `test:e2e` reruns through a booted worker to prove `/auth/billing/status` without widening into checkout, admin, or provider flows. |
| CFCP-016                  | architecture-closed / manual-required / current-proof-missing | WP11 deploy/promotion/rollback was not freshly rerun and real resource identifiers remain external. | Deploy plus post-deploy smoke and rollback produce accepted dev/prod receipts. |
| CFCP-014                  | architecture-closed / local-execution-green / retained-blocker-receipt-present | The WP12 receipt records current local gates, missing accepted WP00-WP11 receipts, and manual-required account/trust/provider/storage/portal/deployment/custody/downstream-ack states. It is not an accepted payment handoff. | Required upstream receipts and dependencies are accepted, then payment records downstream acknowledgment without widening blocked-state truth into readiness. |

## HID Execution Guard

- Scope and completion source:
  - follow `PLAN_EXECUTION_BLUEPRINT.md`, then the assigned workpack and `NEXT_ACTIONS.md`.
  - do not treat file presence, empty scaffolds, or stale docs as runtime correctness.
- Before any checked update, attach:
  - exact runtime or blocker state for the touched slice,
  - a proof pointer under `output/cloudflare-control-plane-plan-proof/`.
- Failure rule: no PR-ready claim until the shared module, auth boundary, storage bindings, and test pyramid each have proof or exact blockers.

## Overclaim boundary

This plan is implementation-present but not completion-ready. Cloudflare correctness remains incomplete until scoped validation stays green, required workpacks have accepted current-head retained receipts, production authority/deployment/custody gates are proven, and WP12 receives downstream payment acknowledgment. Ignored raw output and the WP12 blocker receipt cannot substitute for those states.
