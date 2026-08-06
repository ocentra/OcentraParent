# Cloudflare Control Plane Plan State

Status: engineering-grade Cloudflare control-plane spec is complete; WP00 games infra parity extraction remains validation-blocked by repo-wide `npm run format:check` drift outside its packet; WP01 Cloudflare module scaffold now has a clean Wrangler/Workers Types graph plus focused local validation and a retained receipt, complete only for its narrow scaffold acceptance; the repo-local module is largely implemented. PR #608 merged to `main` as `5af4a1a92` after fresh full CI passed its product, security, and platform jobs, but it proves only the local WP07 dev/seed/proof boundary. PR #604 is closed without merge: its overlapping branch/evidence are preserved, but it is superseded/conflicting and must not be rebased into the current tree. WP02 through WP12 retain their own blocked-state or handoff evidence and remain open; WP01/WP07 do not imply runtime, deployment, authority, payment, or workpack closure.

Research status: aligned against the current Parent repo and a direct inspection of the reusable games Cloudflare module, summarized in `GAMES_INFRA_PARITY_MAP.md`, including its package scripts, wrangler config, route manifest, auth middleware, payment flows, `PaymentDO`, test runner, and module docs. Parent keeps the module and testing patterns; Parent strips game-only economy, Solana, matchmaking, social, AI proxy, and asset-delivery concerns.

## Current ownership interpretation

```text
infra/cloudflare:
  Shared Worker module, env guards, route manifest, auth adapter boundary, DO/D1/KV/R2/Queue bindings, local dev/seeding, test runner, deployment proof, observability/redaction boundary, and consumer handoff gates.

billing-domain and payment-subscription-plan:
  Billing route contracts, product math, provider semantics, subscription lifecycle, invoice/grace/referral qualification, and payment runtime readiness.

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

- `git ls-tree -r HEAD -- output/cloudflare-control-plane-plan-proof` is empty. Earlier workpack proof-root and command-result narratives are not retained evidence in this checkout; every checklist row remains unchecked until its selected workpack reruns and retains its own bundle.
- PR #608 is merged to `main` as `5af4a1a92` with fresh full CI passed across product, security, and platform jobs. Its local-dev seed/proof hardening is integrated, but WP07 remains local-only because no tracked workpack proof bundle exists. PR #604 is closed without merge as superseded/conflicting; preserve its branch/evidence but do not rebase or merge it.
- A current source audit confirms `infra/cloudflare` imports module-local generated billing contracts, not the formerly cited private `packages/billing-domain/src/*` modules. That former import blocker must not be carried into WP02-WP12 routing.
- WP01's selected graph now resolves normally (`wrangler@4.115.0` + `@cloudflare/workers-types@5.20260804.1`) after moving the declared Workers Types range to `^5.20260722.1`; current focused lint, unit `49/49`, contract `14/14`, integration `70/70`, and Cloudflare architecture checks pass. The tracked receipt and ignored detailed proof bundle record the exact local-only result.
- The WP07 follow-up is a proof-only successor from current source, not a revival of PR #604. Its local-dev/seed proof remains a separate workpack decision; this WP01 dependency refresh does not close or start WP07.
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
| CFCP-001 through CFCP-021 | source-present / retained-proof-absent | Worker/module/config/test source exists, but no Cloudflare output proof root is tracked at HEAD. The prior private billing-module import blocker is resolved in source; runtime, deployment, authority, and payment behavior have not been rerun or retained. | Select one workpack, install its declared dependencies, execute its focused validation, retain its proof bundle, and only then update its individual status. |

## HID Execution Guard

- Scope and completion source:
  - follow `PLAN_EXECUTION_BLUEPRINT.md`, then the assigned workpack and `NEXT_ACTIONS.md`.
  - do not treat file presence, empty scaffolds, or stale docs as runtime correctness.
- Before any checked update, attach:
  - exact runtime or blocker state for the touched slice,
  - a proof pointer under `output/cloudflare-control-plane-plan-proof/`.
- Failure rule: no PR-ready claim until the shared module, auth boundary, storage bindings, and test pyramid each have proof or exact blockers.

## Overclaim boundary

This plan is implementation-present but not completion-ready. Cloudflare correctness remains incomplete until scoped validation stays green, proof bundles live under `output/cloudflare-control-plane-plan-proof/`, and the WP12 handoff bundle no longer carries open external billing-boundary blockers, WP02 lint debt, manual-required authority and deployment states, or downstream payment-plan consumption gaps.
