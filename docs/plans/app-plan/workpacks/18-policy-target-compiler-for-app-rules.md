# 18 Policy Target Compiler For App Rules

Sources: [full scope plan](../v0-5-native-apps-full-scope-plan.md),
[test blueprint](../v0-5-native-apps-test-blueprint.md), and
`docs/expectations/policy.md`.

## Where We Are

Policy and app-control catalog foundations exist, but native app targets need a
compiler that respects identity strength, category confidence, authority tier,
schedule, actor, audit, and evidence refs.

2026-08-16 app-plan-code-pass: the Rust compiler and category/risk composition
are code-drafted in `crates/app-game-core`. The compiler remains dry-run only,
keeps hard actions manual-required without authority, and is unvalidated in
this production-code phase. Tests, proof, checklist closure, CI, and runtime
enforcement are deferred.

## Where We Want To Be

Parent-authored app rules compile only when required target evidence exists.
Impossible hard actions degrade to manual_required or not-claimed, never into an
adapter call.

## Scope

- Targets for specific_app, package_id, bundle_id, app_user_model_id,
  desktop_entry_id, executable_hash, publisher, category, unknown_apps,
  newly_installed_apps, portable_apps, vpn_proxy_apps, and all_non_system_apps.
- Observe, warn, ask_parent, time_limit, terminate_running, block_launch,
  hide/suspend/shield, and manual_required decisions.
- Dry-run and enforcement-mode handling.
- Evidence refs, schedule refs, actor refs, and audit refs.

## Touched Paths

- `crates/app-game-core/src/app_game_policy_target_compiler.rs`
- `crates/app-game-core/src/app_game_policy_target_compiler_helpers.rs`
- `crates/app-game-core/src/app_game_category_risk_policy_routing.rs`
- `crates/app-game-core/src/app_game_category_risk_policy_routing_types.rs`

## Tests And Proof

- Policy cannot target fields that the platform has not proved.
- Time_limit requires session summary.
- Terminate_running requires current process proof.
- Block_launch returns manual_required without platform proof.
- Android/iOS/macOS/Linux strong actions require matching authority proof.
- Dry-run cannot enforce.

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

Policy compilation does not prove an adapter can enforce. Strong actions degrade
until platform proof exists.

## Historical completion note - 2026-06-03

- Owner/lane: `codex-c`
- Branch: `codex/app-game-read-model-service-events`
- App-plan proof root:
  `output/app-plan-proof/18-policy-target-compiler-for-app-rules`
- Shared app/game proof root:
  `output/app-game-plan-proof/19-policy-target-compiler-for-app-game-rules`
- The historical TypeScript/package paths below are not tracked implementation
  owners in the current checkout. Current production ownership is Rust-first in
  `crates/app-game-core`; existing contract tests are not rerun in this phase.

Completed app-side proof:

- Specific app, package, bundle, AppUserModelId, desktop-entry, executable-hash,
  and publisher targets require identity proof.
- Unknown app targets compile only from unknown-state proof.
- App category/risk targets require category proof.
- Scheduled app rules require schedule proof.
- Wrong-device, wrong-local-user, and stale evidence are rejected.
- Unproved block-launch compiles to manual-required with disabled handoff.

Deferred:

- App-plan WP17 risk app candidate production remains open.
- Rust/service parity, runtime evaluator execution, portal authoring/preview UI,
  timers, notifications, rollback, and adapter execution remain later work.
