# 22 Performance And Service Health

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `22 Performance And Service Health`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing source layout inspected; no parallel browser truth created.
- [ ] Before-state source snapshot recorded in `output/browser-plan-proof/22-performance-and-service-health/00-source-snapshot.md`.
- [ ] Contracts updated first where this workpack changes behavior.
- [ ] Rust/service/portal parity updated only after contracts exist.
- [ ] Raw evidence artifacts captured where applicable: WP22 uses fixture/read-model proof only and records unimplemented runtime paths as manual-required.
- [ ] Tests/proof listed in this workpack are implemented or explicitly marked manual-required with reason.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots captured for every touched parent portal, child UX, block/warn, policy authoring, or dashboard state; no UI changed, and `ui-not-applicable.md` records why.
- [ ] Security/no-claim negative proof captured where applicable: no default profile attach, no unowned bridge, no unmanaged exact URL claim, no raw debugger URL exposure, and no AI direct enforcement.
- [ ] Manual platform proof captured for real browser/OS claims, including OS/browser version, command steps, screenshots/logs, and manual-required labels.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Managed Runtime Test-Truth Correction — 2026-08-27

PR #709 withdrew the managed-ready, running, bridge-disconnected, and connected
agent-service test cases because they constructed private
`BrowserManagedProfileStoreEntry` or `BrowserManagedLaunch` owner authority.
The mapped `browser_runtime_status.rs`, `browser_runtime_tests.rs`, and
`browser_inventory_read_model_tests.rs` roots remain valid for their retained
fail-closed missing/profile-missing/error states, unmanaged observation,
payload and empty-inventory behavior, and direct inventory/policy models. They
do not prove owner-issued managed profile or launch composition.

The production profile store still returns
`ProtectedCustodyAdapterUnavailable`. Owner-backed managed lifecycle and rapid
connect/disconnect health coverage therefore remain manual-required. Keep
`crates/agent-service/tests/integration/browser_managed_runtime.rs` and
`crates/agent-core/tests/integration/browser_bridge_managed_launch.rs`
missing/open until a real protected owner adapter and launch authority exist.
Do not replace that authority with public constructors, fixtures, or a fake
harness.

## Manual-Required Gaps

Performance targets still need hardware-specific manual proof before release.
Portal 100-tab rendering, URL/video metadata extraction, local AI queue
timeouts, browser-game runtime signals, and cloud-gaming heuristics stay
manual-required until those runtime paths have real platform/provider/UI
artifacts.
