# account-identity-family-plan

## Normalized Header

- plan/thread name: `account-identity-family-plan`
- source thread label: `account-identity-family-plan`
- source thread id: `019ed325-2445-7132-b51a-b1877028c65d`
- worktree: `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent`
- branch: `codex/tracking-plan-full-continuation-a`
- claimed status: `in-progress; WP01 proof pack complete; WP02-WP05 partial; WP06-WP07 open`
- claimed source files/crates/packages: `packages/family-domain`; `packages/setup-domain`; `packages/portal-domain`; `apps/portal`; `crates/family-identity-core`; `crates/provisioning-core`; `docs/plans/account-identity-family-plan`
- claimed tests: `packages/family-domain/tests/unit/setup-lifecycle.test.ts`; `packages/family-domain/tests/unit/invite-recovery-lifecycle.test.ts`; `packages/family-domain/tests/unit/session-lifecycle.test.ts`; `packages/family-domain/tests/unit/token-lifecycle.test.ts`; `packages/family-domain/tests/unit/household-authority.test.ts`; `packages/setup-domain/tests/unit/family-setup-bridge.test.ts`; `packages/setup-domain/tests/unit/registration-entry.test.ts`; `packages/portal-domain/tests/unit/setup-first-run-panel.test.ts`; `apps/portal/tests/setup-first-run-route-panel.test.ts`; `apps/portal/e2e/setup-first-run-ui-proof.spec.ts`; `cargo test -p ocentra-family-identity-core`; `cargo test -p ocentra-provisioning-core readiness`
- claimed proof commands/artifacts: `npm run build --workspace @ocentra-parent/family-domain`; `npm run test --workspace @ocentra-parent/family-domain -- tests/unit/setup-lifecycle.test.ts tests/unit/invite-recovery-lifecycle.test.ts`; `npm run test --workspace @ocentra-parent/family-domain -- tests/unit/household-authority.test.ts tests/unit/session-lifecycle.test.ts tests/unit/token-lifecycle.test.ts`; `npm run test --workspace @ocentra-parent/setup-domain -- tests/unit/registration-entry.test.ts tests/unit/family-setup-bridge.test.ts`; `npm run lint:architecture -- --files packages/family-domain packages/setup-domain`; `cargo test -p ocentra-family-identity-core`; `cargo test -p ocentra-provisioning-core readiness`; `output/account-identity-family-plan-proof/01-auth-provider-decision/`; `output/account-identity-family-plan-proof/02-identity-household-role-model/`; `output/account-identity-family-plan-proof/03-session-token-lifecycle/`; `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/`; `output/account-identity-family-plan-proof/05-device-ownership-authz/`
- claimed blockers: `cloudflare-control-plane-plan` runtime/schema proof; `setup-install-provisioning-plan` first-run route binding; `data-custody-storage-plan` export/delete execution; `device-trust-bootstrap-plan` trusted-device bootstrap; `lan-plan` and `remote-access-plan` transport execution; missing WP06 and WP07 proof roots
- claimed next actions: finish WP02-WP05 proof reconciliation locally, then sequence Cloudflare sibling runtime/custody work, then upgrade WP07 from setup scaffold to real account/family UI proof, then aggregate WP06
- obvious missing evidence fields: full WP02-WP05 named proof files; canonical WP07 proof root; canonical WP06 proof root; sibling runtime/schema proof for Cloudflare custody; route-sync/manual-gap aggregation; `test-results/account-identity-family-plan-*`
- archival note: self-report only; unverified input for structural review

## Raw Self-Report

**COMPLETION_ARCHITECTURE_REPORT**

**Executive Summary**
`account-identity-family-plan` is not closeable yet, but its closure path is now much clearer and more honest than before this lane took it. The fake-green direct lifecycle test is repaired in [setup-lifecycle.test.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/family-domain/tests/unit/setup-lifecycle.test.ts), WP01 now has a real proof pack under [output/account-identity-family-plan-proof/01-auth-provider-decision/](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/output/account-identity-family-plan-proof/01-auth-provider-decision/), and WP02-WP05 now have real but partial proof roots. Final closure still depends on finishing those proof packs, converting WP07 from setup-only scaffold into real account/family route proof, and then consuming sibling-plan contract/runtime proof in WP06. The most important sequencing fact is that `cloudflare-control-plane-plan` and `setup-install-provisioning-plan` are true closure dependencies, but they do not block the next local proof-reconciliation slice.

**Plan Closure Definition**
“Actually done” for this plan means all of the following are true at once:
- WP01-WP05 required proof files from [PROOF_INDEX.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/account-identity-family-plan/PROOF_INDEX.md) exist and are backed by real passing scoped validation or explicit blocker artifacts.
- WP07 has a real proof root under [output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/) proving account/household/device/recovery states, not just setup-domain projection scaffolding.
- WP06 exists under [output/account-identity-family-plan-proof/06-security-proof-and-route-gate/](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/output/account-identity-family-plan-proof/06-security-proof-and-route-gate/) and truthfully aggregates prior proof roots plus route-sync/manual-gap proof.
- [CHECKLIST_INDEX.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/account-identity-family-plan/CHECKLIST_INDEX.md), [PLAN_STATE.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/account-identity-family-plan/PLAN_STATE.md), and the workpack docs match current truth.
- Sibling-plan-owned runtime contracts the plan explicitly depends on are either proven by those owners or recorded as route-sync blockers in WP06. No fake closure through placeholder folders or optimistic docs.

**Exact Docs / Source / Tests / Proof Read**
- Docs read:
  - [WORKPACK_INDEX.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/account-identity-family-plan/WORKPACK_INDEX.md)
  - [PLAN_STATE.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/account-identity-family-plan/PLAN_STATE.md)
  - [CHECKLIST_INDEX.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/account-identity-family-plan/CHECKLIST_INDEX.md)
  - [PROOF_INDEX.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/account-identity-family-plan/PROOF_INDEX.md)
  - [TEST_PROOF_EXPECTATIONS.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/account-identity-family-plan/TEST_PROOF_EXPECTATIONS.md)
  - [PROOF_AND_TEST_INVENTORY.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/account-identity-family-plan/PROOF_AND_TEST_INVENTORY.md)
  - [RESEARCH_AND_DECISIONS.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/account-identity-family-plan/RESEARCH_AND_DECISIONS.md)
  - [workpacks/01-auth-provider-decision.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/account-identity-family-plan/workpacks/01-auth-provider-decision.md)
  - [workpacks/02-identity-household-role-model.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/account-identity-family-plan/workpacks/02-identity-household-role-model.md)
  - [workpacks/03-session-token-lifecycle.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/account-identity-family-plan/workpacks/03-session-token-lifecycle.md)
  - [workpacks/04-invites-recovery-lifecycle.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/account-identity-family-plan/workpacks/04-invites-recovery-lifecycle.md)
  - [workpacks/05-device-ownership-authz.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/account-identity-family-plan/workpacks/05-device-ownership-authz.md)
  - [workpacks/06-security-proof-and-route-gate.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/account-identity-family-plan/workpacks/06-security-proof-and-route-gate.md)
  - [workpacks/07-parent-account-family-setup-ui.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/account-identity-family-plan/workpacks/07-parent-account-family-setup-ui.md)
  - [family-setup-device-roles.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/features/family-setup-device-roles.md)
  - [family-setup.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/expectations/family-setup.md)
  - [portal.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/expectations/portal.md)
- Source read:
  - [household-authority.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/family-domain/src/household-authority.ts)
  - [session-lifecycle.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/family-domain/src/session-lifecycle.ts)
  - [setup-lifecycle.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/family-domain/src/setup-lifecycle.ts)
  - [family-setup-bridge.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/setup-domain/src/family-setup-bridge.ts)
  - [registration-entry.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/setup-domain/src/registration-entry.ts)
  - [setup-first-run-panel.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/portal-domain/src/setup-first-run-panel.ts)
  - [SetupFirstRunRoutePanel.tsx](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/apps/portal/src/SetupFirstRunRoutePanel.tsx)
  - [household_authority.rs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/crates/family-identity-core/src/household_authority.rs)
  - [session_lifecycle.rs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/crates/family-identity-core/src/session_lifecycle.rs)
  - [setup_lifecycle.rs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/crates/family-identity-core/src/setup_lifecycle.rs)
- Tests read:
  - [setup-lifecycle.test.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/family-domain/tests/unit/setup-lifecycle.test.ts)
  - [invite-recovery-lifecycle.test.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/family-domain/tests/unit/invite-recovery-lifecycle.test.ts)
  - [session-lifecycle.test.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/family-domain/tests/unit/session-lifecycle.test.ts)
  - [token-lifecycle.test.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/family-domain/tests/unit/token-lifecycle.test.ts)
  - [household-authority.test.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/family-domain/tests/unit/household-authority.test.ts)
  - [family-setup-bridge.test.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/setup-domain/tests/unit/family-setup-bridge.test.ts)
  - [registration-entry.test.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/setup-domain/tests/unit/registration-entry.test.ts)
  - [setup-first-run-panel.test.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/portal-domain/tests/unit/setup-first-run-panel.test.ts)
  - [setup-first-run-route-panel.test.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/apps/portal/tests/setup-first-run-route-panel.test.ts)
  - [setup-first-run-ui-proof.spec.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/apps/portal/e2e/setup-first-run-ui-proof.spec.ts)
- Proof read:
  - WP01 all seven files under [01-auth-provider-decision](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/output/account-identity-family-plan-proof/01-auth-provider-decision/)
  - WP02 [03-cross-family-negative-proof.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/output/account-identity-family-plan-proof/02-identity-household-role-model/03-cross-family-negative-proof.md) and [16-validation-commands.log](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/output/account-identity-family-plan-proof/02-identity-household-role-model/16-validation-commands.log)
  - WP03 [02-token-expiry-replay-proof.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/output/account-identity-family-plan-proof/03-session-token-lifecycle/02-token-expiry-replay-proof.md) and [16-validation-commands.log](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/output/account-identity-family-plan-proof/03-session-token-lifecycle/16-validation-commands.log)
  - WP04 [01-invite-negative-proof.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/01-invite-negative-proof.md), [02-recovery-state-machine-proof.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/02-recovery-state-machine-proof.md), and [16-validation-commands.log](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/16-validation-commands.log)
  - WP05 [00-device-authority-matrix.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/output/account-identity-family-plan-proof/05-device-ownership-authz/00-device-authority-matrix.md) and [16-validation-commands.log](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/output/account-identity-family-plan-proof/05-device-ownership-authz/16-validation-commands.log)

| Current truth bucket | Live state | Exact evidence | What it means |
| --- | --- | --- | --- |
| `done` | WP01 provider/custody proof pack is real and checklist-synced | [CHECKLIST_INDEX.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/account-identity-family-plan/CHECKLIST_INDEX.md), [WORKPACK_INDEX.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/account-identity-family-plan/WORKPACK_INDEX.md), [01-auth-provider-decision](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/output/account-identity-family-plan-proof/01-auth-provider-decision/) | The provider role and Cloudflare custody split are now explicit and consumable by adjacent plans. |
| `done` | Fake-green direct lifecycle test gap is removed | [setup-lifecycle.test.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/family-domain/tests/unit/setup-lifecycle.test.ts) | Direct invite/recovery tests now match the live schema and pass. |
| `partial` | WP02 proof root exists but only 2 files are real | [02-identity-household-role-model](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/output/account-identity-family-plan-proof/02-identity-household-role-model/) | Cross-family denial is proved, but entity model, role matrix, observer, support, and audit proofs are still missing. |
| `partial` | WP03 proof root exists but only 2 files are real | [03-session-token-lifecycle](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/output/account-identity-family-plan-proof/03-session-token-lifecycle/) | Expiry/replay separation is proved, but lifecycle, refresh, freshness, request-safety, and redaction proofs are missing. |
| `partial` | WP04 proof root exists but only 3 files are real | [04-invites-recovery-lifecycle](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/) | Invite negatives and recovery state machine are proved, but abuse, delete/export handoff, support audit, and invite-state artifacts are missing. |
| `partial` | WP05 proof root exists but only 2 files are real | [05-device-ownership-authz](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/output/account-identity-family-plan-proof/05-device-ownership-authz/) | Device authority matrix is proved, but dedicated negatives and owner-only slices are still missing. |
| `partial` | WP07 portal/setup scaffold exists outside this plan’s proof root | [setup-first-run-panel.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/portal-domain/src/setup-first-run-panel.ts), [SetupFirstRunRoutePanel.tsx](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/apps/portal/src/SetupFirstRunRoutePanel.tsx), [setup-first-run-ui-proof.spec.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/apps/portal/e2e/setup-first-run-ui-proof.spec.ts) | The plan is not missing a route harness; it has a scaffold, but it is still setup-domain projection rather than real account/family closure. |
| `false-green` | Placeholder category folders look like broad coverage but are mostly `.gitkeep` optics | `packages/family-domain/tests/security/**`, `packages/setup-domain/tests/contract/**`, `packages/portal-domain/tests/e2e/**`, `crates/family-identity-core/tests/security/**` | Category scaffolds must not be counted as actual unit/integration/security/e2e coverage for this plan. |
| `false-green` | WP06/WP07 workpack fill-before-done text is stale/misleading | [workpacks/06-security-proof-and-route-gate.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/account-identity-family-plan/workpacks/06-security-proof-and-route-gate.md), [workpacks/07-parent-account-family-setup-ui.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/account-identity-family-plan/workpacks/07-parent-account-family-setup-ui.md) | They still talk about a missing portal setup route/test harness even though the scaffold already exists. |
| `missing` | WP06 proof root absent | [output/account-identity-family-plan-proof/06-security-proof-and-route-gate/](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/output/account-identity-family-plan-proof/06-security-proof-and-route-gate/) | Whole-plan readiness aggregation is not started. |
| `missing` | WP07 proof root absent | [output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/) | No canonical UI proof pack exists yet. |
| `missing` | Cloudflare adapter/schema/runtime proof | [PLAN_STATE.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/account-identity-family-plan/PLAN_STATE.md), [RESEARCH_AND_DECISIONS.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/account-identity-family-plan/RESEARCH_AND_DECISIONS.md) | WP01 is still docs/proof only until sibling runtime work lands. |

**Code Surface And Ownership**
- `packages/family-domain`: owns the TypeScript authority/session/setup contracts. This is the plan’s canonical source-of-truth surface.
- `packages/setup-domain`: consumes those contracts for readiness and registration projection. It is a downstream consumer, not the family authority owner.
- `crates/family-identity-core`: Rust parity for household/session/setup boundaries crossing service/runtime layers.
- `crates/provisioning-core`: downstream readiness consumer of family/setup states. Relevant to proof, not ownership.
- `packages/portal-domain`: route intent/projection layer for setup-first-run UI. It does not own authority.
- `apps/portal`: actual route component/tests/e2e scaffold. It does not own authority.
- Sibling ownership outside this plan:
  - `cloudflare-control-plane-plan`: auth adapter + D1/DO/KV runtime/schema boundary
  - `setup-install-provisioning-plan`: first-run route wiring beyond scaffold
  - `data-custody-storage-plan`: delete/export execution
  - `device-trust-bootstrap-plan`: trusted-device bootstrap
  - `lan-plan` and `remote-access-plan`: transport execution

**Test Surface Inventory**
- No inline `.test` / `.spec` files or `#[cfg(test)]` blocks were found in the owned `src` surfaces I inspected. I do not see a current “move tests out of src” requirement for the plan-owned files I read.
- Actual executable test surfaces currently used by this plan:
  - `packages/family-domain/tests/unit/*.test.ts`
  - `packages/setup-domain/tests/unit/*.test.ts`
  - `packages/portal-domain/tests/unit/setup-first-run-panel.test.ts`
  - `apps/portal/tests/setup-first-run-route-panel.test.ts`
  - `apps/portal/e2e/setup-first-run-ui-proof.spec.ts`
  - `crates/family-identity-core` via cargo test on its external test tree
  - `crates/provisioning-core` readiness suite via cargo test
- Placeholder optics:
  - `packages/family-domain/tests/contract`, `e2e`, `integration`, `security`, `property-based`, `load` exist but are currently `.gitkeep` scaffolds, not plan coverage.
  - The same placeholder pattern exists under `packages/setup-domain/tests`, `packages/portal-domain/tests`, and many `crates/family-identity-core/tests/*` category folders.
- Missing major categories where actually applicable:
  - `contract`: applicable and effectively missing for WP02-WP05 despite scaffold dirs.
  - `security`: applicable and effectively missing as executable category coverage for WP03/WP04/WP05 despite scaffold dirs.
  - `integration`: applicable for WP07 real route binding; current route scaffold tests are not enough to prove account/family integration.
  - `e2e`: applicable for WP07; one existing Playwright proof spec exists, but it is scaffold-level and not account/family closure.
  - `property`: useful for lifecycle/state-machine hardening, but not strictly the next honesty gap.
  - `load`: not currently a closure bar for this plan.

**Proof Inventory**
- Canonical root: [output/account-identity-family-plan-proof/](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/output/account-identity-family-plan-proof/)
- Real now:
  - WP01 complete root
  - WP02 partial root
  - WP03 partial root
  - WP04 partial root
  - WP05 partial root
- Stale/misleading:
  - WP06 workpack says WP07 is blocked by missing route harness, but [SetupFirstRunRoutePanel.tsx](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/apps/portal/src/SetupFirstRunRoutePanel.tsx) and its tests already exist.
  - WP07 workpack repeats the same stale “missing route harness” claim.
- Missing:
  - WP06 root
  - WP07 root
  - `test-results/account-identity-family-plan-*`

**Scoped Validation Inventory**
- Already passing:
  - `npm run build --workspace @ocentra-parent/family-domain`
  - `npm run test --workspace @ocentra-parent/family-domain -- tests/unit/setup-lifecycle.test.ts tests/unit/invite-recovery-lifecycle.test.ts`
  - `npm run test --workspace @ocentra-parent/family-domain -- tests/unit/household-authority.test.ts tests/unit/session-lifecycle.test.ts tests/unit/token-lifecycle.test.ts`
  - `npm run test --workspace @ocentra-parent/setup-domain -- tests/unit/registration-entry.test.ts tests/unit/family-setup-bridge.test.ts`
  - `npm run lint:architecture -- --files packages/family-domain packages/setup-domain`
  - `cargo test -p ocentra-family-identity-core`
  - `cargo test -p ocentra-provisioning-core readiness`
  - `node -e "console.log('provider-decision-docs-only')"`
  - `npm run lint:architecture -- --files docs/plans/account-identity-family-plan`
  - `git diff --check -- docs/plans/account-identity-family-plan/...`
- Currently failing:
  - None in the current scoped slice.
- Historical fail now fixed:
  - direct `family-domain` invite/recovery suite previously failed because helper inputs in [setup-lifecycle.test.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/family-domain/tests/unit/setup-lifecycle.test.ts) were missing `abuseState` and `responseTimingState`.
- Unrun but still needed later:
  - `npm run build --workspace @ocentra-parent/portal-domain`
  - `npm run test --workspace @ocentra-parent/portal -- account`
  - `npm run test --workspace @ocentra-parent/portal -- family`
  - `npm run test:e2e --workspace @ocentra-parent/portal -- account`
  - WP06 focused security/route-gate command set
  - any sibling-plan runtime/schema validations

| Dependency bucket | Plan / surface | Why it matters | What can still proceed now |
| --- | --- | --- | --- |
| `local-now` | WP02-WP05 proof reconciliation inside this plan | Most missing named proof files can be written or explicitly blocked from current passing contract tests. | Continue filling proof artifacts under `output/account-identity-family-plan-proof/02` through `05` and keep checklist rows honest. |
| `needs-coordinator-sequencing` | `cloudflare-control-plane-plan` | Final closure needs real D1/DO/KV auth adapter/schema/runtime proof, not just WP01 docs. | This plan can still finish WP02-WP05 proof locally first. |
| `needs-coordinator-sequencing` | `setup-install-provisioning-plan` | WP07 needs real first-run route binding to account/family state beyond the current setup-domain projection scaffold. | This plan can still document WP07 scaffold truth and prepare exact gap criteria. |
| `needs-sibling-plan-contract` | `data-custody-storage-plan` | WP04 can prove handoff semantics, but not actual export/delete execution. | Local proof can stay at handoff-contract level. |
| `needs-sibling-plan-contract` | `device-trust-bootstrap-plan` | WP05/WP07 cannot claim real trusted-device bootstrap or step-up closure. | Local proof can keep device-trust as manual-required / external. |
| `needs-sibling-plan-contract` | `lan-plan` / `remote-access-plan` | WP05/WP07 cannot claim transport execution proof. | Local proof can still prove authz and source-label separation. |
| `host-platform-limited` | Apple-native parent-app proof, if later demanded | Not currently required by any current truth I inspected. | None of WP01-WP06 needs Apple-only proof right now. |

**Platform Feasibility**
- Windows host now:
  - All current TypeScript/Rust scoped proof commands already used in this slice.
  - Portal route/unit/e2e proof is feasible here.
  - Responsive web/mobile-width proof for WP07 is feasible here.
- Android Studio / device:
  - Only relevant if WP07’s `05-mobile-parent-child-claim-split-proof.md` needs real Android browser or packaged-parent proof beyond responsive web.
  - Not required for current WP01-WP06 honesty.
- WSL / Docker:
  - Feasible for Linux/containerized service or browser runners if later needed.
  - Not a current closure blocker for this plan.
- Truly Apple-host-only:
  - None required by current workpack truth.
  - Only future native iOS/macOS packaging or Apple-native auth flows would create that requirement.

| Ordered slice | Files / domains to touch | Validation to run | Proof to collect | Exit criteria |
| --- | --- | --- | --- | --- |
| 1. Finish WP02-WP05 proof reconciliation | `docs/plans/account-identity-family-plan/CHECKLIST_INDEX.md`, `PLAN_STATE.md`, `WORKPACK_INDEX.md`, `workpacks/02` through `05`, `output/account-identity-family-plan-proof/02` through `05` | Reuse current passing `family-domain`, `setup-domain`, `family-identity-core`, `provisioning-core` commands; add only targeted commands if a missing proof file needs new evidence | Missing named proof files or explicit blocker artifacts for each missing row | Every WP02-WP05 required proof file either exists with real evidence or is explicitly blocked; no fake references remain |
| 2. Sequence Cloudflare sibling runtime/custody work | `docs/plans/cloudflare-control-plane-plan/workpacks/04-route-manifest-and-domain-contracts.md`, `05-auth-admin-support-boundary.md`, `06-storage-do-d1-kv-r2-queue-bindings.md` plus its owned worker/module sources | Sibling-owned scoped auth/storage/runtime validation | Runtime/schema proof that consumes the WP01 contract | This plan can reference real adapter/schema proof instead of docs-only custody |
| 3. Upgrade WP07 from scaffold to real account/family UI proof | `packages/portal-domain/src/setup-first-run-panel.ts`, `packages/portal-domain/tests/unit/setup-first-run-panel.test.ts`, `apps/portal/src/SetupFirstRunRoutePanel.tsx`, `apps/portal/tests/setup-first-run-route-panel.test.ts`, `apps/portal/e2e/setup-first-run-ui-proof.spec.ts`, likely adjacent `setup-domain` read-model/state-machine files if needed | `npm run build --workspace @ocentra-parent/portal-domain`; targeted portal tests; targeted portal e2e; architecture lint for `packages/family-domain packages/portal-domain apps/portal` | Full WP07 proof root `00` through `06` plus `16` | UI proves real account/household/device/recovery distinctions and honest source/custody labels without implying login == trust |
| 4. Consume sibling outputs and write WP06 | `docs/plans/account-identity-family-plan/workpacks/06-security-proof-and-route-gate.md`, `CHECKLIST_INDEX.md`, `PLAN_STATE.md`, `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/*` | WP06 focused command set, only scoped to touched surfaces | `00` through `08` plus `16` in the WP06 root | Route-sync, misuse, request-safety, redaction, and gap register are all explicit and honest |
| 5. Final plan-state reconciliation | `CHECKLIST_INDEX.md`, `PLAN_STATE.md`, `WORKPACK_INDEX.md`, all relevant workpacks | `git diff --check`, relevant scoped command logs only | No new proof root; reconcile docs to current truth | No stale plan-status text, no placeholder optics counted as coverage, no unresolved false-green claims |

**Blocker Taxonomy**
- `local-now`
  - Finish missing WP02-WP05 proof files or explicit blocker artifacts.
  - Reconcile stale WP06/WP07 workpack text to actual route/test harness truth.
- `needs-coordinator-sequencing`
  - `cloudflare-control-plane-plan` for real auth adapter + D1/DO/KV proof.
  - `setup-install-provisioning-plan` for first-run route binding beyond the current projection scaffold.
- `needs-sibling-plan-contract`
  - `data-custody-storage-plan` for export/delete execution.
  - `device-trust-bootstrap-plan` for trusted-device/bootstrap proof.
  - `lan-plan` and `remote-access-plan` for transport execution proof.
- `host-platform-limited`
  - None currently required for honest closure of this plan.

**First Coordinator Ask**
`cloudflare-control-plane-plan` should be the first sibling plan sequenced after my next local WP02-WP05 proof-reconciliation slice. Reason: this plan’s provider/custody contract is now explicit enough to consume, and without real Cloudflare adapter/schema/runtime proof, WP01 remains docs-only and the entire plan cannot honestly close at runtime level.

**Strict Done Bar**
Before this plan can ever be marked done:
- WP01-WP05 proof roots are fully populated or explicitly blocked at the exact artifact level.
- WP07 proof root exists and proves real account/family first-run behavior, not just setup-domain route projection.
- WP06 proof root exists and truthfully aggregates all route-sync and misuse boundaries.
- The Cloudflare runtime/storage contract has real sibling proof behind the WP01 custody decision.
- The setup-first-run UI proof is bound to real account/family state transitions and honest device-trust/degraded/manual-required labeling.
- Placeholder test folders and stale docs are not being counted as coverage or proof.
- All plan docs reflect current truth.

**COORDINATOR_DECISION_REQUEST**
- Recommended next slice: finish WP02-WP05 proof reconciliation locally, starting with the missing named proof files in `02`, `03`, `04`, and `05` roots.
- Recommended predecessor plans: no predecessor is required before that next local slice; after it, sequence `cloudflare-control-plane-plan` first, then `setup-install-provisioning-plan`.
- Estimated risk: medium-high. Core contracts look stable, but closure still depends on cross-plan runtime and UI proof.
- Estimated proof difficulty: high. The remaining work is more about honest proof, route-sync, and sibling integration than raw code volume.
- Whether I should continue immediately or pause for sequencing: continue immediately on the local WP02-WP05 proof-reconciliation slice unless you want to freeze this plan until `cloudflare-control-plane-plan` is actively moving in parallel.

## Optional Addendum

- Earlier audit passes were more explicit about the adjacent-plan contract than the latest architecture report. The exact contract those plans should consume is:
  - external provider is `authn only`
  - [household-authority.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/family-domain/src/household-authority.ts) owns account/household/role/device authority
  - [session-lifecycle.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/family-domain/src/session-lifecycle.ts) owns browser/session/token/freshness/replay rules
  - [setup-lifecycle.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/family-domain/src/setup-lifecycle.ts) owns invite/recovery/abuse/timing-parity/handoff rules
  - [family-setup-bridge.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/setup-domain/src/family-setup-bridge.ts) and [registration-entry.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/setup-domain/src/registration-entry.ts) are downstream readiness consumers
  - Cloudflare custody split is `D1 authoritative`, `Durable Objects coordination only`, `KV non-authoritative`, `R2 no authority state`
- Earlier audit passes also established that no additional TS or Rust code beyond the stale helper repair in [setup-lifecycle.test.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/family-domain/tests/unit/setup-lifecycle.test.ts) had yet been proven necessary before honest WP01-WP05 reclosure.
- Earlier audit passes stated the proof-root status explicitly:
  - WP01 real and complete
  - WP02 real and partial
  - WP03 real and partial
  - WP04 real and partial
  - WP05 real and partial
  - WP06 absent
  - WP07 absent
