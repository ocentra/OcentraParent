# 24 Rollout, Checklist, And PR Gate

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `24 Rollout, Checklist, And PR Gate`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Where We Are

The feature doc and product capability checklist already mark browser URL/tab
evidence and managed browser/domain blocking as in-progress/manual-required
where appropriate.

## Where We Want To Be

Browser work lands only with proof-backed product claims, exact documentation
updates, and no overclaim regressions.

## Scope

- Workpack DONE report requirements.
- Feature doc updates when status/proof/gap changes.
- Product capability checklist updates when status/proof/gap changes.
- Roadmap updates only when milestone scope/order/completion changes.
- Module README updates when ownership/flow/gap changes.
- PR body scope, touched paths, validation, known gaps, and roadmap slice.
- Merge blockers from the test blueprint.
- Browser intelligence claims must name evidence refs, model/provider route,
  prompt/version, confidence, policy decision, action capability, and degraded
  states.
- Social platform claims must name platform, route/account/feed target, evidence
  layer, confidence, parent approval state, policy decision, action capability,
  child-facing state, audit refs, and manual-required gaps.
- Browser-game claims must name platform/game portal/cloud surface, evidence
  layer, runtime signals, metadata/AI confidence, policy target, approval or
  time-budget state, action capability, child-facing state, audit refs, and
  manual-required gaps.

## Touched Paths

- `docs/features/browser-web-control.md`
- `docs/expectations/browser-evidence.md`
- `docs/plans/browser-plan/workpacks/24-rollout-checklist-and-pr-gate.md`
- `docs/plans/browser-plan/implementation-checklist.md`
- `output/browser-plan-proof/24-rollout-checklist-and-pr-gate/`
- Product capability checklist, product roadmap, and module READMEs are not
  edited by WP24 because this gate records browser-plan proof and no final
  product-status, milestone-order, or module-ownership change.

## Tests And Proof

- `cmd /c npm run lanes:guard`
- `cmd /c npm run format:check`
- `git diff --check`
- Focused workpack tests from completed base workpacks are referenced in
  `docs/plans/browser-plan/implementation-checklist.md`.
- `npm run validate` is required before a final PR-ready claim, after rebase
  and all remaining enhancement-track scope is complete.
- Hub guard must be rerun before commit/PR after the accumulated dirty browser
  WIP is reconciled under the active lock.
- `cmd /c node scripts/test/browser-plan-closure-audit-proof.mjs` records the
  current browser-plan closure state and fails if any row outside the known
  real-platform blockers remains unchecked or partial.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing source layout inspected; no parallel browser truth created.
- [ ] Before-state source snapshot recorded in `output/browser-plan-proof/24-rollout-checklist-and-pr-gate/00-source-snapshot.md`.
- [ ] Contracts updated first where this workpack changes behavior; WP24 is a rollout gate and adds no new runtime contract.
- [ ] Rust/service/portal parity updated only after contracts exist; no Rust/service/portal runtime code changed for WP24.
- [ ] Raw evidence artifacts captured where applicable: completed base workpack proof paths are summarized in the main checklist.
- [ ] Tests/proof listed in this workpack are implemented or explicitly marked manual-required with reason.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots captured for every touched parent portal, child UX, block/warn, policy authoring, or dashboard state; no UI changed, and `ui-not-applicable.md` records why.
- [ ] Security/no-claim negative proof captured where applicable: no default profile attach, no unowned bridge, no unmanaged exact URL claim, no raw debugger URL exposure, and no AI direct enforcement.
- [ ] Manual platform proof captured for real browser/OS claims, including OS/browser version, command steps, screenshots/logs, and manual-required labels.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

If proof is missing, the checklist and PR report must say manual-required,
unavailable, degraded, scaffold, or not-claimed rather than implying completion.
AI or metadata-only work must not upgrade browser evidence, social/video, or
app/game, or enforcement status rows without matching proof.

WP24 closes the 24 base browser workpack gate only. The AI, social/video, and
browser-game enhancement tracks remain open until their rows have separate
contracts, runtime proof, UI/manual artifacts, and rollout gates.
