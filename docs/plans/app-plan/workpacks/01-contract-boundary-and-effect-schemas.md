# 01 Contract Boundary And Effect Schemas

Sources: [folder README](../README.md), [source index](../source-index.md),
[current snapshot](../current-app-snapshot.md), [full scope plan](../v0-5-native-apps-full-scope-plan.md),
[platform deep dive](../v0-5-native-apps-platform-deep-dive.md), and
[test blueprint](../v0-5-native-apps-test-blueprint.md).

## Where We Are

`packages/activity-domain` already owns app/game observation and session
contracts. `packages/parent-domain` owns app-control catalog and policy
contracts. Rust protocol mirrors exist in `crates/agent-protocol/src/app_game.rs`.
The current shape is not yet a full native-app evidence, authority, policy, and
enforcement contract stack.

## Where We Want To Be

Every native app workpack starts from typed Effect Schema contracts before Rust,
service, portal, AI, policy, or enforcement code claims support. Contracts must
encode weak evidence, degraded states, authority tiers, and manual-required
states so later layers cannot overclaim.

## Scope

- Native app identity, inventory, runtime, foreground, session, category, risk,
  policy target, approval, authority-tier, capability, action, and enforcement
  result schemas.
- Evidence refs, confidence, reason codes, stale/degraded states, custody,
  redaction, and proof refs.
- AI classification input/output refs, with no action authority.
- Platform state language: observe-only, permission-required,
  managed-device-required, admin/root-required, system-extension-required,
  supervised-device-required, manual-required, and not-claimed.

## Touched Paths

- `packages/activity-domain/src/app-game*.ts`
- `packages/parent-domain/src/app-control-*`
- `packages/parent-domain/src/enforcement-policy-dispatch.ts`
- `crates/agent-protocol/src/app_game.rs`
- related package/crate tests when assigned.

## Tests And Proof

- Effect Schema decode tests accept valid states and reject invalid states.
- No display-name-only deterministic identity.
- Inventory cannot set runtime or foreground fields.
- AI output cannot contain block/terminate/suspend/shield actions.
- Manual-required/unavailable states cannot mark actions as executed.
- Rust protocol parity lands only after TypeScript contracts exist.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-app-snapshot.md), [full scope plan](../v0-5-native-apps-full-scope-plan.md), [platform deep dive](../v0-5-native-apps-platform-deep-dive.md), [test blueprint](../v0-5-native-apps-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Confirm this is native/installed-app scope, not browser pages, browser games, or game-specific product semantics unless the source docs explicitly route that handoff.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing app/game source layout inspected; no parallel app-control truth created.
- [ ] Before-state source snapshot recorded in `output/app-plan-proof/<workpack-id>/00-source-snapshot.md` or explicit docs-only N/A reason.
- [ ] Contracts updated first where this workpack changes behavior.
- [ ] Rust/service/portal parity updated only after TypeScript contracts exist.
- [ ] Raw evidence artifacts captured where applicable: inventory rows, process/package observations, foreground observations, session summaries, journal entries, SQLite/read-model rows, policy decisions, approval requests, authority-tier rows, and enforcement results.
- [ ] Tests/proof listed in this workpack and [test blueprint](../v0-5-native-apps-test-blueprint.md) are implemented or explicitly marked manual-required with reason.
- [ ] Required fixtures are present or N/A with reason for inventory, runtime, foreground, session, policy, enforcement, UI, malicious metadata, stale state, and manual-required state.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots captured for every touched parent portal, child UX, policy authoring, approval, evidence drawer, dashboard, stale, degraded, or manual-required state; if no UI changed, `ui-not-applicable.md` records why.
- [ ] Security/no-claim negative proof captured where applicable: inventory is not usage, running is not foreground, foreground is not content, AI cannot enforce, manual-required cannot call adapters, and private paths/command lines do not leak.
- [ ] Manual platform proof captured for any claim stronger than observe-only, including OS/device version, authority tier, permission/enrollment setup, commands/UI steps, screenshots/logs, rollback, and cleanup.
- [ ] Platform limitations use capability status language: observe-only, permission-required, managed-device-required, admin/root-required, system-extension-required, supervised-device-required, manual-required, or not-claimed, with proof needed to move up.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

Contracts alone do not create runtime, platform, UI, policy, AI, or enforcement
claims.
