# 24 Platform Extension Checklist And Proof Routing

Sources: [platform deep dive](../v0-5-native-apps-platform-deep-dive.md),
[implementation checklist](../implementation-checklist.md), and
[test blueprint](../v0-5-native-apps-test-blueprint.md).

## Where We Are

The platform-specific MAC, IOS, ANDROID, and LINUX rows are recorded as
extension checklist items, not base MVP workpacks. They need proof routing so
future workers can promote one row without bloating the base plan.

## Where We Want To Be

Each platform extension item has a target authority tier, proof pack path,
manual tags, setup proof, action proof, rollback proof, UI label, and product
doc update decision before it can move status.

## Scope

- MAC-01 through MAC-12 routing.
- IOS-01 through IOS-12 routing.
- ANDROID-01 through ANDROID-14 routing.
- LINUX-01 through LINUX-14 routing.
- Proof pack naming and manual test tags.
- Promotion rules from extension checklist to implementation work.

## Touched Paths

- `docs/plans/app-plan/implementation-checklist.md`
- `docs/plans/app-plan/v0-5-native-apps-platform-deep-dive.md`
- platform-specific proof packs under `output/app-plan-proof/<platform>/`
- feature/checklist docs when proof changes status.

## Tests And Proof

- Every promoted extension row names authority tier and proof pack.
- Manual tests use required tags.
- Stronger-than-observe claims require `11-authority-tier-proof.md`,
  `12-permission-setup-proof.md`, and `13-rollback-proof.md`.
- Missing proof leaves row unchecked/manual-required/not-claimed.

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

Platform extension checklist rows are not implementation proof until promoted,
implemented, validated, and linked to proof artifacts.
