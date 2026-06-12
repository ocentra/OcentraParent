<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.2 Trusted Local Evidence Store Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# V0.2 Trusted Local Evidence Store Expectations

This is the milestone-specific expectation file for V0.2 in `docs/product-roadmap.md`.

Supporting expectation files: [data custody](../expectations/data-custody.md), [evidence storage](../expectations/evidence-storage.md), [contracts](../expectations/contracts.md), [portal](../expectations/portal.md), and [static analysis and security](../expectations/static-analysis-security.md).

## Outcome

- The child-device agent can write, rotate, replay, ingest, and query trusted local evidence before capture or enforcement exists.
- Encrypted NDJSON remains the source of truth and SQLite remains the local query/index store.
- Portal visibility comes from the real service path, not fake UI state.

## Acceptance

- Journal entries are encrypted, tamper detection works, and rotated segments replay in order.
- SQLite query state rebuilds from journal replay and handles duplicate ingest safely.
- Recent activity and ingest status are visible through typed service responses.

## Validation

- Run `npm run validate`.
- Include journal crypto, replay, query-store, local WebSocket smoke, and portal E2E evidence in handoff.
