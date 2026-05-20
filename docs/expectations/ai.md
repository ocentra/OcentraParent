# AI Feature Expectations

AI features assist classification, explanation, and recommendations. They do not replace evidence.

## Expected Deliverables

- AI input contract.
- AI output contract.
- Model/provider adapter boundary.
- Prompt/version ownership.
- Evidence references.
- Confidence/unknown state.
- Failure/degraded behavior.
- Human override feedback path where relevant.

## Acceptance

- AI output is schema-validated.
- AI output points to stored evidence.
- Unknown or failed classification is safe and explicit.
- Policy can explain when AI contributed and when it did not.
- Tests cover parser behavior and decision integration without mocking provider truth.

## Non-Goals

- Do not claim AI can see content that was not captured.
- Do not let untraceable AI output directly enforce blocking.
- Do not hide model/provider calls inside unrelated modules.
- Do not make cloud AI mandatory for local-only development.

## Done Signal

AI can classify or explain a narrow evidence-backed case, return a typed output, degrade safely on failure, and keep policy decisions auditable.
