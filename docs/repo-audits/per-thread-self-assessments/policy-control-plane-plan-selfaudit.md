# policy-control-plane-plan

## Normalized Header

- plan/thread name: `policy-control-plane-plan`
- source thread label: `policy-control-plane-plan`
- source thread id: `019ed32a-fdd2-74b0-bb81-6e152680ac97`
- worktree: `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent`
- branch: `codex/tracking-plan-full-continuation-a`
- claimed status: `audit-open`; WP01/WP07/WP08 locally closeable with real owner proof; WP03 partial but locally closable; WP04 partial; WP02/WP05 dependency-open; WP06 still partially false-green because route/proof sync remains inconsistent
- claimed source files/crates/packages: `packages/policy-domain`, `crates/policy-control-core`, dependent seam surfaces in `packages/agent-protocol-domain`, `crates/agent-protocol`, and limited portal proof consumers in `apps/portal/tests`
- claimed tests: `packages/policy-domain/tests/unit/*`, `crates/policy-control-core/tests/unit/*`, `crates/policy-control-core/tests/version-skew/*`, focused policy seam tests in `packages/agent-protocol-domain/tests/unit/*`, `cargo test -p ocentra-parent-agent-protocol policy`, and limited portal preview tests
- claimed proof commands/artifacts: canonical root `docs/proof/policy-control-plane-plan/`; real `01-*.md`, `07-*.md`, `08-*.md`, `06-rollout-proof-pack.md`, `06-manual-required-gap-register.md`, `06-no-overclaim-proof.md`, `00-scope-summary.md`, `16-validation-commands.log`; scoped commands including `npm run test --workspace @ocentra-parent/policy-domain`, `cargo test -p ocentra-policy-control-core`, `cargo test -p ocentra-parent-agent-protocol policy`, narrowed `npm run lint:architecture -- --files ...`, and direct portal `npx vitest run ...`
- claimed blockers: local route/proof drift (`PROOF_INDEX.md`, `PROOF_AND_TEST_INVENTORY.md`, stale `06-route-sync-proof.md`, missing `PLAN_PROOF_MANIFEST.md`), missing WP03/WP04 proof bundles, dependency-owned portal/assistant/device-trust/custody/enforcement/identity surfaces for WP02/WP05
- claimed next actions: repair WP06 route/proof truth; rebuild WP03 `03-*.md` proof bundle; create WP04 delivery/ack/audit proof bundle; coordinate portal and assistant dependency work for WP02/WP05; move counted seam tests out of inline `src` before final closure
- obvious missing evidence fields: no refreshed `03-*.md` bundle, no `04-*.md` bundle, no `PLAN_PROOF_MANIFEST.md`, no full WP02/WP05 rendered proof, no final proof that policy-counted `crates/agent-protocol` tests have moved out of `src`
- archival note: self-report only; unverified input for structural review

## Raw Self-Report

**COMPLETION_ARCHITECTURE_REPORT**

Core contract truth is stronger than the plan docs imply, but the plan is not closeable yet. WP01, WP07, and WP08 now have real owner-surface proof and green scoped validation; WP03 and WP04 are mostly local proof/route work; WP02 and WP05 are still gated by portal/assistant/product surfaces outside this lane. The biggest hidden issue is that WP06 is still partially false-green: the canonical proof root is real, but [PROOF_INDEX.md](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/policy-control-plane-plan/PROOF_INDEX.md), [PROOF_AND_TEST_INVENTORY.md](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/policy-control-plane-plan/PROOF_AND_TEST_INVENTORY.md), [06-route-sync-proof.md](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/proof/policy-control-plane-plan/06-route-sync-proof.md), and missing [PLAN_PROOF_MANIFEST.md](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/proof/policy-control-plane-plan/PLAN_PROOF_MANIFEST.md) still leave route truth inconsistent.

| Area | Exact files read | Notes |
| --- | --- | --- |
| Plan docs | Earlier thread audit covered every `.md` under `docs/plans/policy-control-plane-plan/`; this checkpoint re-read `AGENTS.md`, `README.md`, `PLAN_STATE.md`, `PLAN_HEALTH.md`, `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, `PROOF_INDEX.md`, `PROOF_AND_TEST_INVENTORY.md`, `TEST_PROOF_EXPECTATIONS.md`, `PLAN_EXECUTION_BLUEPRINT.md`, workpacks `01`-`08` | Current truth is based on the live files, not old checkmarks |
| Feature docs | `docs/features/policy-schedules-approvals.md`, `docs/features/parent-assistant-actions.md` | These are the real external dependency docs currently referenced by the gap register |
| Owner source | `packages/policy-domain/src/{policy.ts,authority.ts,policy-compiler.ts,policy-event.ts}`, `crates/policy-control-core/src/{policy_source.rs,policy_authority.rs,policy_compiler.rs,policy_conflict.rs,policy_delivery.rs,policy_event.rs,policy_preview.rs,policy_request.rs}` | Core contract and lifecycle surface |
| Owner tests | `packages/policy-domain/tests/unit/{policy.test.ts,policy-schedule-boundaries.test.ts,policy-decision.test.ts,policy-compiler.test.ts,policy-approval-override.test.ts,authority.test.ts,policy-event.test.ts}`, `crates/policy-control-core/tests/unit/*`, `crates/policy-control-core/tests/version-skew/*` | Real owner coverage exists |
| Dependent seams | `packages/agent-protocol-domain/src/{policy-preview-read-model.ts,policy-control-delivery-read-model.ts,policy-control-audit-redaction.ts,parent-assistant-adapter.ts,browser-policy-adapter.ts,enforcement-policy-dispatch-adapter.ts,app-game-policy-readiness.ts}`, related tests under `packages/agent-protocol-domain/tests/unit/`, `crates/agent-protocol/src/lib.rs`, `crates/agent-protocol/src/activity.rs`, `crates/agent-protocol/tests/contract.rs`, `apps/portal/tests/{policy-preview-route-panel.test.ts,policy-preview-live-activity-state.test.ts}` | Needed for WP03/WP04/WP05/WP02 proof closure |
| Proof | `docs/proof/policy-control-plane-plan/00-scope-summary.md`, all `01-*.md`, `06-rollout-proof-pack.md`, `06-manual-required-gap-register.md`, `06-no-overclaim-proof.md`, `06-route-sync-proof.md`, all `07-*.md`, all `08-*.md`, `16-validation-commands.log` | `03-*.md` are deleted/missing; `PLAN_PROOF_MANIFEST.md` is missing |

| Current truth | Status | Exact evidence | Why it matters |
| --- | --- | --- | --- |
| WP01 source of truth | Done locally | `01-source-of-truth-matrix-proof.md`, `01-schema-negative-proof.md`, `01-version-skew-proof.md`, `01-duplicate-truth-negative-proof.md`, `01-ai-preview-not-write-proof.md`, `01-authz-negative-proof.md` plus owner tests/logs | Contract/proof is real without overclaiming export/delete runtime |
| WP02 parent authoring/preview | Missing dependency closure | `workpacks/02-parent-authoring-preview.md`, `docs/features/policy-schedules-approvals.md`, portal tests only cover preview read states | Rendered authoring, accessibility, mobile, conflict UX are not done here |
| WP03 domain compilers | Partial | [03-domain-policy-compilers.md](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/policy-control-plane-plan/workpacks/03-domain-policy-compilers.md), deleted `docs/proof/policy-control-plane-plan/03-*.md`, real compiler code/tests exist | Locally closable, but proof bundle is currently absent/stale |
| WP04 delivery/ack/audit | Partial | `policy_delivery.rs`, `policy_event.rs`, delivery/audit seam tests exist; no `04-*.md` bundle exists | Real code exists, but closeout proof is missing |
| WP05 ask-parent/overrides | Partial with dependency gap | `policy_request.rs`, `policy_preview.rs`, `parent-assistant-adapter.test.ts`, `docs/features/parent-assistant-actions.md` | Backend contract exists; portal confirmation/chat and child-agent validation are not complete |
| WP06 rollout proof gate | False-green / partial | `06-rollout-proof-pack.md`, `06-manual-required-gap-register.md`, `06-no-overclaim-proof.md` exist, but `06-route-sync-proof.md` is stale and `PLAN_PROOF_MANIFEST.md` is missing | Current `Checked` status is too optimistic |
| WP07 schedule/conflict | Done locally | `07-schedule-timezone-proof.md`, `07-dst-boundary-proof.md`, `07-time-budget-reset-proof.md`, `07-conflict-precedence-proof.md`, `07-offline-timer-recovery-proof.md` plus owner tests/logs | Real DST/time-budget/conflict proof exists |
| WP08 event model | Done locally | `08-event-family-registry-proof.md`, `08-event-idempotency-proof.md`, `08-event-replay-ordering-proof.md`, `08-rollback-event-linkage-proof.md`, `08-event-redaction-proof.md` plus owner tests/logs | Real event/replay/redaction proof exists |

| Code surface | Exact files | Ownership | Current truth |
| --- | --- | --- | --- |
| TS policy contract | `packages/policy-domain/src/policy.ts`, `authority.ts`, `policy-compiler.ts`, `policy-event.ts` | This plan | Real source-of-truth, authority, schedule/compiler, and event schemas |
| Rust policy core | `crates/policy-control-core/src/policy_source.rs`, `policy_authority.rs`, `policy_compiler.rs`, `policy_conflict.rs`, `policy_delivery.rs`, `policy_event.rs`, `policy_preview.rs`, `policy_request.rs` | This plan | Real lifecycle, conflict, delivery, preview, request, and event logic |
| TS seam contracts | `packages/agent-protocol-domain/src/policy-preview-read-model.ts`, `policy-control-delivery-read-model.ts`, `policy-control-audit-redaction.ts`, `parent-assistant-adapter.ts`, `browser-policy-adapter.ts`, `enforcement-policy-dispatch-adapter.ts`, `app-game-policy-readiness.ts` | Shared seam; needed by this plan | Real, but broader package has unrelated barrel debt outside these files |
| Rust seam contracts | `crates/agent-protocol/src/lib.rs`, `src/activity.rs`, `tests/contract.rs` | Shared seam; needed by this plan | Policy contract tests exist, but too many are still inline in `src` |
| Portal proof consumers | `apps/portal/tests/policy-preview-route-panel.test.ts`, `apps/portal/tests/policy-preview-live-activity-state.test.ts` | Portal plan dependency | Real preview-read proof, not full authoring/approval closure |

| Test surface | Status | Organization quality | Missing or must move |
| --- | --- | --- | --- |
| `packages/policy-domain/tests/unit/*` | Real | Good: no inline `src` tests | No dedicated `contract` or `property` bucket; not a blocker for core closure |
| `crates/policy-control-core/tests/unit/*` | Real | Good | No inline `src` tests in this crate |
| `crates/policy-control-core/tests/version-skew/*` | Real | Good | This is the real compatibility category for WP01/WP03/WP08 |
| `packages/agent-protocol-domain/tests/unit/*` | Real | Acceptable but seam-contract heavy | If counted for final WP03/WP04/WP05 proof, these would be clearer under `tests/contract/` |
| `crates/agent-protocol/src/lib.rs`, `crates/agent-protocol/src/activity.rs` policy tests | Real but mislocated | Bad for final done bar | Policy contract tests still live inline in `src`; move them into `crates/agent-protocol/tests/contract/` or `tests/integration/` before counting them as final proof |
| `apps/portal/tests/*.test.ts` | Real | Flat, not major-category partitioned | WP02/WP05 should move policy UI tests into `apps/portal/tests/unit/` or `apps/portal/tests/integration/`; no real `e2e/playwright` coverage exists yet |
| Empty scaffolds | None found in owner surfaces | Good | The problem is stale proof and mislocated seam tests, not empty folders |
| Applicability gaps | `integration`, `contract`, `e2e` are applicable; `load` is not | Mixed | WP03/WP04 need stronger contract/integration grouping; WP02/WP05 need rendered `integration` and likely `playwright`/accessibility proof |

| Proof inventory | Status | Exact files |
| --- | --- | --- |
| Canonical root | Real | `docs/proof/policy-control-plane-plan/` |
| Real closeout bundles | Present | all `01-*.md`, all `07-*.md`, all `08-*.md`, `06-rollout-proof-pack.md`, `06-manual-required-gap-register.md`, `06-no-overclaim-proof.md`, `00-scope-summary.md`, `16-validation-commands.log` |
| Stale route proof | Stale | `docs/proof/policy-control-plane-plan/06-route-sync-proof.md` still says workpack-specific closeout artifacts are absent |
| Stale route docs | Stale | `docs/plans/policy-control-plane-plan/PROOF_INDEX.md`, `PROOF_AND_TEST_INVENTORY.md` |
| Missing proof manifest | Missing | `docs/proof/policy-control-plane-plan/PLAN_PROOF_MANIFEST.md` |
| Missing compiler bundle | Missing | deleted `03-deterministic-output-proof.md`, `03-domain-compiler-matrix-proof.md`, `03-domain-fixture-proof.md`, `03-unsupported-manual-required-proof.md`, `03-version-compat-proof.md` |
| Missing delivery bundle | Missing | no `04-*.md` files exist yet |
| Missing authoring/override bundles | Missing | no `02-*.md` or `05-*.md` closeout bundle in this root |

| Scoped validation inventory | State | Notes |
| --- | --- | --- |
| `npm run test --workspace @ocentra-parent/policy-domain` | Pass | Current proof log and this slice both show green |
| `cargo test -p ocentra-policy-control-core` | Pass | Current proof log shows `97` unit + `28` version-skew |
| `cargo test -p ocentra-parent-agent-protocol policy` | Pass | Re-ran successfully in this checkpoint |
| `npm run test --workspace @ocentra-parent/agent-protocol-domain -- tests/unit/policy-preview-contracts.test.ts tests/unit/policy-control-delivery-read-model.test.ts tests/unit/policy-control-audit-redaction.test.ts tests/unit/parent-assistant-adapter.test.ts` | Logged pass | Present in `16-validation-commands.log`; not rerun in this checkpoint |
| `cd apps/portal && npx vitest run tests/policy-preview-route-panel.test.ts tests/policy-preview-live-activity-state.test.ts` | Logged pass | This is the acceptable scoped portal command |
| `npm run test --workspace @ocentra-parent/portal -- tests/policy-preview-route-panel.test.ts tests/policy-preview-live-activity-state.test.ts` | Fail | Overbroad workspace script pulls unrelated LAN failure; do not use as policy proof |
| `npm run lint:architecture -- --files packages/policy-domain` | Pass | Owner TS slice green |
| `cargo lint-architecture crates/policy-control-core` | Pass | Owner Rust slice green |
| `npm run lint:architecture -- --files packages/agent-protocol-domain` | Fail | Unrelated barrel debt in `src/activity-surface-adapter.ts` and `src/primitives.ts` |
| `npm run lint:architecture -- --files <policy-owned agent-protocol-domain files>` | Pass | Narrowed policy seam file set is architecture-green |
| Unrun but still needed for final closure | Open | WP02/WP05 rendered integration/accessibility/mobile proof; WP03/WP04 replacement bundles; final direct portal proof after dependency plan work lands |

| Dependency | Bucket | Exact files / plans | Why it blocks final closure |
| --- | --- | --- | --- |
| WP06 route truth repair | `local-now` | `docs/plans/policy-control-plane-plan/{PROOF_INDEX.md,PROOF_AND_TEST_INVENTORY.md,PLAN_EXECUTION_BLUEPRINT.md,TEST_PROOF_EXPECTATIONS.md,WORKPACK_INDEX.md}`, `docs/proof/policy-control-plane-plan/{06-route-sync-proof.md,PLAN_PROOF_MANIFEST.md}` | Current route/proof state is still inconsistent |
| WP03 proof rebuild | `local-now` | `docs/proof/policy-control-plane-plan/03-*.md`, compiler surfaces in `packages/policy-domain`, `crates/policy-control-core`, seam policy files in `packages/agent-protocol-domain`, `crates/agent-protocol` | Real code exists; proof bundle is the missing piece |
| WP04 proof bundle | `local-now` | `crates/policy-control-core/src/{policy_delivery.rs,policy_event.rs,policy_source.rs}`, `packages/agent-protocol-domain/src/{policy-control-delivery-read-model.ts,policy-control-audit-redaction.ts}`, new `docs/proof/policy-control-plane-plan/04-*.md` | Code/tests exist; closeout proof does not |
| Portal authoring and approvals | `needs-coordinator-sequencing` | `docs/plans/portal-ux-household-surfaces-plan/workpacks/{05-policy-authoring-control-center.md,06-schedules-time-budgets-and-templates.md,07-parent-requests-and-approvals.md,11-assistant-action-preview-flow.md,14-audit-history-and-copy-debug.md,15-accessibility-responsive-keyboard-ux.md,18-parent-mobile-shell-readiness.md}` | This is the critical path for WP02 and the portal half of WP05 |
| Custody runtime proof | `needs-sibling-plan-contract` | `data-custody-storage-plan` | WP01 contract covers custody fields, not export/delete/sync runtime truth |
| Trusted-parent step-up | `needs-sibling-plan-contract` | `device-trust-bootstrap-plan` | Needed for high-risk policy change proof |
| Enforcement authority / rollback runtime | `needs-sibling-plan-contract` | `v0-8-enforcement-control-plan` | This plan may define rollback linkage, but not enforcement runtime authority |
| Integrated session/role confirmation flow | `needs-sibling-plan-contract` | `account-identity-family-plan` | Core authz matrix is local; end-to-end parent confirmation flow depends on authoritative identity/session proof |
| Apple-specific rendered proof | `host-platform-limited` | future iOS/macOS/Safari-specific authoring proof only if a sibling plan requires it | Not on the current critical path from this Windows host |

| Ordered slice | Exact files / domains | Validation | Proof / exit criteria |
| --- | --- | --- | --- |
| 1. WP06 proof-route truth repair | `PROOF_INDEX.md`, `PROOF_AND_TEST_INVENTORY.md`, `PLAN_EXECUTION_BLUEPRINT.md`, `TEST_PROOF_EXPECTATIONS.md`, `WORKPACK_INDEX.md`, `06-route-sync-proof.md`, create/restore `PLAN_PROOF_MANIFEST.md` | `npm run lint:architecture -- --files docs/plans/policy-control-plane-plan` if needed; reuse existing scoped green code commands | All route docs agree on `docs/proof/policy-control-plane-plan/`; manifest exists; WP06 status is honest |
| 2. WP03 compiler proof rebuild | restore `docs/proof/policy-control-plane-plan/03-*.md`; touch `packages/policy-domain/src/policy-compiler.ts`, `crates/policy-control-core/src/{policy_compiler.rs,policy_source.rs}`, seam policy files under `packages/agent-protocol-domain/src/` as needed | `npm run test --workspace @ocentra-parent/policy-domain`; `cargo test -p ocentra-policy-control-core`; `cargo test -p ocentra-parent-agent-protocol policy`; narrowed `npm run lint:architecture -- --files <policy-owned seam files>` | Replacement `03-*.md` bundle exists; deleted stale proof is gone; WP03 can move from partial to checked honestly |
| 3. WP04 delivery/ack/audit closeout | `crates/policy-control-core/src/{policy_delivery.rs,policy_event.rs,policy_source.rs}`, `packages/agent-protocol-domain/src/{policy-control-delivery-read-model.ts,policy-control-audit-redaction.ts}`, add `docs/proof/policy-control-plane-plan/04-*.md` | `cargo test -p ocentra-policy-control-core`; scoped agent-protocol-domain tests; direct portal preview/live-activity Vitest if parent-visible state is cited | Real `04-*.md` proof for state machine, ack/offline/retry, redaction/audit, per-device/domain status |
| 4. WP02/WP05 dependency execution | Portal workpacks `05/06/07/11/14/15/18`; sibling plan handoffs for device trust, custody, enforcement, identity | direct portal Vitest/Playwright, accessibility/mobile proof, Android or WSL/Docker only if selected work requires it | Real rendered authoring, approval, accessibility, mobile, audit, parent-confirm flow; not just contract proof |
| 5. Test/proof organization cleanup and final sync | move policy-counted Rust seam tests out of `crates/agent-protocol/src/{lib.rs,activity.rs}` into `crates/agent-protocol/tests/contract/` or `tests/integration/`; move flat portal policy tests into `apps/portal/tests/unit/` or `tests/integration/`; sync `PLAN_STATE.md`, `NEXT_ACTIONS.md`, `PLAN_HEALTH.md`, `WORKPACK_INDEX.md` | rerun only the scoped commands that back the touched proof bundle | No inline/src policy tests counted toward closure; plan docs and proof root all agree |
| 6. Final done gate | all open workpacks plus route docs | all scoped pass commands only; no repo-wide sweep required | Every WP01-WP08 is honestly checked, no stale proof references remain, and dependency-owned claims are backed by sibling-plan proof |

Strict done bar: this plan is only actually done when `docs/proof/policy-control-plane-plan/` contains a complete, current `01` through `08` closeout set plus `PLAN_PROOF_MANIFEST.md` and `16-validation-commands.log`; WP02/WP05 have real rendered portal/approval proof; WP03/WP04 have rebuilt proof bundles; scoped architecture is green on all policy-owned TS/Rust/seam files; counted seam tests are no longer hidden in `src`; and no doc still claims output-path or proof-root semantics that differ from the live root.

**COORDINATOR_DECISION_REQUEST**

- Recommended next slice: `WP06 proof-route truth repair`, then `WP03 compiler proof rebuild`.
- Recommended predecessor plans: none for the two local slices; in parallel, start `portal-ux-household-surfaces-plan` workpacks `05/06/07/11/14/15/18` because they are the real critical path for WP02 and WP05.
- Estimated risk: medium. Core code is mostly there; the risk is false closure from stale route/proof/docs and mis-scoped seam validation.
- Estimated proof difficulty: medium for WP06, medium-high for WP03/WP04, high for WP02/WP05 because they require real rendered/UI/approval proof rather than contract-only evidence.
- Whether I should continue immediately or pause for sequencing: continue immediately if you want me on the local-now path (`WP06` then `WP03`); pause only if you want portal/assistant dependency work sequenced first or reassigned.

## Optional Addendum

No addendum after rereading the thread. The raw report above already carries the earlier important findings that remained material: deleted `03-*.md` proof artifacts, missing `PLAN_PROOF_MANIFEST.md`, stale `06-route-sync-proof.md`, false-green `WP06` status, narrowed agent-protocol architecture truth, and the fact that `WP03` is locally closable rather than blocked by portal or service seams.
