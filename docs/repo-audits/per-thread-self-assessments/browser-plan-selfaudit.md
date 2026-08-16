# browser-plan

## Normalized Header

- plan/thread name: `browser-plan`
- source thread label: `browser-plan dedicated thread`
- source thread id: `019ed326-8f83-7ac2-a42e-34e9baa0bfca`
- worktree: `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent`
- branch: `codex/tracking-plan-full-continuation-a`
- claimed status: partial; no `DONE` or `PR_READY`; one WP01 repair slice validated locally; plan-level closure audit still failing
- claimed source files/crates/packages: `packages/browser-domain`; `packages/parent-domain`; `packages/agent-protocol-domain`; `crates/agent-protocol`; `crates/agent-core`; `crates/agent-service`; `apps/portal`; `scripts/test`; `docs/plans/browser-plan`
- claimed tests: `npm run test --workspace @ocentra-parent/browser-domain -- tests/unit/browser-package-exports.test.ts tests/unit/browser-plan-package-exports.test.ts tests/unit/social-applied-schedule-time-budget-proof.test.ts`; `npm run test --workspace @ocentra-parent/browser-domain`; `npm run type-check --workspace @ocentra-parent/browser-domain`; touched-file `npm run lint:architecture -- --files ...`; inventory/platform/enforcement/browser-plan proof scripts still unrun or still failing at plan level
- claimed proof commands/artifacts: `node scripts/test/browser-plan-closure-audit-proof.mjs`; canonical proof root `output/browser-plan-proof/<workpack-file-stem>/`; local WP01 proof root `output/browser-plan-proof/01-contract-boundary-and-effect-schemas/*`; existing relevant artifacts under `test-results/v0-8-browser-domain-adapter-proof/proof.json` and `test-results/windows-managed-unmanaged-browser-enforcement-proof/**`
- claimed blockers: package-wide `packages/browser-domain` re-export debt; browser-plan closure audit failing on all open row families plus missing partial/manual-required markings and missing artifacts; sibling dependency on `v0-8-enforcement-control-plan` for final WP19/WP20 claims; Apple-host-only social proof for `SOCIAL-17`
- claimed next actions: finish WP01 foundation cleanup; then WP03-WP05 inventory/platform proof; then WP06-WP14 managed runtime chain; then WP15-WP21 policy/intervention/enforcement; then AI/SOCIAL/GAME families; then WP22-WP24 closure
- obvious missing evidence fields: non-WP01 proof roots largely absent; `test-results/browser-platform-inventory-matrix-proof/proof.json`; `test-results/social-ios-screen-time-host-proof/proof.json`; `test-results/social-platform-account-feed-proof-artifacts/proof.json`; `test-results/social-platform-account-feed-rollout-gate/proof.json`; Rust browser tests still inline in `src` instead of proper `tests/` categories
- archival note: self-report only; unverified input for structural review

## Raw Self-Report

**COMPLETION_ARCHITECTURE_REPORT**

Browser-plan is not near honest closure yet, but it is also not a mystery anymore. Current repo truth is: one small WP01 repair slice is validated; the canonical browser owner is `packages/browser-domain` plus `packages/agent-protocol-domain`, `crates/agent-protocol`, `crates/agent-core`, `crates/agent-service`, `apps/portal`, and `scripts/test`; the plan docs previously overstated completion; the closure audit still fails on every numbered family plus missing proof artifacts; Windows/Android/Linux proof work can advance now on this host; the first true cross-plan sequencing issue is `v0-8-enforcement-control-plan` for WP19/WP20 final claims.

**Current truth snapshot**

| Area | Exact truth |
| --- | --- |
| Location | `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent` on `codex/tracking-plan-full-continuation-a` |
| Real audit state | `node scripts/test/browser-plan-closure-audit-proof.mjs` still fails: `01-24`, `AI-01..25`, `SOCIAL-01..24`, `GAME-01..24` unchecked; `05`, `SOCIAL-17`, `SOCIAL-23`, `SOCIAL-24` not marked partial/manual-required as expected; four named artifacts missing |
| Validated local progress | WP01 repair landed in `packages/browser-domain/src/social-applied-schedule-time-budget-proof.ts`, `packages/browser-domain/tests/unit/browser-package-exports.test.ts`, `packages/browser-domain/tests/unit/browser-plan-package-exports.test.ts`, plus `output/browser-plan-proof/01-contract-boundary-and-effect-schemas/*` and matching plan-doc updates |
| Real pass set | Targeted `browser-domain` tests, full `browser-domain` tests, `browser-domain` type-check, touched-file architecture gate |
| Still red | `npm run lint:architecture -- --files packages/browser-domain` fails on pre-existing re-export debt; browser-plan closure audit fails |
| Canonical proof root | `output/browser-plan-proof/<workpack-file-stem>/` |
| Hidden proof issue | `test-results/windows-managed-unmanaged-browser-enforcement-proof/run-pl8log/unmanaged-browser-profile/**` currently contains raw browser profile files; that is a real artifact but not acceptable as final proof-safe browser-plan closure evidence without sanitization/regeneration |

**Completion definition**
- The plan is actually complete only when required code is genuinely implemented across the browser, social, game, runtime, portal, and proof surfaces; every claimed row is either checked with real proof or explicitly partial/manual-required with honest proof; scoped validation is green; and `node scripts/test/browser-plan-closure-audit-proof.mjs` passes.
- “Done” for this plan requires:
  - real source closure across `packages/browser-domain`, `packages/parent-domain`, `packages/agent-protocol-domain`, `crates/agent-protocol`, `crates/agent-core`, `crates/agent-service`, `apps/portal`, `scripts/test`;
  - real tests under meaningful major categories where applicable, not empty folders or inline-only Rust test modules;
  - proof packs under `output/browser-plan-proof/<workpack-file-stem>/`;
  - no stale plan state, stale proof pointers, or fake-green category folders being counted as coverage.

**Exact docs/source/tests/proof read**
- Repo/router docs read:
  - `.ocentra-ai/rules/ocentra-parent-rules.mdc`
  - `docs/agent/TASK_ROUTER.md`
  - `docs/agent/WORKTREE_LANE_START.md`
  - `docs/agent/WORKER_LANE_FLOW.md`
  - `docs/agent/PLAN_WORKER_FLOW.md`
- Browser-plan docs read:
  - `docs/plans/browser-plan/AGENTS.md`
  - `docs/plans/browser-plan/PLAN_STATE.md`
  - `docs/plans/browser-plan/NEXT_ACTIONS.md`
  - `docs/plans/browser-plan/WORKPACK_INDEX.md`
  - `docs/plans/browser-plan/CHECKLIST_INDEX.md`
  - `docs/plans/browser-plan/TEST_PROOF_EXPECTATIONS.md`
  - `docs/plans/browser-plan/PROOF_INDEX.md`
  - `docs/plans/browser-plan/source-index.md`
  - `docs/plans/browser-plan/PLAN_HEALTH.md`
  - `docs/plans/browser-plan/pasted-content-coverage-audit.md`
  - `docs/plans/browser-plan/implementation-checklist.md`
  - `docs/plans/browser-plan/workpacks/01-contract-boundary-and-effect-schemas.md`
  - `docs/plans/browser-plan/workpacks/16-policy-target-compiler.md`
- Feature/expectation docs read:
  - `docs/features/browser-web-control.md`
  - `docs/features/social-video-control.md`
  - `docs/expectations/browser-evidence.md`
  - `docs/expectations/enforcement.md`
- Exact source/tests read:
  - `packages/browser-domain/src/social-applied-schedule-time-budget-proof.ts`
  - `packages/browser-domain/src/social-policy-compiler.ts`
  - `packages/browser-domain/package.json`
  - `packages/browser-domain/tests/unit/browser-package-exports.test.ts`
  - `packages/browser-domain/tests/unit/browser-plan-package-exports.test.ts`
  - `packages/browser-domain/tests/unit/social-applied-schedule-time-budget-proof.test.ts`
- Proof/scripts read or run:
  - `scripts/test/browser-plan-closure-audit-proof.mjs`
  - `output/browser-plan-proof/01-contract-boundary-and-effect-schemas/00-scope-summary.md`
  - `output/browser-plan-proof/01-contract-boundary-and-effect-schemas/01-negative-case-proof.md`
  - `output/browser-plan-proof/01-contract-boundary-and-effect-schemas/02-no-claim-boundary.md`
  - `output/browser-plan-proof/01-contract-boundary-and-effect-schemas/16-validation-commands.log`
  - inventoried `test-results/v0-8-browser-domain-adapter-proof/proof.json`
  - inventoried `test-results/windows-managed-unmanaged-browser-enforcement-proof/**`

**Current truth split**

| Bucket | Exact truth |
| --- | --- |
| Done | No numbered browser-plan workpack is honestly closed end-to-end. Only one local WP01 repair checkpoint is coherent and validated. |
| Partial | WP01 now has a real local proof root but still lacks full workpack closure. WP03-WP05 and WP18-WP24 have evidence of source/proof work in the checklist notes, but the closure audit still treats them as open because checklist state, partial/manual-required markings, or required artifacts are not actually aligned. |
| False-green | Old `WORKPACK_INDEX.md`, `PLAN_STATE.md`, and `PLAN_HEALTH.md` had 24/24-complete implications that were false. `packages/browser-domain/tests/*/.gitkeep` and `packages/agent-protocol-domain/tests/*/.gitkeep` major-category folders are optics only until populated. Historical `docs/proof/browser-plan/*` references are stale versus the canonical `output/browser-plan-proof/*` root. |
| Missing | Almost every workpack proof root beyond WP01. Required artifacts for `wp05`, `social17`, `social23`, `social24`. Major-category Rust browser tests in proper `crates/*/tests/` placement. Honest closure of AI/SOCIAL/GAME families. |

**Code surface and ownership**

| Surface | Exact ownership |
| --- | --- |
| Main TS browser contracts | `packages/browser-domain/src/browser-*.ts`, `browser-ai-*.ts`, `social-*.ts`, `browser-game-*.ts` |
| Browser policy/catalog authoring | `packages/parent-domain/src/browser-control-policy.ts`, `browser-control-manifest.ts`, `browser-control-values.ts`, `browser-control-catalog-values.ts` |
| Cross-boundary protocol/read-model TS | `packages/agent-protocol-domain/src/browser-policy-adapter.ts`, `browser-runtime-events.ts`, `browser-intervention-read-model.ts`, `social-*.ts` |
| Rust protocol parity | `crates/agent-protocol/src/browser*.rs`, `browser_policy*.rs`, `browser_intervention*.rs` |
| Rust runtime adapters | `crates/agent-core/src/browser_*`, `activity_store_browser*` |
| Rust service/read-model/API | `crates/agent-service/src/browser_*`, `social_*` browser-plan consumers |
| Portal/UI | `apps/portal/src/browser-*`, browser route/status/intervention surfaces; browser/social Playwright specs in `apps/portal/e2e/` |
| Proof harnesses | `scripts/test/browser*.mjs`, `scripts/test/social*.mjs`, `scripts/test/browser-game*.mjs`, `scripts/test/v0-8-browser-domain-adapter-proof.mjs` |
| Stale ownership text to remove | Old `packages/activity-domain/src/browser*.ts` ownership claims in workpacks; current owner is `packages/browser-domain` for the active TS surface |

**Test surface inventory and required reorganization**

| Surface | Current truth | Required change |
| --- | --- | --- |
| `packages/browser-domain/tests` | Real `unit` coverage is strong. `contract`, `integration`, `e2e`, `security`, `property-based`, `load`, `concurrency`, `observability`, `mutation`, `release` are mostly `.gitkeep` only. | Keep pure schema/value tests in `unit`. Add real `contract` for compiler/public-surface invariants, real `integration` for browser->read-model handoff, real `security` for native-host/unmanaged/no-claim boundaries, real `property-based` where matrix/compiler invariants apply. Do not count empty folders as coverage. |
| `packages/agent-protocol-domain/tests` | Real `unit` coverage exists; other major categories are scaffold-only `.gitkeep`. | Add real `contract` and `integration` tests for `browser-policy-adapter.ts`, `browser-runtime-events.ts`, `browser-intervention-read-model.ts`, and social read-model crossings. |
| `apps/portal/tests` | Real component/state tests exist. `apps/portal/e2e/` has real Playwright specs for browser AI/social surfaces. | Expand Playwright coverage for browser inventory/intervention/manual artifact flows as workpacks close. |
| `crates/agent-protocol/src` | Relevant inline test files still live in `src`: `browser_inventory_tests.rs`, `browser_intervention_tests.rs`, `browser_managed_tests.rs`, `browser_policy_tests.rs`, `browser_read_model_tests.rs`, `enforcement_browser_domain_adapter_proof_tests.rs`, `social_source_custody_mutation_tests.rs` | Move closure-relevant tests into `crates/agent-protocol/tests/contract`, `tests/unit`, `tests/integration`, `tests/version-skew` as applicable. |
| `crates/agent-core/src` | Inline browser/runtime tests still in `src`: `activity_store_browser_tests.rs`, `activity_store_browser_intervention_tests.rs`, `browser_bridge_cdp_adapter_tests.rs`, `browser_bridge_native_host_tests.rs`, `browser_bridge_poll_security_tests.rs`, `browser_bridge_poll_tests.rs`, `browser_bridge_tests.rs`, `browser_event_runtime_tests.rs`, `browser_managed_session_tests.rs`, `browser_windows_inventory_source_tests.rs`, `browser_windows_inventory_tests.rs` | Move into `crates/agent-core/tests/integration`, `tests/security`, and `tests/load` where WP22 requires it. |
| `crates/agent-service/src` | Inline browser/service tests still in `src`: `browser_inventory_read_model_tests.rs`, `browser_policy_api_tests.rs`, `browser_policy_compiler_tests.rs`, `browser_policy_manifest_patch_tests.rs`, `browser_runtime_delivery_tests.rs`, `browser_runtime_stream_tests.rs`, `browser_runtime_tests.rs`, `enforcement_browser_domain_adapter_proof_read_model_tests.rs` | Move into `crates/agent-service/tests/contract` and `tests/integration`; keep API/read-model boundaries explicit. |

**Proof inventory**

| Bucket | Exact truth |
| --- | --- |
| Canonical root | `output/browser-plan-proof/<workpack-file-stem>/` |
| Real now | Only `output/browser-plan-proof/01-contract-boundary-and-effect-schemas/{00-scope-summary.md,01-negative-case-proof.md,02-no-claim-boundary.md,16-validation-commands.log}` exists from this lane’s current work |
| Real but incomplete | WP01 proof root is still not full workpack closure; workpack-specific expectations like `00-source-snapshot.md` and broader checklist completion are still not satisfied |
| Real but needs proof-safety review | `test-results/windows-managed-unmanaged-browser-enforcement-proof/**` exists but includes raw unmanaged browser profile files and cannot be treated as final proof-safe closure evidence without redaction/regeneration |
| Real and relevant | `test-results/v0-8-browser-domain-adapter-proof/proof.json` exists |
| Stale | Old plan references to `docs/proof/browser-plan/*` and earlier “24/24 checked” plan state |
| Missing | Every non-WP01 workpack proof root unless proven otherwise; `test-results/browser-platform-inventory-matrix-proof/proof.json`; `test-results/social-ios-screen-time-host-proof/proof.json`; `test-results/social-platform-account-feed-proof-artifacts/proof.json`; `test-results/social-platform-account-feed-rollout-gate/proof.json` |

**Scoped validation inventory**

| State | Command | Exact result |
| --- | --- | --- |
| Pass | `npm run test --workspace @ocentra-parent/browser-domain -- tests/unit/browser-package-exports.test.ts tests/unit/browser-plan-package-exports.test.ts tests/unit/social-applied-schedule-time-budget-proof.test.ts` | passed |
| Pass | `npm run test --workspace @ocentra-parent/browser-domain` | passed; `101` files / `510` tests |
| Pass | `npm run type-check --workspace @ocentra-parent/browser-domain` | passed |
| Pass | `npm run lint:architecture -- --files packages/browser-domain/src/social-applied-schedule-time-budget-proof.ts packages/browser-domain/tests/unit/browser-package-exports.test.ts packages/browser-domain/tests/unit/browser-plan-package-exports.test.ts` | passed |
| Pass | `git diff --check -- <touched WP01 files>` | passed |
| Fail | `npm run lint:architecture -- --files packages/browser-domain` | fails on pre-existing re-export violations in `packages/browser-domain/src/browser.ts`, `browser-ai-*-schemas.ts`, `browser-control-manifest.ts`, `browser-intervention.ts`, `browser-policy-questionnaire-forest.ts`, `browser-social-ai-analysis-schemas.ts` |
| Fail | `node scripts/test/browser-plan-closure-audit-proof.mjs` | fails on all open families plus missing partial/manual-required markings and missing artifacts |
| Unrun in this lane | `node scripts/test/browser-inventory-model-completion-proof.mjs`, `browser-windows-live-inventory-proof.mjs`, `browser-platform-android-host-proof.mjs`, `browser-platform-android-owned-shell-proof.mjs`, `browser-platform-linux-host-proof.mjs`, `browser-platform-windows-host-proof.mjs`, `browser-performance-health-proof.mjs`, `browser-plan-e2e-manual-proof-artifacts.mjs` | still need fresh scoped execution as owning slices land |

**Dependency map**

| Bucket | Exact dependency truth |
| --- | --- |
| can do now | WP01 package-wide browser-domain cleanup; WP03-WP05 Windows/Android/Linux inventory proof; WP06-WP14 managed runtime/bridge/journal/service/portal chain; test relocation in TypeScript and Rust; most AI/SOCIAL/GAME contract work that stays local to browser-domain/protocol/service/portal |
| needs coordinator/other plan | Final WP19/WP20 enforcement/AppLocker/browser-adapter claims depend on `v0-8-enforcement-control-plan` surfaces: `scripts/test/v0-8-browser-domain-adapter-proof.mjs`, `packages/enforcement-domain/src/v0-8-*.ts`, `crates/agent-protocol/src/constants/v08_browser_domain_adapter_proof.rs`, `crates/agent-service/src/enforcement_*`. Ownership of `packages/browser-domain/src/browser.ts` re-export cleanup should be confirmed if another lane is editing the same files. |
| not feasible on this Windows host | Real Apple-host proof only where rows truly require it, currently most concretely `scripts/test/social-ios-screen-time-host-proof.mjs` / `test-results/social-ios-screen-time-host-proof/proof.json`. No Windows/Android/Linux browser-plan slice discovered so far is blocked by host feasibility. |

**Platform feasibility**

| Platform path | What can be proven here |
| --- | --- |
| Windows host now | Browser inventory, managed/unmanaged session behavior, native-host boundary, AppLocker/WDAC-adjacent proof harnesses, portal browser UI, Playwright UI proofs, browser performance/service health, service read-model/API behavior |
| Android Studio / Samsung device | WP05 Android host inventory, owned browser shell routing, social Android native-app capability proofs, managed-browser Android contract/evidence proofs where the scripts already exist |
| Linux via WSL / Docker | WP05 Linux host package/headless browser proofs, browser plan service/runtime proof scripts, performance/load-style scoped harnesses where Linux tooling is enough |
| Apple-host-only | Real iOS Screen Time host proof for `SOCIAL-17`; any future real macOS browser host execution if those rows are intended to move beyond manual-required |

**No-hand-wave execution plan**

| Ordered slice | Files / domains to touch | Scoped validation | Proof to collect | Exit criteria |
| --- | --- | --- | --- | --- |
| 1. WP01 foundation cleanup | `packages/browser-domain/src/browser.ts`, `browser-ai-analysis-schemas.ts`, `browser-ai-child-ux-schemas.ts`, `browser-ai-knowledge-graph-schemas.ts`, `browser-ai-parent-explanation-schemas.ts`, `browser-ai-policy-evaluator-schemas.ts`, `browser-ai-post-analysis-action-schemas.ts`, `browser-control-manifest.ts`, `browser-intervention.ts`, `browser-policy-questionnaire-forest.ts`, `browser-social-ai-analysis-schemas.ts`; matching tests and WP01 docs | `npm run test --workspace @ocentra-parent/browser-domain`; `npm run type-check --workspace @ocentra-parent/browser-domain`; touched-file and then package-wide `lint:architecture` | Complete WP01 proof root including any workpack-specific files still missing | `packages/browser-domain` architecture gate passes; WP01 checklist/proof/doc truth aligns |
| 2. WP03-WP05 inventory/platform matrix | `packages/browser-domain/tests/unit/browser-inventory.test.ts`, `browser-platform-inventory-matrix.test.ts`; `packages/agent-protocol-domain/tests/unit/browser-inventory-contracts.test.ts`; `crates/agent-protocol/src/browser_inventory.rs`; `crates/agent-core/src/browser_windows_inventory_*`; `crates/agent-service/src/browser_inventory_read_model_tests.rs`; `apps/portal/tests/live-activity-browser-status.test.ts`; `scripts/test/browser-inventory-model-completion-proof.mjs`, `browser-windows-live-inventory-proof.mjs`, `browser-platform-android-host-proof.mjs`, `browser-platform-android-owned-shell-proof.mjs`, `browser-platform-linux-host-proof.mjs`, `browser-platform-windows-host-proof.mjs` | Named inventory/platform scripts plus scoped TS/Rust tests | `output/browser-plan-proof/03-*`, `04-*`, `05-*`; `test-results/browser-platform-inventory-matrix-proof/proof.json`; Windows/Android/Linux host artifacts | WP03-WP05 rows become honest checked or partial/manual-required, with no raw private data overclaim |
| 3. WP06-WP14 managed runtime chain | `packages/browser-domain/tests/unit/browser-managed-profile-store.test.ts`, `browser-managed-session-status.test.ts`, `browser-tab-evidence.test.ts`, `browser-read-model.test.ts`; `crates/agent-core/src/browser_managed_session.rs`, `browser_bridge_*`, `activity_store_browser*`; `crates/agent-service/src/browser_runtime_*`; `apps/portal/tests/live-activity-browser-status.test.ts`; `apps/portal/e2e/browser-ai-parent-explanation-ui-proof.spec.ts` only where browser UI is touched | Scoped package/crate tests plus portal tests | Workpack proof roots `06` through `14`, including runtime/no-claim/manual-required artifacts | Managed runtime to portal chain is genuinely proven, not just documented |
| 4. WP15-WP21 policy/intervention/enforcement | `packages/parent-domain/src/browser-control-policy.ts`, `browser-control-manifest.ts`, `browser-control-values.ts`, `browser-control-catalog-values.ts`; `packages/browser-domain/src/social-policy-compiler.ts`, `browser-game-policy-compiler.ts`, `browser-intervention.ts`, `browser-social-unmanaged-bypass-detector*.ts`; `crates/agent-protocol/src/browser_policy*.rs`, `browser_intervention*.rs`, `enforcement_browser_domain_adapter_proof_tests.rs`; `crates/agent-service/src/browser_policy_*`; `scripts/test/v0-8-browser-domain-adapter-proof.mjs`; `scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs` | Scoped TS/Rust/browser-adapter tests | Proof roots `15` through `21`; sanitized Windows enforcement artifacts | Policy/intervention claims become honest; WP19/WP20 only close once sibling enforcement contracts are settled |
| 5. AI / SOCIAL / GAME families | `packages/browser-domain/tests/unit/browser-ai-*.test.ts`, `social-*.test.ts`, `browser-game-*.test.ts`; `packages/agent-protocol-domain/tests/unit/social-*.test.ts`; `apps/portal/e2e/social-*.spec.ts`; `scripts/test/social-*.mjs`; `scripts/test/browser-game-*.mjs` | Family-specific unit/contract/integration/Playwright scripts | Missing `social17`, `social23`, `social24` artifacts; corresponding AI/SOCIAL/GAME proof packs | `AI-*`, `SOCIAL-*`, `GAME-*` rows are real checked or honest manual-required |
| 6. WP22-WP24 and final closure | `scripts/test/browser-performance-health-proof.mjs`; `scripts/test/browser-plan-e2e-manual-proof-artifacts.mjs`; `scripts/test/browser-plan-closure-audit-proof.mjs`; `docs/plans/browser-plan/{PLAN_STATE.md,PLAN_HEALTH.md,WORKPACK_INDEX.md,implementation-checklist.md}`; `docs/features/browser-web-control.md`; `docs/expectations/browser-evidence.md` | Health proof, artifact proof, closure audit | Complete `22`, `23`, `24` proof roots and final artifact manifest | Closure audit passes and docs/source/tests/proof say the same thing |

**Blocker taxonomy**

| Bucket | Exact blockers |
| --- | --- |
| `local-now` | `packages/browser-domain` re-export debt; missing WP05 partial/manual-required markings and artifacts; missing populated major-category tests beyond unit; Rust browser tests still inline in `src`; missing proof roots beyond WP01 |
| `needs-coordinator-sequencing` | Same-file ownership if another lane is editing `packages/browser-domain/src/browser.ts` and the other re-export files; sequencing for Apple-host artifact owner; decision on whether this lane owns the Rust test migration |
| `needs-sibling-plan-contract` | `v0-8-enforcement-control-plan` contract/proof surfaces for final WP19/WP20 AppLocker/browser-adapter/enforcement claims |
| `host-platform-limited` | Real Apple-host proof for `scripts/test/social-ios-screen-time-host-proof.mjs` and any future macOS host execution that is intended to move beyond manual-required |

**First coordinator ask**
- Sequence `v0-8-enforcement-control-plan` before final WP19/WP20 closure, because browser-plan’s honest AppLocker/OS-block/browser-adapter claims depend on those exact enforcement contracts and proof scripts. This is not a reason to pause now; it is the first sibling dependency that matters for final closure. Immediate local work should still start with WP01 foundation cleanup.

**Strict done bar**
- `node scripts/test/browser-plan-closure-audit-proof.mjs` passes.
- No workpack row is relying on stale docs, stale proof roots, or empty test-category folders.
- `packages/browser-domain` architecture debt is cleared for the browser-plan surface.
- Closure-relevant Rust browser tests are no longer trapped inline in `src`; they live under proper `tests/` major categories where applicable.
- Required `test-results` artifacts exist for WP05 and `SOCIAL-17/23/24`.
- Any Apple-only rows are either backed by real Apple-host proof or explicitly and consistently left manual-required across docs, checklist, and proof.
- Feature/expectation docs match the checklist and proof.

**COORDINATOR_DECISION_REQUEST**
- Recommended next slice: `browser-wp01-foundation-cleanup`
- Recommended predecessor plans: `v0-8-enforcement-control-plan` before browser WP19/WP20 final closure; no predecessor required before immediate WP01/WP03-WP18 work
- Estimated risk: medium-high overall, low-medium for the immediate WP01 slice
- Estimated proof difficulty: high overall, because the plan spans Windows + Android + Linux runtime proof and one Apple-host-only social proof lane
- Continue immediately or pause for sequencing: continue immediately on `browser-wp01-foundation-cleanup`, but pause before WP19/WP20 closure unless enforcement-plan ownership/sequence is confirmed

## Optional Addendum

- Earlier audit passes, before the later `COMPLETION_ARCHITECTURE_REPORT`, found and corrected stale browser-plan status/docs truth in these files:
  - `docs/plans/browser-plan/WORKPACK_INDEX.md`
  - `docs/plans/browser-plan/PLAN_STATE.md`
  - `docs/plans/browser-plan/PLAN_HEALTH.md`
  - `docs/plans/browser-plan/source-index.md`
  - `docs/plans/browser-plan/pasted-content-coverage-audit.md`
- Those earlier passes established that the old generated plan state was falsely implying all numbered workpacks were checked and that the legacy `packages/activity-domain/src/browser*.ts` ownership text was stale for this checkout; the actual TypeScript owner for the active browser-plan surface is `packages/browser-domain`.
- Earlier scoped test failures, before the WP01 repair, were:
  - `packages/browser-domain/tests/unit/browser-package-exports.test.ts` and `browser-plan-package-exports.test.ts` reading `packages/browser-domain/tests/package.json` instead of the package root
  - `packages/browser-domain/src/social-applied-schedule-time-budget-proof.ts` constructing `SocialParentPolicyDecisionCandidateSchema` fixtures without the required `compilerCapabilityState`
- The local WP01 repair fixed those exact issues and added the first real browser-plan proof root under `output/browser-plan-proof/01-contract-boundary-and-effect-schemas/`, but that does not change the broader plan-level closure state.
