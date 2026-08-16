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

`crates/agent-core/src/browser_managed_session.rs` rejects default/unowned paths
in launch planning, but the complete profile store lifecycle is not yet a
product-ready subsystem.

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
- `crates/agent-service/src/browser_runtime_paths.rs`
- `packages/activity-domain/src/browser*.ts`
- `crates/agent-protocol/src/browser_managed.rs`

## Tests And Proof

- Temp-directory integration tests.
- Default profile rejection tests.
- Restart/reload metadata tests.
- Portal DTO redaction tests.

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

## Implementation Note

WP06 is complete on `codex/browser-plan-implementation` with proof under
`output/browser-plan-proof/06-managed-profile-store/`.

Implemented:

- `@ocentra-parent/activity-domain` managed profile store contracts in
  `browser-managed-profile-store.ts`, plus a package export for direct imports.
- Redacted profile summary fields on `BrowserManagedSessionStatusSchema`:
  profile root ref, profile scope id, profile lifecycle state, and policy
  revision.
- Rust protocol parity for `BrowserManagedProfileStoreEntry` and
  `BrowserManagedProfileLifecycleState`.
- Core filesystem lifecycle helpers for create, load, repair, delete, missing
  state, default-profile rejection, and unowned-profile rejection.
- Service runtime path resolution through the profile store before managed
  launch planning; service payloads expose only redacted refs and lifecycle
  labels.

Validation:

- `cmd /c npm run type-check --workspace @ocentra-parent/activity-domain`
- `cmd /c npm run test --workspace @ocentra-parent/activity-domain -- browser.test.ts browser-managed-profile-store.test.ts browser-platform-inventory-matrix.test.ts browser-inventory.test.ts`
- `cargo fmt --package ocentra-parent-agent-core --package ocentra-parent-agent-service --package ocentra-parent-agent-protocol`
- `cargo test -p ocentra-parent-agent-protocol browser_managed`
- `cargo test -p ocentra-parent-agent-core managed_browser`
- `cargo test -p ocentra-parent-agent-service browser_runtime`
- `cmd /c npm run lint:schema-boundaries`

No product capability checklist update was needed: this proves owned profile
store lifecycle and redacted service DTOs, but does not upgrade real browser
launch, bridge connectivity, exact URL evidence, or a user-visible product
capability row.

## Manual-Required Gaps

Profile existence does not prove browser launch, bridge connectivity, or exact
URL evidence.
