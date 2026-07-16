# 02 Source Index And Doc Reconciliation

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `02 Source Index And Doc Reconciliation`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing source layout inspected; no parallel browser truth created.
- [ ] Before-state source snapshot recorded in `output/browser-plan-proof/02-source-index-and-doc-reconciliation/00-source-snapshot.md`.
- [ ] Contracts updated first where this workpack changes behavior; WP02 is doc reconciliation only and adds no runtime contract.
- [ ] Rust/service/portal parity updated only after contracts exist; no Rust/service/portal runtime code changed for WP02.
- [ ] Raw evidence artifacts captured where applicable: WP02 indexes proof-script docs and records docs-only N/A.
- [ ] Tests/proof listed in this workpack are implemented or explicitly marked manual-required with reason.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots captured for every touched parent portal, child UX, block/warn, policy authoring, or dashboard state; no UI changed, and `ui-not-applicable.md` records why.
- [ ] Security/no-claim negative proof captured where applicable: no default profile attach, no unowned bridge, no unmanaged exact URL claim, no raw debugger URL exposure, and no AI direct enforcement.
- [ ] Manual platform proof captured for real browser/OS claims, including OS/browser version, command steps, screenshots/logs, and manual-required labels.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

None. This is documentation coordination only.
