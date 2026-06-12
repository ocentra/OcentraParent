<!-- agent-capsule -->

> Agent Capsule
> Doc: V3 Notifications Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# V3 Notifications Expectations

This is the milestone-specific expectation file for V3 in `docs/product-roadmap.md`.

Supporting expectation files: [notifications](../expectations/notifications.md), [policy](../expectations/policy.md), and [evidence storage](../expectations/evidence-storage.md).

## Outcome

- Parents receive a small number of timely, explainable alerts when attention is needed.
- Notification delivery crosses provider boundaries with minimal content by default.
- Notification intents reference evidence and policy ids without duplicating raw evidence.

## Acceptance

- Alert rules, reason codes, delivery channels, attempts, results, retry state, quiet hours, escalation, parent preference, and audit events are typed.
- Alert bodies minimize child detail and route sensitive drill-in through authenticated parent surfaces.
- Provider failure is visible, retryable where safe, and never disables local child-device safety decisions.

## Validation

- Run `npm run validate`.
- Include contract tests, provider-boundary tests, notification intent integration tests, and parent-surface preference/history coverage when UI exists.
