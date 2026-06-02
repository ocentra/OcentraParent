# 03 Browser Inventory Model

Sources: [full scope plan](../v0-5-managed-browser-full-scope-plan.md) and
[test blueprint](../v0-5-managed-browser-test-blueprint.md).

## Where We Are

Current runtime can report managed status and detect unmanaged process fallback,
but the product inventory model is not yet a complete parent-visible browser
catalog.

## Where We Want To Be

The service can represent installed and running browsers with support tier,
capability flags, reason codes, install type, identity refs, and unmanaged
fallback state.

## Scope

- Browser family, product name, channel, version.
- Install type: system, user, AppX/MSIX, portable, unknown.
- Executable/path/package refs.
- Publisher/signature/hash refs where available.
- Management tier and capability flags.
- Current state and reason codes.

## Touched Paths

- `packages/activity-domain/src/browser*.ts`
- `packages/parent-domain/src/browser-control-*.ts`
- `crates/agent-protocol/src/browser*.rs`
- `crates/agent-service/src/browser_runtime*.rs`

## Tests And Proof

- Unit tests for support matrix and reason-code derivation.
- Contract tests for inventory rows.
- Portal fixtures for mixed inventory.

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

Inventory does not prove URL visibility or blocking capability by itself.
