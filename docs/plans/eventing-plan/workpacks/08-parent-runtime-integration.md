# WP08 Parent Runtime Integration

Scope: prove parent/controller and child-agent runtime use of shared eventing contracts without granting UI or transport layers business-event authority.

Source rows: `05-implementation-workpacks.md` rows 51-56.

Read next:

- `../03-event-taxonomy-and-parent-integration.md`
- `../TEST_PROOF_EXPECTATIONS.md`
- `../PROOF_INDEX.md`
- Consumer plan AGENTS only when runtime behavior crosses into that domain

Expected outcome:

- Parent/controller runtime publishes validated parent intents through typed boundaries.
- Vite/TypeScript UI sends typed intents only and cannot publish business events directly.
- Child commands cross service/transport boundaries as typed contracts, then republish locally on the child side.
- Enforcement command proof records journal-before-action and adapter-result-to-audit/read-model flow.

Expected tests/proof:

- `eventing.parent-runtime.intent-validation`
- `eventing.ui.no-business-event-publish`
- `eventing.child-agent.local-republish`
- `eventing.enforcement.journal-before-action`
- `eventing.adapter-result.audit-read-model`
- Proof includes runtime command log, rejected UI-publish attempt, and consumer-plan proof references.

Failure conditions:

- Do not claim end-to-end child delivery without parent desktop/runtime and transport proof.
- Do not claim enforcement behavior from event emission alone.
- Do not let UI, AI, or portal bypass typed parent confirmation/authority.
