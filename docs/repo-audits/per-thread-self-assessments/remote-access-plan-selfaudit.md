# remote-access-plan

## Normalized Header

- plan/thread name: `remote-access-plan`
- source thread label: `remote-access-plan`
- source thread id: `019ed32b-be9e-7c01-8fc9-b55b73b83983`
- worktree: `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent`
- branch: `codex/tracking-plan-full-continuation-a`
- claimed status: `completion architecture report prepared; execution held for coordinator sequencing; first approved execution slice is ra-01-contract-parity-and-test-repair`
- claimed source files/crates/packages: `packages/remote-access-domain`, `crates/remote-access-core`, `packages/family-domain`, `packages/child-runtime-domain`, `crates/child-runtime`, `packages/agent-protocol-domain`, `crates/agent-protocol`, `crates/agent-service`, `packages/portal-domain`, `apps/portal`, adjacent `packages/screen-domain`, `crates/screen-live-view-core`
- claimed tests: existing `packages/remote-access-domain/tests/unit/*`, `crates/remote-access-core/tests/unit/*`, `crates/child-runtime/tests/unit/runtime_gate.rs`, `crates/child-runtime/tests/integration/tracking_runtime_flow_intent.rs`; missing major remote categories include `contract`, `integration`, `property`, `security`, `playwright`, `load`
- claimed proof commands/artifacts: `npm run test --workspace @ocentra-parent/remote-access-domain` pass; `cargo test -p ocentra-remote-access-core` fail; `cargo test -p ocentra-child-runtime runtime_gate` fail; canonical proof roots claimed as `docs/proof/remote-access-plan/*` and `output/remote-access-plan-proof/<workpack-file-stem>/`; required scripts claimed missing
- claimed blockers: coordinator sequencing ahead of execution, sibling-plan contract clarification for LAN/account/session/device-trust/logging-proof surfaces, local remote protocol/service/portal/proof surfaces still missing
- claimed next actions: wait for coordinator sequencing, then execute `ra-01-contract-parity-and-test-repair` with exact path claims and scoped validation
- obvious missing evidence fields: real remote protocol/service modules, real portal remote UI, remote proof roots/scripts, remote integration/security/load/playwright coverage, Windows/Android/Linux proof artifacts
- archival note: self-report only; unverified input for structural review

## Raw Self-Report

**COMPLETION_ARCHITECTURE_REPORT**

Executive summary: `remote-access-plan` is not a thin polish pass. Current repo truth is a partial contract layer plus adjacent live-view infrastructure, with no real remote session fabric, no real portal surface, no remote proof root, and no parity between TS and Rust models. The accepted completion model is still correct: canonical TS home is `packages/remote-access-domain`, current scope is `view-only + standing paired access`, remote control stays explicitly deferred, and the first execution slice should be `ra-01-contract-parity-and-test-repair` after higher-leverage dependency work lands.

**Plan Closure Definition**
“Actually done” for this plan means all of the following are true on this exact repo/branch shape:

- A single remote-access model exists across `packages/remote-access-domain` and `crates/remote-access-core` for route, capability, standing grant, session, expiry, revoke/remove-device, actor/device scope, and audit refs.
- Parent-facing remote access is a real portal surface, not a placeholder route or reused data-custody copy.
- Protocol and service layers implement real `agent.remote.*` flows for view-only sessions and standing paired access.
- Child/runtime gating enforces wrong-household, wrong-role, revoked, removed-device, expired, missing-trust-handoff, and transport-unavailable states.
- Remote control remains explicitly deferred in code, UI, tests, proof, and copy.
- Tests are real and live under proper `tests/` major categories where applicable.
- Proof is real under the canonical remote proof roots, with Windows/Android/Linux evidence where relevant and no UI-only or LAN-only overclaim.

**Exact Docs / Source / Tests / Proof Read**
- Repo/router docs: `.ocentra-ai/rules/ocentra-parent-rules.mdc`, `.ocentra-ai/rules/ocentra-parent-validation.mdc`, `.ocentra-ai/rules/ocentra-parent-source-shape.mdc`, `.ocentra-ai/rules/ocentra-parent-domain-boundaries.mdc`, `docs/agent/TASK_ROUTER.md`, `docs/agent/WORKER_LANE_FLOW.md`, `docs/agent/WORKTREE_LANE_START.md`, `docs/agent/PLAN_WORKER_FLOW.md`, `docs/agent/HUB_LEDGER_MESSAGING.md`, `docs/PLAN_INDEX.md`.
- All `remote-access-plan` docs: `docs/plans/remote-access-plan/AGENTS.md`, `ARCHIVE_INDEX.md`, `CHECKLIST_INDEX.md`, `DECISIONS.md`, `DOC_INDEX.md`, `NEXT_ACTIONS.md`, `PARENT_REMOTE_UI_EXPECTATIONS.md`, `PLAN_EXECUTION_BLUEPRINT.md`, `PLAN_HEALTH.md`, `PLAN_STATE.md`, `PROOF_AND_TEST_INVENTORY.md`, `PROOF_INDEX.md`, `README.md`, `RELAY_AND_ABUSE_MODEL.md`, `REMOTE_SESSION_AND_ROUTE_MODEL.md`, `REMOTE_VIEW_AND_CONTROL_BOUNDARY.md`, `RESEARCH_AND_UI_GUIDANCE.md`, `ROUTE_INDEX.md`, `SESSION_PAIRING_DISCLOSURE_MODEL.md`, `TEST_PROOF_EXPECTATIONS.md`, `WORKPACK_INDEX.md`, and all six files in `docs/plans/remote-access-plan/workpacks/`.
- Named adjacent docs: `docs/features/screen-visibility-live-view.md`, `docs/features/remote-lan-mobile-platforms.md`, `docs/expectations/screen-evidence.md`, `cloud.md`, `platforms.md`, `static-analysis-security.md`, `tamper-uninstall-protection.md`, `docs/architecture/remote-capability-fabric-v2-plan.md`, `docs/architecture/rustdesk_remote_capabilities_first_pass.md`, `docs/roadmaps/roadmap-v2-parent-owned-remote-access-cloud-relay.md`, `docs/plans/data-custody-storage-plan/AGENTS.md`, `docs/plans/account-identity-family-plan/AGENTS.md`, `docs/plans/portal-ux-household-surfaces-plan/AGENTS.md`, `docs/plans/account-identity-family-plan/workpacks/03-session-token-lifecycle.md`, `05-device-ownership-authz.md`, `docs/plans/screen-plan/workpacks/28-live-view-optional-mode.md`, `39-redacted-summary-only-remote-boundary.md`.
- Source/tests read: `packages/remote-access-domain/src/remote-access-session.ts`, `packages/remote-access-domain/tests/unit/remote-access-session.test.ts`, `package-info.test.ts`, `crates/remote-access-core/src/remote_access_session.rs`, `crates/remote-access-core/tests/unit/session.rs`, `session_authorization.rs`, `packages/family-domain/src/household-authority.ts`, `packages/child-runtime-domain/src/child-runtime-gates.ts`, `crates/child-runtime/src/runtime_gate.rs`, `crates/child-runtime/tests/unit/runtime_gate.rs`, `crates/child-runtime/tests/integration/tracking_runtime_flow_intent.rs`, `packages/portal-domain/src/routes.ts`, `parent-portal-data.ts`, `parent-portal-nav.ts`, `parent-portal-guide-privacy.ts`, `apps/portal/src/portal-route-content.ts`, `portal-state.ts`, `transport.ts`, `ParentPortalRoute.tsx`, `packages/agent-protocol-domain/src/defaults.ts`, `contracts.ts`, `security.ts`, `crates/agent-protocol/src/transport.rs`, `tests.rs`, `route_tests.rs`, `crates/agent-service/src/service_runtime.rs`, `screen_ai_service_event_subscription.rs`, `screen_ai_service_event_subscription/live_view_runtime.rs`, `live_view_service_runtime.rs`, `live_view_service_runtime_tests.rs`, `live_view_worker.rs`, `packages/screen-domain/src/screen-live-view-platform-permission.ts`, `screen-live-view-service-session.ts`, `screen-live-view-parent-ui-persistence.ts`, `crates/screen-live-view-core/src/live_view_runtime.rs`, `live_view_worker.rs`.
- Proof surfaces read/checked: `scripts/test/screen-live-view-platform-permission-proof.mjs`, `scripts/test/screen-live-view-relay-cache-proof.mjs`, and existence checks for `docs/proof/remote-access-plan`, `output/remote-access-plan-proof`, `output/screen-plan-proof`.

**Current Truth**

| State | Surface | Exact evidence |
|---|---|---|
| `done` | Authority split between view and control exists | `packages/family-domain/src/household-authority.ts` already distinguishes `StartRemoteView` vs `StartRemoteControl`; this is the right boundary and should remain. |
| `done` | Basic route primitive exists | `crates/agent-protocol/src/transport.rs` and `packages/agent-protocol-domain/src/defaults.ts` already know `localhost`, `local-network`, `cloud-relay`. |
| `partial` | TS remote contract | `packages/remote-access-domain/src/remote-access-session.ts` models only consent/transport/decision; no route kind, household/device scope, standing grant, revoke/remove-device, expiry, or audit refs. |
| `partial` | Rust remote core | `crates/remote-access-core/src/remote_access_session.rs` models relay/replay/input authority and effects, but does not match TS and still omits the full plan contract. |
| `partial` | Runtime gate reuse | `packages/child-runtime-domain/src/child-runtime-gates.ts` and `crates/child-runtime/src/runtime_gate.rs` reuse remote authorization state, but only as admission gating, not full remote session lifecycle. |
| `partial` | Portal route shell | `packages/portal-domain/src/routes.ts`, `parent-portal-data.ts`, `apps/portal/src/portal-route-content.ts`, `apps/portal/src/transport.ts` expose a placeholder `RemoteAccess` route with no real remote session/view UI. |
| `false-green` | Passing TS unit tests | `npm run test --workspace @ocentra-parent/remote-access-domain` is green, but covers only a trivial contract and could be misread as product progress. |
| `false-green` | Passing screen live-view tests | `screen-domain` and `screen-live-view-core` tests are real, but they prove adjacent screen-plan behavior, not remote-access-plan closure. |
| `false-green` | Optimistic plan wording | `docs/plans/remote-access-plan/PLAN_HEALTH.md` and `PLAN_STATE.md` imply an execution-grade live-view route and proof-backed closure path; the actual proof roots and remote implementation are absent. |
| `missing` | Remote protocol family | No real `agent.remote.route.*`, `agent.remote.capability.*`, `agent.remote.session.*`, `agent.remote.screen.live-view.request` family exists in `packages/agent-protocol-domain` or `crates/agent-protocol`. |
| `missing` | Remote service fabric | `crates/agent-service/src/remote_route.rs`, `remote_session.rs`, `remote_capability.rs`, `remote_audit.rs`, `remote_relay_client.rs` do not exist. |
| `missing` | Portal remote product surface | No real remote-access route panels or parent UX flow exist in `apps/portal/src`. |
| `missing` | Remote proof roots and scripts | `docs/proof/remote-access-plan` and `output/remote-access-plan-proof` do not exist; `scripts/test/remote-capability-contract-proof.mjs`, `remote-relay-loopback-proof.mjs`, `remote-access-portal-proof.mjs` do not exist. |

**Code Surface And Ownership**

| Owner surface | Exact files/directories | What this plan should own |
|---|---|---|
| Canonical TS contract | `packages/remote-access-domain/src` | Route, capability, standing grant, session, revoke/remove-device, audit contracts. |
| Canonical Rust parity | `crates/remote-access-core/src/remote_access_session.rs` | Same model and state machine as TS, plus Rust-side contract enforcement and effect planning. |
| Authz dependency | `packages/family-domain/src/household-authority.ts` | Consumed authority contract for actor role/device scope; remote plan should not replace it. |
| Runtime gate dependency | `packages/child-runtime-domain/src/child-runtime-gates.ts`, `crates/child-runtime/src/runtime_gate.rs` | Start/deny runtime behavior based on remote grant/session states. |
| Protocol surface | `packages/agent-protocol-domain/src/contracts.ts`, `security.ts`, `defaults.ts`, `crates/agent-protocol/src/` | Remote command/event contract and serialization. |
| Service/orchestration | `crates/agent-service/src` | Route selection, session orchestration, relay client, audit emission, redaction, failure handling. |
| Parent portal domain | `packages/portal-domain/src/routes.ts`, `parent-portal-data.ts`, `parent-portal-nav.ts`, likely new remote-access panel/read-model files | Route metadata, labels, route state, copy-state, no-overclaim boundary. |
| Parent portal app | `apps/portal/src/portal-route-content.ts`, `portal-state.ts`, `transport.ts`, new remote route panels | Real remote-access UX and integration. |
| Adjacent dependency only | `packages/screen-domain`, `crates/screen-live-view-core` | Live-view/capture/protected-surface primitives; not the owner of remote product flow. |

**Test Surface Inventory**

| Surface | Existing test inventory | Issues | Missing major categories actually applicable |
|---|---|---|---|
| `packages/remote-access-domain` | `tests/unit/remote-access-session.test.ts`, `package-info.test.ts` | Unit-only, shallow, no parity with Rust | `contract`, `integration`, `property` |
| `crates/remote-access-core` | `tests/unit/session.rs`, `session_authorization.rs` | Compile currently fails; no parity or invariant coverage | `contract`, `property` |
| `crates/child-runtime` | `tests/unit/runtime_gate.rs`, `tests/integration/tracking_runtime_flow_intent.rs` | Compile drift; remote-specific integration is not isolated | `security` where remote denial paths matter |
| `crates/agent-service` | No remote-access tests | Adjacent inline file `src/screen_ai_service_event_subscription/live_view_service_runtime_tests.rs` exists, but is screen-only and should not be copied for remote work | `unit`, `integration`, `security`, `load` |
| `packages/portal-domain` / `apps/portal` | No remote-access tests | No route/state integration coverage; no remote UI proof harness | `unit`, `integration`, `playwright` |
| Proof/test roots | No remote-specific `tests/` scaffolds beyond above | No empty remote test folders found; the bigger issue is missing categories, not empty folders | `e2e/playwright`, `load` where relay behavior is claimed |

Remote-specific inline or `src` tests that must move: none found, because remote-access tests mostly do not exist yet. Adjacent inline tests in `crates/agent-service/src/screen_ai_service_event_subscription/live_view_service_runtime_tests.rs` are a warning, not a migration requirement for this plan. Any new remote service tests should live under `crates/agent-service/tests/{unit,integration,security,load}`.

**Proof Inventory**

| Status | Evidence | Notes |
|---|---|---|
| `real-adjacent` | `scripts/test/screen-live-view-platform-permission-proof.mjs`, `scripts/test/screen-live-view-relay-cache-proof.mjs` | Real scripts exist, but they belong to screen-plan-adjacent proof, not remote-access-plan closure. |
| `stale-or-misleading` | `docs/plans/remote-access-plan/PLAN_HEALTH.md`, `PLAN_STATE.md`, `CHECKLIST_INDEX.md`, `PROOF_AND_TEST_INVENTORY.md` | Status language and checklist shape can be mistaken for progress; the actual remote proof trees are absent. |
| `missing` | `docs/proof/remote-access-plan/` | Required slice manifests named in `PLAN_STATE.md` do not exist. |
| `missing` | `output/remote-access-plan-proof/<workpack-file-stem>/` | Canonical proof root named in `PROOF_INDEX.md` and `PLAN_EXECUTION_BLUEPRINT.md` does not exist. |
| `missing` | `scripts/test/remote-capability-contract-proof.mjs`, `remote-relay-loopback-proof.mjs`, `remote-access-portal-proof.mjs` | Architecture doc names them; repo does not contain them. |

Canonical proof root path:
- `output/remote-access-plan-proof/<workpack-file-stem>/`
- `docs/proof/remote-access-plan/slice-01-*.md`, `slice-02-*.md`, `slice-03-*.md`

**Scoped Validation Inventory**

| Command | Result | Notes |
|---|---|---|
| `npm run test --workspace @ocentra-parent/remote-access-domain` | pass | Real, but narrow and shallow. |
| `npm run test --workspace @ocentra-parent/screen-domain -- screen-live-view` | pass | Adjacent only. |
| `cargo test -p ocentra-screen-live-view-core` | pass | Adjacent only. |
| `cargo test -p ocentra-remote-access-core` | fail | Compile/test drift: private `ocentra_eventing::DomainEvent` import and missing trait-in-scope follow-on errors. |
| `cargo test -p ocentra-child-runtime runtime_gate` | fail | Same eventing privacy drift plus private `EventBus` use in integration coverage. |
| `cargo test -p ocentra-parent-agent-protocol remote` | unrun | Remote protocol family does not exist yet; run once slice 03 lands. |
| `cargo test -p ocentra-parent-agent-service remote` | unrun | Remote service modules do not exist yet. |
| `npm run test --workspace @ocentra-parent/portal -- remote` | unrun | Remote portal surface does not exist yet. |
| `npm run lint:architecture -- --files packages/remote-access-domain packages/agent-protocol-domain packages/portal-domain apps/portal` | unrun | Correct future scoped gate once slices touch these surfaces. |
| `cargo lint-architecture crates/remote-access-core crates/child-runtime crates/agent-protocol crates/agent-service` | unrun | Correct future scoped gate once Rust slices touch these surfaces. |

**Dependency Map**

| Bucket | Exact dependency | Why it matters now vs later |
|---|---|---|
| `can do now` | `packages/remote-access-domain`, `crates/remote-access-core`, current Rust test repair | `ra-01` can define the canonical model and repair existing compile drift without waiting for UI polish. |
| `can do now` | Remote protocol/service scaffolding in `packages/agent-protocol-domain` and `crates/agent-service` | Real remote command/event routing can begin once the canonical model is frozen. |
| `can do now` | Portal remote surface in `packages/portal-domain` and `apps/portal` | Can proceed after protocol/service basics exist; does not require Apple hosts. |
| `needs coordinator/other plan` | LAN truth-sync | Remote route model includes `local-network` vs `cloud-relay`; remote plan should not guess final LAN semantics before that truth-sync lands. |
| `needs coordinator/other plan` | `account-identity-family-plan` contract clarification | Remote standing grants depend on stable actor/role/device/session semantics from `packages/family-domain/src/household-authority.ts` and its owning workpacks. |
| `needs coordinator/other plan` | `device-trust-bootstrap-plan` contract clarification | Remote plan can prove `missing trust handoff blocked` locally, but cannot finalize durable allow-path semantics until trust/step-up contract is stable. |
| `needs coordinator/other plan` | Logging/proof-surface stabilization | Remote proof and redaction claims depend on stable log/event surfaces so proof artifacts are honest and repeatable. |
| `not feasible on this Windows host` | Real iOS runtime proof | Apple-host-only. Current scope does not need it. |
| `not feasible on this Windows host` | Real macOS runtime proof | Apple-host-only. Current scope does not need it. |

**Platform Feasibility**

| Platform path | What can be proven from here |
|---|---|
| Windows host now | TS/Rust contract parity, child/runtime deny paths, portal integration, Windows live-view session UX, revoke/stop visibility, remote diagnostics redaction, local/relay degraded-state UI. |
| Android Studio / emulator / synced Samsung device | Child disclosure banner/session visibility, paired live-view active/stopped states, revoke/remove-device effect on child UX, manual-required states. |
| Linux via WSL / Docker | Relay loopback, replay denial, backpressure/retry storm, cross-household isolation, partial outage behavior, security/load proof. |
| Apple-host-only | Actual macOS/iOS runtime session/disclosure proof if the plan later widens to Apple-native runtime claims. Not required for current scope. |

**Blocker Taxonomy**

| Bucket | Exact items |
|---|---|
| `local-now` | Rust compile drift in `crates/remote-access-core` and `crates/child-runtime`; missing remote proof scripts; missing remote portal surface; missing remote protocol/service modules; missing test categories. |
| `needs-coordinator-sequencing` | Hold execution until the agreed higher-leverage predecessors land; then schedule `ra-01-contract-parity-and-test-repair` with exact path claims. |
| `needs-sibling-plan-contract` | LAN truth-sync, account/session/device authority clarification, trusted-device/step-up handoff, logging/proof-surface stabilization, custody/redaction boundary confirmation. |
| `host-platform-limited` | Real iOS/macOS runtime proof only. |

**No-Hand-Wave Execution Plan**

| Ordered slice | Files/domains to touch | Scoped validation | Proof to collect | Exit criteria |
|---|---|---|---|---|
| `ra-01-contract-parity-and-test-repair` | Expand `packages/remote-access-domain/src/remote-access-session.ts`; add recommended files `packages/remote-access-domain/src/remote-access-route.ts`, `remote-access-capability.ts`, `remote-access-grant.ts`, `remote-access-audit.ts`; align `crates/remote-access-core/src/remote_access_session.rs`; repair `crates/remote-access-core/tests/unit/*.rs`; repair `crates/child-runtime/tests/unit/runtime_gate.rs` as needed for the shared model | `npm run test --workspace @ocentra-parent/remote-access-domain`; `cargo test -p ocentra-remote-access-core`; `cargo test -p ocentra-child-runtime runtime_gate`; scoped architecture gates on these files | `output/remote-access-plan-proof/01-remote-capability-fabric/*`; `docs/proof/remote-access-plan/slice-01-capability-and-grants.md`; negative authz, revoke, remove-device, no-control-overclaim artifacts | TS/Rust parity exists; current Rust drift is fixed; wrong-household/wrong-role/revoked/expired/remove-device states are explicit and tested; remote control remains deferred |
| `ra-02-standing-grant-lifecycle-and-runtime-gate` | `packages/family-domain/src/household-authority.ts`; `packages/child-runtime-domain/src/child-runtime-gates.ts`; `crates/child-runtime/src/runtime_gate.rs`; new tests under `packages/remote-access-domain/tests/contract`, `crates/remote-access-core/tests/property`, `crates/child-runtime/tests/integration` | Repeat slice 01 commands plus focused runtime integration tests | Proof of paired grant, revoke wins, remove-device wins, reconnect-after-revoke denied, missing trust handoff blocked | Standing grant lifecycle is real, runtime denies stale/wrong/revoked states, and trust-handoff absence is enforced honestly |
| `ra-03-remote-protocol-service-route-fabric` | `packages/agent-protocol-domain/src/contracts.ts`, `security.ts`, `defaults.ts`, recommended new `src/remote.ts`; `crates/agent-protocol/src/` command/event surfaces; `crates/agent-service/src/remote_route.rs`, `remote_session.rs`, `remote_capability.rs`, `remote_audit.rs`, `remote_relay_client.rs`, `websocket.rs` | `cargo test -p ocentra-parent-agent-protocol remote`; `cargo test -p ocentra-parent-agent-service remote`; scoped Rust/TS architecture gates | `scripts/test/remote-capability-contract-proof.mjs`; loopback route/session proof; audit/redaction proof | `agent.remote.*` family is real, routed before generic dispatch, and enforces view-only + standing access |
| `ra-04-portal-remote-surface-and-live-view` | `packages/portal-domain/src/routes.ts`, `parent-portal-data.ts`, `parent-portal-nav.ts`, recommended new domain read-model/panel files; `apps/portal/src/portal-route-content.ts`, `portal-state.ts`, `transport.ts`, recommended new `RemoteAccessRoutePanel.tsx`, `RemoteAccessDeviceRoutePanel.tsx`, `RemoteAccessCapabilityRoutePanel.tsx`, `RemoteAccessSessionRoutePanel.tsx`, `RemoteAccessAuditRoutePanel.tsx` | `npm run test --workspace @ocentra-parent/portal -- remote`; scoped `lint:architecture`; existing screen-domain/live-view checks if touched | `scripts/test/remote-access-portal-proof.mjs`; Windows UI artifacts for paired/active/degraded/stopped; Android disclosure artifacts | Parent portal exposes real remote access states, never implies control, and shows revoke/degraded/disclosure truth |
| `ra-05-relay-security-observability-and-rollout-proof` | `crates/agent-service/tests/security`, `tests/load`, relay/client modules, proof scripts, `docs/proof/remote-access-plan/*`, `output/remote-access-plan-proof/05-relay-security-abuse-controls/*`, `06-rollout-proof-and-route-gate/*` | Focused service security/load tests via WSL/Docker; scoped architecture gates; repeat affected protocol/service/portal tests | `scripts/test/remote-relay-loopback-proof.mjs`; rate-limit, replay, cross-household, partial outage, redaction, metrics/alert examples; slice-02/03 manifests | Relay and rollout claims are honest, proof roots exist, no UI-only/LAN-only overclaim remains, and the plan docs point at real artifacts |

**First Coordinator Ask**
Move `account-identity-family-plan` contract clarification first, before scheduling `ra-01`, because the remote grant state machine cannot honestly close durable paired-access allow-paths until actor/role/device/session authority semantics are stable. The exact surface remote plan consumes is already visible in `packages/family-domain/src/household-authority.ts`; what still needs to settle before remote final closure is the sibling-plan contract for session/device ownership freshness and the device-trust/step-up handoff. LAN truth-sync should land before slice 03, and logging/proof-surface stabilization should land before slice 05.

**Strict Done Bar**
Before this plan can ever be marked done, all of the following must be true:

- `packages/remote-access-domain` is the single canonical TS home and matches `crates/remote-access-core`.
- Standing paired access works end to end with explicit revoke/remove-device/expiry/wrong-household/wrong-role denial.
- Remote control is still explicitly deferred in product, contract, proof, and copy.
- Remote tests live in proper `tests/` major categories, not inline `src`, and the applicable categories are present: `unit`, `contract`, `integration`, `property`, `security`, `playwright`, `load`.
- `docs/proof/remote-access-plan/*` and `output/remote-access-plan-proof/*` exist with real artifacts and validation logs.
- Windows, Android, and Linux/WSL/Docker proof exists where relevant; Apple-host-only proof is not implied.
- No placeholder portal route, no fake-green reliance on adjacent screen-plan tests, no stale proof manifests standing in for actual evidence.

**COORDINATOR_DECISION_REQUEST**
- Recommended next slice: `ra-01-contract-parity-and-test-repair`
- Recommended predecessor plans: `account-identity-family-plan` first, then LAN truth-sync before protocol/relay slices, then logging/proof-surface stabilization before rollout-proof closure
- Estimated risk: medium-high, because the current TS/Rust model mismatch affects every downstream slice
- Estimated proof difficulty: high, because honest closure needs cross-surface proof on Windows, Android, and Linux/WSL/Docker, plus redaction/degraded-state evidence
- Should I continue immediately or pause for sequencing: pause for sequencing, then resume with exact path claims on `ra-01-contract-parity-and-test-repair` once the coordinator schedules this plan

## Optional Addendum

- Earlier audit passes found more specific portal-placeholder evidence than the latest report spelled out:
  - `packages/portal-domain/src/parent-portal-data.ts` maps `PortalRoute.RemoteAccess` to `routeContext('parentManage', PARENT_PORTAL_NAV_LABELS.DataPrivacy, 'remote-access')`.
  - `packages/portal-domain/src/parent-portal-nav.ts` does not expose a dedicated remote-access nav item.
  - `packages/portal-domain/src/routes.ts` still uses `PortalDevTextToken.DataCustodyTitle` and `PortalDevTextToken.DataCustodyBody` for `PortalRoute.RemoteAccess`.
  - `apps/portal/src/portal-route-content.ts` has no dedicated remote-access render path.
  - `apps/portal/src/portal-state.ts` hardcodes `resolveAgentTarget()` to localhost.
  - `apps/portal/src/transport.ts` special-cases LAN pairing commands only, not remote session commands.
- Earlier audit passes also captured the exact current Rust failure shape:
  - `cargo test -p ocentra-remote-access-core` fails because tests import private `ocentra_eventing::DomainEvent`; compiler points toward `ocentra_eventing::envelope::DomainEvent`, and `.contract()` trait use then falls out of scope.
  - `cargo test -p ocentra-child-runtime runtime_gate` fails on the same eventing privacy drift and an additional private `EventBus` use in `crates/child-runtime/tests/integration/tracking_runtime_flow_intent.rs`.
- Important workpack-named proof/test identifiers from earlier reads that the last report compressed:
  - WP01: `remote-capability.paired-access`, `remote-capability.live-view-not-control`, `remote-capability.revoked-grant-denied`, `remote-capability.removed-device-denied`, `remote-capability.wrong-household-denied`, `remote-capability.audit-complete`
  - WP02: `live-screen.paired-view`, `live-screen.view-only`, `live-screen.protected-surface-blocked`, `live-screen.relay-unavailable-degraded`, `live-screen.no-raw-retention-default`, `live-screen.child-disclosure-visible`
  - WP04: `remote-grant.paired-access`, `remote-grant.revocation-wins`, `remote-grant.remove-device-wins`, `remote-grant.child-disclosure`, `remote-grant.audit`, `remote-grant.reconnect-after-revoke-denied`, `remote-grant.wrong-household-device-denied`, `remote-grant.support-access-visible`
  - WP05: `relay.rate-limit`, `relay.retry-storm-backpressure`, `relay.cross-household-denied`, `relay.token-replay-denied`, `relay.redacted-diagnostics`, `relay.origin-host-redirect-negative`, `relay.slow-dependency-partial-outage`, `relay.cache-stale-grant-denied`, `relay.alerting-metrics-sanity`
  - WP06: `remote.rollout.capability-model`, `remote.rollout.authz-negative-proof`, `remote.rollout.relay-failure-proof`, `remote.rollout.privacy-proof`, `remote.rollout.no-overclaim`
- Earlier existence checks also showed `output/screen-plan-proof` is missing in this checkout, so even adjacent screen-plan proof paths referenced by remote docs were not inspectable locally during audit.
