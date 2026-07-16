# 03 Browser Inventory Model

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `03 Browser Inventory Model`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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

2026-06-04 codex-d progress: the inventory row contract now carries
publisher-signature and file-hash evidence refs, and Rust protocol/service
payload proof preserves those refs for unmanaged process-derived rows. Focused
contract tests now cover mixed managed, unmanaged, and unsupported catalog rows
and reject empty identity refs. This is still reference-level proof only: live
OS scanning, live publisher/signature/hash extraction, portal dashboard
consumption, and manual platform artifacts remain outside this sub-slice.

2026-06-06 codex-d completion: the browser inventory model is now proof-gated as
complete by `scripts/test/browser-inventory-model-completion-proof.mjs`. The
gate verifies the WP03 contract/Rust/service proof pack, the WP04 live Windows
inventory proof with redacted registry/process/file-hash/signature evidence,
and the WP14 parent portal Browser-route inventory screenshot/Playwright proof.
This closes the inventory model row without claiming exact URL, known active
tab, browser blocking, enforcement, or cross-platform adapter completion.

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
- Contract fixtures for mixed inventory and identity refs.
- Portal fixtures for mixed inventory remain deferred.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing source layout inspected; no parallel browser truth created.
- [ ] Before-state source snapshot recorded in `output/browser-plan-proof/03-browser-inventory-model/00-source-snapshot.md`.
- [ ] Contracts updated first where this workpack changes behavior.
- [ ] Rust/service parity updated only after contracts exist; portal parity remains deferred because no UI surface changed.
- [ ] Raw evidence artifacts captured or marked N/A for this service-derived sub-slice: existing managed status fixtures, unmanaged process rows, and missing-browser status feed the read model; no journal, SQLite, policy, or action behavior changed.
- [ ] Tests/proof listed in this workpack are implemented for contract and service row derivation; mixed inventory and identity-ref contract proof now exists, while live OS scanner extraction, portal fixtures, and manual platform proof remain manual-required.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots captured for every touched parent portal, child UX, block/warn, policy authoring, or dashboard state; no UI changed, so `ui-not-applicable.md` records why.
- [ ] Security/no-claim negative proof captured for this sub-slice: unmanaged rows cannot claim managed exact URL and managed target-list rows cannot claim active-tab support.
- [ ] Manual platform proof captured for real browser/OS claims; no new real OS/browser claim was made, so `09-manual-platform-proof.md` records the N/A boundary.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

Inventory does not prove URL visibility or blocking capability by itself. This
workpack is complete for the shared inventory model because contract,
Rust/service, live Windows inventory, and parent portal consumption proof now
exist. Remaining browser inventory adapter work stays in WP04 and WP05:
Rust `.lnk` binary parsing, AppX/MSIX enumeration, non-Windows platform
adapters, and platform-specific manual proof. Exact URL, active-tab certainty,
policy actions, blocking, rollback, and enforcement stay in their separate
browser-plan workpacks.
