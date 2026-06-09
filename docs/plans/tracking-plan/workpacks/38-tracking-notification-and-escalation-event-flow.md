# WP38 Tracking Notification And Escalation Event Flow

## Purpose

Model tracking notification and escalation as policy-authorized events with
separate intent, dispatch, receipt, retry, quiet-hours, manual-required, audit,
and portal states.

## Source Inputs

- `docs/plans/tracking-plan/workpacks/26-alert-severity-and-notification-model.md`
- `docs/plans/tracking-plan/workpacks/27-escalation-engine.md`
- `docs/plans/tracking-plan/workpacks/36-tracking-detection-cascade-event-flow.md`
- `docs/expectations/notifications.md`
- `docs/expectations/policy.md`

## Target State

Tracking can create notification and escalation intents only after a policy
decision. Provider dispatch results and receipt state are separate from intent.
Provider unavailability is visible as manual-required/degraded state.

## Required Source Behavior

- `notification.intent.created` requires a policy decision ref.
- `notification.dispatch.requested` requires provider capability state.
- `notification.dispatch.result_observed` records accepted, delivered, failed,
  unavailable, manual-required, or retry state.
- `escalation.intent.created` requires policy decision and severity refs.
- `escalation.result_observed` records acknowledged, canceled, advanced,
  failed, unavailable, or manual-required state.
- Notification content minimizes sensitive location detail and uses authenticated
  drill-in for precise evidence.
- Quiet-hours and retry behavior are event state, not hidden provider behavior.

## Tests After Code

- Notification intent requires policy decision ref.
- Provider dispatch requires provider capability.
- SMS unavailable creates manual-required state.
- WhatsApp unavailable creates manual-required state.
- Push unavailable creates manual-required state.

## Matrix Categories / Target Test Locations

Matrix categories: contract, Rust unit/integration, replay/idempotency,
security/AuthZ, service transport, and Playwright/service-backed UI where this
workpack touches portal rendering.

Target Rust tests must follow the crate boundary: `crates/agent-protocol/tests`
for protocol constants/payloads, `crates/agent-core/tests` for runtime and
projection behavior, and `crates/agent-service/tests` for real transport after
service seams are importable. Private module tests are only for internal helper
invariants.

- Provider result observed updates audit/read model.
- Duplicate notification id does not send twice.
- Notification text minimizes sensitive details.
- Escalation requires policy decision and severity rule.
- Parent acknowledgement cancels or modifies escalation according to rules.
- Quiet-hours behavior is visible and auditable.

## Proof After Tests Pass

Proof root:

```text
output/tracking-plan-proof/38-tracking-notification-escalation-event-flow/
```

Provider credentials, external delivery, webhook receipts, production workers,
and product-ready notification/escalation remain unclaimed until real provider
and production artifacts exist.
