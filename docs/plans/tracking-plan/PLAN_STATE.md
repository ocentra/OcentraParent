# Tracking Plan State

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `Tracking Plan State`
> Kind: current state and open gaps.
> Read when: Immediately after plan AGENTS.md; use for current state and no-claim boundaries.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

Generated from the existing `tracking-plan` docs. This is the default resume/status file; large historical docs are linked, not embedded.

## Scope

This folder is the single working plan location for location evidence, geofence rules, expected-place schedules, device status, nearby-place intelligence, AI safety analysis, parent acknowledgements, alerts, escalation, child check-ins, temporary live tracking, missing-device mode, and tracking UI/UX requirements.

## Resume route

1. Read this file.
2. Read `NEXT_ACTIONS.md` when starting/resuming.
3. Read `WORKPACK_INDEX.md`.
4. Open only the assigned workpack.
5. Use `CHECKLIST_INDEX.md` for exact checklist sections.
6. Use `PROOF_INDEX.md` for proof artifacts.

## Current snapshot source

- Snapshot: [current-tracking-snapshot.md](current-tracking-snapshot.md)

## What is already present / proved

- Feature doc exists.
- Expectation doc exists.
- Capability guide exists.
- Schema proposal exists.
- Tracking settings inventory exists with 338 raw settings.
- Focused TypeScript contract proof now exists for tracking evidence, geofence/place models, tracking AI safety evidence, tracking policy/action contracts, and proof-routing states.
- Location posture modes are represented as design inputs: Off, Last known, Check-in, Arrival alerts, Temporary live, and Missing device.
- Capability/degraded vocabulary exists as raw inventory input, including service-disabled, manual-required, offline-last-known-only, and battery-throttled.
- Device-status design inputs exist for last heartbeat, last location sample, last sync, battery percentage, charging state, low-power mode, and pending upload count.

## Open gaps / missing product runtime

- Platform adapter proof.
- Journal/SQLite read models.
- Parent/child UI.
- Notification/escalation engine.
- Retention/delete/export proof.
- Android/iOS foreground and background location runtime.
- Nearby-place provider runtime.
- Expected-place and geofence transition runtime engines.

## Manual-required or no-claim boundaries

- Android background location proof.
- Android geofence proof.
- iOS Always/region/background proof.
- Desktop precise location proof.
- Remote sync and remote AI proof.
- Emergency/critical escalation proof.
- Managed-device/MDM/lost-mode proof.

## Checklist summary

- Full checklist: [implementation-checklist.md](implementation-checklist.md) (not default context).
- Checkbox rows detected: 111 total, 79 checked, 32 unchecked.
- Checklist index: [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md).

## Workpack summary

- Workpacks indexed: 33.
- Workpacks with open checkboxes: 28.
- Workpacks with all detected boxes checked: 5.
- Workpacks with no checkbox status: 0.

### Active/open workpacks

- [WP01 Source Index And Repo Reconciliation](workpacks/01-source-index-and-repo-reconciliation.md) - 0/11 checked, 11 open.
- [WP09 Android Background Location And Geofence Adapter](workpacks/09-android-background-location-and-geofence-adapter.md) - 3/14 checked, 11 open.
- [WP02 Current Tracking Snapshot And Gap Map](workpacks/02-current-tracking-snapshot-and-gap-map.md) - 0/10 checked, 10 open.
- [WP08 Android Foreground Location Adapter](workpacks/08-android-foreground-location-adapter.md) - 3/13 checked, 10 open.
- [WP10 Android Battery Connectivity And Status Adapter](workpacks/10-android-battery-connectivity-and-status-adapter.md) - 7/13 checked, 6 open.
- [WP11 iOS Core Location Foreground Adapter](workpacks/11-ios-core-location-foreground-adapter.md) - 9/15 checked, 6 open.
- [WP12 iOS Background Region Significant-Change Adapter](workpacks/12-ios-background-region-significant-change-adapter.md) - 9/15 checked, 6 open.
- [WP15 Geofence Transition Engine](workpacks/15-geofence-transition-engine.md) - 5/11 checked, 6 open.
- [WP16 Expected-Place Schedule Engine](workpacks/16-expected-place-schedule-engine.md) - 5/11 checked, 6 open.
- [WP17 Parent Acknowledgement And Exception Model](workpacks/17-parent-acknowledgement-and-exception-model.md) - 5/11 checked, 6 open.
- [WP18 Child Check-In Flow](workpacks/18-child-check-in-flow.md) - 5/11 checked, 6 open.
- [WP20 Google Places And POI Provider Adapter](workpacks/20-google-places-and-poi-provider-adapter.md) - 6/12 checked, 6 open.
- [WP03 Contract Boundary And Effect Schemas](workpacks/03-contract-boundary-and-effect-schemas.md) - 6/11 checked, 5 open.
- [WP07 Retention And Custody Model](workpacks/07-retention-and-custody-model.md) - 18/23 checked, 5 open.
- [WP13 Desktop Location And Presence Hint Model](workpacks/13-desktop-location-and-presence-hint-model.md) - 6/11 checked, 5 open.
- [WP19 Nearby-Place Provider Abstraction](workpacks/19-nearby-place-provider-abstraction.md) - 6/11 checked, 5 open.
- [WP23 AI Location Safety Analysis Contracts](workpacks/23-ai-location-safety-analysis-contracts.md) - 6/11 checked, 5 open.
- [WP24 AI Provider Routing](workpacks/24-ai-provider-routing.md) - 6/11 checked, 5 open.
- [WP26 Alert Severity And Notification Model](workpacks/26-alert-severity-and-notification-model.md) - 18/23 checked, 5 open.
- [WP31 Platform Extension Checklists And Proof Routing](workpacks/31-platform-extension-checklists-and-proof-routing.md) - 19/24 checked, 5 open.
- [WP04 Location Evidence Model](workpacks/04-location-evidence-model.md) - 6/10 checked, 4 open.
- [WP05 Device Status Model](workpacks/05-device-status-model.md) - 6/10 checked, 4 open.
- [WP06 Permission And Capability Status Model](workpacks/06-permission-and-capability-status-model.md) - 6/10 checked, 4 open.
- [WP14 Geofence Rule Model](workpacks/14-geofence-rule-model.md) - 6/10 checked, 4 open.
- [WP21 Place-Risk Taxonomy And Ambiguity Model](workpacks/21-place-category-taxonomy-and-ambiguity-model.md) - 6/10 checked, 4 open.

## Default no-read list

- `README_FULL_ORIGINAL.md` unless you need historical full README context.
- Full `implementation-checklist.md` unless `CHECKLIST_INDEX.md` names exact section/row.
- All workpacks; use `WORKPACK_INDEX.md`.
- Source inventories and pasted-content audits unless source ownership is unclear.
- Historical checkpoint/proof docs unless `PROOF_INDEX.md` or the assigned workpack names them.

## Health / consistency

- See `PLAN_HEALTH.md` before claiming the whole plan is complete or stale.

## HID Execution Guard (added 2026-06-12)

- Scope and completion source:
  - follow [PLAN_HID_MATRIX.md](../PLAN_HID_MATRIX.md) execution slice, then this plan's assigned WORKPACK_INDEX.md and NEXT_ACTIONS.md.
  - do not mark this plan complete from checklist deltas alone.
- Before any checked update, attach:
  - a real test run log (or explicit known blocker) from the assigned implementation boundary,
  - a proof manifest under docs/proof/tracking-plan/.
- Required proof manifest names:
  - docs/proof/tracking-plan/slice-01-\*.md
  - docs/proof/tracking-plan/slice-02-\*.md
  - docs/proof/tracking-plan/slice-03-\*.md
  - each proof file must include commands, pass/fail,
    negative-cases, and manual-required notes.
- Failure rule: no PR-ready claim until replay/idempotency, authZ/replay, and rollback/teardown proofs are present for the assigned slice.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.
