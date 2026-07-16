# 26 Performance And Service Health

Sources: [test blueprint](../v0-5-native-apps-test-blueprint.md),
[current snapshot](../current-app-snapshot.md), and
`docs/expectations/evidence-storage.md`.

## Where We Are

App/game observation and portal read models exist, but expanded native app
inventory, runtime polling, foreground events, journaling, replay, policy, and
portal rendering need bounded performance proof.

## Where We Want To Be

Native app evidence collection and read-model rendering stay bounded, stable,
observable, and degraded-state aware under normal and stressed local conditions.

## Scope

- Inventory scan cadence and limits.
- Process/runtime polling cadence and limits.
- Foreground event cadence and debouncing.
- Journal/SQLite write and replay bounds.
- Policy compile bounds.
- Portal render bounds for large app catalogs.
- Service health, adapter_error, stale, and degraded rows.

## Touched Paths

- `packages/parent-domain/src/app-game-performance-health.ts`
- `packages/parent-domain/src/app-game-performance-health-rules.ts`
- `packages/parent-domain/src/app-game-performance-health-proof.ts`
- `packages/parent-domain/tests/app-game-performance-health.test.ts`
- `scripts/test/app-game-performance-health-proof.mjs`
- `output/app-plan-proof/26-performance-and-service-health`
- Cross-recorded shared proof:
  `output/app-game-plan-proof/27-performance-and-service-health`

## Tests And Proof

- Inventory normalize 1,000 apps under target threshold.
- Runtime snapshot normalize 500 processes under target threshold.
- Session replay 100,000 observations under target threshold.
- Portal renders 500 apps without freezing.
- Policy compile 1,000 app rules under target threshold.
- Degraded adapter state is visible.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-app-snapshot.md), [full scope plan](../v0-5-native-apps-full-scope-plan.md), [platform deep dive](../v0-5-native-apps-platform-deep-dive.md), [test blueprint](../v0-5-native-apps-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Confirm this is native/installed-app scope, not browser pages, browser games, or game-specific product semantics beyond the shared app/game evidence spine.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing app/game source layout inspected; no parallel app-control truth created.
- [ ] Before-state source snapshot recorded in `output/app-plan-proof/26-performance-and-service-health/00-source-snapshot.md`.
- [ ] Contracts updated first where this workpack changes behavior.
- [ ] Rust/service/portal parity is not changed by this row; the proof records generated-scale and existing portal intent smoke only.
- [ ] Raw evidence artifacts captured for inventory rows, runtime rows, foreground transitions, journal records, replay observations, policy compile parses, portal intent rows, and degraded adapter health.
- [ ] Tests/proof listed in this workpack and [test blueprint](../v0-5-native-apps-test-blueprint.md) are implemented or explicitly marked manual-required with reason.
- [ ] Required fixtures are present or N/A with reason for inventory, runtime, foreground, session, policy, enforcement, UI, stale state, and manual-required state.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots are not applicable because no UI source changed; `ui-not-applicable.md` records why.
- [ ] Security/no-claim negative proof captured for no live adapter execution, no live platform throughput claim, and no browser DOM render claim.
- [ ] Manual platform proof is recorded as not applicable because this row does not claim stronger-than-observe live platform support.
- [ ] Platform limitations use capability status language and keep unsupported live behavior manual-required or not-claimed.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

CI/generated performance proof does not replace real host/manual platform proof
for adapter-specific behavior. Live OS throughput, encrypted journal disk
throughput and corruption/recovery, browser DOM/Playwright render proof, live
platform adapters, install/store approval, and broad app blocking remain gaps.
