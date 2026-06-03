# 27 E2E And Manual Proof Artifacts

Sources: [test blueprint](../v0-5-native-apps-test-blueprint.md),
[implementation checklist](../implementation-checklist.md), and
`docs/expectations/platforms.md`.

## Where We Are

Existing app/game proof scripts cover scoped pieces. The app plan needs a
standard proof artifact layout for end-to-end and manual platform claims.

## Where We Want To Be

Every app claim has stored JSON, logs, screenshots, command outputs, manual
proof, fixture manifests, and explicit N/A/manual-required notes.

## Scope

- Proof pack template.
- Minimum serious MVP test set routing.
- E2E proof script routing.
- Manual platform proof files.
- Platform authority, permission setup, and rollback proof files.
- UI snapshots.
- Security negative proof.
- Fixture manifest for inventory, runtime, sessions, policy, and UI states.
- Validation command logs and known-gap notes.

## Touched Paths

- `output/app-plan-proof/<workpack-id>/`
- `scripts/test/*app*` when assigned.
- `docs/plans/app-plan/implementation-checklist.md`
- Playwright app specs when assigned.

## Tests And Proof

- `node scripts/test/app-game-plan-rollout-pr-gate.mjs` writes the native app
  cross-record proof pack under
  `output/app-plan-proof/27-e2e-and-manual-proof-artifacts/`.
- Proof pack complete for assigned workpack.
- Platform workpack proof includes `11-authority-tier-proof.md`,
  `12-permission-setup-proof.md`, and `13-rollback-proof.md`.
- Manual proof names host/device/version/authority tier/setup.
- Manual tests include required tags such as `@manual`,
  `@requires-android-device-owner`, `@requires-ios-familycontrols`,
  `@requires-mdm`, `@requires-endpoint-security`, `@requires-applocker`, or
  `@requires-app-control`.
- E2E coverage includes inventory-to-portal, runtime session, foreground
  duration, unknown approval, risk detection, dry-run time limit,
  owned-process enforcement where scoped, and broad-block manual-required.
- Screenshots and logs are saved; N/A reasons are explicit.

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

Manual platform proof is required when CI cannot prove the claimed OS behavior.
Missing manual proof keeps the claim manual-required or not-claimed.
