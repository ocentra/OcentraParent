# parent-desktop-runtime-package-plan

## Normalized Header

- plan/thread name: `parent-desktop-runtime-package-plan`
- source thread label: `dedicated Codex worker thread for parent-desktop-runtime-package-plan`
- source thread id: `019ed32a-5266-7342-8fa8-b03fd9177298`
- worktree: `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent`
- branch: `codex/tracking-plan-full-continuation-a`
- claimed status: `partial; audit complete; completion proposal accepted; slice1 pcrd-build-unblock-and-entrypoint-cleanup complete; plan not done`
- claimed source files/crates/packages: `packages/billing-domain/package.json`, `packages/production-domain/package.json`, `packages/network-domain/package.json`, `packages/parent-domain/src/parent-mobile-runtime.ts`, `packages/parent-domain/src/parent-mobile-runtime-capability-statuses.ts`, `packages/parent-domain/src/billing-entitlement-proof.ts`, plus plan-owned/runtime surfaces in `apps/parent-desktop`, `apps/portal`, `packages/parent-domain`, `packages/setup-domain`, `scripts/release/*`, `scripts/test/*`, and `.github/workflows/*`
- claimed tests: targeted `@ocentra-parent/parent-domain` unit tests passed; desktop shell, parent web distribution, Linux package/update, Android runtime, setup handoff, and release-gate proof remain partial or missing
- claimed proof commands/artifacts: passed scoped build/lint/test commands for slice1; canonical proof root should be `output/parent-client-runtime-distribution-plan-proof/<workpack-id>/`; historical `docs/proof/parent-desktop-runtime-package-plan/` is missing; `test-results/parent-desktop-release-support-proof/` is an empty scaffold and not valid proof
- claimed blockers: distinct parent-client packaging identity not yet closed; setup handoff producer contract is owned by `setup-install-provisioning-plan`; Apple-host proof is host-platform-limited from this Windows lane
- claimed next actions: `pcrd-proof-root-and-parent-web-distribution`, then desktop shell/package proof, parent-client identity closure, Linux update/rollback, Android runtime proof, setup handoff contract, release gate/proof manifest, and Apple manual-required closure
- obvious missing evidence fields: real Windows desktop package artifacts, real Linux update/rollback artifacts, Android emulator proof artifacts, setup handoff contract artifacts, release-gate manifest artifacts, and Apple manual-required evidence
- archival note: self-report only; unverified input for structural review

## Raw Self-Report

**Executive Summary**

`parent-desktop-runtime-package-plan` is not closeable yet. The locally fixable build and entrypoint slice is complete and green. The remaining work is a mix of local packaging and proof work, one real sibling-plan dependency on `setup-install-provisioning-plan` for the setup handoff producer contract, and Apple-host-only proof that stays manual-required from this Windows lane. The next executable local slice is `pcrd-proof-root-and-parent-web-distribution`.

## Plan Closure Definition

This plan is actually done only when all of the following are true:

- parent web, parent desktop, parent Android, and parent iOS distribution boundaries are explicit and do not silently reuse `ocentra-parent-agent*` identity as final product identity
- the desktop artifact boundary is distinct as a parent-client artifact, even if early proof temporarily uses existing agent packaging substrate
- plan-owned code paths build cleanly from the real package graph
- plan-owned tests are real, categorized honestly, and placed under the correct top-level `tests/` major categories where applicable
- proof artifacts exist under a canonical parent-client proof root and are not substituted with old folder names, empty scaffolds, or stale output
- scoped validation is green for each completed slice
- remaining gaps and manual-required platform constraints are explicit

## Docs, Source, Tests, Proof Read

Plan docs read during audit:

- `docs/plans/parent-desktop-runtime-package-plan/AGENTS.md`
- `docs/plans/parent-desktop-runtime-package-plan/ARCHIVE_INDEX.md`
- `docs/plans/parent-desktop-runtime-package-plan/CHECKLIST_INDEX.md`
- `docs/plans/parent-desktop-runtime-package-plan/DECISIONS.md`
- `docs/plans/parent-desktop-runtime-package-plan/DOC_INDEX.md`
- `docs/plans/parent-desktop-runtime-package-plan/NEXT_ACTIONS.md`
- `docs/plans/parent-desktop-runtime-package-plan/PARENT_ANDROID_DISTRIBUTION.md`
- `docs/plans/parent-desktop-runtime-package-plan/PARENT_CLIENT_ARTIFACT_MATRIX.md`
- `docs/plans/parent-desktop-runtime-package-plan/PARENT_CLIENT_ROUTE_BRIDGE_MODEL.md`
- `docs/plans/parent-desktop-runtime-package-plan/PARENT_CLIENT_SCOPE_CORRECTION.md`
- `docs/plans/parent-desktop-runtime-package-plan/PARENT_DESKTOP_DISTRIBUTION.md`
- `docs/plans/parent-desktop-runtime-package-plan/PARENT_IOS_DISTRIBUTION.md`
- `docs/plans/parent-desktop-runtime-package-plan/PARENT_WEB_PORTAL_DISTRIBUTION.md`
- `docs/plans/parent-desktop-runtime-package-plan/parent-desktop-runtime-package-20-step-plan.md`
- `docs/plans/parent-desktop-runtime-package-plan/parent-desktop-runtime-package-test-blueprint.md`
- `docs/plans/parent-desktop-runtime-package-plan/PLAN_EXECUTION_BLUEPRINT.md`
- `docs/plans/parent-desktop-runtime-package-plan/PLAN_HEALTH.md`
- `docs/plans/parent-desktop-runtime-package-plan/PLAN_STATE.md`
- `docs/plans/parent-desktop-runtime-package-plan/PROOF_AND_TEST_INVENTORY.md`
- `docs/plans/parent-desktop-runtime-package-plan/PROOF_INDEX.md`
- `docs/plans/parent-desktop-runtime-package-plan/README.md`
- `docs/plans/parent-desktop-runtime-package-plan/README_FULL_ORIGINAL.md`
- `docs/plans/parent-desktop-runtime-package-plan/RESEARCH_AND_UI_GUIDANCE.md`
- `docs/plans/parent-desktop-runtime-package-plan/ROUTE_INDEX.md`
- `docs/plans/parent-desktop-runtime-package-plan/runtime-package-requirements-guide.md`
- `docs/plans/parent-desktop-runtime-package-plan/SIGNING_STORE_NOTARIZATION_MATRIX.md`
- `docs/plans/parent-desktop-runtime-package-plan/TEST_PROOF_EXPECTATIONS.md`
- `docs/plans/parent-desktop-runtime-package-plan/UPDATE_ROLLBACK_MODEL.md`
- `docs/plans/parent-desktop-runtime-package-plan/WORKPACK_INDEX.md`
- `docs/plans/parent-desktop-runtime-package-plan/workpacks/01-parent-client-scope-and-route-boundary.md`
- `docs/plans/parent-desktop-runtime-package-plan/workpacks/01-tauri-shell-contract-boundary.md`
- `docs/plans/parent-desktop-runtime-package-plan/workpacks/02-local-service-connection-command.md`
- `docs/plans/parent-desktop-runtime-package-plan/workpacks/02-parent-web-portal-distribution.md`
- `docs/plans/parent-desktop-runtime-package-plan/workpacks/03-lan-route-and-controller-state.md`
- `docs/plans/parent-desktop-runtime-package-plan/workpacks/03-parent-desktop-shell-package.md`
- `docs/plans/parent-desktop-runtime-package-plan/workpacks/04-parent-android-package.md`
- `docs/plans/parent-desktop-runtime-package-plan/workpacks/04-parent-observer-read-only-state.md`
- `docs/plans/parent-desktop-runtime-package-plan/workpacks/05-custody-and-source-labels.md`
- `docs/plans/parent-desktop-runtime-package-plan/workpacks/05-parent-ios-package.md`
- `docs/plans/parent-desktop-runtime-package-plan/workpacks/06-parent-local-service-route-bridge.md`
- `docs/plans/parent-desktop-runtime-package-plan/workpacks/06-parent-mobile-bridge-boundary.md`
- `docs/plans/parent-desktop-runtime-package-plan/workpacks/07-parent-client-signing-store-matrix.md`
- `docs/plans/parent-desktop-runtime-package-plan/workpacks/07-windows-installer-and-preview.md`
- `docs/plans/parent-desktop-runtime-package-plan/workpacks/08-cross-platform-package-preview-matrix.md`
- `docs/plans/parent-desktop-runtime-package-plan/workpacks/08-parent-client-update-rollback.md`
- `docs/plans/parent-desktop-runtime-package-plan/workpacks/09-parent-client-launch-smoke-matrix.md`
- `docs/plans/parent-desktop-runtime-package-plan/workpacks/09-update-channel-and-rollback-scaffold.md`
- `docs/plans/parent-desktop-runtime-package-plan/workpacks/10-setup-handoff-contracts.md`
- `docs/plans/parent-desktop-runtime-package-plan/workpacks/10-signing-notarization-store-claims.md`
- `docs/plans/parent-desktop-runtime-package-plan/workpacks/11-proof-ci-release-gate.md`
- `docs/plans/parent-desktop-runtime-package-plan/workpacks/11-support-diagnostics-and-redaction.md`
- `docs/plans/parent-desktop-runtime-package-plan/workpacks/12-privacy-and-release-docs.md`
- `docs/plans/parent-desktop-runtime-package-plan/workpacks/13-desktop-launch-smoke.md`
- `docs/plans/parent-desktop-runtime-package-plan/workpacks/14-tauri-build-and-dev-scripts.md`
- `docs/plans/parent-desktop-runtime-package-plan/workpacks/15-platform-capability-matrix.md`
- `docs/plans/parent-desktop-runtime-package-plan/workpacks/16-release-branch-boundary.md`
- `docs/plans/parent-desktop-runtime-package-plan/workpacks/17-github-actions-artifact-proof.md`
- `docs/plans/parent-desktop-runtime-package-plan/workpacks/18-manual-platform-proof-runbook.md`
- `docs/plans/parent-desktop-runtime-package-plan/workpacks/19-product-checklist-feature-doc-sync.md`
- `docs/plans/parent-desktop-runtime-package-plan/workpacks/20-pr-ci-rollout-gate.md`

Feature and expectation docs re-read:

- `docs/features/production-distribution-support.md`
- `docs/features/child-agent-local-service.md`
- `docs/features/remote-lan-mobile-platforms.md`
- `docs/expectations/platforms.md`
- `docs/expectations/release-installer.md`
- `docs/expectations/real-evidence-proof.md`

Primary source, test, and proof surfaces inspected:

- `apps/parent-desktop`
- `apps/portal`
- `packages/parent-domain`
- `packages/setup-domain`
- `packages/billing-domain`
- `packages/production-domain`
- `packages/network-domain`
- `platforms/android/agent`
- `platforms/ios/OcentraParentMobile*`
- `platforms/ios/OcentraParentAgent*`
- `scripts/release/windows/build-agent-package.ps1`
- `scripts/release/linux/build-agent-package.sh`
- `scripts/release/macos/build-agent-package.sh`
- `scripts/release/android/build-agent-package.mjs`
- `scripts/release/parent-android/build-parent-mobile-package.mjs`
- `scripts/release/parent-ios/build-parent-mobile-simulator-app.sh`
- `scripts/test/parent-desktop-release-support-proof.mjs`
- `scripts/test/parent-mobile-shell-runtime-proof.mjs`
- `scripts/test/parent-mobile-service-bridge-proof.mjs`
- `scripts/test/parent-mobile-controller-observer-handoff-proof.mjs`
- `scripts/test/parent-mobile-package-source-artifact-proof.mjs`
- `scripts/test/parent-desktop-runtime-package-proof.test.mjs`
- `.github/workflows/package-preview.yml`
- `.github/workflows/release.yml`
- `.github/workflows/ci-package-parent-android.yml`
- `.github/workflows/ci-package-parent-ios.yml`

## Current Truth

| Area | Status | Exact truth |
| --- | --- | --- |
| `packages/billing-domain/package.json`, `packages/production-domain/package.json`, `packages/network-domain/package.json`, `packages/parent-domain/src/parent-mobile-runtime.ts`, `packages/parent-domain/src/parent-mobile-runtime-capability-statuses.ts`, `packages/parent-domain/src/billing-entitlement-proof.ts` | done for slice1 | Build unblock and forbidden re-export cleanup landed; scoped build, architecture lint, and targeted parent-domain tests passed. |
| `packages/parent-domain` runtime entrypoints | partial | Parent-domain now builds, but this does not prove packaging or distribution closure. |
| `apps/parent-desktop` | partial | Desktop shell exists and `tauri:check` is wired, but there is no real dedicated desktop test surface under a major `tests/` category and no canonical proof artifacts for distribution. |
| `apps/portal` parent-client distribution | partial | Real app and Playwright surfaces exist, but distribution proof for hosted parent web remains incomplete. |
| `scripts/release/windows/build-agent-package.ps1`, `scripts/release/linux/build-agent-package.sh`, `scripts/release/macos/build-agent-package.sh`, `scripts/release/android/build-agent-package.mjs`, `.github/workflows/package-preview.yml`, `.github/workflows/release.yml` | partial / identity mismatch | Current packaging substrate still emits `ocentra-parent-agent*` identity on several platforms. This can be used as interim substrate for early proof only, not as final closure. |
| `scripts/release/parent-android/build-parent-mobile-package.mjs`, `scripts/release/parent-ios/build-parent-mobile-simulator-app.sh`, `.github/workflows/ci-package-parent-android.yml`, `.github/workflows/ci-package-parent-ios.yml` | partial | Dedicated parent mobile surfaces exist, but proof/artifact closure is not complete. |
| `docs/proof/parent-desktop-runtime-package-plan/` | missing | Historical proof root is absent and cannot support current truth claims. |
| `output/parent-client-runtime-distribution-plan-proof/` | missing | Canonical parent-client proof root does not exist yet. |
| `packages/setup-domain` producer-side handoff surface | missing / partial | The docs refer to `parentInstallPackage`, `parentClientLaunch`, and `parentClientReadiness`, but real producer-backed contract implementation was not found. |
| `packages/parent-domain/tests`, `packages/portal-domain/tests` | false-green / partial | Many top-level major test folders are `.gitkeep` scaffolds only; some real tests are correctly placed, but several real tests remain misplaced or coverage is implied by empty folders. |

## Test Surface Inventory

| Surface | Current truth | Gap |
| --- | --- | --- |
| `packages/parent-domain/tests/unit` | real tests exist for runtime boundary, release-support, service bridge, controller-observer handoff, package info, browser policy compiler, tracking policy compiler | root-level `packages/parent-domain/tests/*.test.ts` files are misplaced and should move under a major category such as `tests/unit` |
| `packages/parent-domain/tests/logging` and `packages/parent-domain/tests/observability` | some real files exist | the surrounding top-level category tree still includes many placeholder-only folders and should not be counted as coverage |
| `packages/parent-domain/tests/{ai-safety,chaos,clock-skew,concurrency,consumer-driven,contract,differential,e2e,human-misuse,integration,invariant,load,migration,monitoring,mutation,quality,release,security}` | mostly scaffold-only | empty folders create optics but not proof; category ownership must be honest |
| `packages/portal-domain/tests` | real unit tests exist | organization remains partial and category folders include placeholder optics |
| `apps/portal/tests` and `apps/portal/e2e` | real app tests and Playwright specs exist | organization is feature-based rather than normalized major categories; hosted parent-web distribution proof still needs explicit artifact capture |
| `apps/parent-desktop` | no real dedicated desktop tests discovered | missing integration/e2e/package proof on the desktop surface |
| `packages/setup-domain/tests/unit` | real producer-side unit tests exist | they do not prove the missing parent-client handoff contract named by this plan |

## Proof Inventory

| Surface | Current truth |
| --- | --- |
| `scripts/test/parent-desktop-release-support-proof.mjs` | harness exists, but no canonical proof artifact set was found in this checkout |
| `scripts/test/parent-mobile-shell-runtime-proof.mjs` | harness exists, but proof root/artifact chain is missing |
| `scripts/test/parent-mobile-service-bridge-proof.mjs` | harness exists, but proof root/artifact chain is missing |
| `scripts/test/parent-mobile-controller-observer-handoff-proof.mjs` | harness exists, but proof root/artifact chain is missing |
| `scripts/test/parent-mobile-package-source-artifact-proof.mjs` | harness exists, but proof root/artifact chain is missing |
| `scripts/test/parent-desktop-runtime-package-proof.test.mjs` | harness exists, but current artifact truth is not materialized under the canonical root |
| `test-results/parent-desktop-release-support-proof/` | empty scaffold, not valid proof |

Canonical proof root required for this plan:

- `output/parent-client-runtime-distribution-plan-proof/<workpack-id>/`

## Scoped Validation Inventory

Already passed:

- `npm run build --workspace @ocentra-parent/billing-domain`
- `npm run build --workspace @ocentra-parent/production-domain`
- `npm run build --workspace @ocentra-parent/network-domain`
- `npm run build --workspace @ocentra-parent/parent-domain`
- `npm run lint:architecture -- --files packages/parent-domain/src/parent-mobile-runtime.ts packages/parent-domain/src/parent-mobile-runtime-capability-statuses.ts packages/parent-domain/src/billing-entitlement-proof.ts`
- `$env:OCENTRA_PARENT_DOMAIN_TEST_SKIP_PROOF_CHAIN='1'; npm test --workspace @ocentra-parent/parent-domain -- tests/unit/parent-mobile-runtime.test.ts tests/unit/billing-entitlement-proof.test.ts`

Known missing or not yet run for later slices:

- scoped parent-web distribution validation
- scoped desktop package validation on Windows
- scoped Linux package/update validation via WSL or Docker
- Android emulator validation
- setup handoff contract validation once producer contract exists
- proof-root manifest validation and release-gate validation

## Dependency Map

| Bucket | Items | Why |
| --- | --- | --- |
| `local-now` | `pcrd-proof-root-and-parent-web-distribution`; `pcrd-desktop-shell-package-and-windows-proof`; `pcrd-parent-client-identity-closure`; `pcrd-linux-update-rollback-substrate`; `pcrd-desktop-android-runtime-proof`; `pcrd-release-gate-and-proof-manifest` | These are implementable or prove-able on this Windows lane with local source, Windows tooling, Android Studio/emulator, and Linux/WSL/Docker. |
| `needs-sibling-plan-contract` | `pcrd-setup-handoff-contract` depends on `setup-install-provisioning-plan` producer-side contract in `packages/setup-domain` | This plan is the consumer-side owner, but the producer-side contract surface is not yet real. |
| `needs-coordinator-sequencing` | final reconciliation of parent-client artifact identity across package scripts, workflows, and release language | This is locally actionable, but coordinator sequencing may be needed if parallel lanes are touching shared release surfaces. |
| `host-platform-limited` | real macOS notarization/signing proof; real iOS build/signing/store proof | These require Apple-host evidence and remain manual-required from this Windows lane. |

## No-Hand-Wave Execution Plan

| Slice | Scope | Files/domains to touch | Validation | Proof exit criteria |
| --- | --- | --- | --- | --- |
| `pcrd-build-unblock-and-entrypoint-cleanup` | completed | `packages/billing-domain/package.json`; `packages/production-domain/package.json`; `packages/network-domain/package.json`; `packages/parent-domain/src/parent-mobile-runtime.ts`; `packages/parent-domain/src/parent-mobile-runtime-capability-statuses.ts`; `packages/parent-domain/src/billing-entitlement-proof.ts` | passed scoped builds, architecture lint, and targeted parent-domain tests | parent-domain build path proven locally fixable |
| `pcrd-proof-root-and-parent-web-distribution` | next | `output/parent-client-runtime-distribution-plan-proof/`; parent-web distribution docs/proof scripts; `apps/portal`; related plan proof docs if needed | scoped parent-web build/test/proof validation only | canonical proof root exists; parent-web distribution artifacts are captured honestly |
| `pcrd-desktop-shell-package-and-windows-proof` | local Windows closure slice | `apps/parent-desktop`; Windows packaging/proof scripts; workflow fragments if slice-scoped | scoped desktop build/package checks on Windows | parent desktop package proof exists under canonical root with explicit identity notes |
| `pcrd-parent-client-identity-closure` | final artifact naming boundary | Windows/Linux/Android packaging scripts, workflows, package naming docs | scoped packaging/workflow validation | parent-client artifact identity stops silently reusing `ocentra-parent-agent*` |
| `pcrd-linux-update-rollback-substrate` | Linux packaging/update proof | Linux scripts, update/rollback docs, proof harness | scoped Linux validation via WSL/Docker | Linux proof exists with update/rollback artifacts |
| `pcrd-desktop-android-runtime-proof` | Android closure slice | Android package/proof scripts, Android workflow/docs, emulator proof harness | emulator-first scoped validation | Android emulator artifacts exist under canonical root; Samsung-only proof remains separate unless explicitly claimed |
| `pcrd-setup-handoff-contract` | producer-consumer contract | `packages/setup-domain`; `packages/parent-domain`; plan docs/proof harness | scoped contract/integration validation once producer exists | setup handoff contract is real and evidenced |
| `pcrd-release-gate-and-proof-manifest` | honest release-gate closure | proof manifests, CI/workflow gates, release docs | scoped gate validation only | release gate reflects actual proof state, no placeholder optics |
| `pcrd-apple-external-proof-or-manual-required-closure` | Apple-host-limited closure | docs/proof manifests only from this lane unless Apple-host evidence arrives | docs-only/manual-required validation here | Apple rows explicitly remain manual-required unless external evidence is supplied |

## First Coordinator Ask

The first true upstream dependency is `setup-install-provisioning-plan` producer-side contract work. This plan can continue immediately on local slices before that, but final WP10 and WP11 closure will need a real producer contract in `packages/setup-domain` for the consumer-side handoff proof to be honest.

## Strict Done Bar

Do not mark this plan done until all of the following are true:

- parent-client artifact identity is distinct and no longer piggybacks on `ocentra-parent-agent*` as final closure
- parent web, desktop, Android, and iOS rows each have honest status backed by real proof or explicit manual-required notes
- canonical proof artifacts live under `output/parent-client-runtime-distribution-plan-proof/`
- empty proof folders, stale `test-results` scaffolds, and placeholder category trees are not counted as evidence
- setup handoff producer and consumer contract proof is real
- scoped validation is green for each finished slice
- remaining Apple-host-only rows are explicitly external-platform/manual-required rather than implied complete

## COORDINATOR_DECISION_REQUEST

- recommended next slice: `pcrd-proof-root-and-parent-web-distribution`
- recommended predecessor plans: `setup-install-provisioning-plan` should move before final WP10/WP11 closure, but not before the next local proof-root/web-distribution slice
- estimated risk: medium, because packaging identity, proof-root truth, and release-surface sharing can create false closure if not kept explicit
- estimated proof difficulty: medium-high, because Windows, Linux, Android, and release proof need real artifacts rather than script existence
- whether this thread should continue immediately or pause for sequencing: continue immediately on the next local slice; pause only before setup-handoff closure or if coordinator sequencing is needed on shared release surfaces

## Optional Addendum

- Earlier audit passes also established that `test-results/parent-desktop-release-support-proof/` is present only as an empty scaffold and must never be counted as proof, even if a later report focuses on the missing canonical proof root instead.
- Earlier audit passes also established that several real `packages/parent-domain/tests/*.test.ts` files still live at the root of `tests/` rather than under a major category such as `tests/unit`, so the current test tree shape overstates category completeness.
