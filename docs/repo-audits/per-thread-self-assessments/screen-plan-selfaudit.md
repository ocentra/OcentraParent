# screen-plan

## Normalized Header

- plan/thread name: `screen-plan`
- source thread label: `codex-a lane manager delegation`
- source thread id: `019ed32c-70ec-7782-aad3-c4e3e5c6b5c8`
- worktree: `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent`
- branch: `codex/tracking-plan-full-continuation-a`
- claimed status: `not complete; paused for sequencing; approved first slice is truth and proof-contract repair`
- claimed source files/crates/packages: `packages/screen-domain`, `packages/tracking-domain`, `packages/activity-domain`, `packages/portal-domain`, `apps/portal`, `crates/agent-protocol`, `crates/agent-core`, `crates/agent-service`, `crates/screen-capture-adapter`, stale `packages/parent-domain` shims
- claimed tests: targeted `@ocentra-parent/screen-domain`, `@ocentra-parent/tracking-domain`, `@ocentra-parent/activity-domain`, and `@ocentra-parent/portal-domain` screen tests passed; Rust screen tests still inline under `src` and should move into proper `tests/` major categories
- claimed proof commands/artifacts: `scripts/test/screen-*.mjs`, `scripts/test/screen-ai-*.mjs`, `scripts/test/tracking-retention-*.mjs`; canonical roots should be `docs/proof/screen-plan/slice-0N-<slug>.md` and `output/screen-plan-proof/<proof-id>/`
- claimed blockers: `local-now`: false-green docs, missing local proof roots, stale shim ownership, inline Rust tests, unrun Windows/Android/Linux proof regeneration; `needs-coordinator-sequencing`: plan held out of immediate execution wave and should follow tracking-plan; `needs-sibling-plan-contract`: `screen-ai-pipeline-plan`, `browser-plan`, `ai-plan`, policy/enforcement proof where claims remain in scope; `host-platform-limited`: real macOS and iOS proof
- claimed next actions: `Slice A: truth and proof-contract repair`, then shim removal/retargeting, contract closure, test relocation, runtime/proof regeneration, and final closure audit
- obvious missing evidence fields: no local `docs/proof/screen-plan` manifests, no local `output/screen-plan-proof` or `output/screen-ai-pipeline-proof` roots, no rerun Rust screen proof on this branch, no rerun Playwright/e2e proof on this branch, no current Apple-host artifacts
- archival note: self-report only; unverified input for structural review

## Raw Self-Report

**Executive Summary**
`screen-plan` is not closeable on this branch today. The main failure is false-green plan state: [PLAN_STATE.md](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/screen-plan/PLAN_STATE.md:57) claims `100/100 checked`, while [WORKPACK_INDEX.md](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/screen-plan/WORKPACK_INDEX.md:39) still shows `22` open workpacks and [PLAN_HEALTH.md](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/screen-plan/PLAN_HEALTH.md:29) already warns about the mismatch. Real implementation exists across `screen-domain`, `tracking-domain`, `activity-domain`, `agent-protocol`, `agent-core`, `agent-service`, `screen-capture-adapter`, `portal-domain`, and `apps/portal`, and targeted TypeScript tests pass, but the proof roots are missing locally, ownership is stale, and several key Rust screen tests still live inline under `src`. The honest path is: repair docs/proof contract, retarget stale `parent-domain` shim ownership, finish remaining contract/runtime gaps, move inline Rust tests into proper `tests/` categories, regenerate Windows/Android/Linux proof, then run the screen closure audit with explicit sibling-plan dependencies.

**Current truth snapshot**

| Area | Exact files/surfaces read or checked | Truth |
|---|---|---|
| Plan docs | Every `.md` under `docs/plans/screen-plan`, including `AGENTS.md`, `PLAN_STATE.md`, `PLAN_HEALTH.md`, `WORKPACK_INDEX.md`, `NEXT_ACTIONS.md`, `PROOF_INDEX.md`, `TEST_PROOF_EXPECTATIONS.md`, `PLAN_EXECUTION_BLUEPRINT.md`, `implementation-checklist.md`, `source-index.md`, all workpacks `01-40`, `workpacks/screen-control-settings-inventory.md`, `workpacks/screen-evidence-analysis-capability-guide.md`, `workpacks/screen-evidence-analysis-schema-proposal.md` | `false-green` |
| Feature/expectation docs | `docs/features/screen-evidence-analysis.md`, `docs/features/screen-visibility-live-view.md`, `docs/expectations/screen-evidence.md`, `docs/architecture/local-screen-evidence-analysis-queue.md` | `partial` |
| Source ownership | `packages/{screen-domain,tracking-domain,activity-domain,portal-domain,parent-domain}`, `apps/portal`, `crates/{agent-protocol,agent-core,agent-service,screen-capture-adapter}` | `partial` |
| Test surface | `packages/screen-domain/tests/unit/*`, `packages/tracking-domain/tests/{unit,contract}/*`, `packages/activity-domain/tests/{unit,contract}/*`, `packages/portal-domain/tests/unit/*`, `apps/portal/tests/*`, `apps/portal/e2e/screen-summary-ui-proof.spec.ts`, Rust inline screen tests under `crates/*/src/*_tests.rs` | `partial` |
| Proof surface | `scripts/test/screen-*.mjs`, `scripts/test/screen-ai-*.mjs`, `scripts/test/tracking-retention-*.mjs`, plus local `output/` and `docs/proof/screen-plan` checks | `missing` locally |
| Validation | Focused architecture and TS package tests run; no repo-wide validation | `partial` |
| Platform proofability | Windows, Android emulator/device, Linux via WSL/Docker feasible here; real macOS/iOS host execution not | `partial` |

**Exact docs/source/tests/proof read**
- Docs read completely: every markdown file in `docs/plans/screen-plan`, plus the named feature, expectation, and architecture docs above.
- Source surfaces inspected or enumerated: `packages/screen-domain/src/*`, `packages/tracking-domain/src/*`, `packages/activity-domain/src/{activity-surface.ts,capture.ts,screen-vlm-journal-read-model.ts}`, `packages/portal-domain/src/{screen-summary-panel.ts,live-activity-state.ts}`, `apps/portal/src/{live-activity-state.ts,screen-settings-service-command-state.ts}`, `crates/agent-protocol/src/{screen_evidence.rs,screen_settings.rs}`, `crates/agent-core/src/{screen_evidence_queue.rs,activity_store_screen_evidence.rs,screen_event_runtime.rs}`, `crates/agent-service/src/{screen_settings_api.rs,screen_ai_analysis_runtime.rs,screen_ai_retention_sweeper_runtime.rs,screen_ai_service_event_subscription/live_view_service_runtime.rs}`, `crates/screen-capture-adapter/src/{lib.rs,trigger_scheduler.rs,desktop_xcap.rs,linux_x11.rs}`.
- Tests read or enumerated: key TS tests in `screen-domain`, `tracking-domain`, `activity-domain`, `portal-domain`, `apps/portal`; Rust inline tests referenced by `crates/agent-protocol/src/lib.rs`, `crates/agent-core/src/lib.rs`, and `crates/agent-service/src/lib.rs`.
- Proof read or enumerated: all `screen-*`, `screen-ai-*`, `tracking-retention-*`, and related proof scripts in `scripts/test/`; checked that `docs/proof/screen-plan` and `output/screen-plan-proof` are missing locally.

**Completion definition**
- The plan is actually done only when `screen-plan` docs, source ownership, categorized tests, proof artifacts, and scoped validation all agree on the same claims.
- No checked workpack can rely on stale docs, deleted proof roots, placeholder folders, or inline-only test placement.
- Canonical code owners must be the real ones: `screen-domain`, `tracking-domain`, `activity-domain`, `agent-protocol`, `agent-core`, `agent-service`, `screen-capture-adapter`, `portal-domain`, `apps/portal`.
- Canonical proof must live under `output/screen-plan-proof/<proof-id>/` with matching `docs/proof/screen-plan/slice-0N-<slug>.md` manifests. The current `PROOF_INDEX.md` vs `implementation-checklist.md` proof-pack mismatch must be resolved first.
- Rust screen tests must end in proper crate `tests/` major categories, not inline `src/*_tests.rs`, before final completion is claimed.
- Final whole-plan closure also requires explicit treatment of sibling-plan contracts and Apple-host-only artifacts, not silent omission.

**Code surface and ownership**

| Surface | Real owner now | Problem to fix |
|---|---|---|
| Screen contracts, settings, result, live-view gating | `packages/screen-domain/src/*` | Plan docs still point at stale `activity-domain` and `parent-domain` paths |
| Tracking retention runtime and proof contracts | `packages/tracking-domain/src/*` | Plan docs and shims blur screen vs tracking ownership |
| Activity read model, projection, capture contract bridge | `packages/activity-domain/src/{activity-surface.ts,capture.ts,screen-vlm-journal-read-model.ts}` | Needs honest mapping in `source-index.md` and closure docs |
| Rust protocol shapes | `crates/agent-protocol/src/{screen_evidence.rs,screen_settings.rs}` | Tests still inline under `src` |
| Queue, store, event runtime | `crates/agent-core/src/{screen_evidence_queue.rs,activity_store_screen_evidence.rs,screen_event_runtime.rs}` | Tests still inline under `src`; proof roots absent |
| Service API/runtime/live-view/deletion | `crates/agent-service/src/{screen_settings_api.rs,screen_ai_analysis_runtime.rs,screen_ai_retention_sweeper_runtime.rs,screen_ai_service_event_subscription/*}` | Tests still inline under `src`; proof roots absent |
| Platform adapter | `crates/screen-capture-adapter/src/*` | Needs regenerated Windows/Android/Linux proof tied to current code |
| Portal domain / app UI | `packages/portal-domain/src/screen-summary-panel.ts`, `apps/portal/src/*` | Current Playwright proof file is not under a `tests/` major category |
| Stale shims | `packages/parent-domain/src/{screen-control-catalog.ts,screen-control-catalog-schema.ts,screen-control-catalog-metadata.ts,screen-control-catalog-data-0.ts,screen-control-catalog-data-1.ts,screen-control-catalog-data-2.ts,tracking-retention-runtime.ts,tracking-retention-runtime-artifact-gate-proof.ts}` | Real architecture problem; must be removed or fully retargeted from plan ownership |

**Test/proof reorganization and missing coverage**
- Current TypeScript test inventory is real and non-empty:
  - `packages/screen-domain/tests/unit/*`
  - `packages/tracking-domain/tests/unit/*` and `tests/contract/*`
  - `packages/activity-domain/tests/unit/*` and `tests/contract/*`
  - `packages/portal-domain/tests/unit/*`
  - `apps/portal/tests/*`
  - `apps/portal/e2e/screen-summary-ui-proof.spec.ts`
- No screen-owned empty test-category folders were found in these package/app surfaces. The false-green is in docs/proof references, not empty test directories.
- Rust screen tests that must move out of inline `src`:
  - `crates/agent-protocol/src/screen_evidence_tests.rs`
  - `crates/agent-protocol/src/screen_settings_tests.rs`
  - `crates/agent-core/src/screen_evidence_queue_tests.rs`
  - `crates/agent-core/src/activity_store_screen_evidence_tests.rs`
  - `crates/agent-service/src/screen_ai_analysis_runtime_tests.rs`
  - `crates/agent-service/src/screen_ai_retention_sweeper_runtime_tests.rs`
  - `crates/agent-service/src/screen_settings_api_tests.rs`
  - `crates/agent-service/src/screen_ai_service_event_subscription/live_view_service_runtime_tests.rs`
- Recommended final category layout:
  - `crates/agent-protocol/tests/contract/*`
  - `crates/agent-core/tests/unit/*`
  - `crates/agent-core/tests/integration/*`
  - `crates/agent-service/tests/unit/*`
  - `crates/agent-service/tests/integration/*`
  - `crates/agent-service/tests/security/*`
  - `apps/portal/tests/playwright/*` or `apps/portal/tests/e2e/*`
- Missing major categories where actually applicable:
  - `integration`: applicable and incomplete for journal -> store -> activity surface -> portal flow.
  - `e2e` / `playwright`: applicable and partially present; current screen proof spec should live under a `tests/` major category.
  - `contract`: present in TS, but Rust contract tests need proper `tests/` placement.
  - `property`: applicable for bounded OCR snippet length, confidence bounds, TTL/backpressure bounds; currently absent.
  - `security`: applicable for raw upload rejection, malformed output rejection, approval/audit gating, tamper handling; currently partial.
  - `load`: applicable only to queue backpressure, cadence, and possibly live-view frame handling. No dedicated screen-owned load suite exists now.
- Proof inventory is real at the harness level in `scripts/test/*`, but stale or missing at the artifact level. Scripts do not count as proof until they emit current artifacts.

**Proof inventory**

| Proof surface | Current truth | Canonical root |
|---|---|---|
| `docs/proof/screen-plan` manifests | missing locally | `docs/proof/screen-plan/slice-01-<slug>.md`, `slice-02-<slug>.md`, `slice-03-<slug>.md` |
| Workpack proof roots | missing locally despite checklist citations | `output/screen-plan-proof/<proof-id>/` |
| Cross-plan final path proof | missing locally | `output/screen-ai-pipeline-proof/<proof-id>/` |
| Script harnesses | real files exist under `scripts/test/` | must emit current artifacts and `16-validation-commands.log` |
| Canonical proof-pack contract | inconsistent in docs today | pick one contract and normalize `PROOF_INDEX.md`, `TEST_PROOF_EXPECTATIONS.md`, `PLAN_EXECUTION_BLUEPRINT.md`, and `implementation-checklist.md` to it |

**Scoped validation inventory**

| Command | Status | Notes |
|---|---|---|
| `npm run lint:architecture -- --files packages/parent-domain/src/screen-control-catalog.ts packages/parent-domain/src/tracking-retention-runtime.ts` | fail | 2 barrel/re-export ban errors; this is a real ownership/architecture issue |
| `npm run test --workspace @ocentra-parent/screen-domain -- tests/unit/screen-evidence.test.ts tests/unit/screen-control-policy-catalog.test.ts tests/unit/screen-live-view-platform-permission.test.ts` | pass | 3 files, 18 tests |
| `npm run test --workspace @ocentra-parent/tracking-domain -- tests/contract/tracking-retention-product-readiness-proof.test.ts` | pass | targeted contract surface green |
| `npm run test --workspace @ocentra-parent/activity-domain -- tests/contract/screen-vlm-journal-read-model.test.ts` | pass | targeted projection contract green |
| `npm run test --workspace @ocentra-parent/portal-domain -- tests/unit/screen-summary-panel.test.ts` | pass | targeted portal intent surface green |
| Rust screen tests | unrun in audit | should be rerun after test relocation, not left inline |
| Playwright screen proof | unrun in audit | should be rerun after proof-contract repair |
| Screen proof harness scripts | unrun in audit | would create artifacts; defer until writable execution slice |

**Dependency graph**

| Dependency | Why it matters | Final closure blocker or later polish |
|---|---|---|
| `screen-ai-pipeline-plan` | final stacked product-path proof, closure audit consumption | final closure blocker |
| `browser-plan` | managed-browser capture, structured extraction, authenticated-account evidence if in scope | final closure blocker if those claims stay in scope; later polish if explicitly non-claimed |
| `ai-plan` | OCR/VLM provider-quality and some no-raw-transfer contracts referenced by final stacked proof | final closure blocker for product-ready AI claims |
| Policy/enforcement sibling surfaces | required only if screen plan keeps enforcement-handoff or policy-action claims active | blocker for those claims, not for basic capture/retention closure |
| `tracking-plan` | overlaps retention/runtime truth and shared `tracking-domain` ownership; reduces churn before screen slice A | sequencing dependency, not a hard technical blocker |
| `child-runtime`, `logging`, `LAN`, `auth/session`, `setup/device-trust` clarifications | repo-level sequencing and conflict reduction | later sequencing leverage, not direct technical blockers for screen code itself |

**Platform feasibility**

| Platform path | What can be proven here | Truth |
|---|---|---|
| Windows host | real capture, scope matrix, managed-browser capture, degraded/protected behavior, queue/deletion, portal/service proofs | feasible now |
| Android Studio emulator | MediaProjection consent/session, foreground-service capture, deletion, no-silent-background behavior | feasible now |
| Synced Samsung device | physical-device MediaProjection capture, deletion, local OCR/VLM on physical screenshot if targeted | feasible now when scheduled |
| Linux via WSL/Docker | WSLg/X11 capture proof, Linux source/doc gating, some runtime/proof harness paths | feasible now |
| macOS | real ScreenCaptureKit pixel proof and host-native permission proof | Apple-host-only |
| iOS | real ReplayKit pixel proof and host-native artifacts | Apple-host-only |

**Ordered slices**

| Slice | Files/domains to touch | Validation to run | Proof to collect | Exit criteria |
|---|---|---|---|---|
| `A. Truth and proof-contract repair` | `docs/plans/screen-plan/{source-index.md,PLAN_STATE.md,PLAN_HEALTH.md,WORKPACK_INDEX.md,NEXT_ACTIONS.md,PROOF_INDEX.md,TEST_PROOF_EXPECTATIONS.md,PLAN_EXECUTION_BLUEPRINT.md,implementation-checklist.md}` and `docs/proof/screen-plan/*` | doc consistency only; no repo-wide validation | create manifest templates and choose canonical root `output/screen-plan-proof/<proof-id>/` | docs no longer disagree on checkmarks, proof root, or ownership |
| `B. Ownership shim removal` | `packages/parent-domain/src/{screen-control-catalog.ts,screen-control-catalog-schema.ts,screen-control-catalog-metadata.ts,screen-control-catalog-data-0.ts,screen-control-catalog-data-1.ts,screen-control-catalog-data-2.ts,tracking-retention-runtime.ts,tracking-retention-runtime-artifact-gate-proof.ts}` and any remaining consumers | focused `npm run lint:architecture -- --files ...` | validation logs only | no screen/tracking plan owner relies on banned `parent-domain` re-exports |
| `C. Contract boundary completion` | `packages/screen-domain/src/*`, `packages/tracking-domain/src/*`, `packages/activity-domain/src/{activity-surface.ts,capture.ts,screen-vlm-journal-read-model.ts}`, `crates/agent-protocol/src/{screen_evidence.rs,screen_settings.rs}` | targeted TS contract tests + Rust protocol tests | contract proof roots for `WP03,05,06,07,08,18,20,39` | schema/state/result/retention/live-view contracts match docs and fail closed |
| `D. Test relocation` | move inline Rust screen tests into `crates/*/tests/{contract,unit,integration,security}`; move `apps/portal/e2e/screen-summary-ui-proof.spec.ts` under `tests/playwright` or `tests/e2e` | focused cargo test commands by new category | validation logs | no final screen-owned Rust tests remain inline under `src` |
| `E. Queue/store/policy runtime` | `crates/agent-core/src/{screen_evidence_queue.rs,activity_store_screen_evidence.rs}`, `crates/agent-service/src/{screen_ai_analysis_runtime.rs,screen_ai_retention_sweeper_runtime.rs,screen_ai_retention_sweeper_deletion_events.rs,screen_ai_service_capture_event_builder.rs,activity_surface_store.rs,activity_surface_read_models.rs}`, portal read-model surfaces | focused Rust + portal integration tests | proof roots for `WP14,15,16,21,22,23` | queue/store/deletion/policy dry-run are real, redacted, and surfaced |
| `F. Local platform proof` | `crates/screen-capture-adapter/src/*`, Android/Linux scripts and adapters, portal surfaces as needed | focused adapter/service tests + targeted proof scripts | Windows, Android emulator, Samsung physical, Linux/WSL artifact roots | all feasible non-Apple platform claims are proved on this host/toolchain |
| `G. Live-view fail-closed closure` | `packages/screen-domain/src/{screen-live-view-platform-permission.ts,screen-live-view-service-session.ts,screen-live-view-parent-ui-persistence.ts}`, `crates/agent-service/src/screen_ai_service_event_subscription/*`, `apps/portal/src/{live-activity-state.ts,screen-settings-service-command-state.ts}` | TS/Rust/live-view scoped tests | `live-view-*` proof roots | live-view decision and fail-closed gates are fully consistent |
| `H. Final closure audit` | screen closure docs plus cross-plan proof references | focused closure scripts only | `external-gates`, `screen-plan-closure-audit`, final stacked proof refs | no false-green remains; all surviving gaps are explicit and intentional |

**Blocker taxonomy**

| Bucket | Exact blockers |
|---|---|
| `local-now` | false-green docs; missing local `docs/proof/screen-plan` and `output/screen-plan-proof` roots; stale `parent-domain` shim ownership; inline Rust tests still under `src`; unrun Windows/Android/Linux proof regeneration; unrun focused Rust/Playwright validation |
| `needs-coordinator-sequencing` | this plan is held out of the immediate execution wave; a writable lane and sequencing slot are required before slice A starts; `tracking-plan` moving first reduces churn around retention ownership |
| `needs-sibling-plan-contract` | final `screen-ai-pipeline-plan` product-path proof; `browser-plan` evidence if managed-browser/authenticated-account claims stay in scope; `ai-plan` proof where final closure wants product-ready OCR/VLM claims; policy/enforcement sibling proof if those claims remain active |
| `host-platform-limited` | real macOS ScreenCaptureKit proof; real iOS ReplayKit proof |

**First coordinator ask / unblock request**
- If another plan should move first, it should be `tracking-plan`.
- Reason: the current stale ownership problem crosses directly through `packages/tracking-domain/src/tracking-retention-runtime.ts` and its old `parent-domain` shim surface. Letting `tracking-plan` finish its truth and ownership clarification first reduces immediate doc churn and import-retarget conflict in `screen-plan` slice A, while not changing the approved screen execution model.

**Strict done bar**
- `screen-plan` docs no longer contain contradictory checkmark/proof/ownership claims.
- `parent-domain` re-export shims are no longer part of the screen plan ownership story and the focused architecture gate is green.
- Screen-owned Rust tests are organized under proper `tests/` major categories, not inline `src/*_tests.rs`.
- All feasible Windows/Android/Linux claims for this branch have current proof artifacts under the canonical screen proof root.
- Any Apple-host-only claims are either backed by real external artifacts or explicitly non-claimed.
- Final closure audit and any required sibling-plan proof refs are present and current.
- No empty folders, stale proof paths, or old checkmarks are being counted as evidence.

**COORDINATOR_DECISION_REQUEST**
- Recommended next slice: `A. Truth and proof-contract repair`
- Recommended predecessor plans: `tracking-plan` first; `screen-ai-pipeline-plan` only before final closure, not before slice A
- Estimated risk: `medium-high`, because the plan is cross-cutting and the docs/proof contract is currently inconsistent
- Estimated proof difficulty: `high`, mostly due to proof regeneration and cross-plan final closure, not basic code existence
- Whether I should continue immediately or pause for sequencing: `pause for sequencing`

## Optional Addendum

- Earlier audit passes also found the worktree was heavily dirty, so any `current branch proof snapshot` wording in `docs/plans/screen-plan/implementation-checklist.md` should not be treated as independently trustworthy until proof is regenerated or restored on this branch.
- Earlier audit passes also saw `git status` showing `docs/proof/screen-plan/PLAN_PROOF_MANIFEST.md` deleted, which reinforces that local proof state is stale/missing rather than merely unread.
