# screen-ai-pipeline-plan

## Normalized Header

- plan/thread name: `screen-ai-pipeline-plan`
- source thread label: `screen-ai-pipeline-plan`
- source thread id: `019ed32c-17ed-79a3-b7ce-3056415153bf`
- worktree: `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent`
- branch: `codex/tracking-plan-full-continuation-a`
- claimed status: paused for sequencing; broad execution held; docs-only truth repair landed; completion architecture and slice-1 ownership prepared
- claimed source files/crates/packages: `crates/agent-service/src/screen_ai_*`, `crates/screen-ai-core/src/screen_ai_pipeline.rs`, `packages/screen-domain/src/screen-ai-*.ts`, `packages/ai-domain/src/local-ai-runtime.ts`, `packages/activity-domain/src/screen-vlm-journal-read-model.ts`, `packages/portal-domain/src/screen-summary-panel.ts`, `packages/portal-domain/src/contracts.ts`, `packages/parent-domain/src/screen-ai-*.ts`, `packages/screen-domain/src/screen-evidence.ts`
- claimed tests: `scripts/test/screen-ai-*.mjs`, `packages/screen-domain/tests/*`, `packages/ai-domain/tests/*`, `packages/activity-domain/tests/*`, `packages/portal-domain/tests/*`, `apps/portal/tests/*`, inline Rust tests in `crates/agent-service/src/screen_ai_*_tests.rs`, broad crate test categories under `crates/agent-service/tests/*`
- claimed proof commands/artifacts: canonical proof root should be `output/screen-ai-pipeline-proof/`; cheap passing checks were `node --check scripts/test/screen-ai-final-product-path-proof.mjs`, `node --check scripts/test/screen-ai-live-operator-artifact-gate.mjs`, `node --check scripts/test/screen-ai-service-winrt-ocr-proof.mjs`, `node --check scripts/test/screen-ai-household-mesh-proof.mjs`; failing gate was `npm run --silent lint:architecture -- --files packages/screen-domain/src/screen-evidence.ts packages/portal-domain/src/contracts.ts packages/parent-domain/src/local-ai-runtime.ts`
- claimed blockers: missing retained `output/screen-ai-pipeline-proof/`; missing `docs/proof/screen-ai-pipeline-plan/PLAN_PROOF_MANIFEST.md`; architecture debt in `screen-evidence.ts`, `contracts.ts`, `local-ai-runtime.ts`; inline/proof logic mixed into `src`; upstream retained artifacts from `screen-plan` and `ai-plan`; coordinator sequencing hold for logging/LAN/eventing/auth/setup/child-runtime/package wave
- claimed next actions: first slice remains proof/test surface normalization and architecture cleanup; then Windows core proof regeneration; then cross-platform feasible proof; then live-operator and final closure
- obvious missing evidence fields: no retained proof root in current checkout; no plan manifest; `docs/proof/screen-ai-pipeline-plan/` directory itself is absent; no current retained screen-AI artifacts to back stale checklist/workpack claims
- archival note: self-report only; unverified input for structural review

## Raw Self-Report

**Executive Summary**

`screen-ai-pipeline-plan` is not close to honest closure on `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent` / `codex/tracking-plan-full-continuation-a`. The core Windows service/runtime and proof-script surface is substantial, but the plan is still blocked by three structural issues before real completion work should scale: proof/test surface sprawl, missing retained proof/manifests, and architecture debt from banned re-export surfaces. The right first slice remains `proof/test surface normalization and architecture cleanup`, and the first coordinator ask is to sequence upstream dependency plans that produce the `screen-plan` and `ai-plan` artifacts this plan cites, so the screen-AI closure work does not churn against moving prerequisites.

**COMPLETION_ARCHITECTURE_REPORT**

**Plan Closure Definition**

“Actually done” for this plan means all of the following are true:

- The live pipeline is genuinely complete for the claimed scope:
  - trigger
  - capture
  - encrypted queue
  - analysis
  - policy dry-run
  - journal/read-model chain
  - portal explanation
  - deletion/retention custody
  - cadence/backpressure controls
  - final product-path accounting
- Required code is complete in the owning runtime/domain surfaces, not just represented by proof models.
- Tests are real, scoped, and placed in proper test locations:
  - TypeScript/JS under package `tests/` categories as applicable
  - Rust under crate `tests/` integration/category folders where runtime behavior is being proved, not primarily in `src/*_tests.rs`
- Proof artifacts are retained under one canonical root:
  - `output/screen-ai-pipeline-proof/`
- A plan manifest exists:
  - `docs/proof/screen-ai-pipeline-plan/PLAN_PROOF_MANIFEST.md`
- Plan docs, workpacks, checklist rows, proof artifacts, and validations agree with each other.
- Scoped validation is green for the touched surfaces, including the architecture gate.

**Exact Docs / Source / Tests / Proof Read**

Docs read in plan folder:
- `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent\docs\plans\screen-ai-pipeline-plan\AGENTS.md`
- `...\ARCHIVE_INDEX.md`
- `...\CHECKLIST_INDEX.md`
- `...\DOC_INDEX.md`
- `...\NEXT_ACTIONS.md`
- `...\PLAN_EXECUTION_BLUEPRINT.md`
- `...\PLAN_HEALTH.md`
- `...\PLAN_STATE.md`
- `...\PROOF_INDEX.md`
- `...\README.md`
- `...\README_FULL_ORIGINAL.md`
- `...\ROUTE_INDEX.md`
- `...\TEST_PROOF_EXPECTATIONS.md`
- `...\WORKPACK_INDEX.md`
- `...\implementation-checklist.md`
- `...\pipeline-proof-matrix.md`
- `...\proof-tiers.md`
- all 10 workpacks under `...\workpacks\01-...md` through `10-...md`

Feature / expectation doc routing truth:
- On current refresh, no direct `docs/features/*` or `docs/expectations/*` path references were found inside the plan folder.
- The plan instead routes through its own workpacks, checklist, and named proof artifacts.

Primary source surfaces inspected:
- `crates/agent-service/src/screen_ai_foreground_runtime.rs`
- `crates/agent-service/src/screen_ai_cadence_runtime.rs`
- `crates/agent-service/src/screen_ai_analysis_runtime.rs`
- `crates/agent-service/src/screen_ai_retention_sweeper_runtime.rs`
- `crates/agent-service/src/screen_ai_service_event_bridge.rs`
- `crates/agent-service/src/screen_ai_service_event_subscription.rs`
- `crates/screen-ai-core/src/screen_ai_pipeline.rs`
- `packages/screen-domain/src/screen-ai-*.ts`
- `packages/ai-domain/src/local-ai-runtime.ts`
- `packages/activity-domain/src/screen-vlm-journal-read-model.ts`
- `packages/portal-domain/src/screen-summary-panel.ts`
- `packages/portal-domain/src/contracts.ts`
- `packages/parent-domain/src/screen-ai-*.ts`
- `packages/screen-domain/src/screen-evidence.ts`

Primary test / proof surfaces inspected:
- `scripts/test/screen-ai-*.mjs`
- `packages/screen-domain/tests/*`
- `packages/ai-domain/tests/*`
- `packages/activity-domain/tests/*`
- `packages/portal-domain/tests/*`
- `apps/portal/tests/*`
- `crates/agent-service/src/screen_ai_*_tests.rs`
- `crates/agent-service/tests/*`

**Current Truth**

| Area | Done | Partial | False-green | Missing |
|---|---|---|---|---|
| Plan docs truth repair | `PLAN_STATE.md`, `WORKPACK_INDEX.md`, `NEXT_ACTIONS.md`, `PLAN_HEALTH.md`, `PROOF_INDEX.md`, `TEST_PROOF_EXPECTATIONS.md`, `PLAN_EXECUTION_BLUEPRINT.md` now reflect audited state | Proof-shape inconsistency still remains across docs | Old checkmarks previously claimed 8 checked workpacks and 134 checked rows | `docs/proof/screen-ai-pipeline-plan/PLAN_PROOF_MANIFEST.md` |
| Core runtime code | Windows service/runtime surfaces exist in `crates/agent-service/src/screen_ai_*` and `crates/screen-ai-core/src/screen_ai_pipeline.rs` | End-to-end closure is not retained locally | Docs previously implied closure without retained artifacts | Canonical retained proof bundle for current checkout |
| TS domain code | `packages/screen-domain`, `packages/ai-domain`, `packages/activity-domain`, `packages/portal-domain` contain substantial proof/runtime logic | Proof logic and product logic are mixed in `src` | `packages/parent-domain/src/screen-ai-*.ts` re-export proof-like surfaces upward as if product-ready | Clear demotion/move strategy for proof-only TS surfaces |
| Tests | Real test trees exist in package `tests/` dirs and `apps/portal/tests` | Screen-AI-specific runtime validation is split across proper tests and inline Rust `src/*_tests.rs` | Large test category trees exist repo-wide, but they are not evidence of screen-AI closure by themselves | Some applicable runtime/security/concurrency coverage still needs dedicated screen-AI placement |
| Proof | Many `scripts/test/screen-ai-*.mjs` exist | Proof runners exist but current checkout retains no canonical proof root | Old plan wording implied proof existed when `output/screen-ai-pipeline-proof/` does not exist | `output/screen-ai-pipeline-proof/` and manifest |
| Validation | Cheap `node --check` passes on selected proof scripts | Focused architecture gate reveals real debt | Old docs implied closure without green architecture validation | Focused green validation across touched runtime/domain surfaces |

**Code Surface And Ownership**

| Surface | Ownership role for this plan | Exact files / dirs |
|---|---|---|
| Runtime capture / cadence / analysis / retention | Core owning implementation | `crates/agent-service/src/screen_ai_foreground_runtime.rs`, `screen_ai_cadence_runtime.rs`, `screen_ai_analysis_runtime.rs`, `screen_ai_retention_sweeper_runtime.rs` |
| Event bridge / read-model handoff | Core owning implementation | `crates/agent-service/src/screen_ai_service_event_bridge.rs`, `screen_ai_service_event_subscription.rs` |
| Pipeline core | Core owning implementation | `crates/screen-ai-core/src/screen_ai_pipeline.rs` |
| Screen-domain proof/runtime contracts | Owning TS domain, but currently mixed with proof logic | `packages/screen-domain/src/screen-ai-browser-trigger-proof.ts`, `screen-ai-adapter-readiness-proof.ts`, `screen-ai-model-output-parser-proof.ts`, `screen-ai-model-runtime-backpressure-proof.ts`, `screen-ai-model-artifact-manifest-proof.ts`, `screen-ai-memory-graph-source-guard-proof.ts`, `screen-ai-enforcement-handoff-guard-proof.ts`, `screen-ai-stricter-parent-rule-proof.ts`, `screen-family-ai-hub-runtime-discovery-proof.ts` |
| AI runtime boundary | Dependency and architecture debt surface | `packages/ai-domain/src/local-ai-runtime.ts`, `packages/parent-domain/src/local-ai-runtime.ts` |
| Activity / journal read model | Owning downstream contract | `packages/activity-domain/src/screen-vlm-journal-read-model.ts` |
| Portal explanation surface | Owning downstream UI/read-model surface | `packages/portal-domain/src/screen-summary-panel.ts`, `packages/portal-domain/src/contracts.ts`, `apps/portal/tests/screen/*` |
| Re-export / barrel debt | Must be cleaned before closure | `packages/screen-domain/src/screen-evidence.ts`, `packages/portal-domain/src/contracts.ts`, `packages/parent-domain/src/local-ai-runtime.ts`, `packages/parent-domain/src/screen-ai-*.ts` |

**Test Surface Inventory**

| Surface | Current inventory | Truth | Action |
|---|---|---|---|
| `packages/screen-domain/tests` | Contains major categories including `unit`, `integration`, `contract`, `e2e`, `property-based`, `security`, `load`, `monitoring`, `observability` | Category tree exists; does not by itself prove screen-AI coverage quality | Keep only categories actually used for screen-AI closure; avoid counting empty category optics as coverage |
| `packages/ai-domain/tests` | Same broad category tree | Same caveat | Only count screen-AI-relevant cases |
| `packages/activity-domain/tests` | Same broad category tree | Contract coverage exists for VLM journal surface | Preserve contract coverage; add only needed screen-AI cases |
| `packages/portal-domain/tests` | Same broad category tree | Many proof-oriented portal files still live in `src` | Prefer UI-facing tests under `tests/unit`, `tests/integration`, and `apps/portal/tests` |
| `apps/portal/tests` | `activity`, `diagnostics`, `local-ai`, `logging`, `portal`, `screen` | Real app-level test surface exists | Use for honest portal/read-model/e2e slices |
| `crates/agent-service/tests` | Very large category tree including `unit`, `integration`, `contract`, `security`, `load`, `observability`, `authz`, `replay`, etc. | Category presence is broad repo scaffolding; not all is screen-AI specific | Do not count category existence as screen-AI completion |
| Inline Rust tests in `src` | `crates/agent-service/src/screen_ai_cadence_runtime_tests.rs`, `screen_ai_analysis_runtime_tests.rs`, `screen_ai_foreground_runtime_tests.rs`, `screen_ai_retention_sweeper_runtime_tests.rs`, `screen_ai_service_event_bridge_tests.rs`, `screen_ai_service_event_subscription_tests.rs` | These are real tests but inline/source-local | These should move to proper `crates/agent-service/tests/` ownership if they represent closure-grade runtime behavior |
| TS proof logic in `src` | `packages/screen-domain/src/screen-ai-*-proof.ts`, `packages/portal-domain/src/*hosted-ui-proof*.ts`, `packages/parent-domain/src/screen-ai-*.ts` | These are not test files, but they blur product/proof boundaries | Review and demote proof-only logic out of `src` where it is not a real exported runtime contract |

Applicable major categories for this plan:
- `unit`: applicable
- `integration`: applicable
- `contract`: applicable
- `e2e` / app-level UI proof: applicable
- `property`: applicable for parser/invariant surfaces
- `security`: applicable for malformed AI output / no direct authority / redaction
- `load`: applicable for cadence/backpressure
- `playwright`: applicable if portal closure relies on real UI walkthroughs
- `auth`: not a primary center of gravity for this plan, unless sibling-plan contracts force it
- `load`: applicable only for cadence/backpressure, not broad repo load

Empty-folder / optics callout:
- The repo contains many top-level category folders in package/crate test trees. They should not be counted as plan coverage unless screen-AI-specific cases are actually present there.

**Proof Inventory**

| Proof area | Real | Stale | Missing | Canonical path |
|---|---|---|---|---|
| Plan-local proof routing docs | `PROOF_INDEX.md`, `TEST_PROOF_EXPECTATIONS.md`, `pipeline-proof-matrix.md` exist | Earlier docs disagreed on `screen-ai-pipeline-plan-proof` vs `screen-ai-pipeline-proof` | Final manifest absent | `output/screen-ai-pipeline-proof/` |
| Current retained proof root | None in current checkout | Old plan implied proof existed | `output/screen-ai-pipeline-proof/` absent | `output/screen-ai-pipeline-proof/` |
| Alternate stale proof root | None present | `output/screen-ai-pipeline-plan-proof/` was a bad/stale route in docs | Directory absent | Do not use |
| Plan manifest | None | N/A | `docs/proof/screen-ai-pipeline-plan/PLAN_PROOF_MANIFEST.md` absent | `docs/proof/screen-ai-pipeline-plan/PLAN_PROOF_MANIFEST.md` |
| Proof scripts | Many real runners under `scripts/test/screen-ai-*.mjs` | Not enough retained outputs to validate plan claims | Retained artifacts for current checkout | `output/screen-ai-pipeline-proof/<scenario-id>/` |

Canonical proof-root path:
- `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent\output\screen-ai-pipeline-proof\`

**Scoped Validation Inventory**

Already passes:
- `node --check scripts/test/screen-ai-final-product-path-proof.mjs`
- `node --check scripts/test/screen-ai-live-operator-artifact-gate.mjs`
- `node --check scripts/test/screen-ai-service-winrt-ocr-proof.mjs`
- `node --check scripts/test/screen-ai-household-mesh-proof.mjs`

Already fails:
- `npm run --silent lint:architecture -- --files packages/screen-domain/src/screen-evidence.ts packages/portal-domain/src/contracts.ts packages/parent-domain/src/local-ai-runtime.ts`
- Current failure surface:
  - `packages/screen-domain/src/screen-evidence.ts`
  - `packages/portal-domain/src/contracts.ts`
  - `packages/parent-domain/src/local-ai-runtime.ts`

Cheap checks already established:
- `output/screen-ai-pipeline-proof` does not exist
- `output/screen-ai-pipeline-plan-proof` does not exist
- `docs/proof/screen-ai-pipeline-plan/PLAN_PROOF_MANIFEST.md` does not exist

Unrun but required later:
- Focused Rust runtime tests for `screen_ai_*`
- Focused TS package tests for `screen-ai` and `screen-vlm-*`
- Real proof-script execution, not just syntax checks
- Focused portal/app tests for the screen explanation chain
- WSL/Linux and Android proof commands when that slice is sequenced

**Dependency Graph**

| Dependency | Type | Why it matters | True blocker for final closure? |
|---|---|---|---|
| `screen-plan` retained capture proof | needs-sibling-plan-contract | This plan cites `output/screen-plan-proof/real-capture/trigger-matrix/proof-summary.json` | Yes |
| `ai-plan` retained analysis proof | needs-sibling-plan-contract | This plan cites `output/ai-plan-proof/real-analysis/proof-summary.json` | Yes |
| logging proof-surface stabilization | needs-coordinator-sequencing | Coordinator explicitly identified this as ahead of screen-AI wave | Yes for sequencing |
| LAN / eventing / auth / setup / child-runtime / package coordination | needs-coordinator-sequencing | Coordinator explicitly identified these as ahead of this plan in the wave | Yes for sequencing |
| enforcement / adapter sibling surfaces | needs-sibling-plan-contract | Late WP10 adapter claims cite broader adapter readiness and product proofs | Partial blocker: blocks honest final closure, not early cleanup slice |
| Apple-host execution | host-platform-limited | Real macOS/iOS runtime proof is not feasible here | Only blocks Apple-specific closure claims |

Upstream plans that truly block final closure:
- `screen-plan`
- `ai-plan`
- whichever active threads own the logging proof-surface stabilization and LAN/eventing/auth/setup/child-runtime/package coordination the coordinator cited

Plans that affect later polish rather than the first slice:
- broader enforcement / adapter-product proof closure
- Apple-host execution threads

**Platform Feasibility**

| Platform | Feasible now on this host | Notes |
|---|---|---|
| Windows | Yes | Core runtime, portal, service proofs, WinRT OCR, retention, event chain |
| Android Studio / emulator | Yes | Android proof is feasible here and should be used when sequenced |
| Synced Samsung device | Conditionally yes | Feasible when device is available and connected |
| WSL/Linux | Yes | Linux proof can run through WSL; Docker-backed proof depends on local tooling and sequencing |
| Docker | Feasible path, but not confirmed as closure default | If required for Linux slice, treat as local execution setup, not an Apple-only issue |
| macOS | No real runtime proof here | Apple-host-only |
| iOS | No real runtime proof here | Apple-host-only |

**No-Hand-Wave Execution Plan**

| Slice | Goal | Files / domains to touch | Validation | Proof to collect | Exit criteria |
|---|---|---|---|---|---|
| 1. Proof/test surface normalization and architecture cleanup | Make the plan executable without false-green scaffolding | `packages/screen-domain/src/screen-evidence.ts`, `packages/portal-domain/src/contracts.ts`, `packages/parent-domain/src/local-ai-runtime.ts`, `packages/parent-domain/src/screen-ai-*.ts`, `crates/agent-service/src/screen_ai_*_tests.rs`, `docs/proof/screen-ai-pipeline-plan/PLAN_PROOF_MANIFEST.md`, plan proof docs | Focused architecture lint; focused package/crate tests for moved surfaces | manifest skeleton plus agreed proof contract | Architecture gate green on touched files; proof contract unambiguous; runtime-grade tests moved or queued with exact ownership |
| 2. Windows core chain regeneration | Prove the real pipeline through WP01-WP07 and WP09 | `crates/agent-service/src/screen_ai_*`, `crates/screen-ai-core/src/screen_ai_pipeline.rs`, `packages/screen-domain`, `packages/activity-domain`, `packages/portal-domain`, `apps/portal/tests/screen`, `scripts/test/screen-ai-service-*.mjs`, `screen-ai-portal-chain-proof.mjs`, `screen-ai-deletion-retention-custody-proof.mjs` | Focused Rust runtime tests, package tests, portal tests, proof-script runs | `prerequisite-merge/`, `service-foreground/`, `service-cadence/`, `service-analysis/`, `service-winrt-ocr/`, `service-winrt-ocr-policy/`, `portal-chain/`, `service-retention-sweeper/`, `deletion-retention-custody/`, `service-disabled-suppression/` | Retained proof exists and matches checklist/workpack claims for Windows core |
| 3. Result / policy / adapter boundary closure | Close WP04-WP05 honesty around invalid output, rule precedence, and dry-run actions | `packages/screen-domain/src/screen-ai-model-output-parser-proof.ts`, `screen-ai-stricter-parent-rule-proof.ts`, `screen-ai-enforcement-handoff-guard-proof.ts`, `scripts/test/screen-ai-invalid-output*.mjs`, `screen-ai-action-dispatch-proof.mjs`, `screen-ai-block-action-dispatch-proof.mjs` | Focused package tests and proof scripts | `action-dispatch/`, `block-action-dispatch/`, invalid-output artifacts | Policy path proves dry-run authority only, rejects malformed AI output, and preserves non-claims |
| 4. Cross-platform feasible slice | Close feasible non-Apple platform proof | `scripts/test/screen-ai-linux-host-adapter-*.mjs`, `screen-ai-android-mobile-control-custody-proof.mjs`, `screen-ai-adapter-readiness-proof.mjs` plus owning domain files | WSL/Linux runs, Android emulator/device runs, focused package tests | `linux-host-adapter-custody/`, `linux-host-adapter-execution/`, `android-mobile-control-custody/`, `adapter-readiness/` | Linux/Android proof retained and categorized honestly |
| 5. Live operator slice | Close WP08 with real operator evidence, not placeholder docs | `scripts/test/screen-ai-live-operator-proof.mjs`, `screen-ai-live-operator-artifact-gate.mjs`, operator manifest input | Harness verify + real manifest-driven run | `live-operator/`, `live-operator-artifact-gate/`, per-scenario redacted bundles | All required live-operator scenarios retained, redacted, and gate passes |
| 6. Final closure slice | Make WP10 honest | `scripts/test/screen-ai-final-product-path-proof.mjs`, `screen-ai-final-adapter-dependency-audit.mjs`, `screen-ai-adapter-blocker-ledger-proof.mjs`, `screen-ai-adapter-dependency-handoff-proof.mjs`, `screen-ai-product-checklist-delta-proof.mjs`, `screen-ai-vlm-*.mjs` | Focused script runs plus final touched-surface validation | `final-product-path/`, `adapter-blocker-ledger/`, `adapter-dependency-handoff/`, `product-checklist-delta/`, `vlm-*` | Workpacks, checklist, proof, and docs all agree with retained artifacts |

**Blocker Taxonomy**

- `local-now`
  - Re-export / barrel debt in:
    - `packages/screen-domain/src/screen-evidence.ts`
    - `packages/portal-domain/src/contracts.ts`
    - `packages/parent-domain/src/local-ai-runtime.ts`
  - Missing canonical plan manifest:
    - `docs/proof/screen-ai-pipeline-plan/PLAN_PROOF_MANIFEST.md`
  - Proof/test surface sprawl and inline Rust `src/*_tests.rs`
  - No retained canonical proof root in current checkout
- `needs-coordinator-sequencing`
  - logging proof-surface stabilization
  - LAN / eventing / auth / setup / child-runtime / package coordination wave ahead of this plan
- `needs-sibling-plan-contract`
  - `screen-plan` retained proof artifacts cited by this plan
  - `ai-plan` retained proof artifacts cited by this plan
  - sibling adapter/enforcement contracts that late WP10 claims rely on
- `host-platform-limited`
  - real macOS runtime proof
  - real iOS runtime proof

**First Coordinator Ask**

If another plan should move first, it should be the active owners of the retained proof outputs this plan cites from `screen-plan` and `ai-plan`, plus the logging proof-surface stabilization work the coordinator already identified. Reason: the first executable slice here is cleanup and normalization, but honest final closure still depends on upstream capture and AI proof artifacts; if those remain unstable, broad screen-AI execution will only create cross-plan churn and rework.

**Strict Done Bar**

Before you can ever mark this plan done, all of the following must be true:

- `docs/proof/screen-ai-pipeline-plan/PLAN_PROOF_MANIFEST.md` exists and matches the retained artifacts.
- `output/screen-ai-pipeline-proof/` exists with real scenario artifacts for the workpacks this plan claims closed.
- No stale `checked` rows remain unbacked by retained proof.
- Architecture lint is green for the touched screen-AI closure surfaces.
- Runtime-grade Rust tests are not primarily hiding in `src/*_tests.rs` when they are closure-critical.
- TS proof-only logic is not being counted as product completion just because it exists in `src`.
- Windows core chain is proved end to end.
- Linux/Android claims are retained where the plan claims them.
- Apple-specific runtime claims are either not claimed or are closed by off-host proof.
- Final workpack/checklist/doc state matches source, tests, proof, and non-claims.

**COORDINATOR_DECISION_REQUEST**

- recommended next slice:
  - `proof/test surface normalization and architecture cleanup`
- recommended predecessor plans:
  - the active `screen-plan` proof owner
  - the active `ai-plan` proof owner
  - the logging proof-surface stabilization thread
  - any coordinator-prioritized LAN/eventing/auth/setup/child-runtime/package threads whose outputs this plan will consume
- estimated risk:
  - high, because the closure path crosses runtime, proof routing, architecture policy, sibling-plan artifacts, and multiple platforms
- estimated proof difficulty:
  - high, because current checkout has zero retained canonical screen-AI proof and the live/operator plus cross-platform slices are non-trivial
- whether I should continue immediately or pause for sequencing:
  - pause for sequencing; be ready to start with the normalization/architecture slice once the coordinator opens the execution wave

## Optional Addendum

- Follow-up correction after rereading this thread: not only is `docs/proof/screen-ai-pipeline-plan/PLAN_PROOF_MANIFEST.md` missing, the parent directory `docs/proof/screen-ai-pipeline-plan/` does not currently exist in this checkout.
- Follow-up correction after rereading this thread: the slice-1 ownership map narrowed the first edit set further than the raw report did. The concrete slice-1 file set is:
  - proof-manifest and proof-route docs:
    - `docs/proof/screen-ai-pipeline-plan/PLAN_PROOF_MANIFEST.md`
    - `docs/plans/screen-ai-pipeline-plan/PROOF_INDEX.md`
    - `docs/plans/screen-ai-pipeline-plan/TEST_PROOF_EXPECTATIONS.md`
    - `docs/plans/screen-ai-pipeline-plan/PLAN_EXECUTION_BLUEPRINT.md`
    - `docs/plans/screen-ai-pipeline-plan/pipeline-proof-matrix.md`
  - proof-route dependent checklist/workpacks:
    - `docs/plans/screen-ai-pipeline-plan/implementation-checklist.md`
    - `docs/plans/screen-ai-pipeline-plan/workpacks/01-prerequisite-merge-and-branch-gate.md`
    - `docs/plans/screen-ai-pipeline-plan/workpacks/02-real-trigger-to-capture-gate.md`
    - `docs/plans/screen-ai-pipeline-plan/workpacks/03-capture-to-ai-analysis-gate.md`
    - `docs/plans/screen-ai-pipeline-plan/workpacks/04-ai-result-to-policy-gate.md`
    - `docs/plans/screen-ai-pipeline-plan/workpacks/05-policy-action-dry-run-gate.md`
    - `docs/plans/screen-ai-pipeline-plan/workpacks/06-journal-read-model-and-portal-gate.md`
    - `docs/plans/screen-ai-pipeline-plan/workpacks/07-deletion-retention-and-custody-gate.md`
    - `docs/plans/screen-ai-pipeline-plan/workpacks/08-live-operator-proof-gate.md`
    - `docs/plans/screen-ai-pipeline-plan/workpacks/09-performance-cadence-and-backpressure-gate.md`
    - `docs/plans/screen-ai-pipeline-plan/workpacks/10-final-rollout-and-pr-gate.md`
  - architecture-debt files:
    - `packages/screen-domain/src/screen-evidence.ts`
    - `packages/portal-domain/src/contracts.ts`
    - `packages/parent-domain/src/local-ai-runtime.ts`
  - screen-AI-adjacent proof/re-export surfaces to review in the same slice:
    - `packages/screen-domain/src/screen-ai-adapter-readiness-proof.ts`
    - `packages/screen-domain/src/screen-ai-browser-trigger-proof.ts`
    - `packages/screen-domain/src/screen-ai-enforcement-handoff-guard-proof.ts`
    - `packages/screen-domain/src/screen-ai-memory-graph-source-guard-proof.ts`
    - `packages/screen-domain/src/screen-ai-model-artifact-manifest-proof.ts`
    - `packages/screen-domain/src/screen-ai-model-runtime-backpressure-proof.ts`
    - `packages/screen-domain/src/screen-ai-stricter-parent-rule-proof.ts`
    - `packages/screen-domain/src/screen-family-ai-hub-runtime-discovery-proof.ts`
    - `packages/parent-domain/src/screen-ai-adapter-readiness-proof.ts`
    - `packages/parent-domain/src/screen-ai-browser-trigger-proof.ts`
    - `packages/parent-domain/src/screen-ai-enforcement-handoff-guard-proof.ts`
    - `packages/parent-domain/src/screen-ai-memory-graph-source-guard-proof.ts`
    - `packages/parent-domain/src/screen-ai-model-artifact-manifest-proof.ts`
    - `packages/parent-domain/src/screen-ai-model-runtime-backpressure-proof.ts`
    - `packages/parent-domain/src/screen-ai-stricter-parent-rule-proof.ts`
  - inline Rust tests to move out of `src`:
    - `crates/agent-service/src/screen_ai_analysis_runtime_tests.rs`
    - `crates/agent-service/src/screen_ai_cadence_runtime_tests.rs`
    - `crates/agent-service/src/screen_ai_foreground_runtime_tests.rs`
    - `crates/agent-service/src/screen_ai_retention_sweeper_deletion_events_tests.rs`
    - `crates/agent-service/src/screen_ai_retention_sweeper_runtime_tests.rs`
    - `crates/agent-service/src/screen_ai_service_event_bridge_tests.rs`
    - `crates/agent-service/src/screen_ai_service_event_subscription_tests.rs`
    - `crates/agent-service/src/screen_ai_analysis_runtime/adapter_tests.rs`
