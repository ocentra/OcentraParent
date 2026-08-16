<!-- agent-capsule -->

> Agent Capsule
> Doc: Activity Surface Service Adapter Handoff
> Kind: architecture/reference documentation; read only when selected by plan route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Activity Surface Service Adapter Handoff

The C-owned Activity UI consumes Rust-owned route snapshots and service-backed
data without owning product data.

Current Rust-first ownership means Activity report, save/history, and tab
read-model commands must be owned by `crates/schema`,
`crates/parent-runtime-core`, or the relevant Rust domain/runtime crate before
the UI consumes them. Any `@ocentra-parent/agent-protocol-domain` adapter is a
temporary migration surface over generated Rust-owned DTOs, not product truth.
The UI renders typed payloads or explicit adapter failure reasons and must not
fall back to UI-check data.

Runtime source of truth remains:

1. Activity UI sends a generated Rust-owned action through HostBridge or the
   dev bridge.
2. Rust service reads the local Activity query store or saved report store.
3. Rust service reports typed unavailable, empty, offline, or ready states.
4. Portal renders the typed result; Vite/dev web does not invent product data.

Current handoff proof:

- `apps/portal/src/live-activity-state.ts` resolves service events into the
  Activity service UI spine.
- `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/activity-ui-intent.ts`
  maps successful adapter results into device slots/report files and keeps
  failed adapter results unavailable.
- `apps/portal/tests/live-activity-surface-adapter.test.ts` proves report,
  history, and Screen/App Use/Browser/Games/Network tab event parsing.
- `apps/portal/tests/activity-ui-intent.test.ts` proves the UI intent layer
  consumes service-backed adapter results without creating fixture-backed
  devices.
