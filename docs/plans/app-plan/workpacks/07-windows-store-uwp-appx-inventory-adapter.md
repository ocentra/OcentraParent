# 07 Windows Store UWP AppX Inventory Adapter

Sources: [full scope plan](../v0-5-native-apps-full-scope-plan.md),
[test blueprint](../v0-5-native-apps-test-blueprint.md), and
[platform deep dive](../v0-5-native-apps-platform-deep-dive.md).

## Where We Are

The Windows inventory path must distinguish Win32 executable identity from
Microsoft Store, UWP, MSIX, AppX, package family, and AppUserModelId identity.
That distinction is not yet product-complete.

## Where We Want To Be

Store/UWP apps become first-class inventory candidates with package identity,
source evidence, confidence, and clear separation from process/runtime proof.

## Scope

- Store/UWP/AppX/MSIX inventory evidence.
- AppUserModelId and package family identity.
- Version, publisher, install state, profile scope, and package source.
- Deduplication with process evidence where a runtime process maps to a package.
- Permission-limited and stale states.

## Touched Paths

- future Windows Store inventory adapter in `crates/agent-core/src/`
- `packages/activity-domain/src/app-game*.ts`
- `crates/agent-protocol/src/app_game.rs`
- `fixtures/app/inventory/windows_store_packages.json`

## Tests And Proof

- Store package fixture creates inventory evidence.
- Package family and AppUserModelId are identity fields, not display labels.
- Store package plus runtime process can merge by explicit package mapping.
- Missing package metadata degrades confidence instead of failing closed.
- Inventory output does not claim use, foreground, or blocking.

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

- App-plan proof root: output/app-plan-proof/07-windows-store-uwp-appx-inventory-adapter
- Shared app/game proof root: output/app-game-plan-proof/07-windows-store-uwp-appx-inventory-adapter
- Product-doc decision: no feature doc, expectation doc, roadmap, or product
  capability checklist status moved because this reconciliation does not add new
  runtime, service, portal, policy, enforcement, or platform capability proof.
- Remaining boundary: app-plan follow-up work still owns app-only authority,
  taxonomy, sessionization, journal/read-model, portal, approval, policy,
  child UX, broad blocking, AI digest, install/purchase, performance, E2E, and
  rollout slices.

## Manual-Required Gaps

Store/UWP inventory does not prove launch blocking, package suspension, or
family-wide enforcement.
