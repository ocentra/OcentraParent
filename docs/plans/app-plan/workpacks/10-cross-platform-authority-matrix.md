# 10 Cross-Platform Authority Matrix

Sources: [platform deep dive](../v0-5-native-apps-platform-deep-dive.md),
[test blueprint](../v0-5-native-apps-test-blueprint.md), and
`docs/expectations/platforms.md`.

## Where We Are

Current product docs keep non-Windows hard-control claims manual-required unless
real proof exists. The app plan now needs a richer authority-tier matrix instead
of possible/impossible platform language.

## Where We Want To Be

Every platform capability row names authority tier, setup cost, adapter proof,
rollback proof, UI label, and manual-required boundary for normal app,
permissioned, managed-device, admin/root, system-extension, supervised, and
kiosk/single-app modes where relevant.

## Scope

- Authority tier contract and capability status contract.
- Per-platform rows for Windows, macOS, Linux, Android, and iOS/iPadOS.
- Normal-app, permissioned, managed-device, admin/root/system-extension, and
  kiosk/single-app modes.
- Setup, permission, enrollment, entitlement, and rollback proof fields.
- UI labels and merge-blocking no-claim gates.

## Touched Paths

- `packages/activity-domain/src/app-game*.ts`
- `packages/parent-domain/src/app-control-*`
- `docs/plans/app-plan/v0-5-native-apps-platform-deep-dive.md`
- `docs/plans/app-plan/implementation-checklist.md`

## Tests And Proof

- Platform rows cannot claim capability without authority tier.
- Manual-required and not-claimed cannot execute adapters.
- UI does not use bare unsupported copy for final platform claims.
- Platform extension checklist links to proof packs.
- Android hide/suspend, iOS shield, macOS hard block, Linux hard block, and
  Windows broad blocking stay manual-required until proof exists.

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

Authority-tier rows do not prove platform behavior until the relevant adapter,
permission, enrollment, action, rollback, and UI proof exists.

## Completion Note - 2026-06-03

Completed through shared app/game WP11 on
`codex/app-game-authority-matrix`. Native app proof root:
`output/app-plan-proof/10-cross-platform-authority-matrix/`.

No Rust/service/portal runtime parity was added because this workpack is the
TypeScript parent-domain authority contract gate. Product checklist status is
unchanged because no live platform capability moved from manual-required or
not-claimed to supported.
