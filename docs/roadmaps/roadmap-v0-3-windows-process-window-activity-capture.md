<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.3 Windows Process And Window Activity Capture Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# V0.3 Windows Process And Window Activity Capture Expectations

This is the milestone-specific expectation file for V0.3 in `docs/product-roadmap.md`.

Supporting expectation files: [capture](../expectations/capture.md), [evidence storage](../expectations/evidence-storage.md), [contracts](../expectations/contracts.md), [portal](../expectations/portal.md), and [platforms](../expectations/platforms.md).

## Outcome

- Windows process and foreground-window observations are captured as typed evidence without blocking, AI decisions, or content inspection.
- Process/window evidence is journaled, ingested, and queryable before portal display.
- Unsupported, unavailable, access-denied, and degraded states are represented honestly.

## Acceptance

- A real Windows run can observe process/window activity and preserve source, adapter, timestamp, and capability state.
- The system does not claim browser URL, page content, chat content, keystrokes, screenshots, or decrypted traffic from V0.3 evidence.
- Portal rows distinguish observed process/window facts from unknown or unsupported states.

## Validation

- Run `npm run validate`.
- Include focused Rust adapter/mapping tests and a manual Windows local run before claiming parent-visible behavior.
