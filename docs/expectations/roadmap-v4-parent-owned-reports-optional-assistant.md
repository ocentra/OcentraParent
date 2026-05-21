# V4 Parent-Owned Reports And Optional Assistant Expectations

This is the milestone-specific expectation file for V4 in `docs/product-roadmap.md`.

Supporting expectation files: [data custody](data-custody.md), [AI](ai.md), [evidence storage](evidence-storage.md), [policy](policy.md), and [contracts](contracts.md).

## Outcome

- Richer parent explanations, reports, Q&A, and optional assistant flows are grounded in stored evidence and parent-owned sources.
- Remote/API assistance remains outside child-device blocking, timers, and ask-parent decisions.
- Ocentra-hosted infrastructure remains stateless by default for report compilation.

## Acceptance

- Report/assistant requests declare parent action, permitted evidence refs, custody boundary, prompt/model version, retention behavior, cited evidence, uncertainty, and failure state.
- Remote/API failures degrade to local-only explanation, unknown, or ask-parent without disabling local safety.
- Remote output cannot override stricter parent rules or typed local policy decisions.

## Validation

- Run `npm run validate`.
- Include schema/parity tests, evidence-citation tests, no-retention/custody tests, and failure/degraded-state tests.
