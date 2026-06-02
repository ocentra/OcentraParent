# 21 Extension And Native Host Boundary

## Where We Are

Current docs keep extension/native-host support optional. The default product
path is managed browser launch/profile plus browser-supported local bridge.

## Where We Want To Be

If added, extension/native-host support is managed-profile-only, permissioned,
origin-validated, schema-validated, heartbeat-monitored, and separately proved.

## Scope

- Extension install/enabled/disabled/permission-required/native-host-missing.
- Minimum permissions for URL/title/tab state.
- Optional managed-profile-only runtime signal observation such as active tab,
  canvas/WebGL/fullscreen/pointer-lock/gamepad events only after separate
  permission and proof.
- Native messaging host origin validation.
- Length-prefixed JSON schema validation.
- Managed session/profile binding.
- Service worker sleep/heartbeat stale state.
- No unmanaged personal profile capture.

## Touched Paths

- `packages/activity-domain/src/browser*.ts`
- `crates/agent-core/src/browser_bridge_*.rs`
- browser extension/native host package paths only when created.

## Tests And Proof

- Contract tests for extension states.
- Security tests for origin, permission, stale heartbeat, schema invalid.
- Manual browser extension install proof.
- Runtime signal proof if extension/native-host becomes the route for browser
  game evidence.

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

No extension security or active-tab claim until extension packaging/distribution
and native-host proof exist.
No browser-game runtime signal claim should depend on extension/native-host
events until that path is separately proved.
