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

- `packages/activity-domain/src/browser-performance-health-schemas.ts`
- `packages/activity-domain/src/browser.ts`
- `packages/activity-domain/tests/browser-performance-health.test.ts`
- `crates/agent-protocol/src/constants/browser.rs`
- `crates/agent-core/src/browser_performance_health.rs`
- `crates/agent-core/src/browser_performance_health_tests.rs`
- `crates/agent-core/src/lib.rs`
- `scripts/test/browser-performance-health-proof.mjs`
- `docs/plans/browser-plan/workpacks/22-performance-and-service-health.md`
- `docs/plans/browser-plan/implementation-checklist.md`
- `docs/features/browser-web-control.md`
- `docs/expectations/browser-evidence.md`
- `output/browser-plan-proof/22-performance-and-service-health/`

## Tests And Proof

- `cmd /c npm run test --workspace @ocentra-parent/activity-domain -- browser-performance-health.test.ts`
- `cmd /c cargo test -p ocentra-parent-agent-core browser_performance_health`
- `cmd /c npm run build:contracts`
- `cmd /c node --check scripts/test/browser-performance-health-proof.mjs`
- `cmd /c node scripts/test/browser-performance-health-proof.mjs`
- `cmd /c npm run format:check`
- `cmd /c cargo fmt --check`
- `cmd /c npm run lint:schema-boundaries`
- `git diff --check`
- Fixture-gated budgets now cover inventory scan, support-matrix derivation,
  100-tab CDP target mapping, journal write per event, 10000-event SQLite
  replay, unmanaged process scan, rapid bridge reconnect, and memory/cache
  lookup invalidation.
- Manual-required rows remain for portal 100-tab render, URL/video metadata
  extraction, local AI queue timeout, browser-game runtime signal collection,
  and cloud-gaming heuristic timeout until those runtime paths have real
  platform/provider/UI proof.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [x] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [x] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [x] Hub lock covers this workpack and exact implementation/docs paths.
- [x] Existing source layout inspected; no parallel browser truth created.
- [x] Before-state source snapshot recorded in `output/browser-plan-proof/22-performance-and-service-health/00-source-snapshot.md`.
- [x] Contracts updated first where this workpack changes behavior.
- [x] Rust/service/portal parity updated only after contracts exist.
- [x] Raw evidence artifacts captured where applicable: WP22 uses fixture/read-model proof only and records unimplemented runtime paths as manual-required.
- [x] Tests/proof listed in this workpack are implemented or explicitly marked manual-required with reason.
- [x] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [x] UI snapshots captured for every touched parent portal, child UX, block/warn, policy authoring, or dashboard state; no UI changed, and `ui-not-applicable.md` records why.
- [x] Security/no-claim negative proof captured where applicable: no default profile attach, no unowned bridge, no unmanaged exact URL claim, no raw debugger URL exposure, and no AI direct enforcement.
- [x] Manual platform proof captured for real browser/OS claims, including OS/browser version, command steps, screenshots/logs, and manual-required labels.
- [x] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [x] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [x] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

Performance targets still need hardware-specific manual proof before release.
Portal 100-tab rendering, URL/video metadata extraction, local AI queue
timeouts, browser-game runtime signals, and cloud-gaming heuristics stay
manual-required until those runtime paths have real platform/provider/UI
artifacts.
