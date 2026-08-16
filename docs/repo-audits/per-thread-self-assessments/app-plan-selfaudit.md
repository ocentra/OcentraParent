# app-plan

## Normalized Header

- plan/thread name: `app-plan`
- source thread label: `app-plan thread`
- source thread id: `019ed326-386c-77c2-bf9a-cbb21536d753`
- worktree: `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent`
- branch: `codex/tracking-plan-full-continuation-a`
- claimed status: not done; completion proposal only; plan truth is stale, architecture debt is real, proof roots are missing, and sibling-plan sequencing is still required
- claimed source files/crates/packages: `docs/plans/app-plan/*`; `packages/app-game-domain`; `packages/agent-protocol-domain`; `packages/parent-domain` app shims; `crates/agent-protocol`; `crates/agent-core`; `crates/agent-service`; `apps/portal`
- claimed tests: `packages/app-game-domain/tests/unit/*`; `packages/agent-protocol-domain/tests/unit/app-game-*.test.ts`; `apps/portal/tests/app-game-*.test.ts`; Rust app-plan tests currently inline under `crates/*/src/*_tests.rs` and must move into `tests/` major categories
- claimed proof commands/artifacts: `node scripts/test/app-game-*.mjs`; canonical human-owned proof root should be `output/app-plan-proof/<workpack-file-stem>/`; raw generated artifacts should live in `test-results/<proof-script-name>/proof.json`
- claimed blockers: stale app-plan docs and checklist truth; missing `output/app-plan-proof/*` and `output/app-game-plan-proof/*`; scoped architecture lint failures on app shims/owners; cross-plan dependencies on `app-game-plan`, `policy-control-plane-plan`, `portal-ux-household-surfaces-plan`, `eventing-plan`, `data-custody-storage-plan`, `v0-8-enforcement-control-plan`, `screen-plan`
- claimed next actions: `Slice 1` app-plan truth repair; `Slice 2` app ownership and architecture cleanup; `Slice 3` Rust app-plan test rehome into `tests/` categories
- obvious missing evidence fields: no local proof roots; most cited `test-results/app-game-*/proof.json` absent; Windows/Android/Linux artifacts not yet attached truthfully; no completed canonical proof schema between `PROOF_INDEX.md` and `implementation-checklist.md`
- archival note: self-report only; unverified input for structural review

## Raw Self-Report

**Executive Summary**
`app-plan` is not close to honest closure on `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent` / `codex/tracking-plan-full-continuation-a` because its docs still point at stale ownership, stale test paths, and nonexistent proof roots. The real codebase already contains substantial native-app implementation across `packages/app-game-domain`, `packages/agent-protocol-domain`, `crates/agent-protocol`, `crates/agent-core`, `crates/agent-service`, and `apps/portal`, but the plan does not describe that truth correctly, some plan-owned files fail the architecture gate, Rust app-plan tests still live inline under `src`, and the plan’s claimed proof packs are largely absent.

The fastest honest path is not “finish everything everywhere.” It is: fix app-plan truth first, clean the verified architecture/ownership debt second, close the Windows end-to-end spine third, then sequence Android/Linux proof and the source-freshness/timer-service chain with the sibling plans that actually own those contracts. `app-game-plan` is the first predecessor plan that should move in parallel because app-plan is currently mirroring its shared runtime/proof spine.

# COMPLETION_ARCHITECTURE_REPORT

**Current truth snapshot**

No implementation edits are in flight from this thread. The proposal below is based on the real checkout state already audited.

| Category | Exact read / inspection scope | Honest status |
|---|---|---|
| Core app-plan docs fully read | [AGENTS.md](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-plan/AGENTS.md), [README.md](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-plan/README.md), [PLAN_STATE.md](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-plan/PLAN_STATE.md), [NEXT_ACTIONS.md](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-plan/NEXT_ACTIONS.md), [WORKPACK_INDEX.md](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-plan/WORKPACK_INDEX.md), [DOC_INDEX.md](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-plan/DOC_INDEX.md), [ROUTE_INDEX.md](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-plan/ROUTE_INDEX.md), [PLAN_EXECUTION_BLUEPRINT.md](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-plan/PLAN_EXECUTION_BLUEPRINT.md), [PLAN_HEALTH.md](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-plan/PLAN_HEALTH.md), [CHECKLIST_INDEX.md](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-plan/CHECKLIST_INDEX.md), [PROOF_INDEX.md](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-plan/PROOF_INDEX.md), [TEST_PROOF_EXPECTATIONS.md](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-plan/TEST_PROOF_EXPECTATIONS.md), [source-index.md](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-plan/source-index.md), [current-app-snapshot.md](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-plan/current-app-snapshot.md), [implementation-checklist.md](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-plan/implementation-checklist.md), `pasted-content-coverage-audit.md`, `v0-5-native-apps-full-scope-plan.md`, `v0-5-native-apps-platform-deep-dive.md`, `v0-5-native-apps-test-blueprint.md`, `ui-ux-requirements-guide.md`, `ARCHIVE_INDEX.md` | Fully read |
| Workpack docs | All `docs/plans/app-plan/workpacks/*.md` scanned programmatically for titles, sections, proof refs, and touched-path density; full direct inspection on `01`, `21`, `24`, `28`, `37`, `74`, `108` | Broad scan done, representative deep reads done |
| Feature docs read | `docs/features/app-game-control.md`, `child-agent-local-service.md`, `policy-schedules-approvals.md`, `app-install-purchase-approval.md`, `enforcement-integrity-tamper.md`, `evidence-store-query.md`, `local-ai-safety-evaluator.md`, `remote-lan-mobile-platforms.md`, `production-distribution-support.md`, `social-video-control.md` | Read |
| Expectation / architecture docs read | `docs/expectations/app-game-evidence.md`, `policy.md`, `enforcement.md`, `platforms.md`, `ai.md`, `evidence-storage.md`, `app-install-purchase-approval.md`, `docs/architecture/app-game-evidence-sessions.md`, `docs/roadmaps/roadmap-v0-5-2-app-game-evidence-sessions.md` | Read |
| Source files directly read | `packages/parent-domain/src/app-game-control-authority.ts`, `packages/parent-domain/src/app-game-category-risk.ts`, `packages/app-game-domain/src/app-game-control-authority.ts`, `packages/app-game-domain/src/app-game-category-risk.ts`, `crates/agent-protocol/src/app_game.rs`, `crates/agent-core/src/activity_store_app_game/app_game_sessionization.rs`, `crates/agent-service/src/activity_surface_read_models/app_use.rs`, `apps/portal/tests/app-game-platform-proof-status-route-panel.test.ts`, `packages/app-game-domain/tests/unit/app-game-control-authority.test.ts`, `packages/app-game-domain/tests/unit/app-game-category-risk.test.ts` | Enough for real ownership / architecture / test-shape conclusions |
| Proof locations inspected | `output/app-plan-proof`, `output/app-game-plan-proof`, `test-results`, plus proof refs extracted from `source-index.md` and all workpack docs | Proof-root truth established |

| Current truth area | Workpacks / surfaces | Truth | Evidence |
|---|---|---|---|
| Plan routing / checklist / proof docs | `WP01-WP03`, top-level app-plan docs | `false-green` | Checklist reset/open, but completion/proof narratives still present |
| TS app contract / authority / category / risk surface | `WP04-WP11`, `WP16-WP28`, `WP53-WP67`, `WP74-WP108`; real code in `packages/app-game-domain/src/*` | `partial` | Real code exists, plan points at stale owners |
| Rust protocol / storage / service spine | `WP12-WP15`, `WP29-WP49`; real code in `crates/agent-protocol`, `crates/agent-core`, `crates/agent-service` | `partial` | Real code and inline tests exist, but end-to-end proof / reorg incomplete |
| Portal app surfaces | `apps/portal/src/AppGame*.tsx`, `apps/portal/tests/app-game-*.test.ts` | `partial` | Route-level tests exist, real parent-visible closure/proof missing |
| Claimed proof packs | `output/app-plan-proof/*`, `output/app-game-plan-proof/*`, many `test-results/app-game-*/proof.json` refs | `missing` / `stale` | Roots absent locally; docs still cite them |
| Architecture compliance | `packages/parent-domain/src/app-game-control-authority.ts`, `packages/parent-domain/src/app-game-category-risk.ts`, `packages/app-game-domain/src/app-game-control-authority.ts`, `packages/app-game-domain/src/app-game-category-risk.ts` | `failing` | Scoped `lint:architecture` already fails on banned re-exports |

**Completion definition**
This plan is actually done only when all of the following are true:
- App-plan docs name the real owners, real tests, real proof roots, and real remaining dependencies.
- Every app-plan workpack is either genuinely complete with code + tests + proof on this checkout, or explicitly delegated to the owning sibling plan instead of implicitly mirrored.
- The native-app path is truly closed where claimed: TS domain -> TS/Rust protocol parity -> Rust ingest/storage/sessionization -> service read model -> portal surfaces -> platform proof.
- App-plan-owned tests are real and categorized under `tests/` major categories where applicable. Rust app-plan tests may not remain inline under `src`.
- No app-plan completion claim relies on stale proof, placeholder folders, or empty output roots.
- Windows, Android, and Linux/WSL/Docker proof claims are backed by real artifacts where relevant.
- Any remaining iOS/mac-only rows are explicitly marked as host-platform-limited and not counted as local completion.

**Code surface and ownership across packages / crates / domains**

| Surface | Real owner now | App-plan doc truth today | Required closure action |
|---|---|---|---|
| TS app contracts / approval / risk / install / timers / notifications | `packages/app-game-domain/src/*` | Stale: early docs still point at missing `packages/activity-domain/src/app-game*.ts` | Make `packages/app-game-domain` explicit canonical TS owner in app-plan docs |
| TS transport / protocol DTOs | `packages/agent-protocol-domain/src/app-game-*.ts` | Partly reflected | Keep as real contract surface; verify row-by-row ownership in plan |
| Rust protocol mirror | `crates/agent-protocol/src/app_game*.rs` | Reflected, but tests remain inline | Move app-plan-owned tests into `crates/agent-protocol/tests/{unit,contract,...}` |
| Rust storage / source / ingest / sessionization | `crates/agent-core/src/activity_store_app_game/*` | Reflected at high level | Reorganize tests out of `src`, then prove Windows path end-to-end |
| Rust service read models / API handoff | `crates/agent-service/src/activity_surface_read_models/*` and related service code | Reflected partially | Close service/read-model/runtime proof or explicitly leave to sibling plan |
| Portal app/game parent surfaces | `apps/portal/src/AppGame*.tsx`, `apps/portal/src/live-activity-state.ts`, `apps/portal/tests/app-game-*.test.ts` | Stale filenames still appear in plan | Update plan to real filenames, then add missing Playwright / e2e as applicable |
| Parent-domain app shims | `packages/parent-domain/src/app-game-*.ts` | Treated like owners in plan | Remove / rewrite banned re-export shims or explicitly make them non-barrel compat layers |

**Test/proof reorganization and missing coverage**

| Area | Current state | Reorg / gap | Applicable missing categories |
|---|---|---|---|
| `packages/app-game-domain` | Strong `tests/unit/*` presence already | Keep as main TS unit surface; align plan docs to these real paths | `integration`, `contract`, `security`, `property`, `load` still thin or absent where relevant |
| `packages/agent-protocol-domain` | `tests/unit/app-game-*.test.ts` exist | Good base; add contract-specific grouping if app-plan keeps protocol ownership | `contract` explicit grouping |
| `apps/portal` | `apps/portal/tests/app-game-*.test.ts` route/panel tests exist | Add `playwright`/real e2e where parent-visible rendering claims matter | `playwright`, `e2e` |
| `crates/agent-protocol` | Many app tests inline in `src`: `app_game_tests.rs`, `app_game_authority_classifier_tests.rs`, `app_game_boundary_read_model_tests.rs`, `app_game_policy_readiness_tests.rs`, etc. | Must move to `crates/agent-protocol/tests/{unit,contract,compatibility}` | `unit`, `contract` |
| `crates/agent-core` | Many app tests inline in `src/activity_store_app_game/*_tests.rs` and nearby `*_tests.rs` | Must move to `crates/agent-core/tests/{unit,integration,property,load}` as applicable | `unit`, `integration`, `property`, `load` |
| `crates/agent-service` | App read-model tests inline under `src/activity_surface_read_models/*_tests.rs` | Must move to `crates/agent-service/tests/{unit,integration,contract}` | `unit`, `integration`, `contract` |
| Empty-folder optics | No meaningful app-plan-owned `tests/` empty shells were identified in the actual app surfaces | Do not manufacture category folders unless they contain real tests | N/A |
| Security / no-claim coverage | Exists partly as proof scripts and negative gates | Needs explicit test/proof ownership for raw path/title leak, AI no-direct-action, manual-required, uninstall/tamper, adapter-not-claimed boundaries | `security` |
| Performance / health | `WP26` exists in plan and some scripts exist | No honest load/throughput artifact pack yet | `load` |

**Proof inventory**

| Proof surface | Real / stale / missing | Notes |
|---|---|---|
| `output/app-plan-proof/*` | `missing` | Root absent locally |
| `output/app-game-plan-proof/*` | `missing` | Root absent locally |
| `test-results/app-game-*/proof.json` cited by app-plan | Mostly `missing` | Many workpacks cite artifacts not present locally |
| `scripts/test/app-game-*.mjs` | `real generators exist` | These are the actual raw-proof entrypoints available now |
| `current-app-snapshot.md`, `source-index.md`, many workpack completion notes | `stale / false-green` | They cite absent proof roots and completion narratives |

Canonical proof-root path for honest closure should be:
- Human-owned workpack proof: `output/app-plan-proof/<workpack-file-stem>/`
- Raw generated script artifact: `test-results/<proof-script-name>/proof.json`

App-plan should link to sibling `output/app-game-plan-proof/*` only where those sibling artifacts actually exist and are intentionally reused.

**Scoped validation inventory**

| Command / check | Status | Notes |
|---|---|---|
| `npm run ledger:doctor` | pass | Coordination sanity only, not code validation |
| `npm run hub:inbox` | pass / empty | Coordination sanity only |
| Custom path-existence audit against app-plan refs | pass as audit | Established missing/stale paths |
| `npm run lint:architecture -- --files packages/parent-domain/src/app-game-control-authority.ts packages/parent-domain/src/app-game-category-risk.ts packages/app-game-domain/src/app-game-control-authority.ts packages/app-game-domain/src/app-game-category-risk.ts` | fail | Real code debt; re-export ban violated |
| Focused TS unit tests for app surfaces | unrun in this thread | Must be run after slice selection |
| Focused Rust tests for app surfaces | unrun in this thread | Must be rehomed under `tests/` first if this plan owns them |
| App proof scripts under `scripts/test/app-game-*.mjs` | largely unrun in this thread | Need real artifact capture, not assumption |
| Repo-wide validation | intentionally unrun | Correctly skipped |

**Platform feasibility**

| Platform bucket | What can be proven from this host / setup | Notes |
|---|---|---|
| Windows host now | Inventory, registry/package discovery, process runtime, foreground capture, sessionization, service read model, portal surfaces, broad-blocking gates where relevant | This is the main local proof path |
| Android Studio / emulator / synced Samsung device | Usage-events flows, accessibility/runtime preflight, child runtime receipts, notification delivery/readiness flows where scripts already exist | Feasible here; not a blocker by policy |
| Linux via WSL / Docker | Docker host preflight, Linux source/runtime gate proofs, some source-freshness / manual-required / adapter boundary proofs | Feasible here; do not count as blocker |
| Apple-host-only | Real macOS/iOS host execution proof | Only relevant for rows truly in scope; otherwise keep as explicit host-platform-limited note |

**Dependency map**

| Bucket | Plan / dependency | Why it matters |
|---|---|---|
| `can do now` | app-plan truth repair | Independent local docs slice |
| `can do now` | app TS ownership + architecture cleanup | The failing files are local and known |
| `can do now` | Windows core spine proof | Real local code and host capability exist |
| `can do now` | Android proof using emulator / Samsung device | Real scripts and host path exist |
| `can do now` | Linux proof via WSL/Docker | Real scripts and host path exist |
| `needs-coordinator-sequencing` | `app-game-plan` | App-plan mirrors shared app/game workpacks and proof roots; sequencing affects truth repair and later closure |
| `needs-coordinator-sequencing` | `policy-control-plane-plan` | `WP74-WP108` source-freshness/timer semantics depend on its policy source truth |
| `needs-sibling-plan-contract` | `portal-ux-household-surfaces-plan` | Final parent-visible surface ownership and UI contract |
| `needs-sibling-plan-contract` | `v0-8-enforcement-control-plan` | Runtime enforcement / broad-blocking / adapter execution truth |
| `needs-sibling-plan-contract` | `eventing-plan` | Durable event / journal / response consumer / read-API chain |
| `needs-sibling-plan-contract` | `data-custody-storage-plan` | Storage / retention / export / durable audit boundaries |
| `needs-sibling-plan-contract` | `screen-plan` | Capture / custody boundaries where app proof touches shared evidence surfaces |
| `host-platform-limited` | real macOS / iOS host proof | External-platform constraint only |

**End-to-end solution path**

| Slice | Scope / workpacks | Exact files / domains to touch | Validation to run | Proof to collect | Exit criteria |
|---|---|---|---|---|---|
| 1. App-plan truth repair | `WP01-WP03` plus top-level plan truth | [source-index.md](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-plan/source-index.md), [current-app-snapshot.md](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-plan/current-app-snapshot.md), [implementation-checklist.md](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-plan/implementation-checklist.md), [PROOF_INDEX.md](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-plan/PROOF_INDEX.md), [WORKPACK_INDEX.md](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-plan/WORKPACK_INDEX.md) | Markdown/link sanity; no code validation needed | `output/app-plan-proof/01-03-*/` docs-truth pack | Plan names real owners, real tests, real proof roots, real dependencies |
| 2. App ownership + architecture cleanup | `WP04-WP11` base contracts | `packages/parent-domain/src/app-game-control-authority.ts`, `packages/parent-domain/src/app-game-category-risk.ts`, `packages/app-game-domain/src/app-game-control-authority.ts`, `packages/app-game-domain/src/app-game-category-risk.ts`, any direct import sites forced by removal of re-exports | `npm run lint:architecture -- --files ...` on touched files; targeted TS tests for touched app-game-domain files | `output/app-plan-proof/04-11-*/` contract/architecture proof plus lint log | No banned re-exports in app-plan-owned slice; docs match real owner package |
| 3. Rust test rehome for app-plan-owned slices | Protocol / core / service tests | `crates/agent-protocol/src/app_game*_tests.rs` -> `crates/agent-protocol/tests/{unit,contract}/...`; `crates/agent-core/src/activity_store_app_game/*_tests.rs` -> `crates/agent-core/tests/{unit,integration,property,load}/...`; `crates/agent-service/src/activity_surface_read_models/*_tests.rs` -> `crates/agent-service/tests/{unit,integration,contract}/...` | Focused `cargo test` per crate; `cargo lint-architecture` on touched crates | Proof pack documenting moved tests and new category layout | App-plan Rust tests no longer live inline under `src` |
| 4. Windows core evidence spine closure | `WP12-WP15`, `WP29-WP49` | `packages/app-game-domain/src/app-game*.ts`, `packages/agent-protocol-domain/src/app-game-*.ts`, `crates/agent-protocol/src/app_game*.rs`, `crates/agent-core/src/activity_store_app_game/*`, `crates/agent-service/src/activity_surface_read_models/app_use.rs`, `apps/portal/src/AppGame*.tsx`, `apps/portal/src/live-activity-state.ts` | Focused Vitest + Cargo tests per touched package/crate | Windows inventory/process/foreground/sessionization/service/portal artifacts | End-to-end native-app read path proven on Windows |
| 5. Policy / approval / risk / install / notification closure | `WP16-WP28`, `WP53-WP67` | `packages/app-game-domain/src/app-game-*`, `packages/agent-protocol-domain/src/app-game-*.ts`, relevant portal panels/tests, proof scripts in `scripts/test/app-game-*.mjs` | Focused Vitest on touched TS packages + portal; architecture gates | `test-results/*/proof.json` plus `output/app-plan-proof/<wp>/` packs | Claims about approval, AI, install, notification, and platform routing are honest and evidenced |
| 6. Android proof closure | Relevant Android workpacks | Existing Android proof scripts and any touched TS/runtime sources | Targeted TS tests plus selected Android proof scripts | Emulator + Samsung artifacts | Android claims backed by real artifacts |
| 7. Linux proof closure | Relevant Linux workpacks | Existing Linux proof scripts and any touched TS/runtime sources | Targeted TS tests plus selected Linux proof scripts | WSL/Docker artifacts | Linux claims backed by real artifacts |
| 8. Source-freshness / timer-service chain | `WP74-WP108` | `packages/app-game-domain/src/app-game-source-*`, `packages/app-game-domain/src/app-game-timer-service-*`, service/protocol/portal counterparts | Focused Vitest + Cargo + architecture on touched domains | Real service-handler, read-API, response-consumer, parent-surface artifacts | Rows stop being cross-record prose and become real completion |
| 9. Final proof + rollout normalization | Final plan closure | Top-level app-plan docs + final proof roots | All scoped gates relevant to touched slices | Canonical proof tree complete | Honest “done bar” satisfied |

**First 3 atomic slices in recommended order**
1. `Slice 1: app-plan truth repair` on the five top-level app-plan docs only.
2. `Slice 2: app ownership + architecture cleanup` on the four verified re-export offenders only.
3. `Slice 3: Rust app-plan test rehome` for `crates/agent-protocol`, `crates/agent-core`, and `crates/agent-service` app-plan-owned tests out of `src` into `tests/` major categories.

**Exact coordinator asks / unblock requests**
- Decide whether `app-plan` is an execution plan or a truthful overlay on `app-game-plan`. This is the first sequencing decision because rows `51`, `68`, `70`, `73`, and much of `74+` are already shared or mirrored.
- Confirm this lane may touch real code after Slice 1, or restrict it to docs-truth only until you re-sequence sibling plans.
- Confirm `packages/app-game-domain` is the canonical TS owner for native-app work and that the `packages/parent-domain/src/app-game-*.ts` shims should be removed or rewritten rather than preserved as barrels.
- Choose the canonical app-plan proof schema so `PROOF_INDEX.md` and `implementation-checklist.md` can converge.
- Sequence `policy-control-plane-plan` before broad execution of `WP74-WP108`, because those rows depend on policy source truth and preview timer semantics.
- Sequence `portal-ux-household-surfaces-plan` before final closure of parent-visible notification/policy/status surfaces if that lane currently owns the final rendering contract.

## COORDINATOR_DECISION_REQUEST
- recommended next slice: `Slice 1: app-plan truth repair`
- recommended predecessor plans: `app-game-plan` first for shared runtime/proof ownership; `policy-control-plane-plan` second before `WP74-WP108`; `portal-ux-household-surfaces-plan` before final parent-visible UI closure
- estimated risk: `high` because the biggest current defect is plan truth mismatch against real code ownership, and later slices cross multiple sibling plans
- estimated proof difficulty: `high` because Windows/Android/Linux artifacts are feasible but numerous, and the proof schema itself must be normalized before closure is honest
- whether I should continue immediately or pause for sequencing: continue immediately on `Slice 1`; pause before widening beyond `Slice 2` unless you explicitly confirm code-touch authority and sibling-plan sequencing

## Optional Addendum

- Important earlier audit finding not spelled out crisply enough in the latest report: [PROOF_INDEX.md](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-plan/PROOF_INDEX.md) and [implementation-checklist.md](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-plan/implementation-checklist.md) currently describe conflicting proof-pack layouts. `PROOF_INDEX.md` expects a universal pack shape such as `00-scope-summary.md`, `01-negative-case-proof.md`, `02-no-claim-boundary.md`, and `16-validation-commands.log`, while `implementation-checklist.md` still requires a different artifact layout such as `00-source-snapshot.md`, `04-journal-sqlite-proof.json`, and `05-policy-action-proof.json`. This is not cosmetic; it must be normalized before any honest completion claim.
- Important earlier audit finding not spelled out crisply enough in the latest report: the entire local `output/app-plan-proof` and `output/app-game-plan-proof` roots were absent, not merely incomplete. The raw audit also found many cited `test-results/app-game-*/proof.json` artifacts absent locally, so current proof references are stale until regenerated.
- Important earlier audit finding not spelled out crisply enough in the latest report: `source-index.md` exact path mapping is unreliable in multiple concrete places. It still points at missing `packages/activity-domain/src/app-game*.ts` files and stale portal filenames such as `apps/portal/src/live-activity-panel.ts`, `apps/portal/src/activity-timeline.ts`, and `apps/portal/src/policy-preview-read-model.ts`, while the real implementation surface is centered in `packages/app-game-domain`, `apps/portal/src/AppGame*.tsx`, and `apps/portal/src/live-activity-state.ts`.
- Important earlier audit finding not spelled out crisply enough in the latest report: `implementation-checklist.md` rows `51`, `68`, `70`, and `73` explicitly point into `../app-game-plan/workpacks/...`, confirming that app-plan is not self-contained and that some later closure must be sequenced with the shared app-game plan rather than claimed locally by folder presence.
