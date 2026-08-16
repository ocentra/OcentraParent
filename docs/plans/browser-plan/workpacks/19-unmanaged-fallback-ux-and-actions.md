# 19 Unmanaged Fallback UX And Actions

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `19 Unmanaged Fallback UX And Actions`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Where We Are

Current contracts and proofs separate unmanaged process fallback from exact URL
control and now expose parent-visible unmanaged fallback action states without
claiming unmanaged exact URL evidence.

## Where We Want To Be

Parents can choose and understand unmanaged browser posture without seeing fake
exact URL claims.

## Production-code pass (2026-08-16)

The read-model fallback projection now remains `unavailable` when the latest
intervention is managed, when no intervention has been observed, or when the
unmanaged enforcement provider is unsupported. Block, redirect, and time-limit
actions on an unmanaged boundary remain `os-block-manual-required` rather than
falling through to report-only. This is a code-drafted,
tests/proof/checklist-deferred slice and does not claim OS blocking, process
termination, relaunch, or exact unmanaged URL control.

Owning production paths:

- `crates/agent-core/src/activity_store_browser_intervention.rs`
- `crates/agent-core/src/activity_store_browser_intervention_fallback_action.rs`
- `crates/agent-core/src/activity_store_browser_intervention_fallback_top_level.rs`

## Scope

- Report only.
- Warn child.
- Ask parent.
- Terminate unmanaged process.
- Relaunch managed browser.
- OS block configured.
- OS block unavailable/manual-required.
- Allowed unmanaged exception.
- Require managed browser for social account creation or social feed/video
  routes.
- Require managed browser for browser-game portals, cloud-gaming, game
  account/purchase routes, and exact game evidence.
- Audit/action state.

## Touched Paths

- `packages/activity-domain/src/browser-intervention*.ts`
- `packages/parent-domain/src/browser-control-policy.ts`
- `packages/parent-domain/src/browser-control-baseline-manifest.ts`
- `crates/agent-service/src/browser_policy_runtime*.rs`
- `crates/agent-protocol/src/browser_intervention*.rs`
- `crates/agent-core/src/activity_store_browser_intervention*.rs`
- `apps/portal/src/browser-intervention-panel.ts`
- `apps/portal/src/portal-browser-route-panels.ts`

## Tests And Proof

- Policy compile tests.
- Service action-state tests.
- Portal read-model parser tests for unmanaged fallback action state.
- Social-platform-in-unmanaged-browser and browser-game-in-unmanaged-browser
  fallbacks are contract/read-model states in this workpack; UI polish and
  real child browser flow snapshots remain later C/UX work.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing source layout inspected; no parallel browser truth created.
- [ ] Before-state source snapshot recorded in `output/browser-plan-proof/19-unmanaged-fallback-ux-and-actions/00-source-snapshot.md`.
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

Terminate/relaunch can be proved narrowly; OS block needs AppLocker/App Control
or platform-specific proof. Unmanaged social use remains app/process-level
bypass, not exact account/feed proof. Unmanaged browser-game use remains
app/process-level bypass, not exact game/runtime proof.
