# V0.6 Local AI Safety Decision Contracts Expectations

This is the milestone-specific expectation file for V0.6 in `docs/product-roadmap.md`.

Supporting expectation files: [AI](ai.md), [policy](policy.md), [contracts](contracts.md), [evidence storage](evidence-storage.md), [platforms](platforms.md), and [platform deliverables](platform-deliverables.md).

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
