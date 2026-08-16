# 28 Rollout Checklist And PR Gate

Sources: [implementation checklist](../implementation-checklist.md),
[test blueprint](../v0-5-native-apps-test-blueprint.md), `AGENTS.md`, and
`.ocentra-ai/rules/ocentra-parent-rules.mdc`.

## Where We Are

App work is coordinated through hub/lane locks and product-doc update rules.
Docs-only planning does not move product capability status.

## Where We Want To Be

Native app work can become PR-ready only after source, docs, tests, proof,
manual-required gaps, product checklist decisions, hub reports, and PR wording
are coherent.

## Scope

- Workpack completion rules.
- Product-doc update rules.
- Feature/checklist/roadmap decision.
- Hub report content.
- PR body content.
- CI/validation expectations.
- Merge-blocking failure guard from the test blueprint.
- No-claim and bare unsupported wording guard.

## Touched Paths

- `docs/plans/app-plan/implementation-checklist.md`
- owning feature/expectation docs when status/proof changes.
- `docs/product-capability-checklist.md` when status/proof/gap changes.
- `docs/product-roadmap.md` only when milestone scope/order/completion changes.
- touched package/crate/app READMEs when implementation changes ownership/flow.
- worker-assigned source paths.

## Tests And Proof

- `git diff --check`.
- Focused package/crate/portal tests.
- Root validation when requested or needed.
- Hub/lane guards.
- `node scripts/test/app-game-plan-rollout-pr-gate.mjs` writes the native app
  rollout/PR-ready proof pack under
  `output/app-plan-proof/28-rollout-checklist-and-pr-gate/`.
- PR-ready proof pack.
- No merge-blocking failure from the test blueprint remains unresolved.
- Platform claims name observe-only, permission-required,
  managed-device-required, admin/root-required, system-extension-required,
  supervised-device-required, manual-required, or not-claimed with proof needed
  to move up.

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

Do not report done or PR-ready if a product claim moved without matching proof.
If proof is missing, the checklist and PR report must say manual-required,
unavailable, degraded, scaffold, or not-claimed rather than implying completion.
