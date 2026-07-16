# data-custody-storage-plan

## Normalized Header

- plan/thread name: `data-custody-storage-plan`
- source thread label: `data-custody-storage-plan dedicated worker lane`
- source thread id: `019ed327-d345-7bf1-ac93-f7a8d645eca0`
- worktree: `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent`
- branch: `codex/tracking-plan-full-continuation-a`
- claimed status: `audit complete; completion proposal delivered; completion architecture report delivered; substrate-truth-repair approved and lane-started; no product-source implementation landed in this archival pass`
- claimed source files/crates/packages: `packages/data-custody-domain`; `packages/production-domain`; `packages/endpoint-domain`; `packages/parent-domain/src/parent-owned-local-export-runtime.ts` as the remaining substrate holdout; `crates/storage-custody-core`; `crates/ocentra-evidence`; downstream consumer surfaces identified in `packages/tracking-domain`, `packages/family-domain`, `packages/portal-domain`, and `apps/portal/e2e`
- claimed tests: `packages/data-custody-domain/tests/unit/data-custody-matrix.test.ts`; `packages/production-domain/tests/unit/parent-owned-sync-export.test.ts`; `packages/production-domain/tests/unit/stateless-report-compiler-status.test.ts`; `packages/endpoint-domain/tests/unit/sync-export.test.ts`; `crates/storage-custody-core/tests/unit/custody_decision.rs`; `crates/storage-custody-core/tests/unit/custody_flow.rs`; `cargo test -p ocentra-evidence`; downstream consumer tests in `packages/tracking-domain/tests/contract/*` and `apps/portal/e2e/tracking-hosted-ui-proof.spec.ts`
- claimed proof commands/artifacts: `scripts/test/parent-owned-sync-export-manifest-proof.mjs`; `scripts/test/stateless-report-compiler-status-proof.mjs`; `scripts/test/sync-export-endpoint-contract-proof.mjs`; `scripts/test/parent-owned-local-export-runtime-proof.mjs`; missing canonical proof roots `docs/proof/data-custody-storage-plan` and `output/data-custody-storage-plan-proof`
- claimed blockers: stale proof-script and proof-path drift; wrong owner-package wiring in proof scripts; failing `crates/storage-custody-core` tests due to private-trait import path; `parent-domain` debt blocking honest local-export runtime proof; final closure depends on explicit downstream tracking and device-trust consumer contracts
- claimed next actions: execute `substrate-truth-repair`; restore canonical proof roots; repair direct-owner proof scripts; fix `storage-custody-core` unit imports; then sequence either `recovery-bundle-and-handoff-contract` or `tracking-consumer-integration`
- obvious missing evidence fields: no honest plan-level proof bundle; no Windows/Android/Linux proof artifacts collected for this plan; no scoped integration/security/property/idempotency/compatibility proof for substrate handoffs; no refreshed parent-domain local-export proof artifact
- archival note: self-report only; unverified input for structural review

## Raw Self-Report

# COMPLETION_ARCHITECTURE_REPORT

## Executive Summary

`data-custody-storage-plan` is not closeable from current repo truth. The owned storage/custody substrate exists only as partial contracts across `packages/data-custody-domain`, `packages/production-domain`, `packages/endpoint-domain`, `crates/storage-custody-core`, and `crates/ocentra-evidence`, with one remaining substrate path still trapped in `packages/parent-domain/src/parent-owned-local-export-runtime.ts`. Canonical proof roots are missing, multiple proof scripts are wired to the wrong owner package or stale test paths, `cargo test -p ocentra-storage-custody-core` fails on a wrong `DomainEvent` import, and no honest plan-level proof bundle currently exists. `tracking-plan` and `device-trust-bootstrap-plan` are downstream consumers of this substrate and should not be counted as substrate completion.

## Plan Closure Definition

"Actually done" for this plan means all of the following are true:

| Requirement | Exact done bar |
| --- | --- |
| Storage/custody substrate ownership is explicit | Direct owner contracts live in `packages/data-custody-domain`, `packages/production-domain`, `packages/endpoint-domain`, `crates/storage-custody-core`, and `crates/ocentra-evidence`, with no false implication that tracking-specific or device-trust-specific runtime is already complete. |
| Local substrate paths are testable | Direct-owner unit and contract tests pass under their owning package or crate; remaining parent-domain holdout paths have honest proof or explicit blocked evidence. |
| Proof roots are canonical and current | `docs/proof/data-custody-storage-plan` and `output/data-custody-storage-plan-proof` exist and contain current, direct-owner artifacts rather than stale parent-domain shims or adjacent-plan screenshots. |
| Proof scripts point at real owners | Storage/custody proof scripts target the real package/crate owners and real test paths, and emit artifacts under the canonical proof root. |
| Consumer boundaries are explicit | `tracking-plan` and `device-trust-bootstrap-plan` consume the substrate via explicit contracts; this plan does not pretend their algorithms or runtime/report behavior are done. |
| Applicable tests exist by risk | Unit, contract, integration, security, invariant/property, idempotency/delete-export, and platform-feasible proof exist where the plan actually owns risk. |
| Scoped validation is green | Cheap owner-scoped package/crate validation passes; no repo-wide validation is required for this plan to report honest local progress. |
| Remaining external constraints are explicit | Apple-host-only proof, if ever relevant, is called out separately and not mixed with avoidable Windows/Android/Linux gaps. |

## Exact Docs / Source / Tests / Proof Read

### Plan docs read

- `docs/plans/data-custody-storage-plan/AGENTS.md`
- `docs/plans/data-custody-storage-plan/PLAN_STATE.md`
- `docs/plans/data-custody-storage-plan/NEXT_ACTIONS.md`
- `docs/plans/data-custody-storage-plan/WORKPACK_INDEX.md`
- `docs/plans/data-custody-storage-plan/DOC_INDEX.md`
- `docs/plans/data-custody-storage-plan/ROUTE_INDEX.md`
- `docs/plans/data-custody-storage-plan/CHECKLIST_INDEX.md`
- `docs/plans/data-custody-storage-plan/TEST_PROOF_EXPECTATIONS.md`
- `docs/plans/data-custody-storage-plan/PROOF_INDEX.md`
- `docs/plans/data-custody-storage-plan/DECISIONS.md`
- `docs/plans/data-custody-storage-plan/DATA_CLASSIFICATION.md`
- `docs/plans/data-custody-storage-plan/KEY_CUSTODY_MODEL.md`
- `docs/plans/data-custody-storage-plan/PARENT_STORAGE_PROVIDER_MATRIX.md`
- `docs/plans/data-custody-storage-plan/BUNDLE_PROTOCOL.md`
- `docs/plans/data-custody-storage-plan/EVENT_MODEL.md`
- `docs/plans/data-custody-storage-plan/UI_EXPECTATIONS.md`
- `docs/plans/data-custody-storage-plan/PLATFORM_KEY_CUSTODY_MATRIX.md`
- `docs/plans/data-custody-storage-plan/PARENT_SAVE_RETRIEVE_APPLY_FLOW.md`
- `docs/plans/data-custody-storage-plan/RESEARCH_AND_UI_GUIDANCE.md`
- `docs/plans/data-custody-storage-plan/README.md`
- `docs/plans/data-custody-storage-plan/PLAN_HEALTH.md`
- `docs/plans/data-custody-storage-plan/PLAN_EXECUTION_BLUEPRINT.md`
- `docs/plans/data-custody-storage-plan/ARCHIVE_INDEX.md`
- `docs/plans/data-custody-storage-plan/workpacks/01-custody-source-of-truth.md`
- `docs/plans/data-custody-storage-plan/workpacks/02-provider-contract-and-crypto-shape.md`
- `docs/plans/data-custody-storage-plan/workpacks/03-parent-owned-cloud-sync.md`
- `docs/plans/data-custody-storage-plan/workpacks/04-device-trust-handoff-contract.md`
- `docs/plans/data-custody-storage-plan/workpacks/05-export-import-backup-recovery.md`
- `docs/plans/data-custody-storage-plan/workpacks/06-report-query-custody.md`
- `docs/plans/data-custody-storage-plan/workpacks/07-platform-key-custody-proof.md`
- `docs/plans/data-custody-storage-plan/workpacks/08-parent-storage-settings-apply-flow.md`
- `docs/plans/data-custody-storage-plan/workpacks/data and AI Ui plan.md`

### Feature docs read

- `docs/features/evidence-store-query.md`
- `docs/features/reports-notifications-sync.md`
- `docs/features/remote-lan-mobile-platforms.md`
- `docs/features/screen-visibility-live-view.md`

### Expectation docs read

- `docs/expectations/data-custody.md`
- `docs/expectations/evidence-storage.md`
- `docs/expectations/sync-export.md`
- `docs/expectations/cloud.md`
- `docs/expectations/notifications.md`
- `docs/expectations/platforms.md`
- `docs/expectations/static-analysis-security.md`

### Source and test surface read

- `packages/data-custody-domain/src/custody-boundary.ts`
- `packages/data-custody-domain/src/data-custody-matrix.ts`
- `packages/data-custody-domain/tests/unit/data-custody-matrix.test.ts`
- `packages/production-domain/src/parent-owned-sync-export.ts`
- `packages/production-domain/src/stateless-report-compiler-status.ts`
- `packages/production-domain/src/stateless-report-compiler-status-values.ts`
- `packages/production-domain/tests/unit/parent-owned-sync-export.test.ts`
- `packages/production-domain/tests/unit/stateless-report-compiler-status.test.ts`
- `packages/endpoint-domain/src/constants/sync-export.ts`
- `packages/endpoint-domain/tests/unit/sync-export.test.ts`
- `packages/parent-domain/src/parent-owned-local-export-runtime.ts`
- `packages/parent-domain/tests/unit/parent-owned-local-export-runtime.test.ts`
- `packages/tracking-domain/src/tracking-retention-runtime.ts`
- `packages/tracking-domain/src/tracking-retention-settings-mutation-proof.ts`
- `packages/tracking-domain/src/tracking-report-export-read-model-proof.ts`
- `packages/tracking-domain/tests/contract/*`
- `packages/portal-domain/src/tracking-retention-settings-hosted-ui-proof.ts`
- `apps/portal/e2e/tracking-hosted-ui-proof.spec.ts`
- `packages/family-domain/src/setup-lifecycle.ts`
- `crates/storage-custody-core/src/storage_custody.rs`
- `crates/storage-custody-core/tests/unit/custody_decision.rs`
- `crates/storage-custody-core/tests/unit/custody_flow.rs`
- `crates/ocentra-evidence/src/lib.rs`
- `scripts/test/parent-owned-sync-export-manifest-proof.mjs`
- `scripts/test/stateless-report-compiler-status-proof.mjs`
- `scripts/test/sync-export-endpoint-contract-proof.mjs`
- `scripts/test/parent-owned-local-export-runtime-proof.mjs`

### Proof/doc roots inspected

- `docs/proof/data-custody-storage-plan` was absent in the working tree, but `git show HEAD:docs/proof/data-custody-storage-plan/PLAN_PROOF_MANIFEST.md` proved the path existed in `HEAD` and had been deleted or not restored in the current worktree.
- `output/data-custody-storage-plan-proof` was absent.

## Current Truth Split

| Surface | Done | Partial | False-green / misleading | Missing |
| --- | --- | --- | --- | --- |
| `packages/data-custody-domain` | Basic custody contract exists | Matrix only seeds eight classes and does not cover the full plan surface | Folder presence and tests could be mistaken for complete substrate coverage | Broader custody taxonomy, cross-surface invariants, and downstream contract use |
| `packages/production-domain` | `parent-owned-sync-export` and `stateless-report-compiler-status` contracts exist with unit tests | They are contract-only owners, not real runtime/export implementation | Old proof scripts still pointed at `parent-domain`, falsely implying the wrong owner | Honest proof rewiring and clearer known-gap wording |
| `packages/endpoint-domain` | Sync/export constant contract exists and unit test passes | Scope is narrow but real | Stale proof path `tests/sync-export.test.ts` produced fake-red proof script failures unrelated to the owner code | Canonical artifact wiring |
| `packages/parent-domain/src/parent-owned-local-export-runtime.ts` | Real substrate path exists | It remains a substrate holdout inside broader parent-domain debt | Existing proof script path drift and package debt can be misread as plan completion or pure non-issue | Honest targeted proof or blocked artifact; eventual extraction or clearer contract ownership |
| `crates/storage-custody-core` | Storage custody core exists with unit tests | Tests fail and no downstream executor/runtime is wired | Presence of test files could be misread as green substrate proof | Import fix, honest green unit proof, then broader invariants and delete/export flows |
| `crates/ocentra-evidence` | Narrow crate tests pass | Scope is limited to evidence shape, not end-to-end custody | Green crate tests do not prove plan-level evidence-query custody closure | Canonical proof bundle and consumer integration proof |
| Plan proof roots | Historical manifest path existed in git | None in working tree | Folder names and old plan proof references imply a proof root that is not actually present | Restore `docs/proof/data-custody-storage-plan` and create `output/data-custody-storage-plan-proof` |
| Tracking/runtime consumers | Tracking contract and proof-builder surfaces exist | Consumer integration is only partial | `src/*proof*.ts` files and placeholder folders must not count as tests | Real consumer integration tests and portal/e2e proof under the downstream plan |
| Device-trust recovery consumer | Recovery handoff flags exist in `packages/family-domain/src/setup-lifecycle.ts` | Persistence/delete-export handoff is not closed | Presence of handoff field names could be mistaken for completed substrate persistence | Explicit downstream recovery-bundle persistence contract and tests |

## Code Surface And Ownership

### Storage/custody substrate owned here

| Owner | Exact files / paths | Current truth |
| --- | --- | --- |
| `@ocentra-parent/data-custody-domain` | `packages/data-custody-domain/src/custody-boundary.ts`; `packages/data-custody-domain/src/data-custody-matrix.ts` | Source-of-truth vocabulary exists but remains too narrow for the full plan. |
| `@ocentra-parent/production-domain` | `packages/production-domain/src/parent-owned-sync-export.ts`; `packages/production-domain/src/stateless-report-compiler-status.ts`; `packages/production-domain/src/stateless-report-compiler-status-values.ts` | These are the correct contract owners for parent-owned sync/export manifest and stateless report compiler status. |
| `@ocentra-parent/endpoint-domain` | `packages/endpoint-domain/src/constants/sync-export.ts` | Narrow direct owner for sync/export endpoint constants. |
| `ocentra-storage-custody-core` | `crates/storage-custody-core/src/storage_custody.rs` | Rust substrate exists but is not yet honestly green due to failing tests and limited behavior surface. |
| `ocentra-evidence` | `crates/ocentra-evidence/src/lib.rs` | Evidence substrate shape exists and tests pass, but this is narrow supporting substrate, not plan closure. |

### Substrate-adjacent but not owned end-to-end here

| Surface | Exact files / paths | Boundary |
| --- | --- | --- |
| Parent local export runtime holdout | `packages/parent-domain/src/parent-owned-local-export-runtime.ts` | This is still part of the substrate contract, but parent-domain debt should not dominate the entire plan. It needs honest proof, not scope expansion. |
| Tracking runtime/report consumers | `packages/tracking-domain/src/tracking-retention-runtime.ts`; `packages/tracking-domain/src/tracking-retention-settings-mutation-proof.ts`; `packages/tracking-domain/src/tracking-report-export-read-model-proof.ts`; `packages/tracking-domain/tests/contract/*` | Owned by `tracking-plan`; this plan supplies the substrate they consume. |
| Portal hosted tracking UI proof | `packages/portal-domain/src/tracking-retention-settings-hosted-ui-proof.ts`; `apps/portal/e2e/tracking-hosted-ui-proof.spec.ts` | Downstream consumer proof, not substrate ownership. |
| Recovery-bundle/delete-export handoff | `packages/family-domain/src/setup-lifecycle.ts`; `docs/plans/device-trust-bootstrap-plan/RECOVERY_RESET_MODEL.md` | Owned by `device-trust-bootstrap-plan`; this plan owns the persistence/delete/export substrate contract, not the trust-state algorithm. |

## Test Surface Inventory

| Category | Present now | Quality | Missing or misleading |
| --- | --- | --- | --- |
| Unit | `packages/data-custody-domain/tests/unit/*`; `packages/production-domain/tests/unit/*`; `packages/endpoint-domain/tests/unit/*`; `crates/storage-custody-core/tests/unit/*`; `crates/ocentra-evidence` crate tests | Proper owner-level unit structure exists in most direct owners | `crates/storage-custody-core` is not green; `packages/parent-domain` targeted unit proof is still blocked by broader package debt |
| Contract | `packages/tracking-domain/tests/contract/*` | Downstream consumer contract tests are in the right folder | They belong to the consumer plan and do not close this substrate plan alone |
| Integration | No honest direct-owner substrate integration suite surfaced | Missing | Export/import/delete/report substrate crossings are not tested end-to-end at the direct-owner boundary |
| E2E / Playwright | `apps/portal/e2e/tracking-hosted-ui-proof.spec.ts` | Real consumer e2e exists downstream | No substrate-owned portal/storage e2e proof for this plan |
| Property / invariant | None found for custody invariants | Missing | Storage-custody-core invariant testing is absent |
| Security / abuse | None found for tampered bundles, wrong-household restore, or custody leakage | Missing | Security-sensitive substrate handoffs are unproved |
| Concurrency / idempotency | No direct-owner delete/export/retry/idempotency proof found | Missing | Hidden risk for bundle persistence and delete/export behavior |
| Migration / compatibility | No versioned bundle compatibility proof surfaced | Missing | Bundle protocol evolution remains unproved |
| Logging / metrics / tracing / monitoring / alerting | No honest plan-owned proof surfaced | Missing | No evidence that custody-sensitive flows are instrumented or redacted correctly |
| Inline or false tests | `packages/tracking-domain/src/*proof*.ts`; `packages/portal-domain/src/*proof*.ts` | These may be useful proof builders | They must not be counted as real tests or major-category coverage |

## Proof Inventory

| Artifact / path | Current truth | Issue |
| --- | --- | --- |
| `docs/proof/data-custody-storage-plan/PLAN_PROOF_MANIFEST.md` | Present in `HEAD`, absent in working tree | Canonical plan proof manifest is currently missing from the worktree |
| `output/data-custody-storage-plan-proof` | Absent | No canonical generated proof root exists |
| `scripts/test/parent-owned-sync-export-manifest-proof.mjs` | Exists | Miswired to `@ocentra-parent/parent-domain` and stale parent-domain paths; should target `production-domain` |
| `scripts/test/stateless-report-compiler-status-proof.mjs` | Exists | Miswired to `@ocentra-parent/parent-domain`; should target `production-domain` |
| `scripts/test/sync-export-endpoint-contract-proof.mjs` | Exists | Points to nonexistent `tests/sync-export.test.ts` instead of `packages/endpoint-domain/tests/unit/sync-export.test.ts` |
| `scripts/test/parent-owned-local-export-runtime-proof.mjs` | Exists | Owner path is still `parent-domain`, which is honest for now, but its test path is stale and it needs a proof-chain bypass plus honest blocked output if broader package debt still stops it |
| Consumer proof files in `packages/tracking-domain/src` and `packages/portal-domain/src` | Exist | Useful support artifacts, but not major-category tests and not sufficient plan proof on their own |

## Scoped Validation Inventory

### Cheap commands already passing

| Command | Result |
| --- | --- |
| `npm run test --workspace @ocentra-parent/data-custody-domain -- tests/unit/data-custody-matrix.test.ts` | pass |
| `npm run test --workspace @ocentra-parent/production-domain -- tests/unit/parent-owned-sync-export.test.ts` | pass |
| `npm run test --workspace @ocentra-parent/production-domain -- tests/unit/stateless-report-compiler-status.test.ts` | pass |
| `npm run test --workspace @ocentra-parent/endpoint-domain -- tests/unit/sync-export.test.ts` | pass |
| `cargo test -p ocentra-evidence` | pass |

### Cheap commands failing

| Command | Failure truth |
| --- | --- |
| `cargo test -p ocentra-storage-custody-core` | Fails because tests import `use ocentra_eventing::DomainEvent;` even though the trait is private there; the correct import is `use ocentra_eventing::envelope::DomainEvent;` |
| `npm run lint:architecture -- --files packages/parent-domain/src/parent-owned-sync-export.ts packages/parent-domain/src/stateless-report-compiler-status.ts packages/parent-domain/src/stateless-report-compiler-status-values.ts` | Fails because the old proof-target files were wrong and collided with broader parent-domain debt |
| `npm run test --workspace @ocentra-parent/endpoint-domain -- tests/sync-export.test.ts` | Fails with `No test files found`, proving the proof script path is stale |
| `npm run test --workspace @ocentra-parent/parent-domain -- tests/parent-owned-sync-export.test.ts` | Fails because this is the wrong owner package and the old proof script points at it anyway |

### Scoped commands still required after substrate-truth-repair

| Command | Purpose |
| --- | --- |
| `cargo test -p ocentra-storage-custody-core` | Confirm direct-owner Rust substrate is honestly green after import repair |
| `npm run test --workspace @ocentra-parent/production-domain -- tests/unit/parent-owned-sync-export.test.ts` | Confirm direct-owner sync/export manifest contract |
| `npm run test --workspace @ocentra-parent/production-domain -- tests/unit/stateless-report-compiler-status.test.ts` | Confirm direct-owner report compiler status contract |
| `npm run test --workspace @ocentra-parent/endpoint-domain -- tests/unit/sync-export.test.ts` | Confirm endpoint contract path after proof-script repair |
| `node scripts/test/parent-owned-sync-export-manifest-proof.mjs` | Generate honest production-domain artifact |
| `node scripts/test/stateless-report-compiler-status-proof.mjs` | Generate honest production-domain artifact |
| `node scripts/test/sync-export-endpoint-contract-proof.mjs` | Generate honest endpoint-domain artifact |
| `node scripts/test/parent-owned-local-export-runtime-proof.mjs` | Generate honest parent-domain artifact or blocked proof |

## Dependency Graph

| Bucket | Dependency | Exact contract / proof needed | Why it matters |
| --- | --- | --- | --- |
| `local-now` | Proof root restoration | Restore `docs/proof/data-custody-storage-plan/PLAN_PROOF_MANIFEST.md`; create `output/data-custody-storage-plan-proof` | Without canonical roots the plan has no honest artifact home |
| `local-now` | Proof script rewiring | Fix `scripts/test/parent-owned-sync-export-manifest-proof.mjs`, `scripts/test/stateless-report-compiler-status-proof.mjs`, `scripts/test/sync-export-endpoint-contract-proof.mjs`, and `scripts/test/parent-owned-local-export-runtime-proof.mjs` | Current scripts point at wrong owners or stale test paths |
| `local-now` | Rust substrate unit repair | Fix imports in `crates/storage-custody-core/tests/unit/custody_decision.rs` and `crates/storage-custody-core/tests/unit/custody_flow.rs` | Current crate is false-red for a local fixable reason |
| `needs-coordinator-sequencing` | Next slice selection | Choose between `recovery-bundle-and-handoff-contract` and `tracking-consumer-integration` after substrate-truth-repair | Both depend on a truthful substrate base; coordinator should sequence the first downstream consumer |
| `needs-sibling-plan-contract` | Tracking consumer closure | `tracking-plan` must supply real integration/contract/e2e proof for retention/runtime/report behavior that consumes this substrate | This plan does not own tracking-specific algorithms or runtime behavior |
| `needs-sibling-plan-contract` | Device-trust recovery closure | `device-trust-bootstrap-plan` must supply recovery-bundle persistence/delete-export handoff tests and proof | This plan owns the substrate boundary, not trust-state behavior |
| `host-platform-limited` | Apple proof if later required | Real iOS/macOS proof would need an Apple host | Not a current blocker on this Windows host unless a later slice makes it relevant |

## Platform Feasibility

| Platform bucket | What can be proven now | Status |
| --- | --- | --- |
| Windows host | Package/crate tests, proof scripts, portal or desktop scoped checks where relevant | Feasible now and expected |
| Android Studio / synced Samsung device | Android consumer proof where a downstream slice needs it | Feasible later; not a blocker for substrate-truth-repair |
| Linux via WSL / Docker | Linux-side storage or service proof if a later slice requires it | Feasible later; not a blocker for substrate-truth-repair |
| Apple-host-only | iOS/macOS proof | External-platform constraint only if a later slice truly needs it |

## No-Hand-Wave Execution Plan

| Slice | Scope | Files / domains | Validation | Proof artifacts | Exit criteria |
| --- | --- | --- | --- | --- | --- |
| 1. `substrate-truth-repair` | Repair proof roots, proof script owner/path drift, and direct-owner Rust false-red | `docs/proof/data-custody-storage-plan/PLAN_PROOF_MANIFEST.md`; `docs/proof/data-custody-storage-plan/slice-01-substrate-truth-repair.md`; `output/data-custody-storage-plan-proof/**`; `scripts/test/parent-owned-sync-export-manifest-proof.mjs`; `scripts/test/stateless-report-compiler-status-proof.mjs`; `scripts/test/sync-export-endpoint-contract-proof.mjs`; `scripts/test/parent-owned-local-export-runtime-proof.mjs`; `crates/storage-custody-core/tests/unit/custody_decision.rs`; `crates/storage-custody-core/tests/unit/custody_flow.rs` | `cargo test -p ocentra-storage-custody-core`; package-scoped unit tests; direct proof script runs | Canonical plan proof manifest and slice artifact doc; generated proof JSON under `output/data-custody-storage-plan-proof`; honest blocked artifact if parent-domain still fails | Direct owner proof paths are truthful and the crate false-red is removed |
| 2. `recovery-bundle-and-handoff-contract` | Clarify substrate-facing recovery-bundle persistence/delete-export handoff without implementing trust-state algorithms | `packages/family-domain/src/setup-lifecycle.ts` only if contract gaps are substrate-owned; related docs/proof under this plan; downstream coordination with `device-trust-bootstrap-plan` | Scoped family-domain or owning-package tests only if touched | Contract proof showing required persistence/delete-export handoff fields and failure modes | Substrate contract is explicit enough that device-trust can implement against it without guesswork |
| 3. `tracking-consumer-integration` | Clarify how tracking retention/runtime/report consumers bind to the substrate without absorbing tracking algorithms | direct interface/proof seams between `tracking-domain` consumer tests and substrate owners; maybe plan docs/proof route only if this plan owns the contract text | Consumer-scoped contract tests under `tracking-plan` ownership | Honest cross-plan contract artifact noting what this plan owns and what tracking must prove | Tracking can consume the substrate without this plan pretending retention runtime is complete |
| 4. `security-and-compatibility-hardening` | Fill real missing risk categories that belong to the substrate | likely new tests under direct owner `tests/integration`, `tests/security`, `tests/property`, or Rust equivalents | scoped direct-owner suites only | Tamper, wrong-household restore, delete/export idempotency, and bundle compatibility artifacts | Plan has real risk-shaped coverage instead of unit-only optics |
| 5. `platform-feasible-proof-bundle` | Collect Windows/Android/Linux proof where the owned substrate actually runs | direct owner packages/crates plus platform proof docs | scoped platform commands only | Plan proof manifest updated with Windows and, if applicable, Android/Linux artifacts | Remaining host limits are only true Apple-host requirements |

## Blocker Taxonomy

| Bucket | Items |
| --- | --- |
| `local-now` | Missing proof roots; stale proof scripts; stale proof paths; `storage-custody-core` test import defect |
| `needs-coordinator-sequencing` | Decide whether `recovery-bundle-and-handoff-contract` or `tracking-consumer-integration` should follow substrate-truth-repair |
| `needs-sibling-plan-contract` | Final closure for tracking retention/runtime/report behavior; final closure for recovery-bundle persistence/delete-export handoff behavior |
| `host-platform-limited` | Any later iOS/macOS proof that truly requires Apple hardware |

## First Coordinator Ask

If one downstream plan should move first after `substrate-truth-repair`, the recommended predecessor is `device-trust-bootstrap-plan` only if the repo priority is recovery-bundle persistence and delete/export handoff correctness; otherwise keep `tracking-plan` behind the substrate repair but ahead of UI polish only when retention/runtime/report behavior needs to consume the fixed substrate contract immediately. The coordinator should pick the first consumer based on which contract is blocking more threads, but that choice should happen after slice 1 lands because the current proof roots and owner mappings are not yet truthful enough for either consumer to build on.

## Strict Done Bar

Before this plan can ever be marked done, all of the following must be true:

- canonical proof roots exist and contain current artifacts for this plan
- direct owner proof scripts target the real owner packages/crates and current test paths
- direct owner unit/contract tests are green, including `cargo test -p ocentra-storage-custody-core`
- the remaining `parent-domain` local-export substrate path has honest proof or is extracted/retired with replacement proof
- missing substrate-owned risk categories have real coverage where applicable: integration, security, property/invariant, idempotency/delete-export, compatibility
- downstream tracking and device-trust plans have explicit consumer contracts and their own real proof for runtime-specific behavior
- no empty folders, inline proof-builder files, stale screenshots, or adjacent-plan artifacts are counted as this plan's completion proof

## COORDINATOR_DECISION_REQUEST

- recommended next slice: `substrate-truth-repair`
- recommended predecessor plans: none before slice 1; after slice 1, sequence either `device-trust-bootstrap-plan` for recovery-bundle/delete-export handoff contract or `tracking-plan` for retention/runtime/report consumer integration
- estimated risk: `medium`, because most immediate work is truth-repair and proof rewiring, but parent-domain debt and downstream consumer boundaries can still mislead closure claims
- estimated proof difficulty: `medium`, because direct-owner proof is feasible on this Windows host, while honest consumer/platform proof still needs later coordination
- whether I should continue immediately or pause for sequencing: continue immediately with `substrate-truth-repair`; pause only after that slice if the coordinator wants to decide the first downstream consumer

## Optional Addendum

- This archival pass did not implement new product source. At the time of export, `substrate-truth-repair` had been lane-started and exact-file claims had been taken, but no product-source or proof-script edits had landed yet in this worktree.
- Earlier audit passes made one additional workpack-level point that the latest completion architecture report did not state as directly: workpacks `01`, `03`, `05`, and `06` have real but partial substrate owner surfaces; workpack `02` remains contract-shape partial; workpack `04` is a downstream handoff boundary rather than a completed recovery implementation; workpacks `07` and `08` remain largely proof- and runtime-incomplete from current repo truth.
