# Cloudflare Control Plane Plan State

Status: engineering-grade Cloudflare control-plane spec is complete. The current tracked checkout contains the committed and current-head-validated WP07 packet under `output/cloudflare-control-plane-plan-proof/07-local-dev-seeding-and-fixtures/`; WP00-WP06 and WP08-WP12 proof roots remain absent. WP12 has a defined expected output path under `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/payment-handoff-proof.md`, but its generated proof is absent/not produced. The checked-in TS sidecar is the operational billing-contract source surface consumed by Cloudflare, `crates/schema` currently reads/exposes that sidecar through a reader/contract-test bridge, and Rust-first canonical generation/ownership remains an explicit migration gap/blocker. Current execution is green across the local dev, test, deploy dry-run, and workflow probe surfaces, while WP07 acceptance/checklist consumption, the other absent proof artifacts and custody gates, production deployment, WP12 handoff, downstream payment acceptance, and manual authority states remain open.

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

- Current tracked checkout output inventory: the committed and current-head-validated WP07 packet is present under `output/cloudflare-control-plane-plan-proof/07-local-dev-seeding-and-fixtures/`. WP00-WP06 and WP08-WP12 proof roots are absent. WP07 branch acceptance and owning checklist/index consumption remain open; its presence does not close production deployment, WP12 handoff, downstream payment acceptance, or manual authority gates.
- Historical plan records retained without present proof roots: WP00-WP06 and WP08-WP11. They are plan-history only and not present artifacts in the current checkout output tree; the committed WP07 packet is the explicit exception recorded below.
- WP00 historical record: parity extraction history; closure remains blocked by repo-wide `npm run format:check` drift outside that historical packet.
- WP01 historical record: module scaffold history; current checkout execution is green and the remaining gap is the absent proof artifact.
- WP02 historical record: wrangler/binding history; current checkout execution is green and the remaining gap is the absent proof artifact. Current module lint truth passed at head `c121ba5eb`, so no current Cloudflare module lint blocker is claimed here.
- WP03 historical record: worker-entrypoint guard history; current checkout execution is green and the remaining gap is the absent proof artifact.
- WP04 historical record: route-manifest and contract history; current checkout execution is green and the remaining gap is the absent proof artifact.
- WP05 historical record: auth/admin/support boundary history; current checkout execution is green and the remaining gap is the absent proof artifact.
- WP06 historical record: storage binding history; current checkout execution is green and the remaining gap is the absent proof artifact. Current module lint truth passed at head `c121ba5eb`, so no current Cloudflare module lint blocker is claimed here.
- WP07 current record: the standalone preflight/import and positive-count seed path are green, the existing logger bridge persists/asserts a five-milestone redacted proof chain, the separate real Wrangler integration suite is green, and the canonical WP07 packet is committed and current-head validated on this branch. Owning checklist/index consumption, runtime deployment, WP12 handoff, and downstream acceptance remain open.
- WP08 historical record: test runner and test pyramid history, with current execution green and only the proof artifact absent from the checkout output tree; current module lint truth passed at head `c121ba5eb`, so no current Cloudflare module lint blocker is claimed here.
- WP09 historical record: portal-to-worker smoke history, with current execution green and only the proof artifact absent from the checkout output tree.
- WP10 historical record: security, property, fuzz, and observability history, with current execution green and only the proof artifact absent from the checkout output tree; current module lint truth passed at head `c121ba5eb`, so no current Cloudflare module lint blocker is claimed here.
- WP11 historical record: deployment and promotion history; current checkout execution is green across deploy dry-run and smoke surfaces, and the remaining gap is the absent proof artifact plus placeholder/manual-required config truth.
- WP12 current record: expected output path `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/payment-handoff-proof.md`; generated proof is absent/not produced in this checkout; the remaining blockers are the absent proof artifact, account/trusted-device/deployment custody states, and no downstream payment acknowledgment being recorded here; current module lint truth passed at head `c121ba5eb`, so no current Cloudflare module lint blocker is claimed here.
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
| CFCP-001 through CFCP-011 | architecture-closed / runtime-present / proof-required | The shared module, guard chain, route surface, auth boundary, storage model, and local runtime exist in source. WP00-WP06 and WP08-WP11 remain historical plan records without present proof roots; the committed WP07 packet is tracked separately by CFCP-012. | `infra/cloudflare/` source, module docs, plan docs, and output-proof expectations agree on current runtime truth and remaining no-claim boundaries. |
| CFCP-021                  | architecture-closed / execution-green / proof-required | WP02 is a historical wrangler/binding record. Current checkout execution is green and the remaining gap is the absent proof artifact; no current Cloudflare module lint blocker is claimed here. | The focused unit plus lint reruns agree with the current checkout, and proof artifact absence remains the open item. |
| CFCP-018                  | architecture-closed / execution-green / proof-required | WP03 is a historical worker-entrypoint record. Current checkout execution is green and the remaining gap is the absent proof artifact. | The scoped unit plus integration reruns confirm worker entrypoint behavior; proof artifact remains open. |
| CFCP-020                  | architecture-closed / execution-green / proof-required | WP04 is a historical route-manifest and domain-contract record. Current checkout execution is green and the remaining gap is the absent proof artifact. | The scoped unit, property, contract, and integration reruns reach live contract and dispatch coverage without widening into billing-domain ownership. |
| CFCP-019                  | architecture-closed / execution-green / proof-required | WP06 is a historical storage-binding record. Current checkout execution is green and the remaining gap is the absent proof artifact; no current Cloudflare module lint blocker is claimed here. | The required families rerun against the current checkout, and the proof artifact remains open. |
| CFCP-012                  | architecture-closed / execution-proven / proof-produced / acceptance-open | WP07 now separates `preflightStatus = ready`, `importCheckStatus = passed`, and standalone `runtimeBootStatus = unproven`; `seed.status = runnable` is backed by positive counts; the structured proof chain is persisted/asserted; and the separate real Wrangler integration suite passes. The WP07 packet is produced on this branch, while production deployment and WP12/downstream acceptance remain open. | Accept the branch packet through the owning checklist/index without promoting local preflight or local runtime evidence into production/payment readiness. |
| CFCP-013                  | architecture-closed / execution-green / proof-required | WP08 is a historical runner record. Current checkout execution is green across the owned families; proof artifact remains absent. | The focused family commands stay green; proof artifact remains open. |
| CFCP-015                  | architecture-closed / execution-green / proof-required | WP10 is a historical security/property/fuzz/observability record. Current checkout execution is green across the owned families; proof artifact remains absent. | `test:security`, `test:property`, `test:fuzz`, and `test:integration` rerun green without widening into extra Cloudflare suites. |
| CFCP-017                  | architecture-closed / execution-green / proof-required | WP09 is a historical portal-to-worker smoke record. Current checkout execution is green across the owned e2e family; proof artifact remains absent. | `test:e2e` reruns through a booted worker to prove `/auth/billing/status` without widening into checkout, admin, or provider flows. |
| CFCP-016                  | architecture-closed / execution-green / proof-required | WP11 is a historical deployment and promotion record. Current checkout execution is green across the deploy dry-run and smoke surface; proof artifact remains absent. | Deploy plus post-deploy smoke reruns produce real dev/prod artifacts. |
| CFCP-014                  | architecture-closed / execution-green / payment-proof-required | WP12 is the only current WP12 expected output path, but the generated proof is absent/not produced in this checkout. WP01-WP06 and WP08-WP11 remain historical plan references without present proof roots; the committed WP07 packet does not satisfy WP12 or downstream payment acceptance. The remaining blockers are the absent WP12 artifact plus the manual-required account, trusted-device, deployment, and downstream payment acknowledgment states; no current Cloudflare module lint blocker is claimed here. | The expected output path is generated only when proof is actually produced, and downstream payment acknowledgment lands without widening blocked-state truth into readiness claims. |

## HID Execution Guard

- Scope and completion source:
  - follow `PLAN_EXECUTION_BLUEPRINT.md`, then the assigned workpack and `NEXT_ACTIONS.md`.
  - do not treat file presence, empty scaffolds, or stale docs as runtime correctness.
- Before any checked update, attach:
  - exact runtime or blocker state for the touched slice,
  - a proof pointer under `output/cloudflare-control-plane-plan-proof/`.
- Failure rule: no PR-ready claim until the shared module, auth boundary, storage bindings, and test pyramid each have proof or exact blockers.

## Overclaim boundary

This plan is implementation-present but not completion-ready. Cloudflare correctness remains incomplete until scoped validation stays green, expected proof bundles are actually produced under `output/cloudflare-control-plane-plan-proof/`, and the WP12 handoff path no longer carries historical-plan-root ambiguity, manual-required authority and deployment states, or downstream payment-plan consumption gaps.
