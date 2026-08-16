# 20 Child-Facing App Warning Block Request UX

Sources: [UI/UX guide](../ui-ux-requirements-guide.md),
[full scope plan](../v0-5-native-apps-full-scope-plan.md), and
[test blueprint](../v0-5-native-apps-test-blueprint.md).

## Where We Are

Child-facing app warning and request UX is not yet complete for native app
limits, approvals, manual-required states, or shield/block fallbacks.

Code-pass status (2026-08-16): Rust app-game-core now bridges a time-budget
decision into the existing child UX notice validator with evidence, reason,
status, and capability refs. This is code-drafted and unvalidated; live child
surface/service delivery, tests, proof, and UI snapshots remain deferred.

## Where We Want To Be

Child UX calmly explains app warnings, time almost finished, time limit reached,
ask-parent requests, manual-required fallbacks, and allowed/denied states
without exposing parent diagnostics or shame-based copy.

## Scope

- Warning, ask-parent, more-time request, limit reached, manual-required, and
  unavailable states.
- Child explanation copy and action buttons.
- Parent audit and child request correlation.
- Platform-specific fallback copy for shield/block/manual-required states.
- Accessibility, narrow viewport, and localization token readiness.

## Touched Paths

- `crates/app-game-core/src/app_game_child_ux.rs`
- `crates/app-game-core/src/app_game_child_ux_types.rs`
- `crates/app-game-core/src/app_game_time_budget.rs`

The prior portal/text-domain and Playwright paths are later UI/proof owners;
this Rust bridge does not claim live rendering or notification delivery.

## Tests And Proof

- UI snapshots for warning, ask-parent, time almost finished, time limit reached,
  and manual-required fallback.
- Child request creates audit/approval refs.
- Manual-required fallback does not imply that the app is blocked.
- Copy avoids exposing private paths, command lines, or parent-only diagnostics.

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

Child UX can request, warn, or explain before the platform can enforce. It must
not claim a block happened unless action proof exists.

## Historical completion note - 2026-06-03

- Owner: `codex-c`
- Branch: `codex/app-game-read-model-service-events`
- App-plan proof root:
  `output/app-plan-proof/20-child-facing-app-warning-block-request-ux/`
- Shared app/game proof root:
  `output/app-game-plan-proof/21-child-facing-warning-and-request-ux/`

Completed:

- Cross-recorded from shared app/game WP21 instead of creating app-only
  duplicate child UX truth.
- Added TypeScript parent-domain child UX contracts and text-domain copy tokens
  for warning, approval-needed, time-limit, ask-parent/more-time request,
  manual-required, unavailable, approved, and denied states.
- Covered the native app target view through native-app and unknown-app target
  kinds while keeping private diagnostics out of child-facing payloads.

Current deferred work remains live native app child UI, portal preview
screenshots, overlay rendering, notification delivery, service persistence,
Rust/WebSocket parity, adapter execution, and broad installed-app blocking.
The historical TypeScript contract/proof does not establish current Rust UI
or runtime completion.
