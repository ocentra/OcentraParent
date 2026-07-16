# screen-ai-pipeline-plan Event Architecture Instruction

## Owns

- screen-AI runtime pipeline, cadence, analysis, retention sweeper, event bridge, parser/degrade rules, portal explanation chain where assigned.

## Must not own

- raw screen capture proof owned by screen-plan;
- AI core/provider proof owned by AI plan;
- enforcement action authority;
- parent-domain screen/AI wrappers.

## Required chain

```text
screen evidence/read model
-> screen-AI service creates analysis request
-> AI runtime returns typed result
-> screen-AI validates/degrades result
-> journal/read model stores explanation
-> portal renders service-backed explanation
```

## Logging/proof

Log prerequisite artifact ids, analysis request, provider result, parser decision, invalid-output fallback, retention/deletion result, portal read-model update, and dry-run/no-action boundary.

## Tests

Move runtime-grade Rust screen-AI tests from `src` into crate `tests/`. TS proof helpers in `src` do not count as tests. Playwright proof requires retained screen and AI artifacts.

## First architecture slice

Run proof/test normalization and architecture cleanup only. Do not run broad proof until screen-plan and AI-plan publish retained prerequisite artifacts.
