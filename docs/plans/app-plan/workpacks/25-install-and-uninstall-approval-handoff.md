# 25 Install And Uninstall Approval Handoff

Sources: [full scope plan](../v0-5-native-apps-full-scope-plan.md),
`docs/expectations/app-install-purchase-approval.md`, and
[platform deep dive](../v0-5-native-apps-platform-deep-dive.md).

## Where We Are

Native app inventory can detect app presence and changes once implemented, but
install, uninstall, purchase, store, MDM, package-manager, and entitlement flows
belong to adjacent approval/platform features unless explicitly assigned.

## Where We Want To Be

The app plan hands install/uninstall approval work to the correct source docs
while still recording app evidence, new-app candidates, unknown app approval,
and platform capability status for installed apps.

## Scope

- Newly installed app evidence handoff.
- Uninstall detection and stale inventory handoff.
- Store purchase/install approval boundary.
- Android package install and Device Owner/Profile Owner handoff.
- iOS Screen Time/MDM/App Store/supervised handoff.
- Linux package-manager and Flatpak/Snap handoff.
- Windows installer/updater detection handoff.

## Touched Paths

- `docs/expectations/app-install-purchase-approval.md` when acceptance changes.
- `docs/plans/app-plan/source-index.md`
- `packages/parent-domain/src/app-control-catalog*.ts`
- platform-specific extension proof paths when assigned.

## Tests And Proof

- New inventory delta can create a review candidate.
- Uninstall/stale inventory does not imply child intent.
- Install approval claims route to adjacent feature docs.
- Package-manager, store, MDM, and supervised-device claims remain
  platform-specific/manual-required until proof exists.

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

Detected install/uninstall state is evidence. Store purchase approval,
package-manager control, MDM install, and uninstall blocking need separate proof.

## Completion Note - 2026-06-03

- Read source docs: native apps plan README, source index, current snapshot,
  full scope plan, platform deep dive, test blueprint, UI/UX guide, main
  checklist, app-game shared plan docs, app-install/purchase expectation, and
  this workpack.
- Kept scope native/installed-app and shared app/game handoff only; browser
  pages, browser games, and E-C-owned child-facing install/purchase approval
  files were not edited.
- Added shared parent-domain proof through
  `packages/parent-domain/src/app-game-install-store-handoff*.ts` and
  `packages/parent-domain/tests/app-game-install-store-handoff.test.ts`.
- Proof output:
  `output/app-plan-proof/25-install-and-uninstall-approval-handoff/` and
  `test-results/app-game-install-store-handoff-proof/proof.json`.
- UI snapshots are not applicable because no UI source changed; the proof pack
  records `06-ui-snapshots/ui-not-applicable.md`.
- Product checklist unchanged because this handoff does not prove live store
  integration, approval UI, package-manager/store interception, billing
  entitlement logic, uninstall blocking, or anti-tamper behavior.
