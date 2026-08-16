<!-- agent-capsule -->

> Agent Capsule
> Doc: V4 Parent-Owned Reports And Optional Assistant Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# V4 Parent-Owned Reports And Optional Assistant Expectations

This is the milestone-specific expectation file for V4 in `docs/product-roadmap.md`.

Supporting expectation files: [data custody](../expectations/data-custody.md), [AI](../expectations/ai.md), [parent assistant chat](../expectations/parent-assistant-chat.md), [evidence storage](../expectations/evidence-storage.md), [policy](../expectations/policy.md), and [contracts](../expectations/contracts.md).

## Outcome

- Richer parent explanations, reports, Q&A, and optional assistant flows are grounded in stored evidence and parent-owned sources.
- Remote/API assistance remains outside child-device blocking, timers, and ask-parent decisions.
- Ocentra-hosted infrastructure remains stateless by default for report compilation.

## Acceptance

- Report/assistant requests declare parent action, permitted evidence refs, custody boundary, prompt/model version, retention behavior, cited evidence, uncertainty, and failure state.
- Assistant chat threads, quick actions, follow-ups, action previews, provider status, and child-device evidence queries use typed contracts before runtime implementation.
- Remote/API failures degrade to local-only explanation, unknown, or ask-parent without disabling local safety.
- Remote output cannot override stricter parent rules or typed local policy decisions.

## Validation

- Run `npm run validate`.
- Include schema/parity tests, evidence-citation tests, no-retention/custody tests, and failure/degraded-state tests.
