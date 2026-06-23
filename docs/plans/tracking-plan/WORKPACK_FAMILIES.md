<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `Tracking Plan Workpack Families`
> Kind: owner-path classifier for selected workpacks.
> Read when: only after `WORKPACK_INDEX.md` selects or names a workpack and owner/proof family is unclear.
> Stop rule: classify the selected workpack only; do not use this file as permission to scan every workpack.
> Proves: routing and owner-path classification only.
> Does not prove: tracking readiness, platform proof, event runtime, notification delivery, policy authority, custody readiness, physical-device proof, or PR readiness.
> Proof rule: if this file changes route/status claims, update `AGENTS.md`, `PLAN_STATE.md`, and any affected selected workpack route.

<!-- /agent-capsule -->

# Tracking Plan Workpack Families

Use this file to classify one selected workpack before opening source. Central schema rule always applies: any cross-boundary tracking shape belongs in `schema-domain` or an approved neutral protocol/event/evidence boundary; tracking-local schemas are private only.

## Source, inventory, and current-state family

```text
Workpacks:
WP01 Source Index And Repo Reconciliation
WP02 Current Tracking Snapshot And Gap Map
large reference workpacks: capability guide, schema proposal, settings inventory

Owners:
tracking-plan for source truth, gap map, proof inventory, no-claim boundaries, and stale-status repair
schema-domain for promoted shared shapes

Rule:
Source reconciliation does not prove runtime behavior. Do not cite planning drafts as product contracts unless promoted to a canonical schema or accepted source doc.
```

## Core contracts and local models family

```text
Workpacks:
WP03 Contract Boundary And Effect Schemas
WP04 Location Evidence Model
WP05 Device Status Model
WP06 Permission And Capability Status Model
WP07 Retention And Custody Model
WP13 Desktop Location And Presence Hint Model
WP14 Geofence Rule Model
WP21 Place-Risk Taxonomy And Ambiguity Model
WP22 Local Parent-Defined Place Database

Owners:
schema-domain for cross-boundary schemas
tracking-domain for tracking helpers/projections/proof adapters
tracking-core for Rust runtime mirrors/parsers
custody/data plans for retention/export/delete policy

Rule:
A model crossing package/crate/plan/event/protocol/UI/policy/notification/custody boundaries is not allowed to stay canonical inside tracking-domain or tracking-core.
```

## Platform adapter and physical proof family

```text
Workpacks:
WP08 Android Foreground Location Adapter
WP09 Android Background Location And Geofence Adapter
WP10 Android Battery Connectivity And Status Adapter
WP11 iOS Core Location Foreground Adapter
WP12 iOS Background Region Significant-Change Adapter
WP31 Platform Extension Checklists And Proof Routing

Owners:
tracking-plan for capability labels, manual-required states, and proof routing
platform/runtime owners for OS behavior, physical-device proof, permission behavior, and background delivery
schema-domain for shared platform/status/capability shapes

Rule:
Emulator/simulator/local proof is not physical-device proof. Background tracking and system geofence claims require matching platform artifacts.
```

## Detection, place, AI, and policy family

```text
Workpacks:
WP15 Geofence Transition Engine
WP16 Expected-Place Schedule Engine
WP17 Parent Acknowledgement And Exception Model
WP18 Child Check-In Flow
WP19 Nearby-Place Provider Abstraction
WP20 Google Places And POI Provider Adapter
WP23 AI Location Safety Analysis Contracts
WP24 AI Provider Routing
WP25 Policy Compiler For Tracking Rules

Owners:
tracking-plan/tracking-core for evidence-derived tracking decisions
schema-domain for shared evidence, place, policy-ref, AI-boundary, and proof shapes
policy-control-plane-plan for policy authority
AI plan for model/provider runtime
provider owners for live provider credentials/delivery

Rule:
Nearby-place and AI outputs are evidence only. Policy is the first authority point for notify, live tracking, escalation, or manual-required actions.
```

## Notification, escalation, live, and missing-mode family

```text
Workpacks:
WP26 Alert Severity And Notification Model
WP27 Escalation Engine
WP28 Temporary Live Tracking Mode
WP29 Missing-Device Mode
WP38 Tracking Notification And Escalation Event Flow

Owners:
schema-domain for notification/escalation/live/missing-mode payloads crossing boundaries
tracking-plan/tracking-core for tracking intent/readiness logic
notification owners for provider dispatch, receipts, retries, quiet-hours runtime, and delivery history
remote/runtime/platform owners for live/current/background runtime behavior

Rule:
Intent proof is not provider delivery. Fixture proof is not production worker proof. Temporary live and missing-device modes must preserve TTL, audit, stale/offline, manual-required, and no-current-location overclaim boundaries.
```

## UI, journal, read-model, and portal family

```text
Workpacks:
WP30 Parent And Child UI/UX Surfaces
WP32 Journal SQLite And Read-Model Proof
WP37 Tracking Event Journal Replay And Projection
WP39 Tracking Portal Event Read-Model Proof

Owners:
tracking-plan for tracking read-model semantics and event-projection proof
portal-ux-household-surfaces-plan/apps/portal for rendered UI proof when selected
agent-core/service/protocol for journal, replay, service transport, and read-model seams
schema-domain for shared read-model DTOs

Rule:
Portal renders service/event projections. UI must not own evidence interpretation, policy decision, notification routing, escalation, live-mode, or audit state.
```

## Event-contract and event-flow family

```text
Workpacks:
WP34 Tracking Event Contracts And Protocol Constants
WP35 Parent Tracking Config Command Event Flow
WP36 Tracking Detection Cascade Event Flow
WP37 Tracking Event Journal Replay And Projection
WP38 Tracking Notification And Escalation Event Flow
WP39 Tracking Portal Event Read-Model Proof

Owners:
schema-domain/protocol for canonical event payloads and constants
ocentra-eventing for generic event envelope, idempotency, request/response, journal/replay, dead-letter, topology, and testkit mechanics
tracking-core for tracking-specific runtime handlers that consume canonical contracts
tracking-domain for TS helper/projection/proof adapters only

Rule:
Do not invent local event strings. Do not create a tracking-private bus. Do not duplicate generic eventing mechanics in tracking source.
```

## Rollout, proof gates, and claim-audit family

```text
Workpacks:
WP33 Proof Gates Fixtures Rollout And PR Gate

Owners:
tracking-plan for proof aggregation, false-green reopen handling, claim-audit state, manual-required gates, and safe wording
selected workpacks for source/runtime proof roots
sibling owners for physical-device, provider-runtime, production-worker, authority, custody, AI, policy, notification, and portal completion proof

Rule:
WP33 may aggregate only accepted proof roots or exact blockers. Proof-file presence is not product readiness; claim approval must stay false until the required real-runtime artifacts or reviewer approvals exist.
```
