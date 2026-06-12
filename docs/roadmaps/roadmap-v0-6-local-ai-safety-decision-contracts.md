<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.6 Local AI Safety Decision Contracts Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# V0.6 Local AI Safety Decision Contracts Expectations

This is the milestone-specific expectation file for V0.6 in `docs/product-roadmap.md`.

Supporting expectation files: [AI](../expectations/ai.md), [policy](../expectations/policy.md), [contracts](../expectations/contracts.md), [evidence storage](../expectations/evidence-storage.md), [platforms](../expectations/platforms.md), and [platform deliverables](../expectations/platform-deliverables.md).

## Outcome

- Child-device local AI, policy, parent/family/device, rule, schedule, permission, runtime, memory, graph, and decision-event shapes are contract-first.
- Rust protocol parity exists before Rust stores, emits, or consumes the shapes.
- Remote/API AI remains outside child-device safety decisions.
- Local AI contracts are platform-neutral and consume typed evidence refs plus
  explicit platform capability/degraded states.

## Acceptance

- Invalid policies, invalid local AI inputs, invalid local AI outputs, and unsupported/degraded runtime states are rejected at boundaries.
- Local AI inputs reference typed stored evidence, parent rules, local runtime status, and optional evidence-backed memory or graph refs.
- Policy decision events can be journaled, but no blocking behavior exists yet.
- Windows, macOS, Linux, Android, and iOS differences are represented as
  capability states or unavailable evidence, not hidden assumptions in the AI
  input shape.

## Validation

- Run `npm run validate`.
- Include TypeScript schema tests and Rust parity tests for every Rust-crossing local AI/policy shape.
- Include contract coverage that proves missing/degraded platform evidence is a
  first-class input state before any AI result can depend on it.
