# WP35 Parent Tracking Config Command Event Flow

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `WP35 Parent Tracking Config Command Event Flow`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: config command/event flow proof only.
> Does not prove: physical mobile runtime behavior, provider delivery, production readiness, or product-ready tracking.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Purpose

Make parent tracking config changes travel through validated Rust service and
event flow instead of portal-local state, proof-only rows, or direct child-agent
commands.

## Source Inputs

- `docs/plans/tracking-plan/workpacks/34-tracking-event-contracts-and-protocol-constants.md`
- `docs/plans/eventing-plan/03-event-taxonomy-and-parent-integration.md`
- `docs/plans/eventing-plan/05-implementation-workpacks.md`
- `docs/expectations/location-geofence.md`
- `docs/expectations/policy.md`

## Target State

The portal sends typed parent tracking config intents only. Rust validates the
intent, publishes tracking config events, applies parent policy, forwards
approved child-agent commands, records audit state, and projects portal read
models from service/event state.

## Central schema boundary

```text
schema-domain owns canonical parent tracking config command/event payloads.
tracking-domain may provide helpers/proof adapters only.
tracking-core may mirror canonical payloads for runtime validation and projection.
portal sends typed intents only and must not own event payload schemas.
```

## Required proof fields

```text
canonical_schema_owner_state
portal_intent_state
request_validation_state
policy_decision_state
approved_rejected_event_state
runtime_command_handoff_state
audit_state
portal_projection_state
idempotency_state
invalid_intent_rejection_state
portal_direct_publish_rejection_state
local_schema_rejection_state
no_platform_runtime_claim
no_product_ready_claim
no_claim
```

## Required Source Behavior

Canonical tracking config event owners for this flow live in
`packages/schema-domain/src/agent-tracking-retention-settings-write-command.ts`
under `AgentTrackingConfigCommandFlowEventType` and
`AgentTrackingConfigUpdateEventType`.

```text
portal parent changes tracking config
  -> local API validates request
  -> parent_controller.parent_action.received
  -> tracking.config.change_requested
  -> policy.evaluation.requested
  -> policy.decision.completed
  -> AgentTrackingConfigCommandFlowEventType.ChangeApproved or AgentTrackingConfigCommandFlowEventType.ChangeRejected
  -> child_agent.command.forward_requested
  -> child_agent.command.received
  -> AgentTrackingConfigUpdateEventType.Applied
  -> audit.entry.committed
  -> portal.read_model.updated
```

Config examples:

- tracking off/on;
- last-known-only;
- check-in mode;
- arrival alerts;
- temporary live mode;
- missing-device mode;
- retention window;
- delete after alert resolved;
- remote sync disabled/enabled where allowed;
- local AI analysis enabled/disabled.

## Tests After Code

- Valid parent config intent updates service-backed state.
- Invalid parent config intent is rejected before event publish.
- Duplicate idempotency key does not apply config twice.
- Portal cannot directly publish tracking config events.
- Portal cannot directly command child-agent tracking runtime.

## Matrix Categories / Target Test Locations

Matrix categories: contract, Rust unit/integration, replay/idempotency,
security/AuthZ, service transport, and Playwright/service-backed UI where this
workpack touches portal rendering.

Target Rust tests must follow the crate boundary: `crates/agent-protocol/tests`
for protocol constants/payloads, `crates/agent-core/tests` for runtime and
projection behavior, and `crates/agent-service/tests` for real transport after
service seams are importable. Private module tests are only for internal helper
invariants.

- Config update is journaled before runtime behavior changes.
- Child-agent unavailable produces manual-required/unavailable read-model state.
- Retention config updates durable local state.
- Remote sync remains disabled by default.
- Remote AI remains disabled unless explicit config allows it.
- Config change appears in portal read model from service state, not portal
  local state.

## Proof After Tests Pass

Proof root:

```text
output/tracking-plan-proof/35-parent-tracking-config-event-flow/
```

The proof must cite real source behavior, real tests, focused command output,
the observed event chain, schema owner for each cross-boundary shape, and
no-claim boundaries.

## Manual-Required Gaps

- This workpack does not prove physical mobile runtime behavior, provider
  delivery, production workers, or product-ready tracking.
