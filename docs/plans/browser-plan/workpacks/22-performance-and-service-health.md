# 22 Performance And Service Health

## Where We Are

Browser bridge polling and proof scripts exist, but the full browser subsystem
needs explicit performance and nonblocking service-health gates.

## Where We Want To Be

Browser inventory, bridge polling, evidence mapping, journal writes, SQLite
replay, unmanaged process scans, URL/video intelligence queues, local AI
provider routing, browser-game runtime signals, cloud-gaming queues, and portal
rendering stay bounded.

## Scope

- Inventory scan latency.
- Support matrix derivation latency.
- CDP target mapping for 100 tabs.
- Journal write time per event.
- SQLite replay of 10,000 events.
- Portal render of 100 tabs.
- Unmanaged process scan budget.
- Rapid bridge connect/disconnect handling.
- URL shape and metadata extraction latency.
- Local AI queue priority and timeout behavior.
- Memory/cache lookup and invalidation cost.
- Browser-game runtime signal collection for canvas/WebGL/gamepad/fullscreen.
- Cloud-gaming bandwidth/session heuristics and timeout behavior.

## Touched Paths

- `crates/agent-core/src/browser_*.rs`
- `crates/agent-service/src/browser_runtime*.rs`
- `apps/portal/src/*browser*.ts`
- proof/performance scripts when added.

## Tests And Proof

- Fixture-based performance tests.
- Service health checks.
- Portal render stress tests.
- Provider timeout/degraded-state tests when intelligence starts.
- Browser-game runtime signal and cloud-gaming performance tests when those
  paths start.

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

Performance targets may need hardware-specific manual proof before release.
