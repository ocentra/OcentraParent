# 19 Time Budget Schedule Bonus-Time Integration

Sources: [full scope plan](../v0-5-native-apps-full-scope-plan.md),
[test blueprint](../v0-5-native-apps-test-blueprint.md), and
`docs/expectations/policy.md`.

## Where We Are

Scoped Windows owned-process time-limit proof exists, and policy foundations
exist. Native app budgets must be wired to replayable app sessions rather than
portal refresh or guessed runtime state.

Code-pass status (2026-08-16): Rust app-game-core already evaluates stored
session summaries with schedule, bonus, and timer state. This pass adds the
missing category/risk compilation handoff. The slice is code-drafted and
unvalidated; service scheduling, restart/runtime wiring, tests, proof, and
adapter execution remain deferred.

## Where We Want To Be

Schedules, time budgets, bonus time, and approvals consume session summaries
with evidence refs. Dry-run and manual-required modes are visible and auditable.

## Scope

- App time budget target and schedule refs.
- Running versus foreground duration budget modes.
- Bonus time and allow-once interactions.
- Dry-run policy decisions.
- Almost-finished, reached-limit, request-more-time, and audit states.
- Manual-required fallback when enforcement cannot run.

## Touched Paths

- `crates/app-game-core/src/app_game_time_budget.rs`
- `crates/app-game-core/src/app_game_time_budget_types.rs`
- `crates/app-game-core/src/app_game_policy_evaluator_runtime.rs`
- `crates/app-game-core/src/app_game_category_risk_policy_routing.rs`

The package-domain and enforcement-adapter paths above are historical routing
references. This code pass does not claim native adapter execution.

## Tests And Proof

- Time budget consumes session summary, not raw portal timer.
- Dry-run records would-limit without terminate/block.
- Bonus time changes policy decision with audit refs.
- Foreground-only budgets use foreground evidence only.
- Manual-required state prevents adapter calls.

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

Time-budget policy can be proof-complete in dry-run before enforcement is
available on a platform.

## Historical completion note - 2026-06-03

- Owner: `codex-c`
- Branch: `codex/app-game-read-model-service-events`
- App-plan proof root:
  `output/app-plan-proof/19-time-budget-schedule-bonus-time-integration/`
- Shared app/game proof root:
  `output/app-game-plan-proof/20-time-budget-schedule-bonus-time-integration/`

Completed:

- Cross-recorded from shared app/game WP20 rather than creating app-only
  duplicate policy truth.
- Added TypeScript parent-domain contracts and tests for app/game time-budget
  decisions that consume stored session refs, schedule evidence, bonus approval
  and audit refs, dry-run/manual-required states, and timer recovery refs.
- Covered the native app target view through all-native-apps, native-app,
  app-category, risk-app, and native-app-session inputs.

Current deferred work remains native app runtime evaluator/service persistence,
restart and scheduler consumption, portal app budget authoring/preview UI,
notification delivery, child request UX, adapter execution, and broad
installed-app blocking. The historical TypeScript proof does not close these
Rust production/runtime gaps.
