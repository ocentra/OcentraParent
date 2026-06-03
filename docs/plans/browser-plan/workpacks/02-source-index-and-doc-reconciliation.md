# 02 Source Index And Doc Reconciliation

Sources: [source index](../source-index.md) and
[current snapshot](../current-browser-snapshot.md).

## Where We Are

Browser information is spread across the feature doc, expectation docs,
architecture docs, policy catalogs, root-level planning docs, checkpoints,
package READMEs, and proof scripts.

## Where We Want To Be

This plan folder becomes the working implementation plan while the existing
feature, expectation, architecture, roadmap, checklist, and README files remain
the source-of-truth layers for product status and ownership.

## Scope

- Keep this folder indexed against all existing browser docs.
- Avoid bulk-moving source docs that other docs already link to.
- Add/update feature/checklist/roadmap only when implementation status or proof
  changes.
- Keep old checkpoint wording historical, not current product truth.

## Touched Paths

- `docs/plans/browser-plan/README.md`
- `docs/plans/browser-plan/source-index.md`
- `docs/plans/browser-plan/current-browser-snapshot.md`
- `docs/plans/browser-plan/workpacks/02-source-index-and-doc-reconciliation.md`
- `docs/plans/browser-plan/implementation-checklist.md`
- `output/browser-plan-proof/02-source-index-and-doc-reconciliation/`
- Feature/checklist/roadmap docs only when status changes. No product status
  change is made for WP02.

## Tests And Proof

- Link/source-index review by reading the browser-plan README, source index,
  current snapshot, full scope plan, test blueprint, UI/UX guide, implementation
  checklist, and this workpack.
- `cmd /c npx prettier --write docs/plans/browser-plan/README.md docs/plans/browser-plan/source-index.md docs/plans/browser-plan/current-browser-snapshot.md docs/plans/browser-plan/workpacks/02-source-index-and-doc-reconciliation.md docs/plans/browser-plan/implementation-checklist.md output/browser-plan-proof/02-source-index-and-doc-reconciliation/*.md`
- `cmd /c npm run format:check`
- `git diff --check`

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [x] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [x] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [x] Hub lock covers this workpack and exact implementation/docs paths.
- [x] Existing source layout inspected; no parallel browser truth created.
- [x] Before-state source snapshot recorded in `output/browser-plan-proof/02-source-index-and-doc-reconciliation/00-source-snapshot.md`.
- [x] Contracts updated first where this workpack changes behavior; WP02 is doc reconciliation only and adds no runtime contract.
- [x] Rust/service/portal parity updated only after contracts exist; no Rust/service/portal runtime code changed for WP02.
- [x] Raw evidence artifacts captured where applicable: WP02 indexes proof-script docs and records docs-only N/A.
- [x] Tests/proof listed in this workpack are implemented or explicitly marked manual-required with reason.
- [x] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [x] UI snapshots captured for every touched parent portal, child UX, block/warn, policy authoring, or dashboard state; no UI changed, and `ui-not-applicable.md` records why.
- [x] Security/no-claim negative proof captured where applicable: no default profile attach, no unowned bridge, no unmanaged exact URL claim, no raw debugger URL exposure, and no AI direct enforcement.
- [x] Manual platform proof captured for real browser/OS claims, including OS/browser version, command steps, screenshots/logs, and manual-required labels.
- [x] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [x] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [x] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

None. This is documentation coordination only.
