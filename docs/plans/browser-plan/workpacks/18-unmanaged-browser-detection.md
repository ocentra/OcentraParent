# 18 Unmanaged Browser Detection

## Where We Are

The service can report an unmanaged browser process fallback in managed status,
and Windows managed/unmanaged proof scripts cover scoped process behavior.

## Where We Want To Be

Browser-like processes outside current managed sessions are recorded as
unmanaged/possible-bypass evidence with no exact URL fields.

## Scope

- Supported browser outside managed session.
- Unsupported browser.
- Portable browser.
- Tor/privacy browser.
- Packaged browser.
- Embedded/unknown browser-like process.
- Social platform opened outside managed session as possible bypass.
- Browser-game portal or cloud-gaming surface opened outside managed session as
  possible bypass.
- Process id, name, path/signature/hash refs, family guess, confidence, reason,
  timestamp.

## Touched Paths

- `crates/agent-core/src/browser_managed_discovery.rs`
- `crates/agent-service/src/browser_runtime_status.rs`
- `packages/activity-domain/src/browser*.ts`
- `scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs`

## Tests And Proof

- Fake process snapshot tests.
- Contract tests forbidding exact URL/tab fields.
- Contract tests forbidding exact social account, route, feed, or video fields
  on unmanaged browser evidence.
- Contract tests forbidding exact browser-game URL, runtime signal, game title,
  account, purchase, or cloud-title fields on unmanaged browser evidence.
- Portal unmanaged bypass fixture tests.

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

Unmanaged detection is evidence of bypass/process use, not the page visited.
It can report possible social bypass, not social account creation proof.
It can report possible browser-game/cloud-gaming bypass, not exact game proof.
