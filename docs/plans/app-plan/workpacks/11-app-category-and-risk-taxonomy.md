# 11 App Category And Risk Taxonomy

Sources: [full scope plan](../v0-5-native-apps-full-scope-plan.md),
[test blueprint](../v0-5-native-apps-test-blueprint.md), and
`docs/expectations/ai.md`.

## Where We Are

App-control catalog data exists, but native app categories and risk labels are
not yet a complete evidence-backed taxonomy for policy targets, parent review,
unknown app handling, and local AI classification.

## Where We Want To Be

Categories and risk labels are source/confidence-bearing candidates. They can
help policy ask, warn, or route manual review, but they do not directly enforce
and they do not change identity.

## Scope

- Category values for school, productivity, social, messaging, video, music,
  ai_chatbot, vpn_proxy, remote_desktop, download_torrent, system, unknown, and
  future mapped app classes.
- Risk candidates for VPN/proxy, remote desktop, torrent/download,
  installer/updater, AI/chatbot, social/video/messaging, and unknown risk.
- Source, confidence, reason code, evidence refs, parent override, and AI
  output fields.
- Policy candidate routing without direct enforcement.

## Touched Paths

- `packages/activity-domain/src/app-game*.ts`
- `packages/parent-domain/src/app-control-catalog*.ts`
- `packages/parent-domain/src/policy.ts`
- `fixtures/app/policy/*`

## Tests And Proof

- Category candidate requires source, confidence, and evidence refs.
- Confidence must be `0..1`.
- Unknown executable with VPN-like name remains candidate, not fact.
- Parent label/override changes display or policy candidate, not raw identity.
- AI category cannot directly block, suspend, shield, or terminate.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-app-snapshot.md), [full scope plan](../v0-5-native-apps-full-scope-plan.md), [platform deep dive](../v0-5-native-apps-platform-deep-dive.md), [test blueprint](../v0-5-native-apps-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Confirm this is native/installed-app scope, not browser pages, browser games, or game-specific product semantics unless the source docs explicitly route that handoff.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing app/game source layout inspected; no parallel app-control truth created.
- [ ] Before-state source snapshot recorded in `output/app-plan-proof/11-app-category-and-risk-taxonomy/00-source-snapshot.md` or explicit docs-only N/A reason.
- [ ] Contracts updated first where this workpack changes behavior.
- [ ] Rust/service/portal parity updated only after TypeScript contracts exist; not touched in this contract-only slice.
- [ ] Raw evidence artifacts captured where applicable: contract fixtures only; runtime, journal, SQLite, policy, approval, authority, and enforcement artifacts are N/A with reason in the proof pack.
- [ ] Tests/proof listed in this workpack and [test blueprint](../v0-5-native-apps-test-blueprint.md) are implemented or explicitly marked manual-required with reason.
- [ ] Required fixtures are present or N/A with reason for inventory, runtime, foreground, session, policy, enforcement, UI, malicious metadata, stale state, and manual-required state.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots captured for every touched parent portal, child UX, policy authoring, approval, evidence drawer, dashboard, stale, degraded, or manual-required state; no UI changed and `ui-not-applicable.md` records why.
- [ ] Security/no-claim negative proof captured where applicable: inventory is not usage, running is not foreground, foreground is not content, AI cannot enforce, manual-required cannot call adapters, and private paths/command lines do not leak.
- [ ] Manual platform proof captured for any claim stronger than observe-only, including OS/device version, authority tier, permission/enrollment setup, commands/UI steps, screenshots/logs, rollback, and cleanup; N/A because no platform authority moved.
- [ ] Platform limitations use capability status language: observe-only, permission-required, managed-device-required, admin/root-required, system-extension-required, supervised-device-required, manual-required, or not-claimed, with proof needed to move up.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Completion Note - 2026-06-03

- Owner/lane: codex-c.
- Branch: `codex/app-game-category-risk-taxonomy`.
- Shared app/game proof:
  `output/app-game-plan-proof/12-app-game-category-and-risk-taxonomy/`.
- Native app proof:
  `output/app-plan-proof/11-app-category-and-risk-taxonomy/`.
- Product-doc decision: feature doc and plan docs updated; product capability
  checklist unchanged because this is contract/test proof only and does not move
  runtime, UI, policy, platform authority, or enforcement status.

## Manual-Required Gaps

Category and risk candidates are not identity proof, content proof, or action
authority.
