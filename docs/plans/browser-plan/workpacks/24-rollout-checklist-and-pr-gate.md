# 24 Rollout, Checklist, And PR Gate

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
- `docs/product-capability-checklist.md`
- `docs/product-roadmap.md`
- touched package/crate/app READMEs when implementation changes.
- `docs/plans/browser-plan/**`

## Tests And Proof

- `git diff --check`.
- Focused workpack tests.
- `npm run validate` or explicit omission record before PR-ready claims.
- Lane/hub guards.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing source layout inspected; no parallel browser truth created.
- [ ] Before-state source snapshot recorded in `output/browser-plan-proof/<workpack-id>/00-source-snapshot.md` or explicit docs-only N/A reason.
- [ ] Contracts updated first where this workpack changes behavior.
- [ ] Rust/service/portal parity updated only after contracts exist.
- [ ] Raw evidence artifacts captured where applicable: bridge/CDP payloads, managed session state, unmanaged process rows, journal entries, SQLite/read-model rows, policy decisions, and action results.
- [ ] Tests/proof listed in this workpack are implemented or explicitly marked manual-required with reason.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots captured for every touched parent portal, child UX, block/warn, policy authoring, or dashboard state; if no UI changed, `ui-not-applicable.md` records why.
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
