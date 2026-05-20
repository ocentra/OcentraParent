# AI Feature Expectations

AI is a core child-device product layer. The default safety evaluator runs locally on the child device against typed evidence and parent rules. API AI is secondary and may assist with parent reports, unknown classification, and remote summaries after privacy boundaries are explicit.

## Expected Deliverables

- AI input contract.
- AI output contract.
- Local model/provider adapter boundary.
- API model/provider adapter boundary only when a feature explicitly needs remote AI.
- Prompt/version ownership.
- Evidence references.
- Parent rule references.
- Decision action: allow, warn, block, time-limit, ask-parent, or unknown.
- Timer/expiry fields for temporary block or time-limit decisions.
- Confidence/unknown state.
- Failure/degraded behavior.
- Human override feedback path where relevant.

## Acceptance

- Local AI output is schema-validated before any policy or enforcement code consumes it.
- AI output points to stored evidence.
- AI output points to the parent rules it used.
- Unknown or failed classification is safe and explicit.
- Policy can explain the local AI decision, the evidence, and the parent rule context.
- API AI is never required for normal child-device blocking.
- API AI responses cannot override a stricter local parent rule.
- Tests cover parser behavior and decision integration without mocking provider truth.

## Non-Goals

- Do not claim AI can see content that was not captured.
- Do not let untyped or untraceable AI output directly enforce blocking.
- Do not hide model/provider calls inside unrelated modules.
- Do not make cloud/API AI mandatory for local child-device safety.
- Do not upload child activity to API AI without explicit privacy and parent-control boundaries.

## Done Signal

Local AI can evaluate a narrow evidence-backed case, such as a page URL, video link, app, or domain plus parent rules; return a typed allow/warn/block/time-limit/ask-parent/unknown decision; degrade safely on failure; and keep policy and enforcement decisions auditable.
