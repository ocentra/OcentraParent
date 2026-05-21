# V3 Notifications Expectations

This is the milestone-specific expectation file for V3 in `docs/product-roadmap.md`.

Supporting expectation files: [notifications](notifications.md), [policy](policy.md), and [evidence storage](evidence-storage.md).

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
