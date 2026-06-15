# 05 Installed App Inventory Model

Sources: [full scope plan](../v0-5-native-apps-full-scope-plan.md),
[test blueprint](../v0-5-native-apps-test-blueprint.md), and
`docs/app-control-settings-inventory.md`.

## Where We Are

The repo has app-control catalog data and app/game contracts, but it does not
yet have a complete native installed-app inventory model with source, custody,
confidence, installed state, stale/degraded state, and no-use guards.

## Where We Want To Be

Inventory rows can represent app presence from many sources without claiming
use. Parent UI and policy can consume inventory as candidate evidence only.

## Scope

- Inventory evidence schema and read model.
- Installed, possibly_installed, stale, permission_limited, adapter_error,
  manual-required, and not-claimed states.
- Source ids, adapter ids, observedAt, evidence refs, confidence, and reason
  codes.
- Category/risk candidates as evidence, not actions.
- Redacted metadata and raw fixture capture.

## Touched Paths

- `packages/activity-domain/src/app-game*.ts`
- `packages/parent-domain/src/app-control-catalog*.ts`
- `crates/agent-protocol/src/app_game.rs`
- future inventory fixture and proof files under `fixtures/app/inventory/`.

## Tests And Proof

- Inventory evidence cannot set running or foreground.
- Inventory rows require evidenceId, sourceId, adapterId, observedAt, deviceId,
  source, state, confidence, and reason codes.
- Duplicate candidates merge only by strong identity fields.
- Weak/partial rows remain weak or permission_limited.
- Parent portal copy labels inventory as presence, not usage.

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

## Completion Reconciliation

Completed on codex/app-plan-proof-reconciliation by mirroring the shared
app/game proof spine into the native app plan.

- App-plan proof root: output/app-plan-proof/05-installed-app-inventory-model
- Shared app/game proof root: output/app-game-plan-proof/05-inventory-evidence-model
- Product-doc decision: no feature doc, expectation doc, roadmap, or product
  capability checklist status moved because this reconciliation does not add new
  runtime, service, portal, policy, enforcement, or platform capability proof.
- Remaining boundary: app-plan follow-up work still owns app-only authority,
  taxonomy, sessionization, journal/read-model, portal, approval, policy,
  child UX, broad blocking, AI digest, install/purchase, performance, E2E, and
  rollout slices.

## Manual-Required Gaps

Inventory rows are not launch, runtime, foreground, content, or enforcement
proof.
