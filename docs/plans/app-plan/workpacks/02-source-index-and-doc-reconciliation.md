# 02 Source Index And Doc Reconciliation

Sources: [source index](../source-index.md), [pasted content coverage audit](../pasted-content-coverage-audit.md),
`docs/features/app-game-control.md`, `docs/expectations/app-game-evidence.md`,
`docs/architecture/app-game-evidence-sessions.md`, and
`docs/app-control-capability-guide.md`.

## Where We Are

Native app truth is spread across product, expectation, architecture, catalog,
platform, enforcement, policy, AI, and generated inventory docs. The app-plan
folder now narrows the native app slice without moving those source documents.

## Where We Want To Be

The app-plan folder is the implementation planning location, while the owning
feature, expectation, architecture, roadmap, checklist, package, and crate docs
remain source-of-truth files. Every future paste or platform source is either
covered here or explicitly left out with a reason.

## Scope

- Maintain source index and pasted-content coverage.
- Link source docs without duplicating or overriding feature truth.
- Track docs-only planning separately from implementation status changes.
- Record when product checklist, roadmap, expectation, or README updates are
  required.
- Keep generated inventories and code outputs out of the plan folder.

## Touched Paths

- `docs/plans/app-plan/**`
- `docs/features/app-game-control.md`
- `docs/expectations/app-game-evidence.md`
- `docs/architecture/app-game-evidence-sessions.md`
- `docs/app-control-capability-guide.md`
- `docs/product-capability-checklist.md` only when status/proof/gap changes.

## Tests And Proof

- Link/read sanity for every changed markdown file.
- `git diff --check`.
- Coverage audit names every pasted attachment or source input used.
- Product docs are not changed unless a status, proof, acceptance contract, or
  gap actually changes.
- Hub/lane guards pass before report or commit.

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

- App-plan proof root: output/app-plan-proof/02-source-index-and-doc-reconciliation
- Shared app/game proof root: output/app-game-plan-proof/02-source-index-and-doc-reconciliation
- Product-doc decision: no feature doc, expectation doc, roadmap, or product
  capability checklist status moved because this reconciliation does not add new
  runtime, service, portal, policy, enforcement, or platform capability proof.
- Remaining boundary: app-plan follow-up work still owns app-only authority,
  taxonomy, sessionization, journal/read-model, portal, approval, policy,
  child UX, broad blocking, AI digest, install/purchase, performance, E2E, and
  rollout slices.

## Manual-Required Gaps

Planning docs do not upgrade capability status. Product status changes require
proof-backed updates in the owning feature docs and product checklist.
