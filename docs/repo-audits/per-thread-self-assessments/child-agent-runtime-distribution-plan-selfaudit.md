# child-agent-runtime-distribution-plan

## Normalized Header

- plan/thread name: `child-agent-runtime-distribution-plan`
- source thread label: `codex-a child-agent-runtime-distribution-plan worker thread`
- source thread id: `019ed326-fbcb-7322-9b96-8028febe80e5`
- worktree: `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent`
- branch: `codex/tracking-plan-full-continuation-a`
- claimed status: `in progress; not closeable yet; partial + false-green + missing work remain; no DONE claim`
- claimed source files/crates/packages: `scripts/release/windows/*`, `scripts/release/linux/*`, `scripts/release/android/build-agent-package.mjs`, `scripts/release/ios/build-simulator-app.sh`, `scripts/test/child-android-*.mjs`, `scripts/test/child-ios-entitlement-capability-proof.mjs`, `scripts/test/mobile-child-agent-capability-proof.mjs`, `scripts/test/tamper-uninstall-artifact-status-proof.mjs`, `packages/child-runtime-domain`, `packages/enforcement-domain`, `packages/setup-domain`, `packages/parent-domain`, `crates/provisioning-core`
- claimed tests: `packages/child-runtime-domain/tests/unit/*.test.ts`, `packages/enforcement-domain/tests/unit/tamper-uninstall-artifact-status.test.ts`, `packages/setup-domain/tests/unit/*.test.ts`, `crates/provisioning-core/tests/unit/*`, `scripts/release/linux/linux-package-baseline.test.mjs`, `scripts/test/windows-package-lifecycle-proof.test.mjs`
- claimed proof commands/artifacts: `npm run test:child-android-device-proof-artifact-gate`, `npm run test:child-ios-entitlement-capability-proof`, `npm run test:mobile-child-agent-capability-proof`, `node scripts/release/windows/package-lifecycle-proof.mjs --artifact-dir ...`, `scripts/smoke/linux-deb-smoke.sh`, `target/release-packages/android/*`, canonical proof root `output/child-agent-runtime-distribution-plan-proof/<workpack-file-stem>/`
- claimed blockers: `no plan-owned proof-root writer`, `child proof contract tests miscategorized under tests/unit`, `tamper proof script points at a non-existent parent-domain test path`, `WP10 depends on setup/device-trust sibling-plan contracts`, `real macOS/iOS runtime proof is Apple-host-limited`
- claimed next actions: `proof-root-materializer-and-test-category-normalization`, `windows-linux-real-package-proof`, `android-emulator-device-proof`, `tamper-uninstall-and-respawn-runtime-proof`, `signing-store-device-owner-matrix`, `setup-device-trust-handoff-contract-proof`, `apple-host external proof pack`, `final WP11 release-gate aggregation`
- obvious missing evidence fields: `no current canonical proof packs under output/child-agent-runtime-distribution-plan-proof/...`, `no current WP10 proof script/artifacts`, `no current Windows/Linux proof logs captured in canonical root`, `no Apple-host runtime artifacts`, `no real Android emulator/device artifacts yet`
- archival note: self-report only; unverified input for structural review

## Raw Self-Report

**Executive Summary**
`child-agent-runtime-distribution-plan` is not closeable yet. The repo has real Windows/Linux/Android/iOS-source packaging and proof scaffolds, plus real setup/tamper contract surfaces, but the plan still lacks a canonical writer to `output/child-agent-runtime-distribution-plan-proof/<workpack>/`, several child proof tests are miscategorized under `tests/unit`, the tamper proof script points at a non-existent parent-domain test path, and WP07/WP09/WP10 have no honest completion proof. I stopped before making repo edits in this checkpoint; only lane `STARTED` reporting and exact-path locks were added.

**Plan Closure Definition**
"Actually done" for this plan means all child-runtime distribution workpacks have real code or honest manual-required boundaries, real tests in truthful package/crate `tests/` categories, real proof packs under `output/child-agent-runtime-distribution-plan-proof/<workpack-file-stem>/`, scoped validation logs, and no stale claims that confuse source-proof/package-proof/device-proof/runtime-proof.

**Exact Docs / Source / Tests / Proof Read**

| Surface | Exact files |
| --- | --- |
| Route/rules | `.ocentra-ai/rules/ocentra-parent-rules.mdc`, `.ocentra-ai/rules/ocentra-parent-validation.mdc`, `.ocentra-ai/rules/ocentra-parent-source-shape.mdc`, `.ocentra-ai/rules/ocentra-parent-domain-boundaries.mdc`, `.ocentra-ai/rules/ocentra-parent-test-rules.mdc`, `docs/agent/TASK_ROUTER.md`, `docs/agent/WORKER_LANE_FLOW.md`, `docs/agent/PLAN_WORKER_FLOW.md`, `docs/PLAN_INDEX.md` |
| Plan docs | `docs/plans/child-agent-runtime-distribution-plan/AGENTS.md`, `PLAN_STATE.md`, `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, `PLAN_EXECUTION_BLUEPRINT.md`, `TEST_PROOF_EXPECTATIONS.md`, `PROOF_INDEX.md`, `PROOF_AND_TEST_INVENTORY.md`, `workpacks/11-proof-ci-release-gate.md` |
| Feature/expectations | `docs/features/child-agent-local-service.md`, `docs/expectations/platforms.md`, `docs/expectations/real-evidence-proof.md`, `docs/expectations/tamper-uninstall-protection.md` |
| Packaging/proof source | `package.json`, `scripts/release/windows/build-agent-package.ps1`, `scripts/release/windows/package-lifecycle-proof.mjs`, `scripts/release/windows/package-lifecycle-host.mjs`, `scripts/release/windows/package-lifecycle-runner.mjs`, `scripts/release/linux/build-agent-package.sh`, `scripts/release/linux/linux-package-baseline.test.mjs`, `scripts/smoke/linux-deb-smoke.sh`, `scripts/release/android/build-agent-package.mjs`, `scripts/release/ios/build-simulator-app.sh` |
| Child proof source/tests | `scripts/test/child-android-*.mjs`, `scripts/test/child-ios-entitlement-capability-proof.mjs`, `scripts/test/mobile-child-agent-capability-proof.mjs`, `packages/child-runtime-domain/src/*.ts`, `packages/child-runtime-domain/tests/unit/*.test.ts` for the Android/iOS/mobile proof contracts |
| Tamper/setup/provisioning | `scripts/test/tamper-uninstall-artifact-status-proof.mjs`, `packages/enforcement-domain/src/tamper-uninstall-artifact-status.ts`, `packages/enforcement-domain/src/tamper-uninstall-artifact-status-read-model.ts`, `packages/enforcement-domain/tests/unit/tamper-uninstall-artifact-status.test.ts`, `packages/parent-domain/src/tamper-uninstall-artifact-status.ts`, `packages/parent-domain/src/tamper-uninstall-artifact-status-read-model.ts`, `packages/setup-domain/src/readiness.ts`, `packages/setup-domain/src/setup-state-machine.ts`, `packages/setup-domain/src/family-setup-bridge.ts`, `packages/setup-domain/tests/unit/readiness.test.ts`, `packages/setup-domain/tests/unit/setup-state-machine.test.ts`, `packages/setup-domain/tests/unit/family-setup-bridge.test.ts`, `crates/provisioning-core/src/provisioning_install.rs`, `crates/provisioning-core/tests/unit/readiness.rs`, `crates/provisioning-core/tests/unit/readiness_flow.rs` |
| Proof/artifacts inspected | `target/release-packages/android/*`, `output/*` proof-pack examples, `test-results/*` references encoded in the child/tamper proof scripts, `target/release-packages/windows` and `target/release-packages/linux` existence checks, `wsl -l -q` |

**Current Truth**

| Workpack | Current truth | Status class |
| --- | --- | --- |
| WP01 child scope/route boundary | Plan boundary exists, but no proof pack under `output/child-agent-runtime-distribution-plan-proof/01-child-agent-scope-and-route-boundary/`; stale docs still overstate proof-runner problems. | partial |
| WP02 Windows package | Real build/proof scripts exist, including install/uninstall-capable lifecycle proof, but no current artifacts in this checkout and no canonical plan-root proof writer. | partial |
| WP03 macOS package | Only plan/docs intent and package script exist; no Apple-host proof from this Windows machine. | missing on this host |
| WP04 Linux package | Real `.deb` builder and real smoke harness exist; WSL is available; no canonical plan-root proof writer and no current Linux artifacts in this checkout. | partial |
| WP05 Android package | Real source/package proof chain exists and current Android artifacts exist, but proofs emit only to `test-results/...`, tests are still under `tests/unit`, and there is no emulator/device proof yet. | partial |
| WP06 iOS capability package | Real source/simulator-scaffold proof exists, but still only `test-results/...`; no Apple entitlement/device proof. | partial |
| WP07 managed respawn | No isolated proof pack or runtime-specific respawn validation yet. | missing |
| WP08 parent-authorized uninstall | Tamper/uninstall artifact-status contract exists, but proof script points at a non-existent parent-domain test path and there is no real platform uninstall/revocation runtime proof. | false-green / partial |
| WP09 signing/store/device-owner matrix | Matrix intent exists in docs only; no proof pack. | missing |
| WP10 setup-device-trust handoff | Real TS/Rust readiness surfaces exist, but no plan-owned proof script/root proving the handoff contract. | partial |
| WP11 proof/CI/release gate | Docs declare `output/...` roots, but no plan-owned materializer/writer currently creates them. | false-green / missing |

**Code Surface And Ownership**

| Surface | Owner | Exact files | Truth |
| --- | --- | --- | --- |
| Child packaging orchestration | release scripts | `scripts/release/windows/*`, `scripts/release/linux/*`, `scripts/release/android/build-agent-package.mjs`, `scripts/release/ios/build-simulator-app.sh` | real |
| Child proof contracts | `@ocentra-parent/child-runtime-domain` | `packages/child-runtime-domain/src/child-android-*.ts`, `child-ios-entitlement-capability-proof.ts`, `mobile-child-agent-capability-proof.ts` | real |
| Tamper/uninstall contract | `@ocentra-parent/enforcement-domain` | `packages/enforcement-domain/src/tamper-uninstall-artifact-status*.ts` | real |
| Misleading export surface | `@ocentra-parent/parent-domain` | `packages/parent-domain/src/child-android-*.ts`, `child-ios-entitlement-capability-proof.ts`, `mobile-child-agent-capability-proof.ts`, `tamper-uninstall-artifact-status*.ts` | architecture debt / misleading ownership |
| Setup-device-trust contract | `@ocentra-parent/setup-domain` + `ocentra-provisioning-core` | `packages/setup-domain/src/readiness.ts`, `setup-state-machine.ts`, `family-setup-bridge.ts`, `crates/provisioning-core/src/provisioning_install.rs` | real |
| Plan-proof aggregation | nobody yet | no current script writes `output/child-agent-runtime-distribution-plan-proof/...` | missing owner implementation |

**Test Surface Inventory**

| Owner | Current tests | Problem | Needed move / added coverage |
| --- | --- | --- | --- |
| `packages/child-runtime-domain` | `tests/unit/child-android-lifecycle-proof.test.ts`, `child-android-storage-protocol-proof.test.ts`, `child-android-service-protocol-proof.test.ts`, `child-android-permission-capability-proof.test.ts`, `child-android-privileged-capability-proof.test.ts`, `child-android-device-proof-artifact-gate.test.ts`, `child-ios-entitlement-capability-proof.test.ts`, `mobile-child-agent-capability-proof.test.ts` | These are contract-schema/proof-shape tests, not unit logic tests. | Move to `packages/child-runtime-domain/tests/contract/` |
| `packages/enforcement-domain` | `tests/unit/tamper-uninstall-artifact-status.test.ts` | Also a contract/read-model honesty test, not a unit-only implementation test. | Move to `packages/enforcement-domain/tests/contract/` |
| Script-adjacent harnesses | `scripts/release/linux/linux-package-baseline.test.mjs`, `scripts/test/windows-package-lifecycle-proof.test.mjs` | Legit script tests, but they do not prove package/runtime completion by themselves. | Keep as auxiliary; do not count as plan closure proof |
| `packages/parent-domain/tests` | many category folders exist as `.gitkeep` scaffolds; no plan-owned tamper contract test there | Empty-folder optics only. | Do not count; no move needed unless parent-domain truly becomes owner |
| Setup/provisioning | `packages/setup-domain/tests/unit/*`, `crates/provisioning-core/tests/unit/*` | Category is truthful for current contract/state-machine level. | Add a plan-owned proof script plus cross-boundary contract evidence for WP10 |
| Missing major categories actually applicable | `integration`, `e2e`, `contract`, `security` | Android device/runtime, Windows install/uninstall, Linux install/remove, setup-device-trust cross-boundary, tamper/uninstall runtime/security boundaries are not covered enough. | Add targeted integration/contract/security proof where named in ordered slices |
| Not actually required now | `property`, `load` | Packaging/distribution risk does not currently justify them as first-order gates. | Optional later, not closure-critical |

**Proof Inventory**

| Proof surface | Current location | Truth | Canonical requirement |
| --- | --- | --- | --- |
| Plan proof roots | `output/child-agent-runtime-distribution-plan-proof/<workpack>/` | declared in docs only; no writer | this is the canonical path |
| Android proof chain | `test-results/child-android-*/proof.json` | real CI/package/source proof, noncanonical | materialize into WP05 and WP11 roots |
| iOS capability proof | `test-results/child-ios-entitlement-capability-proof/proof.json` | real source/simulator-scaffold proof, noncanonical | materialize into WP06 and WP11 roots |
| Mobile aggregate proof | `test-results/mobile-child-agent-capability-proof/proof.json` | real aggregate contract proof, noncanonical | reference from WP05/WP06/WP11 |
| Tamper/uninstall artifact proof | `test-results/tamper-uninstall-artifact-status-proof/proof.json` | real contract proof with wrong test-ownership path in script | materialize into WP08 and WP11 roots |
| Windows package proof | `test-results/windows-package-lifecycle-proof/<timestamp>/proof.json` | real proof harness, parameterizable, noncanonical | materialize into WP02 and WP11 roots |
| Linux smoke proof | `test-results/linux-package-smoke/*.log`, `target/release-packages/linux/linux-baseline.json` | real smoke/artifact evidence, noncanonical | materialize into WP04 and WP11 roots |
| WP10 handoff proof | none | missing | `output/child-agent-runtime-distribution-plan-proof/10-setup-device-trust-handoff/` |
| Apple-host runtime proof | none in this checkout | missing on this host | external Apple-host proof packs for WP03/WP06 if final closure needs them |

**Scoped Validation Inventory**

| State | Command | Truth |
| --- | --- | --- |
| pass | `wsl -l -q` | WSL is available here with `Ubuntu-22.04`; Linux proof is feasible from this host |
| pass | `Get-ChildItem target/release-packages/android` | Current Android APK and checksum artifacts exist in this checkout |
| fail | `docker version --format '{{.Server.Version}}'` | Docker CLI is not available in current PowerShell environment |
| fail | `npm run lanes:guard`, `npm run hub:guard` | lane/session guard currently exits non-zero; exact-file claims still work |
| not yet run in this checkpoint | `npm run test:child-android-device-proof-artifact-gate`, `npm run test:child-ios-entitlement-capability-proof`, `npm run test:mobile-child-agent-capability-proof`, `node scripts/release/windows/package-lifecycle-proof.mjs --artifact-dir ...`, WSL Linux package/smoke commands, targeted vitest/cargo for WP10` | no current-turn proof execution yet; this stop was audit/architecture only |
| stale doc claim, now false by source inspection | `"child Android/iOS proof scripts still point at missing parent-domain test paths"` and `"child proofs still fail through build:contracts"` | current child Android/iOS scripts point at `packages/child-runtime-domain/tests/unit/...` and do not call `build:contracts` |

**Dependency Graph**

| Dependency | Why it matters | Bucket | What I need from it | What can still proceed locally |
| --- | --- | --- | --- | --- |
| `setup-install-provisioning-plan` | WP10 handoff is its consumer boundary | needs-sibling-plan-contract | exact handoff contract/proof expectation and owning artifact language | WP11/WP02/WP04/WP05/WP06/WP08 can proceed |
| `device-trust-bootstrap-plan` | WP10 depends on trusted-device/bootstrap semantics | needs-sibling-plan-contract | final trusted-device/bootstrap refs and no-claim boundary | everything except final WP10 closure |
| `app-plan` | final package/runtime claims depend on real child runtime behavior, not only package/source proof | needs-sibling-plan-contract | if runtime behavior gaps are discovered during Windows/Linux/Android proof | packaging/proof-root/test-normalization can proceed |
| `v0-8-enforcement-control-plan` | WP08 may need integrity/alert/runtime semantics beyond artifact-status contract | needs-sibling-plan-contract | only if parent-authorized uninstall proof expands beyond current contract boundary | contract cleanup and proof-root work can proceed |
| coordinator sequencing only | Apple-host execution timing, Android device access timing, whether another lane is already taking WP10 | needs-coordinator-sequencing | explicit sequencing decision | current slice and Windows/Linux next slice can proceed |

**Platform Feasibility**

| Platform path | Feasible from this host now | Truth |
| --- | --- | --- |
| Windows package/build/lifecycle artifact proof | yes | native host; real Windows proof expected |
| Android package/source proof | yes | current APK artifacts already present |
| Android emulator / Android Studio / physical device proof | yes | feasible path on this host; not verified in this checkpoint, but not an intrinsic blocker |
| Linux package/smoke via WSL | yes | WSL `Ubuntu-22.04` is present |
| Linux via Docker | no current evidence | Docker CLI absent here |
| macOS launchd/package proof | no | host-platform-limited |
| iOS entitlement/device/runtime proof | no real device/Apple-host proof from here | host-platform-limited; source/simulator-scaffold proof only on Windows |

**No-Hand-Wave Ordered Execution Plan**

| Order | Slice | Files / domains to touch | Validation to run | Proof to collect | Exit criteria |
| --- | --- | --- | --- | --- | --- |
| 1 | `proof-root-materializer-and-test-category-normalization` | new plan materializer under `scripts/test/`; child proof scripts in `scripts/test/child-android-*.mjs`, `child-ios-entitlement-capability-proof.mjs`, `mobile-child-agent-capability-proof.mjs`; tamper proof script; move contract tests in `packages/child-runtime-domain/tests/*` and `packages/enforcement-domain/tests/*`; update `PLAN_STATE.md`, `NEXT_ACTIONS.md`, `TEST_PROOF_EXPECTATIONS.md`, `PROOF_AND_TEST_INVENTORY.md`, `workpacks/11-proof-ci-release-gate.md` | targeted vitest for moved tests; `npm run test:child-android-device-proof-artifact-gate`; `npm run test:child-ios-entitlement-capability-proof`; `npm run test:mobile-child-agent-capability-proof`; `node scripts/test/tamper-uninstall-artifact-status-proof.mjs`; `npm run lint:architecture -- --files scripts/test packages/child-runtime-domain/tests packages/enforcement-domain/tests docs/plans/child-agent-runtime-distribution-plan` | WP05/WP06/WP08/WP11 canonical `output/...` roots; honest source/contract proof refs; route-doc truth fixed | canonical proof-root writer exists; proof tests live in truthful categories; stale miswired/build-claims removed |
| 2 | `windows-linux-real-package-proof` | likely no domain-contract change first; mostly `scripts/release/windows/*`, `scripts/release/linux/*`, plan materializer outputs if gaps appear | `npm run release:package:windows`; `node scripts/release/windows/package-lifecycle-proof.mjs --artifact-dir target/release-packages/windows ...`; WSL Linux build + `scripts/smoke/linux-deb-smoke.sh`; architecture lint only if scripts change | WP02 and WP04 roots with real artifact refs, negative/uninstall notes, validation logs | real retained Windows/Linux artifacts exist in canonical roots |
| 3 | `android-emulator-device-proof` | likely `scripts/test/child-android-*.mjs`, Android wrapper source only if proof reveals gaps | current Android proof chain plus emulator/device commands through Android Studio/ADB | WP05 device/runtime artifacts beyond package-local/source proof | foreground service, notification grant, UsageStats, install/runtime artifacts collected or kept explicitly manual/blocked |
| 4 | `tamper-uninstall-and-respawn-runtime-proof` | `scripts/release/windows/package-lifecycle-host.mjs`, Linux smoke/materializer, enforcement/parent read-model only if gap is real | focused Windows install/uninstall lifecycle, Linux remove path, contract tests as needed | WP07 and WP08 real runtime uninstall/respawn evidence | respawn/uninstall claims no longer contract-only |
| 5 | `signing-store-device-owner-matrix` | plan docs plus artifact-manifest readers/materializer | artifact checks only; no broad validation | WP09 proof pack | every platform row explicitly says signed/manual/planned/blocked with evidence refs |
| 6 | `setup-device-trust-handoff-contract-proof` | `packages/setup-domain/*`, `crates/provisioning-core/*`, new proof script, plan docs | targeted `vitest` for `packages/setup-domain/tests/unit/*`; targeted `cargo test -p ocentra-provisioning-core --test ...` or focused unit set; architecture lint if source changes | WP10 root with real TS/Rust contract evidence | handoff inputs/outputs and failure states are proved under this plan |
| 7 | `apple-host external proof pack` | likely docs/proof only in this repo, with artifacts collected elsewhere | Apple-host commands on macOS if a separate host is available | WP03 and any WP06 device/entitlement proof beyond source scaffold | macOS/iOS rows are either externally proved or explicitly left open, never implied |
| 8 | `final WP11 release-gate aggregation` | materializer/docs only | rerun smallest proof/materializer stack; architecture lint on touched surfaces | complete WP11 proof pack | all closed workpacks have canonical proof roots and no stale claims remain |

**Blocker Taxonomy**

| Bucket | Items |
| --- | --- |
| `local-now` | no plan-owned proof-root writer; child proof contract tests miscategorized under `tests/unit`; tamper proof script points at a non-existent parent-domain test path; Windows/Linux canonical roots absent; stale `PLAN_STATE.md` and `NEXT_ACTIONS.md` claims |
| `needs-coordinator-sequencing` | when to schedule Android emulator/physical-device proof; when to schedule Apple-host proof collection; who owns the final WP10 timing |
| `needs-sibling-plan-contract` | `setup-install-provisioning-plan` and `device-trust-bootstrap-plan` for WP10; `app-plan` if runtime gaps appear during platform proof; `v0-8-enforcement-control-plan` only if WP08 expands beyond current artifact-status contract |
| `host-platform-limited` | real macOS package/launchd/notarization proof; real iOS entitlement/background/device/TestFlight/App Store proof |

**First Coordinator Ask**
If you need one predecessor plan identified now, it is `setup-install-provisioning-plan` paired with `device-trust-bootstrap-plan`, but not before slice 1. They are the first true cross-plan prerequisite for honest final closure because WP10 cannot be closed from packaging proof alone; it needs the typed setup/trust handoff contract and proof language from those owner plans.

**Strict Done Bar**
Before this plan can be marked done, all of the following must be true:
1. Closed workpacks have real proof packs under `output/child-agent-runtime-distribution-plan-proof/<workpack-file-stem>/`.
2. Windows and Linux have real retained package/lifecycle evidence from this or another honest run, not docs-only claims.
3. Android has real package proof plus emulator/device proof for every upgraded runtime claim; remaining privileged gaps stay explicit.
4. iOS/macOS are either externally proved on Apple hardware or remain open/manual-required; Windows-host source proof alone does not close them.
5. Child proof-schema tests live in truthful `tests/contract` locations under their owning packages.
6. Tamper/uninstall proof references the real owning enforcement-domain test surface, not a fake parent-domain path.
7. WP10 has a real setup-device-trust handoff proof pack backed by `setup-domain` and `provisioning-core`.
8. WP07/WP08/WP09 are backed by real runtime/matrix evidence, not only contract text.
9. Scoped validation commands and their logs are captured in the proof packs.
10. No stale plan doc claims remain about proof locations, test ownership, or validation blockers.

**COORDINATOR_DECISION_REQUEST**
- recommended next slice: `proof-root-materializer-and-test-category-normalization`
- recommended predecessor plans: none before that slice; first true predecessor for final closure is `setup-install-provisioning-plan` plus `device-trust-bootstrap-plan` before WP10
- estimated risk: medium-high because the plan spans package proof, runtime proof, sibling-plan contracts, and external Apple-host evidence
- estimated proof difficulty: high; Windows/Linux are feasible, Android is feasible but device-dependent, Apple proof is external-host-limited
- whether I should continue immediately or pause for sequencing: continue immediately on slice 1; pause only when approaching WP10 or Apple-host proof if coordinator sequencing is not yet assigned

## Optional Addendum

- This archival pass was completed after a lane `STARTED` report and exact-file locks for a future implementation slice, but before any new repo source/doc/test/proof edits landed for that slice.
- One correction already captured in the raw report is especially important: the current child Android/iOS proof scripts do point at `packages/child-runtime-domain/tests/unit/...` in this checkout, so older "miswired to missing parent-domain tests" wording is stale for those scripts. The still-bad ownership path is the tamper proof script, which cites `packages/parent-domain/tests/tamper-uninstall-artifact-status.test.ts` even though the real owning test currently lives at `packages/enforcement-domain/tests/unit/tamper-uninstall-artifact-status.test.ts`.
- An earlier audit finding that should not be lost: the `packages/parent-domain/src/child-android-*.ts`, `packages/parent-domain/src/child-ios-entitlement-capability-proof.ts`, `packages/parent-domain/src/mobile-child-agent-capability-proof.ts`, and `packages/parent-domain/src/tamper-uninstall-artifact-status*.ts` wrapper files are explicit `export *` re-exports. Under the repo's architecture rule, those are banned barrel-style exports, so they are not just "misleading ownership" but active architecture debt that must not be counted as a clean closure state.
