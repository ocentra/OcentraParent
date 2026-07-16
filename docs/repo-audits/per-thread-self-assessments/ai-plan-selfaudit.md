# ai-plan

## Normalized Header

- plan/thread name: `ai-plan`
- source thread label: `codex-a ai-plan thread`
- source thread id: `019ed325-9169-7dc3-926e-cb985d43e2c9`
- worktree: `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent`
- branch: `codex/tracking-plan-full-continuation-a`
- claimed status: completion architecture report only; not done; not PR_READY
- claimed source files/crates/packages: `packages/ai-domain`, `packages/parent-domain`, `packages/portal-domain`, `packages/screen-domain`, `packages/agent-protocol-domain`, `apps/portal`, `crates/agent-protocol`, `crates/agent-service`, `crates/agent-core`, `crates/screen-ai-core`, sibling surfaces in `packages/browser-domain`, `packages/app-game-domain`, `packages/tracking-domain`
- claimed tests: `packages/ai-domain/tests/{unit,contract,integration,e2e,property,security,load}`, `packages/screen-domain/tests/unit/*`, `packages/agent-protocol-domain/tests/unit/parent-assistant-adapter.test.ts`, `apps/portal/tests/local-ai/ai-runtime-route-panel.test.ts`, `apps/portal/e2e/assistant-chat-ui-proof.spec.ts`, crate `tests/` reorg for `crates/agent-protocol` and `crates/agent-service`
- claimed proof commands/artifacts: `scripts/test/local-ai-*.mjs`, `scripts/test/screen-ai-*.mjs`, `scripts/test/household-ai-*.mjs`, `scripts/test/parent-assistant-*.mjs`; canonical roots `output/ai-plan-proof/<workpack-file-stem>/`, `output/screen-plan-proof/real-capture/*`, `output/screen-ai-pipeline-proof/*`
- claimed blockers: local AI ownership drift in `packages/parent-domain`; missing real categorized AI tests; absent `output/ai-plan-proof/*`; missing `screen-plan` and `screen-ai-pipeline-plan` proof roots for screen-derived closure; portal ownership/sequencing may need coordinator confirmation
- claimed next actions: `ai-ownership-and-architecture-cleanup`; `ai-test-rebase-into-major-categories`; `local-ai-core-through-journal-and-read-model`
- obvious missing evidence fields: actual artifact trees absent locally; many scoped validation commands still unrun; placeholder AI test folders do not count as coverage; sibling-plan proof ownership needs explicit sequencing
- archival note: self-report only; unverified input for structural review

## Raw Self-Report

`COMPLETION_ARCHITECTURE_REPORT`

AI-plan is not closeable from docs alone. The real path is: fix ownership truth first, rebuild the local AI core with real categorized tests, close portal/read-model consumption, then stack screen-derived AI on top of real `screen-plan` and `screen-ai-pipeline-plan` artifacts. This host can do the Windows, Android, and Linux/WSL/Docker parts now; native Apple-host runtime proof is the only platform-limited class, and it is not on the critical path for honest closure here.

**Current truth snapshot**
- Exact worktree/branch: `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent` on `codex/tracking-plan-full-continuation-a`.
- Current repo state: no in-flight edits from this lane, but the worktree is heavily dirty across many unrelated plans/packages, so exact-file sequencing matters.
- Exact docs read: `docs/plans/ai-plan/{AGENTS.md,PLAN_STATE.md,NEXT_ACTIONS.md,WORKPACK_INDEX.md,CHECKLIST_INDEX.md,TEST_PROOF_EXPECTATIONS.md,PROOF_INDEX.md,DOC_INDEX.md,PLAN_HEALTH.md,PLAN_EXECUTION_BLUEPRINT.md,README.md,README_FULL_ORIGINAL.md,ROUTE_INDEX.md,ARCHIVE_INDEX.md,current-ai-snapshot.md,source-index.md,tabagent-source-index.md,model-and-runtime-candidate-strategy.md,household-ai-provider-mesh-plan.md,v0-6-local-ai-contracts-plan.md,v0-7-local-ai-runtime-and-dry-run-plan.md,v0-7-ai-context-builder-plan.md,v0-7-ai-model-routing-and-queue-plan.md,v0-7-ai-memory-graph-plan.md,v0-7-tabagent-reuse-plan.md,v0-7-ai-test-blueprint.md,v0-8-policy-enforcement-handoff-plan.md,v1-screen-ocr-vlm-plan.md,v4-remote-parent-assistant-plan.md,ui-ux-requirements-guide.md,real-ai-analysis-and-pipeline-proof-matrix.md,proof-pack-template.md,pasted-content-coverage-audit.md,implementation-checklist.md}` plus every `docs/plans/ai-plan/workpacks/{01..48}` and `full-platform-portal-ai-execution-plan.md`.
- Exact feature/expectation docs read: `docs/features/local-ai-safety-evaluator.md`, `docs/features/parent-assistant-actions.md`, `docs/expectations/ai.md`, `docs/expectations/parent-assistant-chat.md`.
- Exact source sampled/read: `packages/ai-domain/src/{local-ai.ts,parent-assistant.ts,local-ai-context.ts,local-ai-context-builder.ts,local-ai-provider-scheduler.ts,local-ai-runtime.ts,local-ai-activity-memory-graph.ts,local-ai-remote-assistant-boundary-proof.ts}`, `packages/parent-domain/src/{local-ai.ts,parent-assistant.ts,local-ai-context.ts,local-ai-provider-scheduler.ts}`, `packages/portal-domain/src/{local-ai-runtime-panel.ts,parent-assistant-chat.ts,activity-memory-graph.ts}`, `apps/portal/src/{live-activity-state.ts,detail-list.ts,policy-preview-details.ts}`, `packages/screen-domain/src/{screen-ocr-worker.ts,screen-vlm-worker.ts,screen-ai-model-output-parser-proof.ts,screen-ai-invalid-output-degrade-proof.ts}`, `packages/agent-protocol-domain/src/parent-assistant-adapter.ts`, `crates/agent-protocol/src/{local_ai.rs,local_ai_runtime.rs,parent_assistant.rs}`, `crates/agent-service/src/local_ai_*`, `crates/agent-service/src/parent_assistant_*`, `crates/agent-core/src/activity_store_memory_graph*`, `crates/screen-ai-core/src/screen_ai_pipeline.rs`, plus inventories over `packages/browser-domain`, `packages/app-game-domain`, and `packages/tracking-domain`.
- Exact test/proof inventory read: `packages/ai-domain/tests/{unit,contract,integration,e2e,security,observability}`, `apps/portal/tests/local-ai/ai-runtime-route-panel.test.ts`, `apps/portal/e2e/assistant-chat-ui-proof.spec.ts`, `packages/screen-domain/tests/unit/*`, `packages/agent-protocol-domain/tests/unit/parent-assistant-adapter.test.ts`, `crates/screen-ai-core/tests/{unit.rs,unit/pipeline_decision.rs}`, and `scripts/test/{local-ai-*.mjs,screen-ai-*.mjs,household-ai-*.mjs,parent-assistant-*.mjs,browser-*.mjs,app-game-*.mjs,tracking-*.mjs}`.

| Current truth | Done | Partial | False-green | Missing |
|---|---|---|---|---|
| Ownership/contracts | `packages/ai-domain/src/local-ai.ts`, `parent-assistant.ts`, `local-ai-context.ts`, `local-ai-provider-scheduler.ts` are the real TS owners | Rust/service parity exists in `crates/agent-protocol/src/{local_ai.rs,parent_assistant.rs}` and `crates/agent-service/src/local_ai_*` | `docs/plans/ai-plan/source-index.md` still points at `packages/parent-domain`; `packages/parent-domain/src/{local-ai.ts,parent-assistant.ts,...}` are 36 banned re-export wrappers | Clean ownership migration and consumer import cleanup |
| Local AI stack | Deterministic/text/runtime/context/journal/memory files exist in `packages/ai-domain/src/*` | Explanation/read-model/portal consumption are only partly stacked | Workpack checkmarks overstate closure without artifacts | Full end-to-end local stack with real tests and proof |
| Test surface | `packages/ai-domain/tests/unit/*` is real; some portal/screen/unit tests exist | `apps/portal/e2e/assistant-chat-ui-proof.spec.ts`, `packages/screen-domain/tests/unit/*` cover slices | `packages/ai-domain/tests/contract/.gitkeep`, `integration/.gitkeep`, `e2e/.gitkeep`; `security/*` and `observability/*` are mostly placeholder taxonomy folders | Real `contract`, `integration`, `e2e`, `property`, `security`, `load`, and portal Playwright coverage where applicable |
| Proof | Real proof scripts exist under `scripts/test/*.mjs` | Proof families are defined in `implementation-checklist.md` | Docs cite `output/ai-plan-proof/*`, `output/screen-ai-pipeline-proof/*`, `output/screen-plan-proof/*` that do not exist locally | Real artifact roots populated |
| Validation | Audit/discovery commands passed | Focused package/crate validation is mostly unrun | Old green plan text implies validation closure | Scoped package/crate/domain validation matrix |

**Completion definition**
- `packages/ai-domain` is the truthful AI owner, and the stale `packages/parent-domain` AI wrapper/export layer is removed or replaced with a compliant migration.
- Real tests exist under major categories where applicable: `unit`, `contract`, `integration`, `e2e`, `property`, `security`, `load`, and Playwright/e2e for portal surfaces.
- Screen-derived AI claims are backed by real `output/screen-plan-proof/real-capture/*` and real `output/screen-ai-pipeline-proof/*` prerequisites.
- Canonical proof root `output/ai-plan-proof/<workpack-file-stem>/` is populated for every claimed workpack, plus the declared cross-plan roots.
- Focused lint/architecture/build/test/proof commands are green on the touched surfaces.

**Code surface and ownership**
- TS owner: `packages/ai-domain/src/*`
- Stale compatibility layer to remove: `packages/parent-domain/src/{local-ai*.ts,parent-assistant*.ts,household-ai-provider-*.ts}` and `packages/parent-domain/package.json`
- Portal consumers: `packages/portal-domain/src/{local-ai-runtime-panel.ts,parent-assistant-chat.ts,activity-memory-graph.ts}`, `apps/portal/src/{live-activity-state.ts,detail-list.ts,policy-preview-details.ts}`
- Screen/AI bridge: `packages/screen-domain/src/*`, `crates/screen-ai-core/src/screen_ai_pipeline.rs`
- Protocol/service/core: `packages/agent-protocol-domain/src/parent-assistant-adapter.ts`, `crates/agent-protocol/src/{local_ai.rs,local_ai_runtime.rs,parent_assistant.rs}`, `crates/agent-service/src/local_ai_*`, `crates/agent-service/src/parent_assistant_*`, `crates/agent-core/src/activity_store_memory_graph*`
- Sibling consumers: `packages/browser-domain/src/*ai*`, `packages/app-game-domain/src/app-game-ai-classifier-boundary.ts`, `packages/tracking-domain/src/*tracking-ai*`

**Test/proof reorganization and missing coverage**
- Keep `packages/ai-domain/tests/unit/*` as unit coverage.
- Replace placeholders with real files in `packages/ai-domain/tests/{contract,integration,e2e,property,security,load}`; `load` is applicable because workpack `47` requires queue/resource proof.
- Keep portal proof in real browser/e2e locations, not unit-only. Expand `apps/portal/e2e/*` for runtime status, explanation, degraded, unavailable, and remote-disabled states.
- Move inline/src Rust tests that this plan relies on into crate `tests/` folders with major categories. Exact candidates:
  - `crates/agent-protocol/src/{local_ai_runtime_tests.rs,local_ai_runtime_provider_proof_tests.rs,parent_assistant_tests.rs}` -> `crates/agent-protocol/tests/{contract,integration}/...`
  - `crates/agent-service/src/{local_ai_runtime_status_tests.rs,local_ai_provider_scheduler_tests.rs,parent_assistant_api_tests.rs,parent_assistant_runtime_tests.rs}` -> `crates/agent-service/tests/{contract,integration,load}/...`
  - `crates/agent-core/src/{activity_store_memory_graph_tests.rs,activity_store_screen_evidence_tests.rs}` -> `crates/agent-core/tests/integration/...` if counted toward AI-plan closure
- Placeholder false-green examples: `packages/ai-domain/tests/contract/.gitkeep`, `packages/ai-domain/tests/integration/.gitkeep`, `packages/ai-domain/tests/e2e/.gitkeep`, `packages/ai-domain/tests/security/authn/.gitkeep`, `packages/ai-domain/tests/observability/alerting/.gitkeep`.

**Proof inventory**
- Real proof assets present now: proof scripts under `scripts/test/*.mjs`.
- Stale proof claims: `docs/plans/ai-plan/implementation-checklist.md` references artifact trees that are absent locally.
- Canonical proof roots required:
  - `output/ai-plan-proof/<workpack-file-stem>/`
  - `output/screen-plan-proof/real-capture/*`
  - `output/screen-ai-pipeline-proof/{live-operator-artifact-gate,action-dispatch,event-driven-runtime,household-mesh-screen-ai,service-winrt-ocr,service-winrt-ocr-policy,adapter-readiness,family-ai-hub-runtime-discovery}/*`
- Missing AI roots include at least `local-ai-runtime-provider-proof`, `local-ai-provider-scheduler-proof`, `local-ai-stored-evidence-context`, `local-ai-stored-evidence-integration-proof`, `local-ai-deterministic-classifier-proof`, `local-ai-result-journal-sqlite-proof`, `local-ai-remote-assistant-boundary-proof`, `local-ai-plan-closure-audit`, `screen-winrt-ocr-worker`, `screen-summary-ai-context`, `screen-summary-parent-explanation*`, `screen-ai-model-output-parser-proof`, `screen-ai-invalid-output-degrade-proof`, `screen-vlm-*`.

**Scoped validation inventory**
- Cheap checks already passing:
  - `git rev-parse --abbrev-ref HEAD`
  - source/test/proof inventory via `rg`, `Get-ChildItem`, and `Test-Path`
- Cheap scoped validation already failing:
  - `npm run lint:architecture -- --files packages/parent-domain/src/local-ai.ts packages/parent-domain/src/parent-assistant.ts packages/parent-domain/src/local-ai-context.ts packages/parent-domain/src/local-ai-provider-scheduler.ts`
  - Failure reason: barred `export *` AI re-exports in `packages/parent-domain/src/*`
- Cheap scoped validation still unrun but required:
  - `npm run build --workspace @ocentra-parent/ai-domain`
  - `npm run test --workspace @ocentra-parent/ai-domain`
  - `npm run build --workspace @ocentra-parent/portal-domain`
  - `npm run test --workspace @ocentra-parent/portal-domain`
  - `npm run build --workspace @ocentra-parent/screen-domain`
  - `npm run test --workspace @ocentra-parent/screen-domain`
  - `npm run test --workspace @ocentra-parent/portal -- ai`
  - `cargo test -p ocentra-parent-agent-protocol local_ai`
  - `cargo test -p ocentra-parent-agent-protocol parent_assistant`
  - `cargo test -p ocentra-parent-agent-service local_ai`
  - `cargo test -p ocentra-parent-agent-service parent_assistant`
  - `cargo test -p ocentra-screen-ai-core`

| Dependency bucket | Exact dependency | Why it matters |
|---|---|---|
| `local-now` | `docs/plans/ai-plan/*`, `packages/ai-domain/src/*`, `packages/parent-domain/src/local-ai*.ts`, `packages/parent-domain/package.json`, `packages/portal-domain/src/*`, `apps/portal/src/*` | Ownership cleanup, local AI core, portal read-model, and test/proof reorg can proceed immediately |
| `needs-coordinator-sequencing` | write ownership for `packages/parent-domain/package.json` and portal AI files; huge existing dirty tree | Prevents stepping into another active slice while fixing AI ownership/portal surfaces |
| `needs-sibling-plan-contract` | `output/screen-plan-proof/real-capture/*`; `output/screen-ai-pipeline-proof/*`; browser/app-game/tracking proof roots consumed by workpacks `33-37` | These are real prerequisites for honest screen-derived and cross-domain closure |
| `host-platform-limited` | native Apple-host runtime proof only, if someone later insists on it | Not required for current honest Windows/Android/Linux closure path |

| Ordered slice | Files/domains to touch | Validation | Proof/artifacts | Exit criteria |
|---|---|---|---|---|
| `1. ai-ownership-and-architecture-cleanup` | `docs/plans/ai-plan/{source-index.md,current-ai-snapshot.md,PLAN_STATE.md,implementation-checklist.md}`, `packages/parent-domain/src/{local-ai*.ts,parent-assistant*.ts,household-ai-provider-*.ts}`, `packages/parent-domain/package.json`, import call sites | focused `lint:architecture` on `packages/ai-domain` + `packages/parent-domain` | none yet beyond source truth | AI ownership is truthful and the banned wrapper layer is gone |
| `2. ai-test-rebase-into-major-categories` | `packages/ai-domain/tests/{contract,integration,e2e,property,security,load}`, `apps/portal/e2e/*`, `crates/agent-protocol/tests/*`, `crates/agent-service/tests/*` | scoped package/crate tests | no artifact claim yet; just real coverage | placeholder folders replaced with real categorized tests |
| `3. local-ai-core-through-journal-and-read-model` | `packages/ai-domain/src/{local-ai-context*.ts,local-ai-text-*.ts,local-ai-result-journal-sqlite-proof.ts,local-ai-activity-memory-graph*.ts,parent-assistant.ts}`, `crates/agent-protocol/src/{local_ai*.rs,parent_assistant.rs}`, `crates/agent-service/src/local_ai_*`, `packages/portal-domain/src/{local-ai-runtime-panel.ts,parent-assistant-chat.ts,activity-memory-graph.ts}`, `apps/portal/src/live-activity-state.ts` | scoped builds/tests for ai-domain, portal-domain, protocol, service | `output/ai-plan-proof/local-ai-*` roots | local-only AI path is real end-to-end without screen dependencies |
| `4. screen-derived-ai-stack` | `packages/screen-domain/src/*`, `crates/screen-ai-core/src/screen_ai_pipeline.rs`, screen-related portal surfaces | screen-domain + screen-ai-core tests | `output/ai-plan-proof/screen-*`, plus consumed `output/screen-plan-proof/*` and `output/screen-ai-pipeline-proof/*` | OCR/VLM/router claims become real |
| `5. sibling-domain-ai-consumers` | `packages/browser-domain/src/*ai*`, `packages/app-game-domain/src/app-game-ai-classifier-boundary.ts`, `packages/tracking-domain/src/*tracking-ai*`, related scripts | scoped sibling package tests | browser/app-game/tracking proof roots | workpacks `33-37` become honest or are explicitly narrowed out |
| `6. negative-gates-and-resource-proof` | AI/security/load tests plus resource/backpressure code paths | security/load scoped tests | `output/ai-plan-proof/*negative*`, resource proofs | no fake-green safety/perf claims remain |
| `7. rollout-gate` | `docs/plans/ai-plan/workpacks/48-rollout-checklist-and-pr-gate.md` and final proof manifests | final scoped matrix only | all declared roots present | every checklist row can be checked honestly |

**Platform feasibility**
- Windows host now: TS/Rust local validation, portal Playwright, Windows runtime/provider status, screen OCR/VLM route consumers, degraded portal, action-dispatch consumption, and local proof harnesses.
- Android Studio/device: mobile dormant/fallback, Android emulator readiness, Samsung-device-backed custody/runtime proofs, any Android-specific route eligibility the plan claims.
- Linux via WSL/Docker: Linux host adapter, custody, queue/resource, and service-runtime proofs where the scripts/crates already support them.
- Apple-host-only: native iOS/macOS runtime proof if a slice later insists on live Apple execution. That is not a prerequisite for the current honest finish path.

**First coordinator ask / unblock request**
If one predecessor plan should move first for final closure, it is `screen-plan`, immediately followed by `screen-ai-pipeline-plan`. Reason: AI-plan's screen workpacks and its final rollout gate cite `output/screen-plan-proof/real-capture/*` and `output/screen-ai-pipeline-proof/*` as inputs. Without those exact artifacts, the screen/OCR/VLM portion of AI-plan cannot be closed honestly, even if the local AI core is finished.

**Strict done bar**
Before this plan can ever be marked done:
- no AI ownership lies remain in docs or exports;
- no banned `parent-domain` AI re-export wrappers remain;
- real tests exist in the applicable major categories, with no placeholder optics counted;
- any plan-relied Rust tests that still live in `src/*_tests.rs` have been moved into proper crate `tests/` folders or are clearly supplemental only;
- `output/ai-plan-proof/*` exists for every claimed workpack;
- required `screen-plan` and `screen-ai-pipeline-plan` roots exist for screen-derived claims;
- scoped builds/tests/lint/architecture/proof commands are green on touched surfaces;
- portal screenshots/Playwright proof exist wherever UI is claimed;
- workpack `48` can be checked with no handwave.

**COORDINATOR_DECISION_REQUEST**
- Recommended next slice: `ai-ownership-and-architecture-cleanup`
- Recommended predecessor plans: `screen-plan` first for `real-capture` artifacts, then `screen-ai-pipeline-plan` for service/runtime producer-consumer artifacts
- Estimated risk: medium-high, because ownership cleanup intersects stale exports and a very dirty shared worktree
- Estimated proof difficulty: high, because real screen-derived artifacts and cross-plan proof stacking are required
- Continue immediately or pause for sequencing: continue immediately on slice 1 if you grant ownership for `packages/parent-domain` AI exports and the local AI plan docs; otherwise pause only long enough for sequencing, not for more auditing

## Optional Addendum

- Earlier audit passes established via `rg` over `packages/parent-domain/src` that the stale AI wrapper layer is not just conceptual drift: there are 36 one-line `export * from '@ocentra-parent/ai-domain/...'` files, and focused `npm run lint:architecture -- --files packages/parent-domain/src/local-ai.ts packages/parent-domain/src/parent-assistant.ts packages/parent-domain/src/local-ai-context.ts packages/parent-domain/src/local-ai-provider-scheduler.ts` failed on the sampled wrappers. The exact wrapper count matters because the cleanup is a real migration slice, not a single-file touch-up.
- Earlier audit passes also found source-hygiene drift in `packages/ai-domain/src`: generated `.js`, `.d.ts`, and `.map` files live alongside the `.ts` sources even though `packages/ai-domain/package.json` exports `dist/*`. That build-output pollution should not be counted as implementation progress or proof, and it should be cleaned or explicitly ignored before treating the package layout as completion-ready.
- Consumer scans during earlier audit passes found active repo code importing `@ocentra-parent/ai-domain` directly in live surfaces such as `packages/screen-domain/src/*`, `packages/agent-protocol-domain/src/parent-assistant-adapter.ts`, and multiple `scripts/test/local-ai-*.mjs` / `screen-ai-*.mjs` proof runners, while no meaningful active consumer imports of `@ocentra-parent/parent-domain/local-ai` or `@ocentra-parent/parent-domain/parent-assistant` were found. That supports treating the `parent-domain` AI wrappers as stale compatibility baggage rather than a live dependency surface that must be preserved.
