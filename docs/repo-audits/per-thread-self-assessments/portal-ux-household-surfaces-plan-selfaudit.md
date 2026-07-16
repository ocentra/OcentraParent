# portal-ux-household-surfaces-plan

## Normalized Header

- plan/thread name: `portal-ux-household-surfaces-plan`
- source thread label: dedicated Codex thread for `docs/plans/portal-ux-household-surfaces-plan`
- source thread id: `019ed32b-60cb-7901-8eb7-1d03c518aa54`
- worktree: `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent`
- branch: `codex/tracking-plan-full-continuation-a`
- claimed status: not near honest closure; partial WP02/WP10 start-route work exists; focused `portal-domain` tests and narrow architecture gate are green; portal consumer truth remains red
- claimed source files/crates/packages: `apps/portal`, `packages/portal-domain`, especially `apps/portal/src/ParentPortalRoute.tsx`, `apps/portal/src/SetupFirstRunRoutePanel.tsx`, `apps/portal/src/live-activity-state.ts`, `apps/portal/tests/live-activity-surface-adapter.test.ts`, `apps/portal/tests/setup-first-run-route-panel.test.ts`, `apps/portal/e2e/setup-first-run-ui-proof.spec.ts`, `packages/portal-domain/src/setup-first-run-panel.ts`, `packages/portal-domain/src/parent-portal-service-state.ts`, `packages/portal-domain/src/routes.ts`, `packages/portal-domain/tests/unit/setup-first-run-panel.test.ts`
- claimed tests: `npm run test --workspace @ocentra-parent/portal-domain -- tests/unit/setup-first-run-panel.test.ts tests/unit/parent-portal-service-state.test.ts` passed; `cd apps/portal && npx vitest run tests/setup-first-run-route-panel.test.ts tests/live-activity-surface-adapter.test.ts` stayed mixed with `live-activity-surface-adapter.test.ts` red; `apps/portal/e2e/setup-first-run-ui-proof.spec.ts` exists but proof is not yet landed
- claimed proof commands/artifacts: `npm run lint:architecture -- --files apps/portal/src/SetupFirstRunRoutePanel.tsx packages/portal-domain/src/setup-first-run-panel.ts apps/portal/src/live-activity-state.ts packages/portal-domain/src/parent-portal-service-state.ts apps/portal/tests/setup-first-run-route-panel.test.ts packages/portal-domain/tests/unit/setup-first-run-panel.test.ts apps/portal/tests/live-activity-surface-adapter.test.ts` passed; canonical proof roots `output/portal-ux-household-surfaces-plan-proof/` and `docs/proof/portal-ux-household-surfaces-plan/` are missing
- claimed blockers: local LAN add-device consumer truth remains red; many plan-owned routes still resolve through `ProductSurfacePending`; true closure depends on `account-identity-family-plan` and `policy-control-plane-plan`; empty test category folders create false-green optics
- claimed next actions: `slice-01-start-route-and-lan-consumer-truth`, `slice-02-route-ownership-and-policy-ia-normalization`, `slice-03-proof-pipeline-and-test-layout-cleanup`
- obvious missing evidence fields: no canonical proof artifacts under the plan proof roots; no Windows/Android/Linux proof captures for this plan; no resolved consumer-path validation log for the LAN state adapter; no sibling-plan coordination outcome artifacts
- archival note: self-report only; unverified input for structural review

## Raw Self-Report

`COMPLETION_ARCHITECTURE_REPORT`

Executive summary:

This plan is not close to honest closure. One real local slice is in flight for WP02 and WP10 on the Start route and LAN consumer path, focused `portal-domain` unit coverage is green, and the focused architecture gate over the touched files is green. Portal consumer truth is still red in `apps/portal/tests/live-activity-surface-adapter.test.ts`, many plan-owned routes still resolve through `ProductSurfacePending`, the canonical proof roots for this plan do not exist, and `packages/portal-domain/tests/*` still contains empty category folders that should not count as coverage.

## Current truth snapshot

| Area | Current truth |
| --- | --- |
| Route ownership | Many plan-owned routes still fall through `ProductSurfacePending` in `packages/portal-domain/src/routes.ts`, so the route map is not genuinely owned by completed household surfaces. |
| Start route / WP02 | `apps/portal/src/ParentPortalRoute.tsx` now mounts `SetupFirstRunRoutePanel`, backed by `apps/portal/src/SetupFirstRunRoutePanel.tsx` and `packages/portal-domain/src/setup-first-run-panel.ts`. This is a route-contract projection, not full setup/account/install/trust completion. |
| LAN pairing consumer / WP10 | `apps/portal/tests/live-activity-surface-adapter.test.ts` remains red, which means the portal consumer is still not proving real `lanAddDeviceReadModel` truth. |
| Tests | Narrow `packages/portal-domain/tests/unit/setup-first-run-panel.test.ts` and `packages/portal-domain/tests/unit/parent-portal-service-state.test.ts` are green. `apps/portal/tests/setup-first-run-route-panel.test.ts` is green. The combined targeted `apps/portal` run stays red because of LAN consumer parsing. |
| Proof | `apps/portal/e2e/setup-first-run-ui-proof.spec.ts` exists, but there is no canonical artifact set under `output/portal-ux-household-surfaces-plan-proof/` and no mirrored proof pack under `docs/proof/portal-ux-household-surfaces-plan/`. |
| Test organization | `packages/portal-domain/tests/` contains empty top-level category directories such as `contract`, `integration`, `e2e`, `property-based`, `security`, and `load`. Those are optics, not coverage. |
| Dependencies | `account-identity-family-plan` and `policy-control-plane-plan` still affect final closure, but there is still local plan-owned work that can proceed now. |

## Completion definition

True completion for this plan means plan-owned parent portal household surfaces are actually implemented on the real route map, not delegated to pending placeholders; the owned projections in `packages/portal-domain` and `apps/portal` are consistent with sibling-plan contracts; required tests live under real major categories and cover real route, consumer, contract, degraded-state, and proof behavior; canonical proof artifacts exist under the plan proof roots; and scoped validation is green without relying on stale docs, placeholder folders, or fake-green test optics.

## End-to-end solution path

1. Finish the current atomic slice around the Start route and LAN consumer truth.
   Files and surfaces:
   `apps/portal/src/ParentPortalRoute.tsx`
   `apps/portal/src/SetupFirstRunRoutePanel.tsx`
   `packages/portal-domain/src/setup-first-run-panel.ts`
   `apps/portal/src/live-activity-state.ts`
   `apps/portal/tests/setup-first-run-route-panel.test.ts`
   `apps/portal/tests/live-activity-surface-adapter.test.ts`
   `packages/portal-domain/tests/unit/setup-first-run-panel.test.ts`
   Exit criteria:
   Start route projection remains coherent, and the targeted `apps/portal` consumer-path validation is green.

2. Replace remaining `ProductSurfacePending` ownership across plan-owned routes with real, plan-owned route consumers or explicitly document them as blocked by sibling-plan contracts.
   Primary surfaces:
   `packages/portal-domain/src/routes.ts`
   `apps/portal/src/ParentPortalRoute.tsx`
   plan-owned route panel components and tests in `apps/portal/tests/*`
   Exit criteria:
   each plan-owned route is either backed by a real panel/consumer path or called out as blocked by a named contract from a sibling plan.

3. Normalize route and IA truth where current route contracts and manage navigation do not line up.
   Primary surfaces:
   `packages/portal-domain/tests/unit/contracts.test.ts`
   `apps/portal/tests/policy-preview-route-panel.test.ts`
   related route/panel contract sources under `packages/portal-domain/src/`
   Exit criteria:
   route ownership, navigation exposure, and preview panel attachments agree.

4. Remove false-green test optics and fill the missing major coverage categories only where the plan risk actually requires them.
   Primary surfaces:
   `packages/portal-domain/tests/`
   `apps/portal/tests/`
   `apps/portal/e2e/`
   Exit criteria:
   empty scaffolds no longer imply coverage, and missing route/contract/e2e/degraded-state coverage is either added or explicitly marked as absent.

5. Establish the proof pipeline and land canonical artifacts.
   Primary surfaces:
   `output/portal-ux-household-surfaces-plan-proof/`
   `docs/proof/portal-ux-household-surfaces-plan/`
   Playwright proof specs under `apps/portal/e2e/`
   Exit criteria:
   proof artifacts, command logs, negative states, and skipped-risk notes all exist in the canonical roots.

## Dependency map

| Bucket | Item | Why it matters |
| --- | --- | --- |
| can do now | `slice-01-start-route-and-lan-consumer-truth` | This is local to `apps/portal` and `packages/portal-domain` and already partially in motion. |
| can do now | proof-root creation and artifact discipline | The plan can create honest proof roots without waiting on sibling plans. |
| can do now | test layout cleanup and false-green removal | Empty scaffolds and misplaced coverage are local hygiene problems. |
| needs coordinator/other plan | `account-identity-family-plan` | Final Start-route and household/profile truth depends on account, role, and household authority contracts this plan should consume, not invent. |
| needs coordinator/other plan | `policy-control-plane-plan` | Policy authoring, schedules, approvals, and enforcement route truth depends on policy contract ownership and route/IA sequencing. |
| needs coordinator/other plan | sequencing for broad WP02 and WP05-WP07 closure | After the current Start-route slice, broad closure should follow the real upstream contract order. |
| not feasible on this Windows host | real native iOS/macOS proof | This host can support Windows, Android, and Linux/WSL or Docker proof, but not native Apple-host execution. No current claim requires Apple-host-only proof to continue local closure work. |

## Test/proof reorganization and missing coverage

- `packages/portal-domain/tests/` still has empty major-category folders. They should not be counted as contract, integration, e2e, property, security, load, monitoring, or consumer-driven coverage.
- Real coverage currently lives mostly under `packages/portal-domain/tests/unit/` and `apps/portal/tests/`.
- The plan still lacks honest category-complete coverage for:
  - route ownership and placeholder elimination across all plan-owned routes
  - consumer-path integration for LAN add-device truth
  - degraded, empty, stale, and manual-required UI states across the broader household surfaces
  - proof-producing Playwright flows with canonical artifact output
- `apps/portal/e2e/setup-first-run-ui-proof.spec.ts` is a real direction for proof, but it does not close the plan alone.

## Validation/proof commands and artifacts

Already passing:

- `npm run test --workspace @ocentra-parent/portal-domain -- tests/unit/setup-first-run-panel.test.ts tests/unit/parent-portal-service-state.test.ts`
- `npm run lint:architecture -- --files apps/portal/src/SetupFirstRunRoutePanel.tsx packages/portal-domain/src/setup-first-run-panel.ts apps/portal/src/live-activity-state.ts packages/portal-domain/src/parent-portal-service-state.ts apps/portal/tests/setup-first-run-route-panel.test.ts packages/portal-domain/tests/unit/setup-first-run-panel.test.ts apps/portal/tests/live-activity-surface-adapter.test.ts`

Still red:

- `cd apps/portal && npx vitest run tests/setup-first-run-route-panel.test.ts tests/live-activity-surface-adapter.test.ts`

Required artifact roots before this plan can be called done:

- `output/portal-ux-household-surfaces-plan-proof/<workpack-file-stem>/`
- `docs/proof/portal-ux-household-surfaces-plan/`

## First 3 atomic slices in recommended order

| Slice | Files and surfaces | Exit criteria |
| --- | --- | --- |
| `slice-01-start-route-and-lan-consumer-truth` | `apps/portal/src/ParentPortalRoute.tsx`, `apps/portal/src/SetupFirstRunRoutePanel.tsx`, `packages/portal-domain/src/setup-first-run-panel.ts`, `apps/portal/src/live-activity-state.ts`, `apps/portal/tests/setup-first-run-route-panel.test.ts`, `apps/portal/tests/live-activity-surface-adapter.test.ts`, `packages/portal-domain/tests/unit/setup-first-run-panel.test.ts` | targeted `apps/portal` run goes green and the Start route remains a real contract projection rather than placeholder copy |
| `slice-02-route-ownership-and-policy-ia-normalization` | `packages/portal-domain/src/routes.ts`, `apps/portal/src/ParentPortalRoute.tsx`, relevant route panel contract sources and `apps/portal/tests/policy-preview-route-panel.test.ts` | plan-owned routes stop depending on generic pending shells; route and IA truth align |
| `slice-03-proof-pipeline-and-test-layout-cleanup` | `packages/portal-domain/tests/`, `apps/portal/e2e/`, `output/portal-ux-household-surfaces-plan-proof/`, `docs/proof/portal-ux-household-surfaces-plan/` | empty-folder optics are removed or neutralized, proof artifacts land in canonical roots, and plan-local validation is reproducible |

## Exact coordinator asks / unblock requests

- Sequence `account-identity-family-plan` ahead of broad WP02 closure so this plan consumes real household, role, and setup-readiness contracts instead of inventing them locally.
- Sequence `policy-control-plane-plan` ahead of broad WP05-WP07 closure so policy authoring, schedules, approvals, and enforcement route truth is contract-owned by the policy plan.
- Allow this plan to continue immediately on `slice-01-start-route-and-lan-consumer-truth` because it is local, already partially implemented, and reduces present red validation.

## COORDINATOR_DECISION_REQUEST

- recommended next slice: `slice-01-start-route-and-lan-consumer-truth`
- recommended predecessor plans: `account-identity-family-plan` first, then `policy-control-plane-plan`
- estimated risk: medium-high
- estimated proof difficulty: high
- whether this thread should continue immediately or pause for sequencing: continue immediately on slice 01, then pause before broader WP02 and WP05-WP07 closure if sequencing is not confirmed

## Optional Addendum

- Earlier audit passes also found stale historical workpack docs that should not be trusted as current proof or completion evidence:
  - `docs/plans/portal-ux-household-surfaces-plan/workpacks/manage UI proof checklist.md`
  - `docs/plans/portal-ux-household-surfaces-plan/workpacks/policy Ui fix.md`
  - `docs/plans/portal-ux-household-surfaces-plan/workpacks/portal and account Ui fix.md`
- Earlier scoped architecture validation also showed existing re-export debt in plan-adjacent portal-domain sources:
  - `packages/portal-domain/src/contracts.ts`
  - `packages/portal-domain/src/parent-portal-guides.ts`
  - `packages/portal-domain/src/tracking-hosted-ui-proofs.ts`
  Those broader architecture failures were not part of the later narrow green gate and still matter for honest closure.
- Earlier audit passes also found an IA and route-contract mismatch: `packages/portal-domain/tests/unit/contracts.test.ts` asserts that Manage navigation does not expose `RuleManagement`, `Schedules`, `Approvals`, or `Enforcement`, while `apps/portal/tests/policy-preview-route-panel.test.ts` still attaches the policy preview panel to those legacy routes.
- Earlier failing `apps/portal/tests/live-activity-surface-adapter.test.ts` assertions were not just generic red state. The specific failures showed `state.lanAddDeviceReadModel?.selectedDeviceReadiness` and `state.lanAddDeviceReadModel?.addDeviceState` resolving to `undefined` where the tests expected real LAN add-device readiness and paired-state truth.
