# 19 Unmanaged Fallback UX And Actions

## Where We Are

Current docs and proofs separate unmanaged process fallback from exact URL
control, but parent-facing UX and action states are not complete.

## Where We Want To Be

Parents can choose and understand unmanaged browser posture without seeing fake
exact URL claims.

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

- `packages/parent-domain/src/browser-control-policy.ts`
- `crates/agent-service/src/browser_policy_runtime*.rs`
- `apps/portal/src/browser-intervention-panel.ts`
- `apps/portal/src/portal-browser-route-panels.ts`

## Tests And Proof

- Policy compile tests.
- Service action-state tests.
- Playwright unmanaged fallback states.
- Playwright states for social-platform-in-unmanaged-browser fallback.
- Playwright states for browser-game-in-unmanaged-browser fallback.

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

Terminate/relaunch can be proved narrowly; OS block needs AppLocker/App Control
or platform-specific proof. Unmanaged social use remains app/process-level
bypass, not exact account/feed proof. Unmanaged browser-game use remains
app/process-level bypass, not exact game/runtime proof.
