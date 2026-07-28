# Cloudflare Control Plane Plan State

Status: engineering-grade Cloudflare control-plane spec is complete; WP00 games infra parity extraction remains validation-blocked by repo-wide `npm run format:check` drift outside its packet; WP01 Cloudflare module scaffold has retained proof and current scoped validation (module lint, 49-test unit family, and architecture gate) and is complete for its narrow scaffold acceptance; the repo-local module is largely implemented. WP02 through WP12 retain their own blocked-state or handoff proof roots and remain open; WP01 does not imply runtime, deployment, authority, or payment closure.

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

- WP00 now has a real parity-extraction proof root under `output/cloudflare-control-plane-plan-proof/00-games-infra-parity-extraction/`, and it intentionally limits claims to keep/adapt/strip mapping plus explicit carried non-goals rather than runtime readiness; final closure remains blocked because `npm run format:check` currently fails on repo-wide Prettier drift outside WP00.
- WP01 has a retained module-scaffold proof root under `output/cloudflare-control-plane-plan-proof/01-cloudflare-module-scaffold/`. A 2026-07-28 current-code audit confirmed `infra/cloudflare` now uses local generated billing contracts rather than the previously cited missing billing-domain imports; module lint, the 49-test unit family, and the directory architecture gate pass. Its closure is limited to module-tree/script honesty and explicit placeholder boundaries.
- WP05 now has a real auth/admin/support proof root under `output/cloudflare-control-plane-plan-proof/05-auth-admin-support-boundary/`, and it intentionally limits claims to explicit auth states, adapter-method honesty, manual-required webhook fallback, and support-safe rejection payloads; final closure remains blocked because the focused unit, security, and integration families still hit the external missing billing boundary import through `infra/cloudflare/src/index.ts`.
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
| CFCP-001 through CFCP-011 | architecture-closed / runtime-present / WP01 proof-current | The shared module, guard chain, route surface, auth boundary, storage model, and local runtime exist in source. WP01's package/tree/no-claim state is now retained and command-backed; later workpack proof and runtime readiness remain separately open. | `infra/cloudflare/` source, module docs, plan docs, and output proof roots agree on current runtime truth and remaining no-claim boundaries. |
| CFCP-021                  | architecture-closed / proof-present / wrangler-binding-blocked | WP02 now has a real wrangler/binding proof root. The owned env-binding test proves explicit dev/prod binding names, placeholder-only secret custody, and no wildcard production origin claim, but the workpack-required `npm --prefix infra/cloudflare run test:unit` and `npm --prefix infra/cloudflare run lint` remain blocked outside the packet. The unit runner still fails before worker boot because `infra/cloudflare/src/index.ts` cannot import `packages/billing-domain/src/billing-checkout-portal-boundary.js`, and module-wide lint still reports broader missing billing-domain imports plus existing `src/fixtures.ts` TypeScript return-path errors. | `output/cloudflare-control-plane-plan-proof/02-wrangler-env-bindings/` records the direct owned proof, exact negative/no-claim boundaries, rollback scope, and blocked reruns; the missing billing-domain boundaries and broader Cloudflare lint debt are cleared and the required unit plus lint reruns execute without outside-slice failures. |
| CFCP-018                  | architecture-closed / proof-present / worker-boot-blocked    | WP03 now has a blocked worker-entrypoint proof bundle: guard-adjacent suites that do not import `src/index.ts` still pass, but any scoped unit or integration suite that boots the worker fails because `infra/cloudflare/src/index.ts` cannot import `packages/billing-domain/src/billing-checkout-portal-boundary.js`. As a result, live fetch-path, CORS, request-size, kill-switch, and safe-error behavior are not runtime-proven through a booted worker. | `output/cloudflare-control-plane-plan-proof/03-worker-entrypoint-runtime-guards/` records the split between passing non-boot guard suites and blocked boot-dependent suites; the missing billing-domain boundary import is restored and the scoped unit plus integration reruns execute through the worker entrypoint. |
| CFCP-020                  | architecture-closed / proof-present / contract-runtime-blocked | WP04 now has a blocked route-manifest proof bundle: `infra/cloudflare/src/routes.ts` remains the single shared route manifest, the unit suite now pins the exact route-key list, and the property suite now proves `/auth/billing/manual-invoice` is the only elevated support-owned exception inside `/auth/billing/*`. However, `tests/contract/billing-api-contract.test.ts` cannot import `packages/billing-domain/src/billing-checkout-portal-boundary.js`, and every boot-dependent unit, property, or integration file that reaches `infra/cloudflare/src/index.ts` blocks on that same missing module before route dispatch. | `output/cloudflare-control-plane-plan-proof/04-route-manifest-and-domain-contracts/` records the exact manifest/auth proofs, the manual-invoice exception, and the blocked contract/runtime validations; the missing billing-domain boundary import is restored and the scoped unit, property, contract, and integration reruns reach live contract and dispatch coverage without widening into billing-domain ownership. |
| CFCP-019                  | architecture-closed / proof-present / storage-binding-blocked | WP06 now has a real storage-binding proof root. The owned `env-bindings` test proves explicit DO/D1/KV/Queue/R2 ownership and no-child-data boundaries, but the workpack-required `test:unit`, `test:integration`, and `test:property` reruns still block before worker boot because `infra/cloudflare/src/index.ts` cannot import `packages/billing-domain/src/billing-checkout-portal-boundary.js`. | `output/cloudflare-control-plane-plan-proof/06-storage-do-d1-kv-r2-queue-bindings/` records the direct owned proof, exact negative/no-claim boundaries, rollback scope, and blocked family reruns; the missing billing-domain boundary import is restored and the required families rerun without the worker import failure. |
| CFCP-012                  | architecture-closed / proof-present / runtime-blocked        | WP07 now has a real proof bundle plus a passing owned integration test and workflow probe, but local start and seed remain blocked by missing billing-domain runtime boundary modules: `packages/billing-domain/src/billing-checkout-portal-boundary.js`, `billing-referral-boundary.js`, `billing-support-admin-api-boundary.js`, `billing-account-runtime-boundary.js`, and `billing-support-admin-runtime-boundary.js`. | `output/cloudflare-control-plane-plan-proof/07-local-dev-seeding-and-fixtures/` records the explicit local start, seed, teardown, fixture-family, and no-claim truth; the missing billing-domain boundaries are restored and the scoped workflow probe plus owned integration test rerun without blocked start/seed states. |
| CFCP-013                  | architecture-closed / runner-proof-present / runtime-blocked | WP08 now has an explicit runner manifest limited to the required unit, integration, e2e, contract, security, property, and fuzz files, but the runtime-facing families still block on missing billing-domain boundary modules: `packages/billing-domain/src/billing-checkout-portal-boundary.js`, `billing-referral-boundary.js`, `billing-support-admin-api-boundary.js`, `billing-support-admin-runtime-boundary.js`, and `billing-account-runtime-boundary.js`. Module-wide `npm --prefix infra/cloudflare run lint` also still sees extra out-of-WP08 test debt after the required-family runner is narrowed. | `output/cloudflare-control-plane-plan-proof/08-testing-runner-and-test-pyramid/` records the runner manifest, exact file and assertion mapping, focused command results, and carried blockers; upstream billing-domain boundaries are restored and the focused family commands plus module lint rerun green under `.../08-testing-runner-and-test-pyramid/` and `.../10-security-fuzz-property-observability/`. |
| CFCP-015                  | architecture-closed / proof-present / runtime-blocked        | WP10 now has a canonical blocked-state proof root for security, property, fuzz, and carried observability, but the current reruns still block on `packages/billing-domain/src/billing-checkout-portal-boundary.js` imported through `infra/cloudflare/src/index.ts`. Under current truth, only `tests/security/redaction.test.ts` and `tests/property/route-auth-state.property.test.ts` still pass inside the required families; the integration rerun that carries `OBS-03` does not boot. | `output/cloudflare-control-plane-plan-proof/10-security-fuzz-property-observability/` records the exact family files, assertion IDs, current command outputs, and no-claim boundaries; the missing billing-domain boundary import is restored and `test:security`, `test:property`, `test:fuzz`, and `test:integration` rerun green without widening into extra Cloudflare suites. |
| CFCP-017                  | architecture-closed / proof-present / consumer-smoke-blocked | WP09 now has a blocked portal-to-worker smoke proof bundle: the owned e2e runner stays narrowed to `tests/e2e/portal-to-worker-billing-status.test.ts`, but the worker fails before route execution because `infra/cloudflare/src/index.ts` cannot import `packages/billing-domain/src/billing-checkout-portal-boundary.js`. The consumer contract remains explicit about redacted portal-visible states, but no runtime smoke pass can be claimed. | `output/cloudflare-control-plane-plan-proof/09-portal-to-worker-e2e-smoke/` records the exact e2e selection, blocked runner output, and no-claim boundary; the missing billing-domain boundary import is restored and `test:e2e` reruns through a booted worker to prove `/auth/billing/status` without widening into checkout, admin, or provider flows. |
| CFCP-016                  | architecture-closed / proof-present / deployment-blocked     | WP11 now has a blocked deployment proof bundle: dev and production dry-runs both stop before publish, emit `--env` warnings without matching `[env.*]` sections, and fail to bundle because `infra/cloudflare/src/index.ts` and `src/fixtures.ts` still import missing billing-domain runtime boundary modules. Both configs also remain placeholder-backed with manual-required auth/key refs, so no promotion or rollback proof exists yet. | `output/cloudflare-control-plane-plan-proof/11-deployment-and-environment-promotion/` records the dry-run outputs, placeholder/manual-required config truth, and manual-required rollback boundary; upstream billing-domain boundaries are restored, real resource IDs and secret custody are provisioned, and deploy plus post-deploy smoke reruns produce real dev/prod artifacts. |
| CFCP-014                  | architecture-closed / handoff-proof-present / payment-blocked | WP12 now records an explicit blocked handoff against the current proof-present Cloudflare inventory relevant to payment assumptions: WP01, WP02, WP03, WP04, WP05, WP06, WP07, WP08, WP09, WP10, and WP11 all have real proof roots, but those accepted roots still carry exact external blockers and manual-required states rather than runnable readiness. The dominant blocker family remains the missing billing-domain runtime boundary modules led by `packages/billing-domain/src/billing-checkout-portal-boundary.js`; WP02 also carries broader missing billing-domain imports plus existing `infra/cloudflare/src/fixtures.ts` TypeScript return-path lint debt, and no downstream payment acknowledgment is recorded here. | `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/payment-handoff-proof.md` exists, lists the accepted WP01-WP11 proof roots plus their exact carried blockers and manual-required states, and downstream payment acknowledgment lands without widening blocked-state proof into readiness claims. |

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
