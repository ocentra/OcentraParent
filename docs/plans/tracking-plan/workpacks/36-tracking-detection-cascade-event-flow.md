# WP36 Tracking Detection Cascade Event Flow

## Purpose

Turn tracking evidence into ordered event-driven detection, nearby-place/AI
analysis, policy decisions, live tracking, notification, escalation, audit, and
portal read-model updates.

## Source Inputs

- `docs/plans/tracking-plan/workpacks/34-tracking-event-contracts-and-protocol-constants.md`
- `docs/plans/tracking-plan/workpacks/35-parent-tracking-config-command-event-flow.md`
- `docs/expectations/location-geofence.md`
- `docs/expectations/ai.md`
- `docs/expectations/policy.md`
- `docs/expectations/notifications.md`

## Target State

Location evidence can move through detection and decision subscribers without
shortcuts. AI and nearby-place analysis remain evidence only. Parent policy is
the first authority point for notify, live tracking, escalation, or
manual-required decisions.

## Required Source Behavior

```text
location.evidence.observed
  -> geofence.transition.evaluated
  -> expected_place.status.evaluated
  -> nearby_place.analysis.requested if needed
  -> nearby_place.analysis.completed
  -> tracking.detection.completed
  -> ai.analysis.requested if ambiguity/risk requires it
  -> ai.analysis.completed
  -> policy.evaluation.requested
  -> policy.decision.completed
  -> tracking.live_mode.start_requested or notification.intent.created or escalation.intent.created
  -> child_agent.command.received if live mode/cadence changes
  -> notification.dispatch.requested if parent must be informed
  -> notification.dispatch.result_observed
  -> audit.entry.committed
  -> portal.read_model.updated
```

Scenarios:

1. Normal expected-place sample observes only.
2. Stale/offline sample creates degraded parent-visible state, not accusation.
3. Unexpected place candidate requests nearby-place analysis.
4. Restricted or suspicious nearby place requests policy evaluation and may
   create notify/live/escalate intents only when policy authorizes it.
5. Low accuracy or ambiguous nearby place cannot create critical alert.
6. Parent acknowledgement modifies future alert behavior without erasing
   evidence.
7. Critical alert requires configured parent policy and sufficient evidence.

## Tests After Code

- Event chain preserves correlation/causation id from evidence to portal read
  model.
- Geofence transition event cites location evidence and geofence rule refs.
- Expected-place event cites schedule/rule refs.
- Nearby-place ambiguity prevents accusation copy.

## Matrix Categories / Target Test Locations

Matrix categories: contract, Rust unit/integration, replay/idempotency,
security/AuthZ, service transport, and Playwright/service-backed UI where this
workpack touches portal rendering.

Target Rust tests must follow the crate boundary: `crates/agent-protocol/tests`
for protocol constants/payloads, `crates/agent-core/tests` for runtime and
projection behavior, and `crates/agent-service/tests` for real transport after
service seams are importable. Private module tests are only for internal helper
invariants.

- AI receives summaries/evidence refs only.
- AI cannot publish policy, enforcement, live-mode, notification, or escalation
  events.
- Policy is first authority point for notify/live/escalate.
- Weak/stale/low-accuracy evidence cannot create critical alert.
- Live tracking start requires policy decision or explicit parent command.
- Live tracking has TTL, stop condition, reason, visibility, and audit ref.
- Notification dispatch requires provider/capability state.
- Provider unavailable creates manual-required/degraded state.
- Audit is committed before portal read-model update.
- Duplicate evidence/event does not duplicate notification or live-mode.
- Handler failure creates dead-letter/audit and does not silently disappear.

## Proof After Tests Pass

Proof root:

```text
output/tracking-plan-proof/36-tracking-detection-cascade-event-flow/
```

The proof must include observed chain rows and must not upgrade physical-device,
provider, production, authority, or product-ready claims.
