# 04 Windows Browser Inventory Adapter

## Where We Are

The repo has Windows-oriented managed browser proof scripts and process
snapshots, but not a full product inventory adapter that reconciles registry,
known paths, AppX/MSIX packages, shortcuts, and running processes.

## Where We Want To Be

Windows inventory can detect supported, candidate, unsupported, packaged,
portable, and block-only browsers without collecting URLs.

## Scope

- Edge Stable/Beta/Dev/Canary.
- Chrome and Chrome for Testing.
- Brave, Vivaldi, Opera, Opera GX, Chromium.
- Firefox Stable/Developer/Nightly as later-adapter unless proved.
- Tor, DuckDuckGo, Arc, portable, AppX/MSIX, and browser-like processes.
- Registry, known paths, Start Menu, AppX/MSIX, process snapshot, signature/hash
  refs.

## Touched Paths

- `crates/agent-core/src/browser_managed_discovery.rs`
- `crates/agent-service/src/browser_runtime*.rs`
- `packages/activity-domain/src/browser*.ts`
- `crates/agent-protocol/src/browser*.rs`

## Tests And Proof

- Fixture-backed Windows inventory parser tests.
- Fake process snapshot integration tests.
- Manual Windows inventory proof artifact.

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

Detecting a browser does not claim exact URL or app-control blocking.
