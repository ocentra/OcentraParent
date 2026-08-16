# v0-8-enforcement-control-plan

## Normalized Header

- plan/thread name: `v0-8-enforcement-control-plan`
- source thread label: `dedicated plan thread`
- source thread id: `019ed32f-1235-72f2-a6ff-990a8d6b8ec0`
- worktree: `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent`
- branch: `codex/tracking-plan-full-continuation-a`
- claimed status: `audit complete; completion architecture reported; not done`
- claimed source files/crates/packages: `packages/enforcement-domain`, `packages/app-game-domain`, `packages/agent-protocol-domain`, `packages/browser-domain`, `packages/logging-domain`, `packages/portal-domain`, `apps/portal`, `crates/agent-core`, `crates/agent-protocol`, `crates/agent-service`, `scripts/test/v0-8-*`, `scripts/test/app-game-*`, `scripts/test/managed-browser-*`, `scripts/test/tamper-*`
- claimed tests: `focused TS workspace tests`, `focused cargo tests`, `crate/package contract tests`, `portal Playwright still missing for v0.8`, `inline Rust src tests need migration into crate tests folders`
- claimed proof commands/artifacts: `node scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs`, `node scripts/test/managed-browser-intervention-proof.mjs`, `node scripts/test/managed-browser-service-proof.mjs`, `node scripts/test/v0-8-integrity-alert-status-bridge.mjs`, `node scripts/test/tamper-integrity-audit-contract-proof.mjs`, `node scripts/test/tamper-uninstall-artifact-status-proof.mjs`, `node scripts/test/app-game-adapter-execution-readiness-proof.mjs`, `node scripts/test/app-game-adapter-execution-readiness-live-surface-proof.mjs`, `node scripts/test/app-game-adapter-dispatch-preflight-live-handoff-proof.mjs`, `node scripts/test/v0-8-enforcement-product-control-spine.mjs`, `node scripts/test/v0-8-enforcement-control-plan-proof.mjs`, canonical root `output/v0-8-enforcement-control-plan-proof/<workpack-file-stem>/`
- claimed blockers: `duplicate codex-a writer claims on v0-8 proof scripts/docs`, `app-game sibling contract/proof ownership for readiness/preflight/host-capability`, `Apple host only if anyone tries to upgrade Apple privileged enforcement claims`
- claimed next actions: `slice-01-proof-router-truth`, `slice-02-windows-browser-integrity-boundaries`, `slice-03-app-game-service-bridge`
- obvious missing evidence fields: `missing test-results/v0-8-integrity-alert-status-bridge/proof.json`, `missing app-game readiness/preflight proof artifacts`, `missing managed-browser proof artifacts`, `no v0.8-specific portal Playwright proof`, `corrupted PROOF_INDEX`, `false-green WP18`, `Rust inline tests still in src`
- archival note: self-report only; unverified input for structural review

## Raw Self-Report

**Executive Summary**
`v0-8-enforcement-control-plan` is not close to honest closure. Current truth is: `WP01/02/03/07/09` look materially done, `WP18` is false-green, `WP04/05/06/08/10/11/13/14/15/16/17` are partial, and `WP12/19/20` are still missing as real completion slices. The immediate repo-level sequencing issue is writable ownership on the claimed `codex-a` proof files; the first real predecessor for final closure is the app-game readiness/preflight/host-capability contract path unless you hand that surface to this lane.

**COMPLETION_ARCHITECTURE_REPORT**

**1. Current truth snapshot**
Docs read in this thread:
- [docs/plans/v0-8-enforcement-control-plan/AGENTS.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/v0-8-enforcement-control-plan/AGENTS.md)
- [PLAN_STATE.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/v0-8-enforcement-control-plan/PLAN_STATE.md), [WORKPACK_INDEX.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/v0-8-enforcement-control-plan/WORKPACK_INDEX.md), [PROOF_INDEX.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/v0-8-enforcement-control-plan/PROOF_INDEX.md)
- Open workpacks `04,05,06,08,10,11,12,13,14,15,16,17,19,20`
- Feature/expectation set named by this plan: `browser-web-control`, `app-game-control`, `network-domain-control`, `policy-schedules-approvals`, `enforcement-integrity-tamper`, `enforcement`, `policy`, `browser-evidence`, `app-game-evidence`, `network-flow-evidence`, `tamper-uninstall-protection`

Source/proof/test surfaces inspected in this thread:
- [scripts/test/v0-8-enforcement-control-plan-proof.mjs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/scripts/test/v0-8-enforcement-control-plan-proof.mjs)
- [scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs)
- [scripts/test/app-game-adapter-execution-readiness-proof.mjs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/scripts/test/app-game-adapter-execution-readiness-proof.mjs)
- [scripts/test/app-game-adapter-execution-readiness-live-surface-proof.mjs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/scripts/test/app-game-adapter-execution-readiness-live-surface-proof.mjs)
- [scripts/test/app-game-adapter-dispatch-preflight-live-handoff-proof.mjs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/scripts/test/app-game-adapter-dispatch-preflight-live-handoff-proof.mjs)
- [scripts/test/managed-browser-intervention-proof.mjs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/scripts/test/managed-browser-intervention-proof.mjs)
- [scripts/test/managed-browser-service-proof.mjs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/scripts/test/managed-browser-service-proof.mjs)
- [scripts/test/v0-8-integrity-alert-status-bridge.mjs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/scripts/test/v0-8-integrity-alert-status-bridge.mjs)
- [scripts/test/tamper-integrity-audit-contract-proof.mjs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/scripts/test/tamper-integrity-audit-contract-proof.mjs)
- [scripts/test/tamper-uninstall-artifact-status-proof.mjs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/scripts/test/tamper-uninstall-artifact-status-proof.mjs)
- [crates/agent-core/src/enforcement_adapter.rs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/crates/agent-core/src/enforcement_adapter.rs), [crates/agent-core/src/enforcement_app_time_limit_tests.rs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/crates/agent-core/src/enforcement_app_time_limit_tests.rs)
- [crates/agent-service/src/activity_api/app_game_adapter_execution_readiness_payload.rs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/crates/agent-service/src/activity_api/app_game_adapter_execution_readiness_payload.rs)
- [crates/agent-service/src/enforcement_api/enforcement_pre_action_journal.rs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/crates/agent-service/src/enforcement_api/enforcement_pre_action_journal.rs)
- [packages/app-game-domain/src/app-game-adapter-execution-readiness.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/app-game-domain/src/app-game-adapter-execution-readiness.ts)
- [packages/agent-protocol-domain/src/app-game-adapter-execution-readiness.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/agent-protocol-domain/src/app-game-adapter-execution-readiness.ts)
- [packages/agent-protocol-domain/src/app-game-adapter-dispatch-preflight.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/agent-protocol-domain/src/app-game-adapter-dispatch-preflight.ts)
- [packages/browser-domain/src/browser-intervention.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/browser-domain/src/browser-intervention.ts)
- [packages/enforcement-domain/src/v0-8-enforcement-product-control-spine.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/enforcement-domain/src/v0-8-enforcement-product-control-spine.ts)
- [packages/enforcement-domain/src/v0-8-integrity-alert-status-bridge.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/enforcement-domain/src/v0-8-integrity-alert-status-bridge.ts)
- [packages/logging-domain/src/tamper-integrity-audit.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/logging-domain/src/tamper-integrity-audit.ts)
- [packages/portal-domain/src/parent-portal-service-state.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/portal-domain/src/parent-portal-service-state.ts), [apps/portal/src/live-activity-state.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/apps/portal/src/live-activity-state.ts)

| Truth bucket | Workpacks / items | Current reality |
| --- | --- | --- |
| `done` | `WP01`, `WP02`, `WP03`, `WP07`, `WP09` | Real source + proof packs exist and align closely enough with plan intent. |
| `partial` | `WP04`, `WP05`, `WP06`, `WP08`, `WP10`, `WP11`, `WP13`, `WP14`, `WP15`, `WP16`, `WP17` | Real source exists, but proof routing/artifacts, service integration, or portal closure is incomplete. |
| `false-green` | `WP18`, plan-level proof router | [PROOF_INDEX.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/v0-8-enforcement-control-plan/PROOF_INDEX.md) contains literal patch text; [v0-8-enforcement-control-plan-proof.mjs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/scripts/test/v0-8-enforcement-control-plan-proof.mjs) cites a missing integrity bridge artifact. |
| `missing` | `WP12`, `WP19`, `WP20` | No dedicated, honest, plan-owned completion proof path exists yet. |

**2. Completion definition**
Actually done means all of the following are true:
- Every workpack has real source ownership, real tests, real proof artifacts, and honest plan docs.
- Canonical proof root is populated per slice: `output/v0-8-enforcement-control-plan-proof/<workpack-file-stem>/`.
- Rust tests for this plan's touched surfaces live under crate `tests/` major categories where appropriate, not source-inline as the primary placement.
- Portal proof is real-service `playwright`, not mocked UI.
- Remaining non-claims stay explicit: no broad app blocking, no exact managed/unmanaged URL control, no anti-tamper hardening, no Apple privileged enforcement upgrade without host artifacts.
- Scoped validation and architecture gates pass on the touched files.

**3. End-to-end solution path**

| Surface | Ownership / exact files | What must happen for closure |
| --- | --- | --- |
| Contract + product-control spine | [packages/enforcement-domain/src/v0-8-enforcement-product-control-spine.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/enforcement-domain/src/v0-8-enforcement-product-control-spine.ts), [scripts/test/v0-8-enforcement-product-control-spine.mjs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/scripts/test/v0-8-enforcement-product-control-spine.mjs), `crates/agent-protocol`, `crates/agent-service` | Keep this as the unified plan-level state model; make it the single aggregation target for timers, approvals, integrity, app-game readiness, and portal consumption. |
| Owned-process enforcement | [scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs), [crates/agent-core/src/enforcement_adapter.rs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/crates/agent-core/src/enforcement_adapter.rs) | Finish `WP04` with dedicated v0.8 proof routing for success, mismatch, unavailable, no-op, and broad-target manual-required. |
| App/game readiness + preflight | [packages/app-game-domain/src/app-game-adapter-execution-readiness.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/app-game-domain/src/app-game-adapter-execution-readiness.ts), [packages/agent-protocol-domain/src/app-game-adapter-dispatch-preflight.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/agent-protocol-domain/src/app-game-adapter-dispatch-preflight.ts), [crates/agent-service/src/activity_api/app_game_adapter_execution_readiness_payload.rs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/crates/agent-service/src/activity_api/app_game_adapter_execution_readiness_payload.rs) | Remove stale `parent-domain` proof indirection, regenerate readiness/preflight/host-capability artifacts, then route them into `WP05` and `WP13`. |
| Managed browser boundary | [scripts/test/managed-browser-intervention-proof.mjs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/scripts/test/managed-browser-intervention-proof.mjs), [scripts/test/managed-browser-service-proof.mjs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/scripts/test/managed-browser-service-proof.mjs), [packages/browser-domain/src/browser-intervention.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/browser-domain/src/browser-intervention.ts) | Close `WP06` as managed-session-only intervention, keep exact URL/manual-required non-claim explicit. |
| Approval + audit + child-facing | [packages/policy-domain/tests/unit/policy-approval-override.test.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/policy-domain/tests/unit/policy-approval-override.test.ts), [crates/agent-service/src/enforcement_api/enforcement_pre_action_journal.rs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/crates/agent-service/src/enforcement_api/enforcement_pre_action_journal.rs), [packages/browser-domain/src/social-child-approval-block-ux.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/browser-domain/src/social-child-approval-block-ux.ts), [packages/text-domain/src/social-child-approval-block-ux-text.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/text-domain/src/social-child-approval-block-ux-text.ts) | Finish `WP10`, `WP11`, `WP12` together: durable approval transitions, journal reconstruction, stable child-facing reason bundle, no AI text. |
| Integrity + tamper non-claim | [packages/enforcement-domain/src/v0-8-integrity-alert-status-bridge.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/enforcement-domain/src/v0-8-integrity-alert-status-bridge.ts), [packages/logging-domain/src/tamper-integrity-audit.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/logging-domain/src/tamper-integrity-audit.ts), [scripts/test/tamper-integrity-audit-contract-proof.mjs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/scripts/test/tamper-integrity-audit-contract-proof.mjs) | Close `WP15` and `WP16` with dedicated artifacts, keeping all anti-tamper claims manual-required. |
| Portal + Playwright | [packages/portal-domain/src/parent-portal-service-state.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/portal-domain/src/parent-portal-service-state.ts), [apps/portal/src/live-activity-state.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/apps/portal/src/live-activity-state.ts), new `apps/portal/e2e/<v0-8>.spec.ts` | Close `WP14` and `WP19` with service-backed rendering only and one honest Playwright proof spec. |
| Proof composition + rollout docs | [docs/plans/v0-8-enforcement-control-plan/PROOF_INDEX.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/v0-8-enforcement-control-plan/PROOF_INDEX.md), [scripts/test/v0-8-enforcement-control-plan-proof.mjs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/scripts/test/v0-8-enforcement-control-plan-proof.mjs) | Rebuild `WP18` honestly from real slice artifacts, then finish `WP20` plan/feature/expectation doc truth sync. |

**4. Dependency map**

| Bucket | Items | Exact dependency |
| --- | --- | --- |
| `can do now` | `WP04`, `WP06`, `WP08`, `WP15`, `WP16`, `WP17`, `WP18`, `WP20` | All have local source/proof surfaces already present; closure work is mostly proof routing, test relocation, and scoped validation. |
| `needs-coordinator-sequencing` | writable lane ownership | Active `codex-a` writer already claims the core v0.8 proof scripts and blocks honest write/claim work from this thread. |
| `needs-sibling-plan-contract` | `WP05`, then `WP13/14/19` | App-game readiness/preflight/host-capability proof is the real upstream contract set: `scripts/test/app-game-adapter-execution-readiness*.mjs`, `scripts/test/app-game-adapter-dispatch-preflight-live-handoff-proof.mjs`, `packages/app-game-domain`, `packages/agent-protocol-domain`, `crates/agent-service/activity_api`. |
| `host-platform-limited` | Apple privileged enforcement upgrade only | Real macOS/iOS privileged proof would need an Apple host if anyone wants to upgrade those rows beyond `manual-required`/`not-applicable`. Not required for honest current-plan closure. |

**5. Test/proof reorganization and missing coverage**

| Area | Current test surface | Required reorg / missing coverage |
| --- | --- | --- |
| Rust owned-process / approval / audit | `crates/agent-core/src/enforcement_app_time_limit_tests.rs`, `enforcement_approval_audit_tests.rs`, `activity_store_enforcement_audit_tests.rs` | Move into `crates/agent-core/tests/unit/` and `tests/integration/`; add replay/no-op/idempotency negative cases where applicable. |
| Rust service/product-control | `crates/agent-service/src/enforcement_os_adapter_product_proof_read_model_tests.rs`, `product_control_spine_tests.rs`, `product_control_api_tests.rs`, `integrity_alert_status_bridge_read_model_tests.rs`, `enforcement_integrity_runtime_audit_read_model_tests.rs` | Move into `crates/agent-service/tests/integration/` and `tests/contract/`; stop relying on source-inline modules. |
| Rust protocol contracts | multiple `crates/agent-protocol/src/*_tests.rs` for enforcement/app-game/integrity | Move into `crates/agent-protocol/tests/contract/` for clear contract ownership. |
| TS package tests | mostly already under `tests/unit` in `packages/enforcement-domain`, `packages/app-game-domain`, `packages/agent-protocol-domain`, `packages/portal-domain` | Keep structure; add missing unit/contract coverage for child-facing reason bundle and host-capability normalization. |
| Portal e2e | no v0.8-specific Playwright proof found | Add `apps/portal/e2e/<v0-8-service-backed-proof>.spec.ts` under `playwright`/`e2e`; no fake service handlers. |
| Empty-folder optics | `crates/ocentra-eventing/tests/*/.gitkeep` categories exist | Do not count these as v0.8 coverage for `WP11`; they are placeholder optics, not real tests. |
| Major categories actually applicable | `unit`, `integration`, `contract`, `playwright/e2e`, some `security/replay` negatives | `property` and `load` are not required for honest v0.8 closure today; do not invent them. |

**6. Validation/proof commands and artifacts**

| Inventory | Exact command / check | Current truth |
| --- | --- | --- |
| cheap pass observed from existing proof packs | `node scripts/test/v0-8-enforcement-policy-dispatch-proof.mjs`, `node scripts/test/v0-8-supported-adapter-runtime-proof.mjs`, `node scripts/test/v0-8-enforcement-product-control-spine.mjs`, `node scripts/test/v0-8-enforcement-integrity-runtime-audit.mjs`, `node scripts/test/v0-8-cross-platform-enforcement-capability-proof.mjs`, `node scripts/test/v0-8-broad-os-adapter-runtime-proof.mjs` | Artifacts/logs exist, but I did not rerun them in this reporting turn. |
| cheap fail / defect observed now | `rg` on [PROOF_INDEX.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/v0-8-enforcement-control-plan/PROOF_INDEX.md), [v0-8-enforcement-control-plan-proof.mjs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/scripts/test/v0-8-enforcement-control-plan-proof.mjs), [app-game-adapter-execution-readiness-proof.mjs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/scripts/test/app-game-adapter-execution-readiness-proof.mjs); `Test-Path` on missing proof roots | Confirmed patch text corruption, stale `parent-domain` import, missing integrity bridge/app-game/managed-browser proof artifacts. |
| required unrun focused commands | `node scripts/test/managed-browser-intervention-proof.mjs`, `node scripts/test/managed-browser-service-proof.mjs`, `node scripts/test/v0-8-integrity-alert-status-bridge.mjs`, `node scripts/test/tamper-integrity-audit-contract-proof.mjs`, `node scripts/test/tamper-uninstall-artifact-status-proof.mjs`, `node scripts/test/app-game-adapter-execution-readiness-proof.mjs`, `node scripts/test/app-game-adapter-execution-readiness-live-surface-proof.mjs`, `node scripts/test/app-game-adapter-dispatch-preflight-live-handoff-proof.mjs`, one new portal Playwright spec | These are the real missing proof runs for honest closure. |
| required architecture gates | `npm run lint:architecture -- --files <touched ts/js>` and `cargo lint-architecture <touched rs>` | Must be run slice-by-slice before closure. |

Canonical proof root:
- `output/v0-8-enforcement-control-plan-proof/<workpack-file-stem>/`

**7. First 3 atomic slices in recommended order**

| Slice | Files / domains to touch | Validation | Proof | Exit criteria |
| --- | --- | --- | --- | --- |
| `slice-01-proof-router-truth` | [PROOF_INDEX.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/v0-8-enforcement-control-plan/PROOF_INDEX.md), [PLAN_STATE.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/v0-8-enforcement-control-plan/PLAN_STATE.md), [WORKPACK_INDEX.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/v0-8-enforcement-control-plan/WORKPACK_INDEX.md), [scripts/test/v0-8-enforcement-control-plan-proof.mjs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/scripts/test/v0-8-enforcement-control-plan-proof.mjs) | `node --check` on touched scripts, scoped `lint:architecture` | repaired proof router + empty/missing artifact map | No patch text, no missing-artifact citations, no false checked claims. |
| `slice-02-windows-browser-integrity-boundaries` | `crates/agent-core`, `crates/agent-service`, `packages/enforcement-domain`, `packages/browser-domain`, `packages/logging-domain`, `scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs`, `managed-browser-*`, `v0-8-integrity-alert-status-bridge.mjs`, `tamper-*` | focused `cargo test`, `npm run test`, `lint:architecture`, `cargo lint-architecture` | `WP04`, `WP06`, `WP15`, `WP16`, `WP17` proof packs | Dedicated v0.8 artifacts exist and pass; Rust tests moved out of `src` for touched surfaces. |
| `slice-03-app-game-service-bridge` | `packages/app-game-domain`, `packages/agent-protocol-domain`, `crates/agent-protocol`, `crates/agent-service/activity_api`, `packages/portal-domain` app-game panels, `scripts/test/app-game-adapter-execution-readiness*.mjs` | focused TS/Rust tests + architecture gates | `test-results/app-game-adapter-execution-readiness-proof/proof.json`, `...live-surface...`, `...dispatch-preflight...` | `WP05` honestly closed; `WP13` unblocked with real service payload truth. |

**8. Exact coordinator asks / unblock requests**
- Free or retarget the existing `codex-a` claims on the v0.8 proof scripts and plan docs before assigning implementation here.
- Decide one of two paths for the app-game dependency:
- either sequence app-game-plan `WP166/167/168/178` first,
- or transfer ownership of those exact proof scripts and owner files to this lane for the duration of v0.8 closure.
- Confirm WSL is the accepted Linux proof path for this plan on this host so `docker` remains optional, not blocking.
- If portal/UI stays split, keep service/product-control truth with this lane and only hand off visual polish after `WP13` lands.

**COORDINATOR_DECISION_REQUEST**
- Recommended next slice: `slice-01-proof-router-truth`
- Recommended predecessor plans:
- immediate sequencing prerequisite: free the current duplicate `codex-a` writer claims
- first sibling plan predecessor for final closure: `app-game-plan` `WP166/167/168/178`, unless you explicitly transfer that surface here
- Estimated risk: `medium-high`
- Estimated proof difficulty: `medium` for `WP04/06/15/16/17`, `high` for `WP05/10/11/12/13/14/19`
- Continue immediately or pause for sequencing: `pause for sequencing`, then continue immediately once writable ownership and app-game dependency routing are decided

## Optional Addendum

- Earlier audit passes found a second stale `WP05` coupling beyond the proof wrapper itself: [packages/agent-protocol-domain/tests/unit/app-game-adapter-execution-readiness.test.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/agent-protocol-domain/tests/unit/app-game-adapter-execution-readiness.test.ts) still hardcodes `test-results/v0-8-windows-app-time-limit-adapter-mvp/proof.json`.
- Earlier audit passes found that [scripts/test/v0-8-enforcement-control-plan-proof.mjs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/scripts/test/v0-8-enforcement-control-plan-proof.mjs) is incomplete both by citation and by orchestration: it runs only supported-adapter, product-control spine, integrity runtime audit, cross-platform, broad-OS runtime, and the policy approval override unit path. It does not run the missing integrity-bridge, managed-browser, or app-game proof scripts.
- Earlier audit passes found existing runtime evidence already proving narrow partiality for `WP04` and `WP06`: `test-results/windows-managed-unmanaged-browser-enforcement-proof/2026-06-17T02-03-51-932Z.json` shows PID-required guard rejection, process-name mismatch rejection without termination, runtime terminate success, managed-browser manual-required, and exact managed/unmanaged URL/title/content still unclaimed.
