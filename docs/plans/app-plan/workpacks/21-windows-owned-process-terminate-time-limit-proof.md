# 21 Windows Owned-Process Terminate Time-Limit Proof

Sources: [current snapshot](../current-app-snapshot.md),
[test blueprint](../v0-5-native-apps-test-blueprint.md), and
`crates/agent-core/src/enforcement_app_time_limit.rs`.

## Where We Are

Scoped Windows owned-process terminate/time-limit proof exists. That proof is
valuable but must remain clearly separate from broad app launch blocking,
package blocking, AppLocker, App Control, or cross-platform enforcement.

## Where We Want To Be

Owned-process enforcement has precise contracts, target recheck, result states,
rollback/audit refs, dry-run behavior, and parent/child UI proof while preserving
the no-claim boundary for broader app blocking.

## Scope

- Owned process target proof.
- Time-limit reached decision.
- Target changed, alreadyExited, terminated, failed, permissionLimited, and
  manualRequired result states.
- Dry-run no-action proof.
- Audit, rollback, and UI proof.
- Explicit distinction from AppLocker/App Control/broad blocking.

## Touched Paths

- `crates/agent-core/src/enforcement_app_time_limit*`
- `packages/parent-domain/src/enforcement-policy-dispatch.ts`
- `crates/agent-protocol/src/app_game.rs`
- `scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs`
- portal proof paths when UI-facing.

## Tests And Proof

- Ocentra-owned test app can be terminated only after target recheck.
- Dry-run records would-limit without terminate.
- Already-exited and target-changed states are represented.
- Result includes policy decision, target, evidence, adapter capability, and
  audit refs.
- Proof pack says this is not broad app blocking.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-app-snapshot.md), [full scope plan](../v0-5-native-apps-full-scope-plan.md), [platform deep dive](../v0-5-native-apps-platform-deep-dive.md), [test blueprint](../v0-5-native-apps-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Confirm this is native/installed-app scope, not browser pages, browser games, or game-specific product semantics unless the source docs explicitly route that handoff.
- [ ] Hub lock covers this workpack and exact implementation/docs paths. Core/service files stayed read-only because codex-a held broad locks.
- [ ] Existing app/game source layout inspected; no parallel app-control truth created.
- [ ] Before-state source snapshot recorded in `output/app-plan-proof/21-windows-owned-process-terminate-time-limit-proof/00-source-snapshot.md`.
- [ ] Contracts updated first where this workpack changes behavior. Runtime contracts were unchanged; the proof harness was extended.
- [ ] Rust/service/portal parity updated only after TypeScript contracts exist. No Rust/service/portal code changed in this pass.
- [ ] Raw evidence artifacts captured where applicable: real-service WebSocket events, timer state, encrypted journal/store refs, dry-run, stale mismatch, recover/cancel, expiry, and no-claim notes.
- [ ] Tests/proof listed in this workpack and [test blueprint](../v0-5-native-apps-test-blueprint.md) are implemented or explicitly marked manual-required with reason.
- [ ] Required fixtures are present or N/A with reason for inventory, runtime, foreground, session, policy, enforcement, UI, malicious metadata, stale state, and manual-required state.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots captured for every touched parent portal, child UX, policy authoring, approval, evidence drawer, dashboard, stale, degraded, or manual-required state; no UI changed, so `ui-not-applicable.md` records why.
- [ ] Security/no-claim negative proof captured where applicable: inventory is not usage, running is not foreground, foreground is not content, AI cannot enforce, manual-required cannot call adapters, and private paths/command lines do not leak.
- [ ] Manual platform proof captured for any claim stronger than observe-only, including OS/device version, authority tier, permission/enrollment setup, commands/UI steps, screenshots/logs, rollback, and cleanup. This pass records scoped service proof plus manual-required boundary, not new broader platform authority.
- [ ] Platform limitations use capability status language: observe-only, permission-required, managed-device-required, admin/root-required, system-extension-required, supervised-device-required, manual-required, or not-claimed, with proof needed to move up.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

Terminate current owned process is not broad app blocking, install blocking, or
launch allowlist proof.
