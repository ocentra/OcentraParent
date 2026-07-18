# Cloudflare Control Plane Plan State

Status: engineering-grade Cloudflare control-plane spec is complete, the current tracked checkout contains no WP00-WP12 proof roots, WP12 has a defined expected output path under `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/payment-handoff-proof.md` but its generated proof is absent/not produced, and the repo-local module is largely implemented; scoped validation is real, but the current blocker is unrun focused gates, the live generated billing-contract source-surface rerun gap, and the still-open `src/fixtures.ts` TypeScript return-path debt.

Research status: aligned against the current Parent repo and a direct inspection of the reusable games Cloudflare module, summarized in `GAMES_INFRA_PARITY_MAP.md`, including its package scripts, wrangler config, route manifest, auth middleware, payment flows, `PaymentDO`, test runner, and module docs. Parent keeps the module and testing patterns; Parent strips game-only economy, Solana, matchmaking, social, AI proxy, and asset-delivery concerns.

## Current ownership interpretation

```text
infra/cloudflare:
  Shared Worker module, env guards, route manifest, auth adapter boundary, DO/D1/KV/R2/Queue bindings, local dev/seeding, test runner, deployment proof, observability/redaction boundary, and consumer handoff gates.

infra/cloudflare/src/generated/billing-contracts:
  Consumed generated edge surface emitted from `crates/schema`; the Worker reads it, but it is not the canonical owner of billing contracts.

crates/schema:
  Canonical billing-contract generator and shared schema shapes.

billing-domain, billing-core, and payment-subscription-plan:
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

- Current tracked checkout output inventory: none. No WP00-WP12 proof roots are present in this checkout.
- Historical plan records retained in this plan: WP00-WP11. They are plan-history only and not present artifacts in the current checkout output tree.
- WP00 historical record: parity extraction history; closure remains blocked by repo-wide `npm run format:check` drift outside that historical packet.
- WP01 historical record: module scaffold history; closure remains blocked because the scoped lint rerun has not yet been refreshed here against the live generated billing-contract source surface.
- WP02 historical record: wrangler/binding history, with the remaining gap being unrun focused unit and lint reruns against the live generated billing-contract source surface plus existing `src/fixtures.ts` TypeScript return-path errors.
- WP03 historical record: worker-entrypoint guard history, with the remaining gap being unrun scoped unit and integration reruns against the live generated billing-contract source surface.
- WP04 historical record: route-manifest and contract history, with the remaining gap being unrun contract and integration reruns against the live generated billing-contract source surface.
- WP05 historical record: auth/admin/support boundary history, with the remaining gap being unrun focused unit, security, and integration reruns against the live generated billing-contract source surface.
- WP06 historical record: storage binding history, with the remaining gap being unrun unit, integration, and property reruns against the live generated billing-contract source surface plus existing `src/fixtures.ts` TypeScript return-path errors.
- WP07 historical record: local dev, seeding, and fixtures history, with the remaining gap being unrun local-start and seed reruns against the live generated billing-contract source surface.
- WP08 historical record: test runner and test pyramid history, with the remaining gap being unrun narrowed family reruns against the live generated billing-contract source surface and module lint debt outside the packet.
- WP09 historical record: portal-to-worker smoke history, with the remaining gap being unrun e2e reruns against the live generated billing-contract source surface.
- WP10 historical record: security, property, fuzz, and observability history, with the remaining gap being unrun scoped family reruns against the live generated billing-contract source surface plus existing `src/fixtures.ts` lint debt.
- WP11 historical record: deployment and promotion history, with the remaining gap being unrun deploy and smoke reruns against the live generated billing-contract source surface plus placeholder/manual-required config truth.
- WP12 current record: expected output path `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/payment-handoff-proof.md`; generated proof is absent/not produced in this checkout; payment remains blocked because focused gates are unrun here, WP02 still carries `src/fixtures.ts` TypeScript return-path lint debt, account/trusted-device/deployment states remain manual-required or blocked, and no downstream payment acknowledgment is recorded here.
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
| CFCP-001 through CFCP-011 | architecture-closed / runtime-present / proof-required | The shared module, guard chain, route surface, auth boundary, storage model, and local runtime exist in source, while WP00-WP11 are historical plan records and absent from the current tracked checkout output tree. | `infra/cloudflare/` source, module docs, plan docs, and output-proof expectations agree on current runtime truth and remaining no-claim boundaries. |
| CFCP-021                  | architecture-closed / proof-required / wrangler-binding-blocked | WP02 is a historical wrangler/binding record. The current tracked checkout still has unrun focused unit and lint reruns against the live generated billing-contract source surface, and module-wide lint still reports existing `src/fixtures.ts` TypeScript return-path errors. | The focused unit plus lint reruns execute against the live source surface and the existing `src/fixtures.ts` debt is resolved without outside-slice failures. |
| CFCP-018                  | architecture-closed / proof-required / worker-boot-blocked    | WP03 is a historical worker-entrypoint record. The current tracked checkout still has unrun scoped unit and integration reruns against the live generated billing-contract source surface, so live fetch-path, CORS, request-size, kill-switch, and safe-error behavior are not runtime-proven here. | The scoped unit plus integration reruns execute through the worker entrypoint and confirm the live generated billing-contract surface. |
| CFCP-020                  | architecture-closed / proof-required / contract-runtime-blocked | WP04 is a historical route-manifest and domain-contract record. The current tracked checkout still has unrun contract and integration reruns against the live generated billing-contract source surface, so boot-dependent coverage is not claimed here. | The scoped unit, property, contract, and integration reruns reach live contract and dispatch coverage without widening into billing-domain ownership. |
| CFCP-019                  | architecture-closed / proof-required / storage-binding-blocked | WP06 is a historical storage-binding record. The current tracked checkout still has unrun unit, integration, and property reruns against the live generated billing-contract source surface, and the current packet still carries `src/fixtures.ts` TypeScript return-path lint debt. | The required families rerun against the live source surface and the existing `src/fixtures.ts` debt is resolved. |
| CFCP-012                  | architecture-closed / proof-required / runtime-blocked        | WP07 is a historical local-dev/seeding record. The current tracked checkout still has unrun local-start and seed reruns against the live generated billing-contract source surface. | The scoped workflow probe plus owned integration test rerun without blocked start/seed states. |
| CFCP-013                  | architecture-closed / proof-required / runtime-blocked | WP08 is a historical runner record. The current tracked checkout still has unrun narrowed family reruns against the live generated billing-contract source surface and module lint debt outside the packet. | The focused family commands plus module lint rerun green under `.../08-testing-runner-and-test-pyramid/` and `.../10-security-fuzz-property-observability/`. |
| CFCP-015                  | architecture-closed / proof-required / runtime-blocked        | WP10 is a historical security/property/fuzz/observability record. The current tracked checkout still has unrun scoped family reruns against the live generated billing-contract source surface and carries the existing `src/fixtures.ts` lint debt. | `test:security`, `test:property`, `test:fuzz`, and `test:integration` rerun green without widening into extra Cloudflare suites. |
| CFCP-017                  | architecture-closed / proof-required / consumer-smoke-blocked | WP09 is a historical portal-to-worker smoke record. The current tracked checkout still has unrun e2e reruns against the live generated billing-contract source surface. | `test:e2e` reruns through a booted worker to prove `/auth/billing/status` without widening into checkout, admin, or provider flows. |
| CFCP-016                  | architecture-closed / proof-required / deployment-blocked     | WP11 is a historical deployment and promotion record. The current tracked checkout still has unrun deploy and smoke reruns against the live generated billing-contract source surface, and the configs remain placeholder-backed with manual-required auth/key refs. | Deploy plus post-deploy smoke reruns produce real dev/prod artifacts. |
| CFCP-014                  | architecture-closed / proof-required / payment-blocked | WP12 is the only current WP12 expected output path, but the generated proof is absent/not produced in this checkout. WP01 through WP11 are historical plan references and must not be claimed as present artifacts here. The remaining blockers are unrun focused gates, existing `src/fixtures.ts` TypeScript return-path lint debt, and manual-required account, trusted-device, and deployment states; no downstream payment acknowledgment is recorded here. | The expected output path is generated only when proof is actually produced, and downstream payment acknowledgment lands without widening blocked-state truth into readiness claims. |

## HID Execution Guard

- Scope and completion source:
  - follow `PLAN_EXECUTION_BLUEPRINT.md`, then the assigned workpack and `NEXT_ACTIONS.md`.
  - do not treat file presence, empty scaffolds, or stale docs as runtime correctness.
- Before any checked update, attach:
  - exact runtime or blocker state for the touched slice,
  - a proof pointer under `output/cloudflare-control-plane-plan-proof/`.
- Failure rule: no PR-ready claim until the shared module, auth boundary, storage bindings, and test pyramid each have proof or exact blockers.

## Overclaim boundary

This plan is implementation-present but not completion-ready. Cloudflare correctness remains incomplete until scoped validation stays green, expected proof bundles are actually produced under `output/cloudflare-control-plane-plan-proof/`, and the WP12 handoff path no longer carries historical-plan-root ambiguity, WP02 lint debt, manual-required authority and deployment states, or downstream payment-plan consumption gaps.
