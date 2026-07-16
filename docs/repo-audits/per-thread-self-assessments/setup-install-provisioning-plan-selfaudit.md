# setup-install-provisioning-plan

## Normalized Header

- plan/thread name: `setup-install-provisioning-plan`
- source thread label: `setup-install-provisioning-plan`
- source thread id: `019ed32e-cd01-7d01-adb4-66cba4589938`
- worktree: `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent`
- branch: `codex/tracking-plan-full-continuation-a`
- claimed status: partial; WP01/WP02/WP04/WP07 owned slices claimed green, WP03 and WP05 claimed partial, WP06 claimed false-green/stale, no `DONE` or `PR_READY`
- claimed source files/crates/packages: `packages/setup-domain`, `packages/production-domain`, `packages/parent-domain`, `packages/billing-domain/package.json`, `packages/production-domain/package.json`, `packages/network-domain/package.json`, `packages/portal-domain`, `apps/portal`, `crates/provisioning-core`, `crates/child-runtime`, `packages/child-runtime-domain`
- claimed tests: `packages/setup-domain/tests/unit/*`, `packages/production-domain/tests/unit/*`, `packages/portal-domain/tests/unit/setup-first-run-panel.test.ts`, `apps/portal/tests/setup-first-run-route-panel.test.ts`, `apps/portal/tests/live-activity-network-flow.test.ts`, `apps/portal/tests/logging/portal-dev-log-route.test.ts`, `apps/portal/e2e/setup-first-run-ui-proof.spec.ts`, `crates/provisioning-core/tests/unit/*`, `crates/child-runtime/tests/unit/runtime_gate.rs`, `crates/child-runtime/tests/integration/tracking_runtime_flow_intent.rs`, `packages/parent-domain/tests/unit/parent-desktop-release-support.test.ts`
- claimed proof commands/artifacts: `output/setup-install-provisioning-plan-proof/01-*` through `07-*`; scoped builds/tests for `@ocentra-parent/setup-domain`, `@ocentra-parent/production-domain`, `@ocentra-parent/portal-domain`, `@ocentra-parent/child-runtime-domain`; scoped Rust tests for `ocentra-provisioning-core` and `ocentra-child-runtime`; Playwright proof for `apps/portal/e2e/setup-first-run-ui-proof.spec.ts`; failing `@ocentra-parent/parent-domain` build/test; stale `output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/*`
- claimed blockers: WP06 stale proof drift; WP03 local package export mismatch; WP05 redaction proof still blocker-only; sibling-plan proof gaps for `account-identity-family-plan`, `parent-desktop-runtime-package-plan`, `child-agent-runtime-distribution-plan`, `app-plan`, `lan-plan`, `device-trust-bootstrap-plan`, `data-custody-storage-plan`, `policy-control-plane-plan`
- claimed next actions: `WP06 truth-sync`, then `WP03` export-surface repair and focused rerun, then WP03/WP06 proof refresh, then WP05 redaction ownership resolution, then sibling-proof consumption
- obvious missing evidence fields: real account/provider/session execution proof; real parent signed installer/update/rollback proof; real child runtime/package/platform proof; real LAN/device-trust proof; custody/policy baseline proof; fresh WP06 aggregate after WP07; successful `@ocentra-parent/parent-domain` rerun after export fix
- archival note: self-report only; unverified input for structural review

## Raw Self-Report

**Executive Summary**
`setup-install-provisioning-plan` is not closeable yet, but its state is now clear. The real green core is WP01, WP02, WP04, and WP07 at their owned slice boundaries; WP03 and WP05 are only partial; WP06 is currently false-green because the plan indexes say “done” while the aggregate proof pack still says rollout is blocked and still references WP07 as missing. The highest-leverage local sequence is `WP06 truth-sync` first, then the already-prepared `WP03 export-surface repair`, then a refreshed aggregate. Final closure is still blocked by sibling-plan proof for account, parent runtime distribution, child runtime/package, LAN/device trust, data custody, and policy baseline.

**Exact Read Surface**
Docs read:
- `.ocentra-ai/rules/ocentra-parent-rules.mdc`
- `.ocentra-ai/rules/ocentra-parent-validation.mdc`
- `.ocentra-ai/rules/ocentra-parent-source-shape.mdc`
- `.ocentra-ai/rules/ocentra-parent-domain-boundaries.mdc`
- `docs/agent/TASK_ROUTER.md`
- `docs/agent/WORKER_LANE_FLOW.md`
- `docs/agent/PLAN_WORKER_FLOW.md`
- `docs/PLAN_INDEX.md`
- `docs/plans/setup-install-provisioning-plan/AGENTS.md`
- `docs/plans/setup-install-provisioning-plan/PLAN_STATE.md`
- `docs/plans/setup-install-provisioning-plan/NEXT_ACTIONS.md`
- `docs/plans/setup-install-provisioning-plan/WORKPACK_INDEX.md`
- `docs/plans/setup-install-provisioning-plan/CHECKLIST_INDEX.md`
- `docs/plans/setup-install-provisioning-plan/TEST_PROOF_EXPECTATIONS.md`
- `docs/plans/setup-install-provisioning-plan/PROOF_INDEX.md`
- `docs/plans/setup-install-provisioning-plan/workpacks/01-family-web-info-site.md`
- `docs/plans/setup-install-provisioning-plan/workpacks/02-registration-login-entry.md`
- `docs/plans/setup-install-provisioning-plan/workpacks/03-parent-install-journey.md`
- `docs/plans/setup-install-provisioning-plan/workpacks/04-child-install-permission-journey.md`
- `docs/plans/setup-install-provisioning-plan/workpacks/05-pairing-readiness-recovery.md`
- `docs/plans/setup-install-provisioning-plan/workpacks/06-rollout-proof-and-route-gate.md`
- `docs/plans/setup-install-provisioning-plan/workpacks/07-first-run-setup-ui-and-state-machine.md`
- `docs/features/family-setup-device-roles.md`
- `docs/features/production-distribution-support.md`
- `docs/expectations/family-setup.md`
- `docs/expectations/release-installer.md`

Source/tests read:
- `packages/setup-domain/src/registration-entry.ts`
- `packages/setup-domain/tests/unit/registration-entry.test.ts`
- `packages/setup-domain/src/setup-state-machine.ts`
- `packages/setup-domain/tests/unit/setup-state-machine.test.ts`
- `packages/production-domain/src/family-web-route-map.ts`
- `packages/production-domain/src/family-web-route-map-read-model.ts`
- `packages/production-domain/tests/unit/family-web-route-map.test.ts`
- `packages/production-domain/src/production-release-public-runtime-handoff-values.ts`
- `packages/production-domain/src/production-release-public-runtime-handoff-read-model.ts`
- `packages/production-domain/tests/unit/production-release-public-runtime-handoff.test.ts`
- `packages/parent-domain/src/parent-desktop-release-support.ts`
- `packages/parent-domain/tests/unit/parent-desktop-release-support.test.ts`
- `packages/parent-domain/tests/unit/parent-desktop-release-support-fixtures.ts`
- `packages/parent-domain/src/*` importers matching `billing-account-runtime-boundary*`, `billing-entitlement*`, `network-control-catalog*`, `production-release-public-*`, `production-support-*`, `public-support-contact-status*`, `stateless-report-compiler-status*`
- `packages/billing-domain/package.json`
- `packages/production-domain/package.json`
- `packages/network-domain/package.json`
- `packages/portal-domain/src/setup-first-run-panel.ts`
- `packages/portal-domain/tests/unit/setup-first-run-panel.test.ts`
- `apps/portal/src/ParentPortalRoute.tsx`
- `apps/portal/src/SetupFirstRunRoutePanel.tsx`
- `apps/portal/src/dev-logger.ts`
- `apps/portal/tests/setup-first-run-route-panel.test.ts`
- `apps/portal/tests/live-activity-network-flow.test.ts`
- `apps/portal/tests/logging/portal-dev-log-route.test.ts`
- `apps/portal/e2e/setup-first-run-ui-proof.spec.ts`
- `crates/provisioning-core/src/lib.rs`
- `crates/provisioning-core/src/provisioning_install.rs`
- `crates/provisioning-core/tests/unit/readiness.rs`
- `crates/provisioning-core/tests/unit/readiness_flow.rs`
- `crates/child-runtime/src/runtime_gate.rs`
- `crates/child-runtime/tests/unit/runtime_gate.rs`
- `crates/child-runtime/tests/integration/tracking_runtime_flow_intent.rs`

Proof read:
- `output/setup-install-provisioning-plan-proof/01-family-web-info-site/16-validation-commands.log`
- `output/setup-install-provisioning-plan-proof/02-registration-login-entry/16-validation-commands.log`
- `output/setup-install-provisioning-plan-proof/03-parent-install-journey/16-validation-commands.log`
- `output/setup-install-provisioning-plan-proof/04-child-install-permission-journey/16-validation-commands.log`
- `output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/05-redacted-pairing-log-proof.md`
- `output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/redacted-bootstrap-logs-proof.md`
- `output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/00-rollout-proof-pack.md`
- `output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/01-route-sync-proof.md`
- `output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/02-platform-readiness-matrix.md`
- `output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/04-manual-required-gap-register.md`
- `output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/05-product-status-safe-wording-proof.md`
- `output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/16-validation-commands.log`
- `output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/16-validation-commands.log`

**Current Truth**

| Slice | Current truth | Evidence | Status |
| --- | --- | --- | --- |
| WP01 | Owned public-route/data-boundary slice is real; deploy/custom-domain/public-runtime is still blocker-only | `workpacks/01-family-web-info-site.md`, `01-family-web-info-site/16-validation-commands.log` | `done` at owned slice, not full closure |
| WP02 | Typed account-entry/handoff slice is real; live provider/session/household is not | `workpacks/02-registration-login-entry.md`, `02-registration-login-entry/16-validation-commands.log` | `done` at owned slice, blocked by sibling proof |
| WP03 | Setup/production-domain proof is real; `@ocentra-parent/parent-domain` still does not build cleanly; local export mismatch is now identified | `workpacks/03-parent-install-journey.md`, `03-parent-install-journey/16-validation-commands.log`, package.json diffs | `partial` |
| WP04 | Child install/permission state-model slice is real; real runtime/package/platform proof remains external | `workpacks/04-child-install-permission-journey.md`, `04-child-install-permission-journey/16-validation-commands.log` | `done` at owned slice, blocked by sibling proof |
| WP05 | Pairing/readiness model and scoped TS/Rust tests are real; redaction proof is still blocker-only | `workpacks/05-pairing-readiness-recovery.md`, `05-redacted-pairing-log-proof.md`, `redacted-bootstrap-logs-proof.md` | `partial` |
| WP07 | First-run route projection, portal tests, and Playwright proof are genuinely green | `workpacks/07-first-run-setup-ui-and-state-machine.md`, `07-first-run-setup-ui-and-state-machine/16-validation-commands.log` | `done` at owned slice |
| WP06 | Aggregate route/proof gate is stale; indexes mark done while WP06 proof pack still says WP07 is missing and whole-plan rollout is blocked | `WORKPACK_INDEX.md`, `CHECKLIST_INDEX.md`, `workpacks/06-rollout-proof-and-route-gate.md`, `06-rollout-proof-and-route-gate/00-rollout-proof-pack.md`, `16-validation-commands.log` | `false-green` |
| Plan-wide | No honest `DONE` state exists yet | all above | `not done` |

**Code Surface And Ownership**
- `packages/setup-domain`
  - Owns typed journey/state contracts used by WP02, WP03, WP04, WP05, WP07.
  - Exact files in scope: `src/registration-entry.ts`, `src/setup-state-machine.ts`, corresponding unit tests.
- `packages/production-domain`
  - Owns public route/public-runtime-handoff/read-model contracts used by WP01, WP02, WP03.
  - Exact files in scope: `src/family-web-route-map.ts`, `src/family-web-route-map-read-model.ts`, `src/production-release-public-runtime-handoff-values.ts`, `src/production-release-public-runtime-handoff-read-model.ts`, corresponding unit tests.
- `packages/parent-domain`
  - Consumer/adjacent source surface for WP03 release-support read models.
  - Exact files in scope: `src/parent-desktop-release-support.ts`, `tests/unit/parent-desktop-release-support.test.ts`, `tests/unit/parent-desktop-release-support-fixtures.ts`.
  - Currently blocked by upstream package export resolution.
- `packages/billing-domain`, `packages/production-domain`, `packages/network-domain`
  - Local dependency-export surfaces now blocking WP03 validation.
  - Exact files in scope: their `package.json` export maps.
- `packages/portal-domain` and `apps/portal`
  - Own WP07 route projection and UI proof surface.
  - Exact files in scope: `src/setup-first-run-panel.ts`, `src/SetupFirstRunRoutePanel.tsx`, `src/ParentPortalRoute.tsx`, `src/dev-logger.ts`, related tests and Playwright spec.
- `crates/provisioning-core`, `crates/child-runtime`, `packages/child-runtime-domain`
  - Adjacent proof surfaces consumed by WP04/WP05 for readiness/runtime state.
  - Exact files in scope listed above.
- Out of scope by design
  - `account-identity-family-plan`
  - `parent-desktop-runtime-package-plan`
  - `child-agent-runtime-distribution-plan`
  - `app-plan`
  - `lan-plan`
  - `device-trust-bootstrap-plan`
  - `data-custody-storage-plan`
  - `policy-control-plane-plan`
  - `payment-subscription-plan`

**Test Surface Inventory**
- `packages/setup-domain/tests/unit`
  - Present and real.
  - No move issue observed.
  - Missing categories: none locally urgent beyond existing unit coverage.
- `packages/production-domain/tests/unit`
  - Present and real.
  - Missing category that actually matters: an explicit export-resolution/contract test for package subpath imports used by `@ocentra-parent/parent-domain`.
- `packages/portal-domain/tests/unit`
  - Present and real.
  - No move issue observed.
- `apps/portal/tests` and `apps/portal/e2e`
  - Real tests exist, but organization is below the requested bar.
  - `apps/portal/tests/setup-first-run-route-panel.test.ts`, `apps/portal/tests/live-activity-network-flow.test.ts`, and `apps/portal/tests/logging/portal-dev-log-route.test.ts` should move under major categories such as `apps/portal/tests/integration/` and `apps/portal/tests/security/` if kept separate.
  - `apps/portal/e2e/setup-first-run-ui-proof.spec.ts` should move under `apps/portal/tests/e2e/` or `apps/portal/tests/playwright/` to satisfy the “tests under major categories” bar.
- `crates/provisioning-core/tests/unit`
  - Present and real.
  - Missing category that actually matters: dedicated redaction/security proof for setup/bootstrap logging if this plan owns that producer.
- `crates/child-runtime/tests/unit` and `tests/integration`
  - Present and real.
  - No move issue observed.
- `packages/parent-domain/tests/unit`
  - Real tests exist but are not currently runnable due import/export failure before the suite executes.
- Empty-folder optics
  - None observed in the inspected plan-owned test surfaces.
- Inline or `src` tests that must move
  - None observed in the inspected plan-owned package/crate surfaces.

**Proof Inventory**
Canonical root:
- `output/setup-install-provisioning-plan-proof/`

By workpack:
- `01-family-web-info-site/`
  - Real for owned route/data-boundary proof.
  - Still blocker-only for deploy/custom-domain/runtime proof.
- `02-registration-login-entry/`
  - Real for owned handoff/state proof.
  - Live provider/session/household execution still external.
- `03-parent-install-journey/`
  - Real but incomplete.
  - Validation log is now partially stale because newer inspection found a concrete export-map root cause.
- `04-child-install-permission-journey/`
  - Real for owned state-model proof.
- `05-pairing-readiness-recovery/`
  - Partial.
  - `05-redacted-pairing-log-proof.md` and `redacted-bootstrap-logs-proof.md` are blocker artifacts.
- `07-first-run-setup-ui-and-state-machine/`
  - Real and green.
- `06-rollout-proof-and-route-gate/`
  - Stale and false-green at plan-reporting level.
  - Its proof pack/log still contradict current disk state and current WP07 truth.

**Scoped Validation Inventory**
Already passing:
- `node -e "console.log('family-web-info-site-docs-only')"`
- `npm run lint:architecture -- --files docs/plans/setup-install-provisioning-plan`
- `npm run build --workspace @ocentra-parent/production-domain`
- `npm run test --workspace @ocentra-parent/production-domain`
- `node -e "console.log('registration-login-entry-handoff')"`
- `npm run build --workspace @ocentra-parent/setup-domain`
- `npm run test --workspace @ocentra-parent/setup-domain -- registration-entry`
- `npm run test --workspace @ocentra-parent/setup-domain`
- `npm run test --workspace @ocentra-parent/child-runtime-domain`
- `cargo test -p ocentra-provisioning-core`
- `cargo test -p ocentra-child-runtime runtime_gate`
- `npm run build --workspace @ocentra-parent/portal-domain`
- `npm run test --workspace @ocentra-parent/portal-domain -- setup-first-run-panel.test.ts`
- `Push-Location apps/portal; npx vitest run tests/logging/portal-dev-log-route.test.ts tests/setup-first-run-route-panel.test.ts tests/live-activity-network-flow.test.ts; Pop-Location`
- `Push-Location apps/portal; npx tsc -p tsconfig.json --noEmit; Pop-Location`
- `Push-Location apps/portal; npx vite build; Pop-Location`
- `$env:OCENTRA_PARENT_PORTAL_PLAYWRIGHT_SPEC='setup-first-run-ui-proof.spec.ts'; node scripts/test/portal-playwright-runner.mjs`

Currently failing:
- `npm run build --workspace @ocentra-parent/parent-domain`
- `npm run test --workspace @ocentra-parent/parent-domain -- parent-desktop-release-support`

Known unrun but required next:
- `npm run lint:architecture -- --files packages/billing-domain/package.json packages/production-domain/package.json packages/network-domain/package.json`
- `npm run build --workspace @ocentra-parent/billing-domain`
- `npm run build --workspace @ocentra-parent/production-domain` after export-map change
- `npm run build --workspace @ocentra-parent/network-domain`
- rerun `npm run build --workspace @ocentra-parent/parent-domain`
- rerun `npm run test --workspace @ocentra-parent/parent-domain -- parent-desktop-release-support`
- rerun `npm run lint:architecture -- --files docs/plans/setup-install-provisioning-plan` after WP06 truth-sync

Avoidable local execution gap:
- `npm run build --workspace @ocentra-parent/portal` still resolves a missing local Vite bin on this host, but direct `npx tsc` and `npx vite build` already pass and are the real scoped proof.

**Dependency Graph**

| Dependency | Blocks final closure? | Exact reason | First effect |
| --- | --- | --- | --- |
| `account-identity-family-plan` | Yes | WP02 cannot become live-complete without provider/session/household/invite/recovery proof | WP02, WP05, WP07 |
| `parent-desktop-runtime-package-plan` | Yes | WP03 cannot claim signed installer/update/rollback/runtime distribution readiness without owner proof | WP03 |
| `child-agent-runtime-distribution-plan` | Yes | WP04 cannot claim real child package/platform execution | WP04 |
| `app-plan` | Yes | WP04/WP05 need real child runtime/platform permission behavior | WP04, WP05 |
| `lan-plan` | Yes | WP05 cannot claim real pairing/discovery/LAN proof without physical or real transport evidence | WP05 |
| `device-trust-bootstrap-plan` | Yes | WP05/WP07 cannot claim trusted-device/bootstrap approval readiness | WP05, WP07 |
| `data-custody-storage-plan` | Yes | WP05/WP07 readiness matrix explicitly includes custody state | WP05, WP07 |
| `policy-control-plane-plan` | Yes | WP05/WP07 readiness matrix explicitly includes policy-baseline state | WP05, WP07 |
| `payment-subscription-plan` | No for core closure, yes for later polish | affects public/download/account/subscription status surfaces, not the minimum setup journey bar | later public-surface polish |
| `portal-ux-household-surfaces-plan` | No for core closure if WP07 stays narrow | broader household shell polish beyond owned Start route | later polish |
| deployment/public-runtime owner, likely `cloudflare-control-plane-plan` or equivalent | Conditional | only a final blocker if this plan’s done bar requires real preview/custom-domain/public-runtime proof rather than manual-required public-entry truth | WP01 |

**Platform Feasibility**
- Windows host now
  - Can prove all docs/proof sync, TS package builds, Rust tests, Playwright portal proof, and parent-domain import/export repair.
- Android Studio/device
  - Can prove real child bootstrap/install/permission/pairing flows once sibling child-runtime/LAN/device-trust surfaces are ready.
  - Not a blocker for current local-now slices.
- WSL/Docker
  - Can prove Linux-oriented parent/runtime surfaces when runtime-owner proof is ready.
  - Not a blocker for current local-now slices.
- Apple-host-only
  - Real macOS notarization/signing/store proof and real iOS distribution/store proof remain Apple-host-limited and belong to runtime-owner plans, not this current local slice.

**No-Hand-Wave Execution Plan**

| Order | Slice | Files / domains to touch | Validation | Proof / exit criteria |
| --- | --- | --- | --- | --- |
| 1 | WP06 truth-sync | `docs/plans/setup-install-provisioning-plan/PLAN_STATE.md`, `NEXT_ACTIONS.md`, `workpacks/06-rollout-proof-and-route-gate.md`, `output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/{00,01,02,04,05,16}*` | `npm run lint:architecture -- --files docs/plans/setup-install-provisioning-plan` | WP06 proof pack matches actual proof roots and blocker state; no more WP07-missing contradiction |
| 2 | WP03 export-surface repair | `packages/billing-domain/package.json`, `packages/production-domain/package.json`, `packages/network-domain/package.json` | package.json architecture lint; builds for billing/production/network; rerun `@ocentra-parent/parent-domain` build/test | parent-domain either goes green or the remaining error surface becomes smaller and precise |
| 3 | WP03 proof refresh | `output/setup-install-provisioning-plan-proof/03-parent-install-journey/16-validation-commands.log`, `docs/plans/setup-install-provisioning-plan/workpacks/03-parent-install-journey.md`, `PLAN_STATE.md`, WP06 aggregate files | consume slice-2 results only; no broad validation | WP03 no longer says vague “broader unrelated failures” if export fix changed the truth |
| 4 | WP05 redaction ownership resolution | likely `crates/provisioning-core/*`, `crates/child-runtime/*`, or update only `output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/*` plus WP05 doc if ownership is external | scoped search plus only the producer’s test path if local | either real redaction proof exists or the blocker is reassigned to an exact sibling owner with named proof expected |
| 5 | Sibling-proof consumption wave | setup-plan docs/proofs only: WP02/WP03/WP04/WP05/WP07 proofs and WP06 aggregate | smallest consuming checks only | account/runtime/LAN/device-trust/custody/policy proofs are reflected in setup readiness and blocker surfaces |
| 6 | Public entry/runtime decision | WP01 `05-deploy-preview-proof-or-blocker.md`, WP06 matrix/gap register | preview/runtime proof only if owner surface exists | either real preview/custom-domain/public-runtime proof exists or coordinator explicitly excludes it from the done signal |
| 7 | Final close audit | `PLAN_STATE.md`, `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, `CHECKLIST_INDEX.md`, WP06 proof pack | rerun only the minimal workpack commands still relevant | no false-green, no blocker-only owned behavior, all done-bar dependencies consumed |

**Blocker Taxonomy**
`local-now`
- WP06 aggregate proof/docs are stale and contradictory.
- WP03 local dependency export surfaces in `packages/billing-domain/package.json`, `packages/production-domain/package.json`, and `packages/network-domain/package.json` still need focused validation rerun.
- Portal test organization is below the requested “major categories under tests/” bar.
- WP05 redaction proof is still unresolved at producer level.

`needs-coordinator-sequencing`
- Decide whether live preview/custom-domain/public-runtime proof is part of this plan’s strict done bar or should remain an external deployment-owner signoff.
- Sequence sibling-proof consumption after local-now slices so this plan does not absorb ownership from account/runtime/LAN/device-trust/custody/policy.
- If test-organization cleanup is required before this plan can be called done, sequence that explicitly as a local follow-up slice.

`needs-sibling-plan-contract`
- `account-identity-family-plan`
- `parent-desktop-runtime-package-plan`
- `child-agent-runtime-distribution-plan`
- `app-plan`
- `lan-plan`
- `device-trust-bootstrap-plan`
- `data-custody-storage-plan`
- `policy-control-plane-plan`

`host-platform-limited`
- Real macOS notarization/signing/store proof
- Real iOS distribution/store/device proof
- Any Apple-platform runtime proof owned by sibling runtime plans

**First Coordinator Ask**
No sibling plan needs to move before the next local slice. After local-now slices finish, the first sibling plan that should move is `account-identity-family-plan`, because live provider/session/household proof is the earliest cross-plan prerequisite that affects WP02 directly and cascades into WP05 and WP07 readiness.

**Strict Done Bar**
Before this plan can honestly be marked done, all of the following must be true:
- WP06 is refreshed and internally consistent with the filesystem.
- WP03 local export/import blocker is resolved or reduced to only true sibling-plan proof gaps.
- WP05 no longer relies on unnamed blocker-only redaction artifacts for behavior this plan claims.
- WP01, WP02, WP04, and WP07 remain green after any dependent state-shape changes.
- Live proof from sibling owners is consumed for account, parent runtime distribution, child runtime/package, LAN/device trust, data custody, and policy baseline.
- Any required public entry/runtime/deploy proof is either real or explicitly removed from this plan’s done signal by coordinator decision.
- Test layout for the plan-owned portal surfaces meets the requested major-category organization bar if that bar is enforced for closure.
- `PLAN_STATE.md`, `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, `CHECKLIST_INDEX.md`, all workpacks, and the canonical proof root tell the same story.

**COORDINATOR_DECISION_REQUEST**
- Recommended next slice: `WP06 truth-sync`
- Recommended predecessor plans: none before `WP06 truth-sync`; after local-now slices, `account-identity-family-plan` should be first among siblings
- Estimated risk: medium for WP06 truth-sync, medium-high for WP03 export repair, high for final cross-plan closure
- Estimated proof difficulty: low for WP06 truth-sync, medium for WP03 rerun, high for sibling-proof consumption
- Whether I should continue immediately or pause for sequencing: continue immediately through `WP06 truth-sync`; pause for coordinator sequencing only after the local-now WP06 and WP03 slices are complete

## Optional Addendum

- Earlier audit passes established an additional false-green detail that the latest report did not foreground strongly enough: `docs/plans/setup-install-provisioning-plan/WORKPACK_INDEX.md` currently marks `WP03`, `WP05`, and `WP06` as `done`, but current thread evidence says `WP03` is only partial, `WP05` is only partial because redaction proof is still blocker-only, and `WP06` is stale/false-green.
