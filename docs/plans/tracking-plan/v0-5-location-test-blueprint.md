# V0.5 Location Test Blueprint

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `V0.5 Location Test Blueprint`
> Kind: test blueprint reference; read only when local expectations route here.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Core Test Rule

```text
No location claim without source, accuracy, timestamp, freshness, custody, retention, and permission state.
No geofence claim without location evidence and geofence rule refs.
No nearby-place claim without accuracy and ambiguity.
No risk alert without policy decision and evidence refs.
No AI-only escalation.
No background mobile claim without platform proof.
```

## Required Test Layers

- Domain contract/parser tests for every tracking schema and branded id.
- Runtime integration tests for geofence, expected-place, nearby-place,
  policy, acknowledgement, escalation, journal, and retention behavior.
- Rust protocol conversion tests after TypeScript contracts are mirrored.
- Real local transport smoke tests for service/WebSocket behavior.
- Playwright tests for parent and child UI states.
- Manual Android/iOS/desktop proof for platform-specific claims.
- Security negative tests for no-claim boundaries and default-disabled remote
  sync/AI behavior.

## Required Fixtures

Fixtures must include fresh, stale, offline, denied-permission,
low-accuracy-boundary, ambiguous-nearby-place, holiday-exception,
parent-acknowledged, child-check-in, temporary-live-expired, missing-device,
retention-deleted, remote-sync-disabled, and remote-AI-disabled states.

## Proof Matrix

| Claim family      | Required proof                                                         |
| ----------------- | ---------------------------------------------------------------------- |
| Location evidence | Contract proof plus runtime sample with source/accuracy/freshness.     |
| Device status     | Heartbeat, battery, connectivity, low-power, and pending upload proof. |
| Geofence          | Rule ref, evidence ref, transition proof, stale/ambiguity negatives.   |
| Expected place    | Schedule ref, exception handling, grace/accuracy proof.                |
| Nearby place      | Provider response, radius, distance, ambiguity, no-exact-place proof.  |
| AI safety         | Input/output refs, confidence, no-final-action negative tests.         |
| Alert/escalation  | Policy decision, acknowledgement, provider minimization proof.         |
| Retention/custody | Delete/export/tombstone and cache invalidation proof.                  |
| UI                | Screenshots for all required visible states.                           |
| Platform behavior | Manual real-device proof for Android/iOS/desktop claims.               |

## CI Gates

CI should fail implementation PRs that introduce untested tracking contracts,
inline app/runtime strings, naked raw string annotations, test doubles,
unproved protocol conversions, remote sync enabled by default, remote AI
enabled by default, or UI/product claims without proof artifacts. Documentation
only edits need formatting, link/structure checks, and lane/hub guards.

## Test Folder Structure

```text
tests/tracking/
  unit/
    location_evidence.test.ts
    device_status.test.ts
    capability_status.test.ts
    retention_policy.test.ts
    geofence_rule.test.ts
    geofence_transition.test.ts
    expected_place_rule.test.ts
    nearby_place_evidence.test.ts
    location_ai_input_output.test.ts
    location_policy_decision.test.ts
    location_alert.test.ts
    parent_acknowledgement.test.ts
    location_exception.test.ts
    escalation_rule.test.ts

  integration/
    geofence_engine.test.ts
    expected_place_engine.test.ts
    nearby_place_provider.test.ts
    ai_safety_pipeline.test.ts
    policy_alert_compile.test.ts
    alert_escalation.test.ts
    retention_delete.test.ts
    journal_sqlite_tracking_ingest.test.ts

  platform/
    android_location.manual.test.ts
    android_geofence.manual.test.ts
    android_background_permission.manual.test.ts
    ios_core_location.manual.test.ts
    ios_region_monitoring.manual.test.ts
    desktop_location_hints.manual.test.ts

  e2e/
    school_expected_place.e2e.ts
    cinema_during_school.e2e.ts
    hospital_urgent.e2e.ts
    holiday_exception.e2e.ts
    parent_acknowledgement.e2e.ts
    child_checkin.e2e.ts
    temporary_live_tracking.e2e.ts
    missing_device.e2e.ts

playwright/
  tracking-dashboard.spec.ts
  tracking-alerts.spec.ts
  tracking-geofence-editor.spec.ts
  tracking-expected-place.spec.ts
  tracking-exceptions.spec.ts
  tracking-evidence-drawer.spec.ts
  tracking-retention.spec.ts
  tracking-platform-capability.spec.ts
```

## Unit Tests

- `LocationEvidence` requires source.
- `LocationEvidence` requires observed and collected time.
- GPS-like source requires `accuracyMeters`.
- Hint-only source cannot become precise.
- `permission_denied` has no coordinates unless last-known is explicit.
- Stale state preserved.
- Retention ref required.
- Custody label required.
- Confidence stays `0..1`.
- AI result cannot include final action.
- Critical alert requires policy decision.

## Integration Tests

- Enter home geofence.
- Exit school geofence.
- Dwell at school.
- Low accuracy near boundary returns ambiguous.
- Holiday exception suppresses school schedule alert.
- Hospital category still alerts if configured.
- Nearby place provider returns multiple possible places.
- AI low confidence maps unknown candidate.
- Policy creates alert with evidence refs.
- Acknowledgement cancels escalation.
- Retention delete removes history.

## E2E Tests

- Child arrived home.
- Child left school during school hours.
- Child near cinema during school hours.
- Child near hospital unexpectedly.
- Device offline after expected arrival.
- Parent marks holiday exception.
- Parent acknowledges safe.
- Parent asks child to check in.
- Child replies safe/help.
- Temporary live tracking expires.
- Missing device mode shows last-known plus status.

## Playwright Tests

- Tracking disabled state.
- Permission required state.
- Last known card.
- Fresh/stale/offline badges.
- Accuracy circle.
- Map/list switch.
- Geofence editor.
- Expected-place schedule.
- Alert card.
- Evidence drawer.
- AI explanation drawer.
- Parent acknowledgement.
- Holiday exception.
- Child check-in.
- Live tracking timer.
- Missing-device mode.
- Retention/delete UI.
- Platform capability matrix.

## Merge-Blocking Failures

Block merge if:

- LAN/IP displayed as GPS;
- location missing accuracy/source/timestamp/freshness;
- stale displayed as live;
- nearby POI displayed as exact place with low accuracy;
- AI triggers notification without policy decision;
- critical alert suppressed by generic exception;
- parent acknowledgement ignored;
- retention delete fails;
- remote sync runs by default;
- remote AI runs by default;
- background tracking claimed without Android/iOS proof.

## Required Proof Pack

Each workpack proof root:

```text
output/tracking-plan-proof/<workpack-id>/
```

Each applicable proof pack must include:

```text
00-source-snapshot.md
01-contract-proof.log
02-platform-permission-proof.md
03-runtime-location-evidence.json
04-device-status-proof.json
05-geofence-transition-proof.json
06-expected-place-proof.json
07-nearby-place-proof.json
08-ai-analysis-proof.json
09-policy-alert-proof.json
10-journal-sqlite-proof.json
11-ui-snapshots/
12-playwright-proof.log
13-security-negative-proof.log
14-retention-delete-proof.json
15-manual-platform-proof.md
16-validation-commands.log
```

## Done Signal Per Workpack

Each workpack must include the exact validation commands, their results, proof
artifact paths, product-doc updates or justification, and known
manual-required gaps before it can be reported DONE or PR-ready.

## Minimum Serious MVP Test Set

This is the first target test set. It is not the complete product test goal and
does not replace the 33-workpack proof checklist.

- Last-known location with accuracy/freshness/custody labels.
- Stale/offline/permission-denied/low-accuracy states.
- Enter and exit a geofence with evidence/rule refs.
- Expected-place missed-arrival flow with holiday exception.
- Parent acknowledgement cancels escalation.
- Child check-in response resolves or escalates according to policy.
- Nearby-place ambiguity does not become an exact-place claim.
- AI explanation cannot alert without policy decision.
- Retention delete removes history and cached UI.
- Temporary live tracking expires automatically.

## Worker Instruction Template

Before editing, a worker should cite the assigned workpack, lock the intended
paths, list expected implementation and proof files, run focused tests, update
product docs/checklist only when status or proof changes, and report DONE with
branch, commit, touched files, validation, proof artifacts, and known gaps.
