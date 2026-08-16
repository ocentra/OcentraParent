# V0.5 Location Tracking Full Scope Plan

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `V0.5 Location Tracking Full Scope Plan`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

This plan turns the location/geofence/device-status draft into an
implementation and proof sequence. It is broader than a map feature. The product
goal is safe, explainable tracking with custody, retention, policy, and
platform proof.

## Core Rule

Location evidence proves reported device location, not child intent or safety
by itself.

```text
Track with permission.
Analyze with evidence.
Alert with policy.
Escalate with acknowledgement logic.
Never turn uncertainty into accusation.
```

## Scope

- Last known location
- Live/recent/stale/offline states
- Device status
- Permission/capability state
- Geofence rules
- Expected-place schedule
- Parent acknowledgement
- Exceptions/holiday/trip state
- Nearby-place evidence
- Alerts/escalation
- Child check-in
- Temporary live tracking
- Missing-device mode
- Retention/delete/export
- Journal/SQLite/read-model proof
- Parent and child UI
- Platform proof

## Non-Goals

- No LAN/IP as GPS.
- No AI-only emergency claim.
- No remote sync by default.
- No remote AI by default.
- No continuous tracking claim without platform proof.
- No exact place claim from low-accuracy GPS or nearby POI alone.
- No auto-contacting police/emergency services in MVP.

## Product Questions

Ocentra tracking should answer:

- Where was the child device last reported?
- How fresh is the location?
- How accurate is it?
- Is the device online?
- Is the battery low or throttling location?
- Was the child expected somewhere else?
- Is this normal for this child, profile, schedule, or exception?
- Is the reported location near a risky, sensitive, or important place?
- Should the parent be notified?
- Should the parent acknowledge?
- Should Ocentra escalate if the parent does not respond?
- Should the child be asked to check in?
- Is a holiday, trip, parent acknowledgement, or exception active?

## Existing Repo State

The repo has a feature doc, expectation docs, tracking settings inventory,
capability guide, schema proposal, and this plan folder. It does not yet have
the runtime contracts, platform adapters, journal/read-model proof, or UI
surfaces needed to claim product-complete tracking.

## Contract Families

- `LocationEvidence`
- `DeviceStatusEvidence`
- `LocationCapabilityStatus`
- `LocationRetentionPolicy`
- `GeofenceRule`
- `GeofenceTransition`
- `ExpectedPlaceRule`
- `NearbyPlaceEvidence`
- `LocationAiSafetyInput`
- `LocationAiSafetyResult`
- `LocationPolicyDecision`
- `LocationAlert`
- `LocationParentAcknowledgement`
- `LocationException`
- `LocationEscalationRule`

## Mode Map

| Mode                  | Product behavior                                                       | Required proof                                                               |
| --------------------- | ---------------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| Off                   | No new collection. Existing retained history follows retention policy. | Policy compile and UI disabled-state proof.                                  |
| Last known            | Show newest location evidence with source, freshness, and accuracy.    | Journal/read-model proof and stale labeling.                                 |
| Check-in              | Child or device sends periodic/requested location confirmation.        | Child prompt, optional sample, audit, and no-shame copy proof.               |
| Arrival alerts        | Geofence transitions only.                                             | Geofence rule, transition, location evidence, permission, and alert proof.   |
| Expected-place safety | Schedule plus geofence plus stale/offline checks.                      | Expected-place engine, exception, policy, and alert proof.                   |
| Temporary live        | Parent-approved high-frequency tracking for a duration.                | Authorization, disclosure, cadence, auto-expiry, audit, and retention proof. |
| Missing device        | Last known plus battery/connectivity prominence and parent actions.    | Status/read-model proof and no false current-location claims.                |
| Strict safety         | Expected place plus restricted zones plus acknowledgement escalation.  | Parent-configured rules and escalation proof.                                |

## Evidence Model

Every tracking decision must cite evidence refs instead of copying unbounded
location data across layers. Minimum evidence fields are source, observed time,
collected time, accuracy or hint quality, freshness state, custody label,
retention ref, permission/capability state, confidence, reason codes, and
degraded-state markers.

## Parent And Child UI Requirements

Parent UI must show what is known, why it is trusted, what is stale, and what
the parent can do next. Child UI must support calm check-in, disclosure for
temporary live tracking, and no-shame copy. Both surfaces need deleted-history,
permission-denied, offline, low-accuracy, and unsupported-platform states.

## Proof Routing

Each implementation slice writes proof under
`output/tracking-plan-proof/<workpack-id>/`. Contract proof, runtime proof,
platform manual proof, UI screenshots, retention proof, and validation logs
must be linked from the assigned workpack before DONE or PR-ready status.

## Validation Expectations

Use focused contract/parser tests first, then integration tests, real local
transport smoke, Playwright UI screenshots, manual device proof, and finally
the repo validation gate when the slice affects runtime behavior. Documentation
only changes need formatting, link/structure checks, and lane/hub guards.

## Minimum Serious MVP

This is the minimum first target, not the final tracking goal. It is the first
credible checkpoint for implementation and proof sequencing; it is not all 33
workpacks product-complete.

The first target is last-known location plus status/freshness, geofence
enter/exit, expected-place schedule check, parent acknowledgement, child
check-in, retention/delete/export, and UI states for disabled,
permission-required, stale, offline, and low-accuracy data. Background mobile
claims remain manual-required until real-device proof exists.

Do not treat this checkpoint as PR-ready or full-scope complete unless its
runtime, UI, product-doc, validation, and proof-tier evidence are all filled.
Remaining workpacks keep their own required tiers and missing-proof reasons.
The current first-checkpoint reconciliation is tracked in
`output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/proof-summary.json`
as `minimumSeriousMvpAuditSummary`. `scripts/test/tracking-plan-runtime-proof.mjs`
also writes the full generated `minimumSeriousMvpAudit` to
`output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/00-run-metadata.json`.

## Implementation Sequence

1. Reconcile source docs, pasted drafts, and generated inventory.
2. Document current snapshot and gap map.
3. Add schema-backed contracts in domain packages.
4. Add parser/brand tests and negative claim tests.
5. Mirror Rust protocol only after TypeScript contracts are explicit and tested.
6. Add location/status/geofence journal and SQLite paths.
7. Add read models and WebSocket/service events.
8. Add Android/iOS/desktop adapters behind platform proof gates.
9. Add expected-place, acknowledgement, exception, retention, and policy
   compiler logic.
10. Add nearby-place abstraction and AI safety evidence contracts.
11. Add alert/escalation/notification intent boundaries.
12. Add parent and child UI surfaces.
13. Add E2E, Playwright, real-device, and manual proof packs.
14. Update feature docs, capability checklist, and roadmap only when status or
    proof changes.

## Final Quality Bar

Tracking is credible only when:

- parent can see child device location with accuracy/freshness/custody;
- parent can define home/school/activity/safe/restricted geofences;
- parent can define expected-place schedules;
- Ocentra can detect enter/exit/dwell with proof;
- Ocentra can detect not-where-expected with grace/accuracy logic;
- Ocentra can identify stale/offline/battery-throttled/device-disabled states;
- Ocentra can analyze nearby places without overclaiming;
- AI can summarize concern but cannot alert alone;
- parent can acknowledge, suppress, create exception, request child check-in,
  or start live tracking;
- escalation follows configured rules;
- retention/delete/export works;
- platform limits are visible and proof-gated.

## Done Signal

The plan is implementation-ready only when every assigned workpack lists its
current state, target state, touched paths, proof root, manual-required gaps,
and DONE/PR-ready evidence block. The product feature is complete only when
runtime proof, platform proof, UI proof, and product-doc status updates exist.
