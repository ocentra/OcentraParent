# WP38 Tracking Notification And Escalation Event Flow

## Purpose

Model tracking notification and escalation as policy-authorized events with separate intent, dispatch, result, retry, quiet-hours, manual-required, audit, and portal states.

## Central schema boundary

```text
schema-domain owns canonical alert and escalation payloads used by tracking.
notification owner owns provider dispatch, receipt, retry, quiet-hours runtime, and delivery history.
tracking-core consumes canonical contracts for tracking-driven intent creation only.
tracking-domain may provide helper/proof adapters only.
```

## Source Inputs

- `docs/plans/tracking-plan/workpacks/26-alert-severity-and-notification-model.md`
- `docs/plans/tracking-plan/workpacks/27-escalation-engine.md`
- `docs/plans/tracking-plan/workpacks/36-tracking-detection-cascade-event-flow.md`
- `docs/expectations/notifications.md`
- `docs/expectations/policy.md`

## Target State

Tracking can create alert and escalation intents only after a policy decision. Provider result state is separate from intent. Provider unavailability is visible as manual-required/degraded state.

## Required proof fields

```text
canonical_schema_owner_state
policy_decision_ref_state
alert_intent_state
dispatch_request_state
dispatch_result_state
escalation_intent_state
escalation_result_state
provider_capability_state
quiet_hours_state
retry_state
sensitive_detail_minimization_state
audit_state
portal_state
no_provider_runtime_claim
no_product_ready_claim
no_claim
```

## Required Source Behavior

- Alert intent requires a policy decision ref.
- Dispatch request requires provider capability state.
- Dispatch result records accepted, delivered, failed, unavailable, manual-required, or retry state.
- Escalation intent requires policy decision and severity refs.
- Escalation result records acknowledged, canceled, advanced, failed, unavailable, or manual-required state.
- Message content minimizes sensitive detail and uses authenticated drill-in for precise evidence.
- Quiet-hours and retry behavior are event state, not hidden provider behavior.

## Tests After Code

- Alert intent requires policy decision ref.
- Provider dispatch requires provider capability.
- Provider unavailable creates manual-required state.
- Provider result observed updates audit/read model.
- Duplicate id does not send twice.
- Message text minimizes sensitive details.
- Escalation requires policy decision and severity rule.
- Parent acknowledgement cancels or modifies escalation according to rules.
- Quiet-hours behavior is visible and auditable.

## Proof After Tests Pass

Proof root:

```text
output/tracking-plan-proof/38-tracking-notification-escalation-event-flow/
```

Provider credentials, external delivery, webhook receipts, production workers, and product-ready alert/escalation remain unclaimed until real provider and production artifacts exist.
