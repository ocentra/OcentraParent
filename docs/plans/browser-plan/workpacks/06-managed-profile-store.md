# 06 Managed Profile Store

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `06 Managed Profile Store`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Where We Are

Canonical source has Browser-managed profile contracts and filesystem helpers,
but it does not have an authenticated custody owner or an atomic, recoverable
profile lifecycle. Source-only branch
`codex/browser-wp06-repair-round3-aug24` at `5671c06a2` was independently
rejected on 2026-08-24 and is not integrated. Its plain JSON authority remains
same-user forgeable, returned records retain only paths rather than owner-bound
handles, portable no-follow coverage is incomplete, and create/delete/recovery
remain fail-closed stubs.

## Where We Want To Be

Managed profiles are Ocentra-owned, per child/device/browser, durable,
repairable, redacted in UI, and rejected when unsafe.

## Scope

- Profile id and profile path ref.
- Ocentra-owned profile root.
- Per child/device/browser scoping.
- Create, load, repair, delete, missing-profile state.
- Default profile rejection.
- Portal redaction.
- Custody label and policy revision.

## Touched Paths

- `crates/agent-core/src/browser_managed_session.rs`
- `crates/agent-core/src/browser_managed_session/store.rs`
- `crates/agent-core/src/browser_managed_session/store/`
- `crates/agent-protocol/src/browser_managed.rs`
- `crates/agent-service/src/browser_runtime_paths.rs`

## Tests And Proof

- `crates/agent-protocol/tests/contract/browser_managed_profile_store.rs`:
  strict wire/schema and non-authority contract cases.
- `crates/agent-core/tests/unit/browser_managed_profile_store_authority.rs`:
  forged/copy/replay/mismatched owner receipt and identity substitution.
- `crates/agent-core/tests/unit/browser_managed_profile_store_path_custody.rs`:
  link/reparse escape, handle identity, and replacement/TOCTOU negatives.
- `crates/agent-core/tests/unit/browser_managed_profile_store_recovery.rs`:
  concurrent mutation, interrupted replacement, restart, deletion, and
  corruption recovery.
- `crates/agent-service/tests/integration/browser_managed_profile_store_runtime.rs`:
  real protected-owner adapter/caller composition and fail-closed unavailable
  state.

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

## Independent Source Decision (2026-08-24)

The first bounded repair is rejected for production integration:

- `BrowserManagedProfileStoreEntry` is unauthenticated JSON. A same-user writer
  can preserve attacker-readable constants, replace a profile directory, and
  fabricate a `Ready` or `Deleted` record.
- `BrowserManagedProfileStoreRecord` returns `PathBuf` values after dropping its
  directory guard, so identity/custody is not retained through use.
- no-follow handling does not cover Windows, every supported Unix target, and
  unsupported platforms with a handle-bound open; inspection and open remain
  replaceable.
- metadata replacement, directory creation, staging, rename, and deletion
  return `UnsafePath`, so the claimed atomic/restart-recoverable lifecycle is
  not implemented.
- no production caller constructs the private configuration or consumes the
  repaired store.

The repair must keep `Ready` and terminal deletion unavailable until a
dependency-owned protected-custody owner provides an authenticated receipt/key
and retained root/profile identity. Each supported platform needs real safe
mutation/open custody; unsupported platforms must reject rather than fall back
to path-only checks. This source decision writes no test, proof, runtime,
PR_READY, or `DONE` claim.

## Manual-Required Gaps

Profile existence does not prove owner custody, browser launch, bridge
connectivity, exact URL evidence, parent policy authority, or product runtime
composition. WP07 remains blocked on an accepted WP06 owner boundary.
