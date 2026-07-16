# cloudflare-control-plane-plan

## Normalized Header

- plan/thread name: `cloudflare-control-plane-plan`
- source thread label: `cloudflare-control-plane-plan`
- source thread id: `019ed327-5d2a-7311-b438-e18475a2a68c`
- worktree: `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent`
- branch: `codex/tracking-plan-full-continuation-a`
- claimed status: `CFCP-A truth-sync coherent; CFCP-B scoped integration green; CFCP-C partial/open; plan not done`
- claimed source files/crates/packages: `infra/cloudflare/src/{index.ts,env.ts,routes.ts,auth/verifier.ts,billing-binding-read-model.ts,fixtures.ts,testing.ts}`, `infra/cloudflare/{wrangler.toml,wrangler.production.toml,.dev.vars.example}`, `infra/cloudflare/docs/{ARCHITECTURE.md,AUTH_BOUNDARY.md,DEPLOYMENT.md,LOCAL_DEV.md,STORAGE_BINDINGS.md,TESTING.md}`, `packages/billing-domain/src/{billing-checkout-portal-boundary.ts,billing-support-admin-api-boundary.ts}`
- claimed tests: `infra/cloudflare/tests/{unit,integration,e2e,contract,property,security,fuzz}` with `worker-health.test.ts` aligned and scoped integration green
- claimed proof commands/artifacts: `npm --prefix infra/cloudflare run test:integration` passing; earlier scoped passes claimed for `test:unit`, `test:contract`, `test:security`, `test:property`, `test:fuzz`, `test:e2e`; canonical proof root `output/cloudflare-control-plane-plan-proof/`; approved payment handoff artifact `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/payment-handoff-proof.md`
- claimed blockers: `queue/dead-letter runtime truth incomplete`, `canonical proof artifacts absent on disk`, `final auth/provider contract depends on account-identity-family-plan`, `trusted-device final proof depends on device-trust-bootstrap-plan`, `deployment/promotion proof depends on real Cloudflare environment ownership`
- claimed next actions: `CFCP-C1 queue/dead-letter plus negative-path security/property/fuzz hardening`, then `proof materialization under output/...`, then auth closure, deployment proof, and payment handoff gate
- obvious missing evidence fields: `output/cloudflare-control-plane-plan-proof/` missing entirely on disk; no real WP01-WP12 proof artifacts; no deployment/promotion artifacts; no final payment handoff artifact
- archival note: self-report only; unverified input for structural review

## Raw Self-Report

**COMPLETION_ARCHITECTURE_REPORT**

Cloudflare control plane is not scaffold-only; it is a mostly implemented monolith under `infra/cloudflare` with real unit/integration/e2e/contract/security/property/fuzz suites. `CFCP-A` is now coherent: plan docs and module docs point at the canonical proof root `output/cloudflare-control-plane-plan-proof/`, and `CFCP-B` is green after aligning [worker-health.test.ts](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/infra/cloudflare/tests/integration/worker-health.test.ts). Final closure is still blocked by three things: no real proof artifacts yet, incomplete `CFCP-C` hardening around queue/dead-letter and negative-path fuzz/security/property coverage, and later auth/deployment/handoff dependencies.

**Plan Closure Definition**

Actually done for this plan means:
- `infra/cloudflare` is honestly documented as the current monolith source of truth, with placeholder subdirs explicitly non-counting.
- Real tests under `infra/cloudflare/tests/{unit,integration,e2e,contract,property,security,fuzz}` cover the required matrix, and scoped commands are green.
- Real proof artifacts exist under `output/cloudflare-control-plane-plan-proof/00-12/...`, including the approved payment gate at `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/payment-handoff-proof.md`.
- Final auth/trusted-device/deployment claims are either genuinely proven or explicitly blocked by named sibling-plan contracts or environment ownership.

**Exact Read Surface**

| Surface | Exact read |
| --- | --- |
| Plan docs | All markdown under `docs/plans/cloudflare-control-plane-plan/`, including `AGENTS.md`, `PLAN_STATE.md`, `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, `PLAN_EXECUTION_BLUEPRINT.md`, `PLAN_EXECUTION_SCORECARD.md`, `PROOF_INDEX.md`, `PROOF_AND_TEST_INVENTORY.md`, `REQUIRED_TEST_ASSERTION_MATRIX.md`, `PARENT_CLOUDFLARE_MODULE_SPEC.md`, `SOURCE_SURFACE_STATUS_MATRIX.md`, `TESTING_STRATEGY.md`, `TEST_PROOF_EXPECTATIONS.md`, `DECISIONS.md`, `GAMES_INFRA_PARITY_MAP.md`, `AUTH_BOUNDARY_MODEL.md`, `ROUTE_MANIFEST_MODEL.md`, `STORAGE_BINDING_MODEL.md`, `LOCAL_DEV_AND_SEEDING_MODEL.md`, `DEPLOYMENT_MODEL.md`, `SECURITY_PRIVACY_OBSERVABILITY.md`, `CHECKLIST_INDEX.md`, and workpacks `00` through `12`. |
| Feature / expectation docs | No direct `docs/features/*` or `docs/expectations/*` references are currently routed by this plan; the plan uses its own model docs above as the feature/expectation source. |
| Owned source | [infra/cloudflare/src/index.ts](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/infra/cloudflare/src/index.ts), `env.ts`, `routes.ts`, `auth/verifier.ts`, `billing-binding-read-model.ts`, `fixtures.ts`, `testing.ts`; `wrangler.toml`, `wrangler.production.toml`, `.dev.vars.example`; module docs `ARCHITECTURE.md`, `AUTH_BOUNDARY.md`, `LOCAL_DEV.md`, `DEPLOYMENT.md`, `STORAGE_BINDINGS.md`, `TESTING.md`. |
| Dependent domain surface | `packages/billing-domain/src/billing-checkout-portal-boundary.ts`, `packages/billing-domain/src/billing-support-admin-api-boundary.ts` as consumed contract boundaries. |
| Tests read | `infra/cloudflare/tests/integration/{admin-auth-rejection,billing-status-auth,checkout-portal-hosted,payment-routes-real,pricing-public,provider-webhooks,reconciliation-auth-boundary,webhook-signature-rejection,worker-health,worker-runtime-real}.test.ts`; `tests/security/{cors-origin-rejection,interactive-billing-boundary,no-provider-secrets-in-client,redaction,request-smuggling}.test.ts`; `tests/property/{billing-idempotency,route-auth-state}.property.test.ts`; `tests/fuzz/provider-webhook-payload.fuzz.test.ts`; plus unit/contract/e2e files during the earlier audit/validation pass. |
| Proof | Canonical root inspected: `output/cloudflare-control-plane-plan-proof/` is currently missing entirely. Legacy-note refs only remain in `PROOF_INDEX.md` and `TEST_PROOF_EXPECTATIONS.md`. |

**Current Truth**

| Surface | Done | Partial | False-green | Missing |
| --- | --- | --- | --- | --- |
| Runtime ownership | `infra/cloudflare` is real, not scaffold-only | Runtime is concentrated in `src/index.ts`, `billing-binding-read-model.ts`, `fixtures.ts` | Placeholder dirs `src/{handlers,flows,providers,queues,observability,storage,durable-objects}` can look larger than they are | Source-shape refactor is intentionally not a closure blocker |
| Docs / plan truth | `CFCP-A` truth-sync is coherent in modified plan/module docs | Legacy warning notes remain intentionally in `PROOF_INDEX.md` and `TEST_PROOF_EXPECTATIONS.md` | Old docs previously implied scaffold/manual-only states | Real proof files are still absent |
| Scoped baseline | `CFCP-B` green: `test:integration` passes | Only integration was rerun at this checkpoint | Previous `worker-health` expectation was stale | No persisted proof artifact for the passing run |
| Security / property / fuzz / queue | Families exist with real files | Coverage is thinner than the required matrix on dead-letter, queue failure, negative fuzz, and some smuggling edges | Reconciliation counters and dead-letter ownership are documented more strongly than runtime fallback behavior | `CFCP-C` implementation/proof slice still open |
| Deployment / promotion | Wrangler config and env contract exist | Commands are defined | Config presence is not deployment proof | Real resource IDs, secrets, rollback, promoted smoke, artifact capture |
| Payment handoff | Approved target artifact path is known | Downstream dependency is explicit | Payment cannot infer readiness from source/tests alone | `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/payment-handoff-proof.md` |

**Code Surface And Ownership**

| Surface | Owner truth |
| --- | --- |
| `infra/cloudflare/src/index.ts` | Main worker entrypoint, route handlers, queue helper, health route, auth enforcement, idempotent DO behavior. |
| `infra/cloudflare/src/billing-binding-read-model.ts` | Local binding-backed billing state, audit, reconciliation visibility, seed-backed reads/writes. |
| `infra/cloudflare/src/fixtures.ts` | Seed fixtures, summary builders, local-safe runtime data. |
| `infra/cloudflare/src/env.ts` | Binding validation and missing-binding detection. |
| `infra/cloudflare/src/routes.ts` | Manifest ownership and auth-state route classification. |
| `infra/cloudflare/src/auth/verifier.ts` | Public/parent/trusted-device/admin/support/provider/internal auth boundary. |
| `infra/cloudflare/src/testing.ts` | Harness, queue recorders, local DO/queue env setup. |
| `packages/billing-domain/src/*` | External schema/contracts consumed by this plan; not owned here. |
| Placeholder dirs | `src/handlers`, `flows`, `providers`, `queues`, `observability`, `storage`, `durable-objects` are README-only and must not be counted as implementation. |

**Test Surface Inventory**

| Category | Current files | Status |
| --- | --- | --- |
| `unit` | Real suite under `infra/cloudflare/tests/unit` | Applicable and present |
| `integration` | 10 real files under `infra/cloudflare/tests/integration` | Applicable and present; green at current checkpoint |
| `e2e` | `infra/cloudflare/tests/e2e/portal-to-worker-billing-status.test.ts` | Applicable and present |
| `contract` | `infra/cloudflare/tests/contract/billing-api-contract.test.ts` | Applicable and present |
| `property` | `route-auth-state.property.test.ts`, `billing-idempotency.property.test.ts` | Applicable and present, but thin on queue/dead-letter invariants |
| `security` | 5 real files under `infra/cloudflare/tests/security` | Applicable and present, but request-smuggling coverage is narrower than matrix intent |
| `fuzz` | `provider-webhook-payload.fuzz.test.ts` | Applicable and present, but mostly happy-path signed payload variation today |
| `load` | None | Not a current closure blocker; `TESTING_STRATEGY.md` marks load/k6 as later rollout gate |
| Empty / misplaced tests | None found in major top-level categories; no `src/**/*.test.ts` in owned module | No move required |

**Proof Inventory**

| Proof surface | State |
| --- | --- |
| Canonical root `output/cloudflare-control-plane-plan-proof/` | Missing on disk |
| WP00-WP11 artifacts | Missing |
| WP12 handoff artifact | Missing |
| Legacy `docs/proof/...` references | Stale as live targets; now retained only as explicit legacy-warning notes in `PROOF_INDEX.md` and `TEST_PROOF_EXPECTATIONS.md` |
| Current real evidence | In-thread command outputs only; not yet materialized into proof files |

**Scoped Validation Inventory**

| Command | State | Notes |
| --- | --- | --- |
| `npm --prefix infra/cloudflare run test:integration` | Passes now | Previously failed only on `worker-health`; fixed by aligning optional binding expectation |
| `npm --prefix infra/cloudflare run test:unit` | Passed earlier in this thread | Not rerun after docs-only changes; no runtime change since |
| `npm --prefix infra/cloudflare run test:contract` | Passed earlier in this thread | Not rerun at current checkpoint |
| `npm --prefix infra/cloudflare run test:security` | Passed earlier in this thread | Still needs deeper `CFCP-C` coverage |
| `npm --prefix infra/cloudflare run test:property` | Passed earlier in this thread | Still needs deeper queue invariants |
| `npm --prefix infra/cloudflare run test:fuzz` | Passed earlier in this thread | Still needs negative-path expansion |
| `npm --prefix infra/cloudflare run test:e2e` | Passed earlier in this thread | Not rerun at current checkpoint |
| `npm --prefix infra/cloudflare run lint` | Unrun in this checkpoint | Cheap and should run when `CFCP-C` code/tests change |
| `npm run lint:architecture -- --files ...` | Unrun in this checkpoint | Required before completion for touched TS surfaces |
| `npm --prefix infra/cloudflare run deploy:dev` / `deploy` | Unrun | Needs real env ownership and is not part of current cheap local checkpoint |

**Dependency Graph**

| Bucket | Dependency | Exact impact |
| --- | --- | --- |
| `local-now` | `CFCP-C` queue/dead-letter/security/property/fuzz hardening inside `infra/cloudflare` | Can proceed immediately without external auth/device-trust decisions |
| `needs-coordinator-sequencing` | Real Cloudflare environment ownership for `wrangler` deploy/promotion, secret injection, rollback smoke | Blocks WP11 proof and therefore final WP12 handoff |
| `needs-sibling-plan-contract` | `account-identity-family-plan` | Needed to replace adapter/manual-required auth-provider truth with owned account/session authority |
| `needs-sibling-plan-contract` | `device-trust-bootstrap-plan` | Needed for final trusted-device proof on `/auth/billing/entitlement-snapshot` and `/auth/billing/license-check` |
| `needs-sibling-plan-contract` | `payment-subscription-plan` | Downstream consumer; not a blocker for `CFCP-C`, but required to consume final WP12 handoff |
| `host-platform-limited` | Apple-native proof | Not a core blocker for this plan; only relevant if downstream Apple purchase/native consumer proof is demanded |

**Platform Feasibility**

| Platform | What can be proven |
| --- | --- |
| Windows host now | All docs work, `npm` scoped validation, Wrangler-local runtime, unit/integration/e2e/contract/security/property/fuzz, architecture lint |
| WSL / Docker | Optional extra Linux-shaped proof for local runtime/deploy tooling if wanted; not required to unblock `CFCP-C` |
| Android Studio / device | Only relevant for downstream consumer handoff verification, not core Cloudflare runtime closure |
| Apple-host-only | None for core worker/module closure; only native Apple consumer/provider edge proof if explicitly required later |

**Ordered Slices From Here**

| Slice | Files / domains | Validation | Proof to collect | Exit criteria |
| --- | --- | --- | --- | --- |
| 1. `CFCP-C1` queue and dead-letter runtime truth | `infra/cloudflare/src/index.ts`, `src/testing.ts`, `tests/integration/reconciliation-auth-boundary.test.ts` or a new queue-focused integration file, `tests/property/billing-idempotency.property.test.ts`, `tests/security/request-smuggling.test.ts`, `tests/fuzz/provider-webhook-payload.fuzz.test.ts` | `test:integration`, `test:security`, `test:property`, `test:fuzz`, `lint`, architecture lint on touched files | `output/cloudflare-control-plane-plan-proof/10-security-fuzz-property-observability/` | Dead-letter behavior is either really implemented and tested or honestly downgraded in docs/contracts; no fake queue claim remains |
| 2. `CFCP-C2` proof materialization for current green scope | `output/cloudflare-control-plane-plan-proof/01-10/...`, plus any small proof index/doc touch needed | No new broad validation; reuse passing scoped commands | Populate WP01-WP10 artifact roots with command logs, assertion IDs, no-claim boundaries | Canonical proof root exists on disk and records real evidence for A/B/C slices |
| 3. `CFCP-D` auth/provider contract closure | `infra/cloudflare/src/auth/verifier.ts`, `AUTH_BOUNDARY_MODEL.md`, auth/security/integration tests | `test:unit`, `test:integration`, `test:security`, architecture lint | WP05 and WP09 proof artifacts | Manual-required adapter language is reduced to only genuinely external contract edges |
| 4. `CFCP-E` deployment/promotion proof | `wrangler.toml`, `wrangler.production.toml`, `.dev.vars.example`, `DEPLOYMENT.md` | `deploy:dev`, `deploy`, promoted smoke only | `output/cloudflare-control-plane-plan-proof/11-deployment-and-environment-promotion/` | Real envs, rollback notes, and promoted smokes are captured |
| 5. `CFCP-F` payment handoff gate | `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/payment-handoff-proof.md`, `payment-subscription-plan` WP00 pointer | Aggregate only; no repo-wide validation | Final handoff artifact at approved path | Payment plan can consume Cloudflare truth without inference or hidden blockers |

**Blocker Taxonomy**

- `local-now`
  - `CFCP-C` queue/dead-letter behavior is underimplemented relative to docs.
  - Fuzz/security/property suites need negative-path expansion to match the matrix.
  - Canonical proof root exists only in docs; no on-disk proof artifacts yet.

- `needs-coordinator-sequencing`
  - Real Cloudflare deployment/promotion proof requires environment owner, resource IDs, secret custody, and rollback path sequencing.

- `needs-sibling-plan-contract`
  - `account-identity-family-plan` must define the real account/session authority adapter.
  - `device-trust-bootstrap-plan` must define the trusted-device authority source.
  - `payment-subscription-plan` must consume the final WP12 handoff after Cloudflare proof is complete.

- `host-platform-limited`
  - No core blocker on this Windows host.
  - Apple-native proof is only downstream/native-surface work, not worker-plan closure.

**First Coordinator Ask**

Move `CFCP-C1` first on this lane. It is the highest-yield local slice because it can eliminate the current false-green around queue/dead-letter and deepen the weakest real coverage without waiting on any sibling plan. Do not sequence `account-identity-family-plan` or `device-trust-bootstrap-plan` ahead of `CFCP-C1`; they matter for final auth/trusted-device closure, not for the current honest local hardening.

**Strict Done Bar**

Before this plan can ever be marked done:
- `infra/cloudflare` docs, source, and tests all agree on the current monolith truth.
- `output/cloudflare-control-plane-plan-proof/` exists with real WP01-WP12 artifacts.
- Scoped `unit`, `integration`, `e2e`, `contract`, `security`, `property`, and `fuzz` commands are green for the final touched surfaces; `load` remains optional unless the plan definition changes.
- Queue/dead-letter, reconciliation visibility, auth boundary, and redaction claims are backed by real tests, not just contracts/docs.
- Deployment/promotion proof is real.
- `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/payment-handoff-proof.md` exists and is the artifact consumed by payment.

**COORDINATOR_DECISION_REQUEST**

- recommended next slice: `CFCP-C1` queue/dead-letter plus negative-path security/property/fuzz hardening
- recommended predecessor plans: none before `CFCP-C1`; after that, `account-identity-family-plan` and `device-trust-bootstrap-plan` should move before final auth closure, and deployment environment ownership must be sequenced before WP11/WP12
- estimated risk: medium
- estimated proof difficulty: medium-high because proof root is entirely absent and deployment/handoff proof is still zero-state
- whether I should continue immediately or pause for sequencing: continue immediately on `CFCP-C1`; pause only before auth finalization or deployment promotion if coordinator has a different sequencing preference

## Optional Addendum

- Earlier audit passes established that most runtime behavior is concentrated in `infra/cloudflare/src/index.ts`, `infra/cloudflare/src/billing-binding-read-model.ts`, and `infra/cloudflare/src/fixtures.ts`, while `infra/cloudflare/src/{handlers,flows,providers,queues,observability,storage,durable-objects}` remain mostly README-only placeholder surfaces that must not be counted as implementation.
- Earlier audit and follow-up trace work also established a concrete queue gap behind the broader `CFCP-C` summary: `queueReconciliationEvent` in `infra/cloudflare/src/index.ts` currently sends only to `BILLING_RECONCILIATION_QUEUE` and has no real `BILLING_DEAD_LETTER_QUEUE` fallback path, even though plan docs and contracts claim dead-letter ownership and visibility.
- Earlier audit passes found the canonical proof root `output/cloudflare-control-plane-plan-proof/` missing entirely on disk; all current evidence is still in-thread command output rather than materialized proof artifacts.
