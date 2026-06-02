# 09 CDP Version And Target Adapter

## Where We Are

`poll_chromium_bridge` reads `/json/version` and `/json/list`, filters page
targets, maps them into activity events, and marks active state unknown.

## Where We Want To Be

The CDP adapter is fixture-backed, malformed-payload-safe, redaction-safe, and
explicit about tab-list-only capability.

## Scope

- `/json/version` parser.
- `/json/list` parser.
- Page target filtering.
- Internal/blank page handling.
- Missing URL and malformed payload rejection.
- Bridge-missing, timeout, invalid JSON, and adapter-error states.
- Redacted debugger endpoint handling.

## Touched Paths

- `crates/agent-core/src/browser_bridge_poll.rs`
- `crates/agent-core/src/browser_bridge_http.rs`
- `crates/agent-core/src/browser_bridge_tests.rs`
- `crates/agent-core/src/browser_bridge_poll_tests.rs`

## Tests And Proof

- Fake HTTP server tests.
- CDP fixture parser tests.
- Oversized/malformed payload tests.

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

CDP target visibility does not prove active tab unless separate proof exists.
