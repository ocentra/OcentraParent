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

Current source checkpoint (2026-08-18):

- `agent-service` now exposes a bounded parent-intent command marker and rejects
  malformed payloads. A valid marker returns `manual-required` with journal,
  Eventing publication, event id, and child transport all explicitly unclaimed.
- The service does not trust caller source/route fields, caller-provided policy
  state, caller-provided tracking state, or an in-process bus as authority.
- This is a safe fail-closed ingress seam only. It is not a validated parent
  publisher, consumer dispatch, child delivery, or replay implementation.

Current reviewed topology remains bounded to the existing helper contracts and
websocket/result surfaces. The planned owner
`crates/parent-runtime-core/src/parent_runtime_intent_ingress.rs` and both
expected ingress test roots remain absent. No live parent-runtime ingress,
  durable journal, publish, event-id, or child-transport classification is made.
- The functional owner remains missing at
  `crates/parent-runtime-core/src/parent_runtime_intent_ingress.rs`. That owner
  must consume opaque Account-session authority and the public Tracking/Policy
  producer contracts, then hand durable dispatch to their owning consumers. It
  must not accept authority scalars or business events from the portal/service
  request.

Required owner dependencies before functional source can land:

- Account Identity WP03 for authenticated session/currentness/revocation.
- Tracking WP40 for trusted ingress, durable journal, replay, and projection.
- Policy WP03/WP04/WP08 for compiled state, delivery/receipt, and event model.
- Enforcement WP11 for durable before/after/result journal history.
- Child Runtime WP10 for authenticated child ingress and local republish.

Expected tests/proof:

- `eventing.parent-runtime.intent-validation`
- `eventing.ui.no-business-event-publish`
- `eventing.child-agent.local-republish`
- `eventing.enforcement.journal-before-action`
- `eventing.adapter-result.audit-read-model`
- Proof includes runtime command log, rejected UI-publish attempt, and consumer-plan proof references.

Expected test source remains intentionally unwritten during the production
source wave. The later test wave must add:

- `crates/agent-service/tests/parent_runtime_intent_ingress.rs` for malformed,
  unknown-field, unauthenticated, source-spoof, and no-false-success cases.
- `crates/parent-runtime-core/tests/integration/parent_runtime_intent_ingress.rs`
  for opaque-session authority, canonical tracking/policy producer use,
  durable dispatch/replay, consumer ownership, and child-local-republish
  boundaries.

Failure conditions:

- Do not claim end-to-end child delivery without parent desktop/runtime and transport proof.
- Do not claim enforcement behavior from event emission alone.
- Do not let UI, AI, or portal bypass typed parent confirmation/authority.
