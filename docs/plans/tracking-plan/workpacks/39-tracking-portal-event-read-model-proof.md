# WP39 Tracking Portal Event Read Model Proof

## Purpose

Ensure parent and child tracking UI surfaces render service/event projections instead of owning evidence interpretation, policy decisions, notification routing, live-mode escalation, or audit state.

## Central schema boundary

```text
schema-domain owns canonical tracking read-model DTOs that cross service/portal boundaries.
tracking-core and service/projectors own projection behavior when selected.
portal-domain/apps/portal render read models and must not own tracking evidence, policy, alert, escalation, custody, or audit authority.
tracking-domain may provide helper/proof adapters only.
```

## Source Inputs

- `docs/plans/tracking-plan/workpacks/30-parent-and-child-ui-ux-surfaces.md`
- `docs/plans/tracking-plan/workpacks/35-parent-tracking-config-command-event-flow.md`
- `docs/plans/tracking-plan/workpacks/36-tracking-detection-cascade-event-flow.md`
- `docs/plans/tracking-plan/workpacks/37-tracking-event-journal-replay-and-projection.md`

## Target State

Portal config controls send typed parent intents only. Portal tracking displays render event-projected read models with evidence refs, policy refs, capability state, retention/custody state, notification/escalation state, uncertainty, and manual-required gaps.

## Required proof fields

```text
canonical_schema_owner_state
portal_intent_state
service_projection_state
read_model_dto_state
evidence_ref_state
policy_ref_state
capability_state
custody_state
notification_state
escalation_state
uncertainty_state
manual_required_state
audit_chain_state
correlation_causation_state
screenshot_state
accessibility_state
no_portal_authority_claim
no_runtime_delivery_claim
no_product_ready_claim
no_claim
```

## Required Source Behavior

- Parent config form sends typed command/intents only.
- Tracking status reads service/event projection.
- Live tracking status renders TTL, stop condition, reason, visibility, and audit refs.
- Suspicious or ambiguous place state renders uncertainty-safe copy.
- Notification and escalation state render intent/dispatch/result/manual boundaries.
- Dead-letter/error/manual-required states are visible.
- Audit/correlation/causation drawer shows event chain refs.
- Child-facing live-tracking disclosure renders from service state.

## Tests After Code

- Parent config form sends typed command.
- Tracking config state renders from service read model.
- Live tracking status renders TTL/stop reason/audit.
- Suspicious place alert renders uncertainty and safe copy.
- Nearby-place ambiguity is visible.
- Manual-required provider state is visible.
- Notification dispatch state is visible.
- Dead-letter/error state is visible.
- Audit chain drawer shows correlation and causation.
- Child-facing live-tracking disclosure is visible.
- Ambiguous/weak evidence does not produce accusation copy.

## Proof After Tests Pass

Proof root:

```text
output/tracking-plan-proof/39-tracking-portal-event-read-model-proof/
```

Proof must include screenshots/accessibility output only after service/event state and tests exist. Hosted screenshots alone do not prove full product parent/child runtime UI or child-device delivery.
