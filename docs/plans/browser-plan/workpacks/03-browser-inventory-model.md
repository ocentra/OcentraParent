# 03 Browser Inventory Model

Sources: [full scope plan](../v0-5-managed-browser-full-scope-plan.md) and
[test blueprint](../v0-5-managed-browser-test-blueprint.md).

## Where We Are

Current runtime can report managed status and detect unmanaged process fallback,
but the product inventory model is not yet a complete parent-visible browser
catalog.

2026-06-02 codex-d progress: the activity-domain and Rust protocol now define
browser inventory row/read-model contracts, and the Rust service can derive a
single honest inventory row from the existing managed-session status boundary.
This proves the read-model shape for managed target-list sessions, unmanaged
process-only detections, and missing-browser unavailable state. It does not yet
prove installed-browser OS scanning, mixed multi-browser catalog population, or
portal dashboard consumption.

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

- [x] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [x] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [x] Hub lock covers this workpack and exact implementation/docs paths.
- [x] Existing source layout inspected; no parallel browser truth created.
- [x] Before-state source snapshot recorded in `output/browser-plan-proof/03-browser-inventory-model/00-source-snapshot.md`.
- [x] Contracts updated first where this workpack changes behavior.
- [x] Rust/service parity updated only after contracts exist; portal parity remains deferred because no UI surface changed.
- [x] Raw evidence artifacts captured or marked N/A for this service-derived sub-slice: existing managed status fixtures, unmanaged process rows, and missing-browser status feed the read model; no journal, SQLite, policy, or action behavior changed.
- [x] Tests/proof listed in this workpack are implemented for contract and service row derivation; OS scanner, mixed inventory, publisher/signature/hash refs, and portal fixtures remain manual-required.
- [x] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [x] UI snapshots captured for every touched parent portal, child UX, block/warn, policy authoring, or dashboard state; no UI changed, so `ui-not-applicable.md` records why.
- [x] Security/no-claim negative proof captured for this sub-slice: unmanaged rows cannot claim managed exact URL and managed target-list rows cannot claim active-tab support.
- [x] Manual platform proof captured for real browser/OS claims; no new real OS/browser claim was made, so `09-manual-platform-proof.md` records the N/A boundary.
- [x] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [x] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [x] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

Inventory does not prove URL visibility or blocking capability by itself.
Remaining inventory work requires the Windows inventory adapter, cross-platform
inventory matrix, real installed-browser scanning, multi-row catalog fixtures,
publisher/signature/hash evidence refs, portal dashboard consumption, and
manual platform proof before this workpack can be marked complete.
