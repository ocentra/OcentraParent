<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.5 Live Activity Portal Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# V0.5 Live Activity Portal Expectations

This is the milestone-specific expectation file for V0.5 in `docs/product-roadmap.md`.

Supporting expectation files: [portal](../expectations/portal.md), [evidence storage](../expectations/evidence-storage.md), [contracts](../expectations/contracts.md), and [documentation](../expectations/documentation.md).

## Outcome

- The local parent visibility surface connects to the real Rust service and shows health, evidence-store, activity, source, and diagnostics state.
- Portal controls send typed intents or queries and never execute child-device work directly.
- Copy/debug output is useful for handoff without exposing secrets or raw private content.

## Acceptance

- One primary result/timeline/table surface updates predictably instead of appending fake cards.
- Empty, loading, stale, degraded, and failure states are visible and not confused with successful data.
- Browser-visible warnings/errors on touched portal routes are treated as product issues.

## Validation

- Run `npm run validate`.
- Include Playwright coverage against the real service path plus browser console checks for touched routes.
