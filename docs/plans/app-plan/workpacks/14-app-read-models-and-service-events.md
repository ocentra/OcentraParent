# 14 App Read Models And Service Events

Sources: [source index](../source-index.md), [test blueprint](../v0-5-native-apps-test-blueprint.md),
and [UI/UX guide](../ui-ux-requirements-guide.md).

## Where We Are

Portal live activity and policy-preview surfaces exist, but native app inventory,
running-now, foreground-now, session, unknown, risk, approval, and capability
state are not yet complete service-backed read models.

## Where We Want To Be

The service emits typed app read models and events that the portal can render
without scanning the OS, reading SQLite directly, running timers, invoking AI,
or enforcing.

## Scope

- Inventory read model.
- Running-now and foreground-now read models.
- Session summary and daily rollup read models.
- Unknown/new app candidate read models.
- Risk/category candidate read models.
- Policy decision, approval request, authority tier, capability status, and
  enforcement result read models.
- Stale, degraded, manual-required, and not-claimed states.

## Touched Paths

- `packages/activity-domain/src/activity-surface.ts`
- `packages/activity-domain/tests/activity-surface.test.ts`
- `packages/agent-protocol-domain/tests/activity-surface-adapter.test.ts`
- `crates/agent-protocol/src/app_game.rs`
- `crates/agent-protocol/src/activity_surface.rs`
- `crates/agent-core/src/activity_store.rs`
- `crates/agent-core/src/activity_store_app_game.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_journal_sqlite_ingest.rs`
- `crates/agent-service/src/activity_surface_adapter.rs`
- `crates/agent-service/src/activity_surface_read_models.rs`
- `crates/agent-service/src/activity_surface_read_models/app_use.rs`
- `crates/agent-service/src/activity_surface_read_models/app_use/source.rs`
- `crates/agent-service/src/activity_surface_read_models/games.rs`
- `crates/agent-service/src/activity_surface_read_models/shared.rs`
- `crates/agent-service/src/activity_surface_store.rs`

## Tests And Proof

- Service read models decode from protocol shapes.
- Portal gets state from service/read-models only.
- Stale/degraded/manual-required states round trip.
- Evidence refs and policy decision refs are preserved.
- No portal-side scanning, timers, AI classification, or enforcement calls.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-app-snapshot.md), [full scope plan](../v0-5-native-apps-full-scope-plan.md), [platform deep dive](../v0-5-native-apps-platform-deep-dive.md), [test blueprint](../v0-5-native-apps-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Confirm this is native/installed-app scope, not browser pages, browser games, or game-specific product semantics unless the source docs explicitly route that handoff.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing app/game source layout inspected; no parallel app-control truth created.
- [ ] Before-state source snapshot recorded in `output/app-plan-proof/14-app-read-models-and-service-events/00-source-snapshot.md`.
- [ ] Contracts updated first where this workpack changes behavior.
- [ ] Rust/service/portal parity updated only after TypeScript contracts exist.
- [ ] Raw evidence artifacts captured where applicable: inventory, runtime, foreground, launcher, daily rollup, and SQLite/read-model rows are covered by the shared app/game projection proof; policy, approval, authority-tier, and enforcement results are N/A for this read-model-only slice.
- [ ] Tests/proof listed in this workpack and [test blueprint](../v0-5-native-apps-test-blueprint.md) are implemented or explicitly marked manual-required with reason.
- [ ] Required fixtures are present or N/A with reason for inventory, runtime, foreground, session, policy, enforcement, UI, malicious metadata, stale state, and manual-required state.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots captured for every touched parent portal, child UX, policy authoring, approval, evidence drawer, dashboard, stale, degraded, or manual-required state; no UI changed, so `ui-not-applicable.md` records why.
- [ ] Security/no-claim negative proof captured where applicable: inventory is not usage, running is not foreground, foreground is not content, AI cannot enforce, manual-required cannot call adapters, and private paths/command lines do not leak.
- [ ] Manual platform proof captured for any claim stronger than observe-only; no stronger claim was made, so the proof pack records N/A.
- [ ] Platform limitations use capability status language: observe-only, permission-required, managed-device-required, admin/root-required, system-extension-required, supervised-device-required, manual-required, or not-claimed, with proof needed to move up.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

Portal-visible app state must come from service-backed read models, not local UI
inference.

## Completion Notes - 2026-06-03

Completed on branch `codex/app-game-read-model-service-events` with proof under
`output/app-plan-proof/14-app-read-models-and-service-events/`.

This slice completes the service-backed app-use read-model portion of the
workpack by reusing the shared app/game evidence spine. It does not complete
unknown approval, risk/category production classification, policy decision,
enforcement result, child UX, portal dashboard, or live platform adapter claims;
those stay in later app-plan/app-game workpacks.
