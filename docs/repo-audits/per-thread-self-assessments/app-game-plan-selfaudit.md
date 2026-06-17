# app-game-plan

## Normalized Header

- plan/thread name: `app-game-plan`
- source thread label: `codex-a dedicated app-game-plan thread`
- source thread id: `019ed325-e4c1-78e3-83c4-0cc1b1e2b833`
- worktree: `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent`
- branch: `codex/tracking-plan-full-continuation-a`
- claimed status: not done; completion architecture report plus earlier audit findings preserved
- claimed source files/crates/packages: `docs/plans/app-game-plan/*`, `packages/app-game-domain`, `packages/portal-domain`, `apps/portal/src`, `packages/parent-domain/src/app-game*.ts`, `packages/activity-domain/src/activity-surface.ts`, `crates/agent-protocol`, `crates/agent-core`, `crates/agent-service`, `platforms/android/agent/app/src/main`
- claimed tests: `packages/app-game-domain/tests`, `packages/portal-domain/tests`, `apps/portal/tests`, inline Rust app-game tests under `crates/agent-protocol/src`, `crates/agent-core/src`, `crates/agent-service/src`
- claimed proof commands/artifacts: `scripts/test/app-game-*`, `scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs`, `test-results/app-game-*`, canonical proof root claimed as `output/app-game-plan-proof/<workpack-file-stem>/`
- claimed blockers: stale plan routing/docs, missing canonical proof root, parent-domain re-export facade failing architecture rules, shared checklist/export coordination, optional Apple-host-only proof for Apple rows
- claimed next actions: `truth-ownership`, `proof-root-normalization`, `source-policy-timer`, then notification/child-UX, Windows execution/product surface, Android/Linux proof consolidation, final strict test-placement cleanup
- obvious missing evidence fields: full scoped validation matrix not yet rerun, canonical `output/app-game-plan-proof` artifacts absent, final `docs/product-capability-checklist.md` publication unresolved, Apple host proof absent if Apple claims are promoted
- archival note: self-report only; unverified input for structural review

## Raw Self-Report

**COMPLETION_ARCHITECTURE_REPORT**

`app-game-plan` is not closeable yet, but it is not waiting on a single monolithic blocker either. The honest path is: fix plan/doc truth and canonical ownership first, remove the failing parent-domain app-game façade, normalize proof roots, then complete the real source -> policy -> timer -> notification/child UX -> adapter/product -> platform-proof chain with scoped validation and host-feasible Windows/Android/Linux proof. Final closure is gated more by false-green cleanup and shared-contract sequencing than by missing raw code volume alone.

**Read Set**
- Docs fully read: all `247` markdown files under [docs/plans/app-game-plan](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-game-plan), plus [docs/features/app-game-control.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/features/app-game-control.md), [docs/features/app-install-purchase-approval.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/features/app-install-purchase-approval.md), [docs/features/enforcement-integrity-tamper.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/features/enforcement-integrity-tamper.md), [docs/expectations/app-game-evidence.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/expectations/app-game-evidence.md), and [docs/expectations/enforcement.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/expectations/enforcement.md).
- Source and tests inspected: [packages/app-game-domain](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/app-game-domain), [packages/portal-domain](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/portal-domain), [apps/portal/src](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/apps/portal/src), [apps/portal/tests](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/apps/portal/tests), [packages/parent-domain/src](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/parent-domain/src), [packages/activity-domain/src/activity-surface.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/activity-domain/src/activity-surface.ts), [crates/agent-protocol](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/crates/agent-protocol), [crates/agent-core](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/crates/agent-core), [crates/agent-service](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/crates/agent-service), and Android package sources under [platforms/android/agent/app/src/main](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/platforms/android/agent/app/src/main).
- Proof inventory inspected: [docs/plans/app-game-plan/TEST_PROOF_EXPECTATIONS.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-game-plan/TEST_PROOF_EXPECTATIONS.md), [scripts/test](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/scripts/test) app-game proof scripts, `test-results/app-game-*`, and the absence of `output/app-game-plan-proof`.

**Current Truth Snapshot**

| Bucket | Current truth |
| --- | --- |
| `done` | Real TS domain exists in [packages/app-game-domain](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/app-game-domain); real Rust protocol/core/service app-game surfaces exist; real portal/domain surfaces exist; scoped app-game-domain unit tests run and pass for representative slices. |
| `partial` | Source freshness, policy preview/readiness, timer, notification, child UX, adapter readiness, platform-proof status, Windows/Android/Linux proof surfaces all exist, but many are still visibility/preflight/manual-required layers rather than full runtime/product closure. |
| `false-green` | [docs/plans/app-game-plan/source-index.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-game-plan/source-index.md) routes to many nonexistent files; [WORKPACK_INDEX.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-game-plan/WORKPACK_INDEX.md) marks rows “checked” while the canonical proof root is missing; `169` `packages/parent-domain/src/app-game*.ts` re-export shims make the surface look wired while failing architecture rules; broad TS test category trees exist, but most non-unit categories hold only one file each. |
| `missing` | Canonical plan proof root `output/app-game-plan-proof/<workpack-stem>/`; direct canonical ownership docs; real broad Windows blocking proof; real Android Device/Profile Owner proof if required; real Linux restriction/enforcement proof if claimed; final shared checklist/export contract reconciliation. |

**Plan Closure Definition**
- “Actually done” means the plan’s docs, source ownership, tests, proof artifacts, and status language all agree on one real system.
- [packages/app-game-domain](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/app-game-domain) must be the canonical TS owner for app-game behavior. The current app-game façade in [packages/parent-domain/src](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/parent-domain/src) cannot remain as banned re-exports.
- The live chain must be real and validated for host-feasible slices: source capture/preflight -> source freshness -> policy preview/readiness -> timer/read model -> notification/child UX -> adapter readiness/preflight/result/execute -> parent surface.
- Proof must exist in both `test-results/...` and `output/app-game-plan-proof/<workpack-stem>/`.
- Apple rows may remain manual-required or external-proof-required; Windows, Android, and Linux rows may not.

**Code Surface And Ownership**

| Surface | Exact paths | Truth owner | Closure status |
| --- | --- | --- | --- |
| TS canonical domain | [packages/app-game-domain/src](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/app-game-domain/src) | `app-game-plan` | Real owner, but docs and consumers are not fully reconciled |
| Rust protocol/core/service | [crates/agent-protocol/src](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/crates/agent-protocol/src), [crates/agent-core/src](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/crates/agent-core/src), [crates/agent-service/src](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/crates/agent-service/src) | `app-game-plan` runtime chain | Real owner, but tests are still mostly inline under `src` |
| Parent portal/domain | [packages/portal-domain/src](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/portal-domain/src), [apps/portal/src](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/apps/portal/src) | `app-game-plan` consumer surface | Real owner, still needs stronger integration/e2e proof |
| Android package proof surface | [platforms/android/agent/app/src/main](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/platforms/android/agent/app/src/main) | `app-game-plan` host-feasible proof path | Real and usable from this host |
| Activity bridge | [packages/activity-domain/src/activity-surface.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/activity-domain/src/activity-surface.ts) | consumer only | Thin bridge, not canonical owner |
| Parent-domain app-game façade | [packages/parent-domain/src/app-game*.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/parent-domain/src) | should not remain owner | Failing architecture gate; must be removed or replaced |

**Test Surface Inventory**

| Surface | Inventory | Truth |
| --- | --- | --- |
| TS app-game domain | [packages/app-game-domain/tests](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/app-game-domain/tests) has populated major categories; `unit` has `170` files, `security` has `9`, most other categories have `1` file each | No empty major category folders, but non-unit breadth is thin and easy to overread |
| TS portal domain | [packages/portal-domain/tests](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/portal-domain/tests) mirrors the same category tree; `unit` has `27` files, `security` has `9`, most other categories have `1` file each | Same issue: categories exist, but many are singleton suites |
| Portal app | [apps/portal/tests](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/apps/portal/tests) has `24` files across feature folders `activity`, `diagnostics`, `local-ai`, `logging`, `portal`, `screen` | Real tests exist, but they are not organized by major categories like `unit`, `integration`, `e2e`, or `playwright` |
| TS inline/src tests | Quick scan found no `.test.ts`/`.spec.ts` under `packages/app-game-domain/src`, `packages/portal-domain/src`, or `apps/portal/src` | Good |
| Rust app-game tests | `44` inline `_tests.rs` / `app_game*_tests.rs` files across `crates/agent-protocol/src`, `crates/agent-core/src`, and `crates/agent-service/src` | These should move or be mirrored into crate `tests/` major categories before claiming strict closure |

**Missing Or Weak Coverage**
- `integration`: applicable and still too thin for the source -> service -> portal chain.
- `e2e` / `playwright`: applicable for the parent portal and currently not visible as a strong dedicated app-game flow.
- `contract`: present in TS folders, but Rust contract suites should not stay buried in `src/*_tests.rs`.
- `property`: applicable for timer/state/idempotency behavior and currently thin.
- `security`: present but still incomplete against the checklist’s negative no-claim cases.
- `load`: folders exist, but there is no evidence of substantial load proof for the app-game chain.
- `unit`: strong in TS, uneven in Rust due to placement.

**Proof Inventory**

| State | Exact paths | Truth |
| --- | --- | --- |
| real | [scripts/test](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/scripts/test) contains large app-game proof coverage; `test-results` exists | Real proof tooling and some real artifacts exist |
| stale | [docs/plans/app-game-plan/source-index.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-game-plan/source-index.md), many workpack proof references in [implementation-checklist.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-game-plan/implementation-checklist.md) | Docs still imply proof layouts and source ownership that do not match the checkout |
| missing | `output/app-game-plan-proof/<workpack-file-stem>/` | Canonical plan proof root from [TEST_PROOF_EXPECTATIONS.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-game-plan/TEST_PROOF_EXPECTATIONS.md) does not exist |

Canonical proof root path:
```text
output/app-game-plan-proof/<workpack-file-stem>/
```

**Scoped Validation Inventory**

| Command | State | Notes |
| --- | --- | --- |
| `npm run test --workspace @ocentra-parent/app-game-domain -- tests/unit/app-game-control-authority.test.ts tests/unit/app-install-purchase-approval.test.ts` | pass | `2` files, `20` tests |
| `npm run lint:architecture -- --files packages/parent-domain/src/app-game-adapter-execution-readiness.ts packages/parent-domain/src/app-game-control-authority.ts packages/parent-domain/src/app-game-install-store-handoff.ts` | fail | `BARREL/REEXPORT BAN` on all three files |
| `npm run build --workspace @ocentra-parent/app-game-domain` | unrun | required before closure |
| `npm run test --workspace @ocentra-parent/app-game-domain` | unrun | required before closure |
| `cargo test -p ocentra-parent-agent-protocol app_game` | unrun | required before closure |
| `cargo test -p ocentra-parent-agent-core app_game` | unrun | required before closure |
| `cargo test -p ocentra-parent-agent-service app_game` | unrun | required before closure |
| `npm run test --workspace @ocentra-parent/portal -- app` | unrun | required before closure |
| Focused proof scripts under `scripts/test/app-game-*` | mostly unrun in this audit turn | many are required for closure |

**Dependency Graph And Blocker Taxonomy**

| Bucket | Exact dependency | Truth |
| --- | --- | --- |
| `local-now` | Fix [docs/plans/app-game-plan/source-index.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-game-plan/source-index.md); remove `packages/parent-domain/src/app-game*.ts` façade; normalize proof roots; complete source/policy/timer/notification/adapter chains; run Windows/Android/Linux proof | Can proceed immediately on this branch |
| `needs-coordinator-sequencing` | Final publication to [docs/product-capability-checklist.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/product-capability-checklist.md); any remaining shared export/checklist ownership churn; decision on whether provider runtime and Windows broad-blocking enforcement are in scope for “done” | Does not block local-now work, but blocks honest final closure language |
| `needs-sibling-plan-contract` | Shared native app/game routing and any remaining shared `packages/parent-domain/package.json` contract; [docs/plans/app-plan](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-plan) is the obvious sibling reference point because `app-game-plan` cross-references it for shared native app evidence/routing | Needed before final cross-plan closure, not before first local slice |
| `host-platform-limited` | Real macOS and iOS runtime proof if Apple rows must be promoted above manual-required/external-proof-required | Apple-host only |

**Platform Feasibility**

| Platform path | What can be proven now |
| --- | --- |
| Windows host | Architecture lint, TS/Rust scoped validation, Windows owned-process time-limit path, Windows local policy evidence, Windows broad-blocking authority preflight, portal/service/domain integration |
| Android Studio + Samsung device | Package install/launch, UsageEvents capability/preflight/count/sample/replay, Accessibility declaration/runtime/overlay preflight, authority preflight, child-runtime-local proof scripts |
| Linux via WSL/Docker | WSL runtime, Docker host preflight, WSLg/X11/Wayland readiness, foreground-capture readiness, active-window tool probing |
| Apple-host only | macOS MDM/Endpoint/System Extension proof and iOS FamilyControls/ManagedSettings/DeviceActivity proof |

**No-Hand-Wave Execution Plan**

| Ordered slice | Files/domains to touch | Validation | Proof | Exit criteria |
| --- | --- | --- | --- | --- |
| `1. truth-ownership` | [docs/plans/app-game-plan/source-index.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-game-plan/source-index.md), [current-app-game-snapshot.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-game-plan/current-app-game-snapshot.md), [implementation-checklist.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-game-plan/implementation-checklist.md), `packages/parent-domain/src/app-game*.ts`, consumers that still import that façade | `npm run lint:architecture -- --files packages/app-game-domain packages/parent-domain packages/portal-domain apps/portal docs/plans/app-game-plan` | source snapshot under `output/app-game-plan-proof/02-source-index-and-doc-reconciliation/` | docs route to real files; app-game imports no longer depend on banned re-exports; architecture gate passes |
| `2. proof-root-normalization` | proof harnesses in [scripts/test](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/scripts/test), plan proof docs | rerun touched proof scripts only | `test-results/<proof>/proof.json` plus `output/app-game-plan-proof/<workpack-stem>/proof.json` and `00-source-snapshot.md` | every claimed-complete workpack has canonical artifacts |
| `3. source-policy-timer` | `crates/agent-core/src/activity_store_app_game/*`, `crates/agent-service/src/activity_api/app_game_*`, `packages/app-game-domain/src/app-game-source-freshness-*.ts`, `app-game-policy-*.ts`, `app-game-timer-*.ts`, portal/domain timer and policy panels | app-game-domain build/test, protocol/core/service app_game tests, portal app tests | `scripts/test/app-game-source-freshness-policy-consumption-proof.mjs`, `app-game-source-freshness-preview-gate-proof.mjs`, `app-game-policy-readiness-service-proof.mjs`, `app-game-policy-readiness-portal-renderer-proof.mjs`, `app-game-timer-proof-chain.mjs` | live parent-safe policy/timer chain exists and is backed by service/read-model/portal proof |
| `4. notification-child-ux-runtime` | `packages/app-game-domain/src/app-game-notification-*.ts`, `app-game-child-facing-ux*.ts`, `crates/agent-service/src/activity_api/app_game_child_runtime_transport_receipt_*`, portal notification surfaces | package tests, service tests, portal tests | `scripts/test/app-game-notification-*.mjs`, `app-game-child-device-delivery-readiness-proof.mjs`, `app-game-child-runtime-transport-receipt-*.mjs` | outbox/scheduler/provider/preference/receipt/parent-surface behavior is real, not just status rows |
| `5. windows-execution-product-surface` | `packages/app-game-domain/src/app-game-adapter-*.ts`, `app-game-platform-proof-status.ts`, protocol/service adapter files, portal adapter panel | app-game-domain tests, protocol/service app_game tests, portal tests, architecture lint | `scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs`, `app-game-adapter-execution-readiness-proof.mjs`, `app-game-adapter-dispatch-preflight-live-handoff-proof.mjs`, `app-game-adapter-dispatch-execution-audit-proof.mjs`, `app-game-blocking-time-limit-done-gate-proof.mjs` | scoped Windows owned-process execute path is real; broad blocking remains honestly blocked unless separately proved |
| `6. android-linux-proof-consolidation` | Android package sources under [platforms/android/agent/app/src/main](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/platforms/android/agent/app/src/main), Linux proof models in `packages/app-game-domain/src/app-game-linux-*.ts`, platform proof status surfaces | touched package/domain tests and service payload tests | Android UsageEvents/Accessibility/authority/physical-device proofs; Linux WSL/Docker/foreground/active-window proofs | Windows/Android/Linux rows have honest proof-backed status |
| `7. strict test-placement cleanup and final closure sync` | Rust test relocation into crate `tests/` major categories, any thin TS non-unit suites that need real coverage, final docs/status sync | all scoped package/crate validations above | refreshed canonical proof root for all claimed-complete workpacks | tests are placed honestly, proof roots are real, docs/status no longer overclaim |

**First Coordinator Ask**
- If any sibling must move before final closure, it is the sibling owner of shared native-app evidence routing and final checklist/export contract, most likely [docs/plans/app-plan](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-plan) or the active lane holding [docs/product-capability-checklist.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/product-capability-checklist.md) and `packages/parent-domain/package.json`.
- Why: `app-game-plan` can do its local-now closure work immediately, but final honest closure still requires that shared checklist/export contract to stop deferring completion language across many workpacks.

**Strict Done Bar**
- [source-index.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-game-plan/source-index.md) and related plan docs point only at real current files.
- No app-game façade remains in `packages/parent-domain/src` that fails architecture rules.
- TS and Rust app-game tests are placed honestly under `tests/` major categories where applicable, with Rust inline `src/*_tests.rs` no longer serving as the primary closure evidence.
- Scoped validation is green for touched packages/crates/domains.
- Canonical proof artifacts exist under `output/app-game-plan-proof/<workpack-stem>/`.
- Windows, Android, and Linux claims are backed by real proof; Apple claims are either still external/manual-required or proved on the correct host.
- Shared checklist/export dependencies are resolved or explicitly kept out of completion scope by coordinator decision.

**COORDINATOR_DECISION_REQUEST**
- Recommended next slice: `truth-ownership`
- Recommended predecessor plans: `app-plan` only for final shared routing/checklist/export reconciliation; it does not need to precede local-now work
- Estimated risk: high
- Estimated proof difficulty: high
- Continue immediately or pause for sequencing: continue immediately on `truth-ownership`; pause only before final closure wording if the shared checklist/export owner is still unresolved

## Optional Addendum

- Earlier audit passes explicitly verified that representative exact paths still cited in [docs/plans/app-game-plan/source-index.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/app-game-plan/source-index.md) do not exist in this checkout: `packages/activity-domain/src/app-game.ts`, `packages/activity-domain/tests/app-game.test.ts`, `apps/portal/src/activity-timeline.ts`, `apps/portal/src/policy-preview-panel.ts`, `apps/portal/src/policy-preview-read-model.ts`, `apps/portal/src/portal-capability-guidance.ts`, `apps/portal/src/portal-device-rule-scope.ts`, and `packages/parent-domain/tests/app-game-source-gated-policy-preview-timer-handoff.test.ts`.
- Earlier audit passes also identified six explicit open reference workpacks still called out by the plan state/index and not ready to be treated as complete: `workpacks/app-control-capability-guide.md`, `workpacks/app-control-schema-proposal.md`, `workpacks/app-control-settings-inventory.md`, `workpacks/game-control-capability-guide.md`, `workpacks/game-control-schema-proposal.md`, and `workpacks/game-control-settings-inventory.md`.
