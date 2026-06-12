# Tracking Plan Next Actions

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `Tracking Plan Next Actions`
> Kind: resume queue and highest-open work.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This file is the short resume list for the next worker. It is derived from open workpack/checklist status and does not replace the assigned workpack.

## How to use

1. Confirm the hub assignment and lane.
2. Pick only the assigned workpack from the list below.
3. Open that workpack and exact checklist rows only.
4. Do not claim `DONE` unless the assigned workpack's acceptance/proof rows are updated and validation is listed.

## Highest-open workpacks by unchecked boxes

- [WP01 Source Index And Repo Reconciliation](workpacks/01-source-index-and-repo-reconciliation.md): 11 open of 11 boxes.
- [WP09 Android Background Location And Geofence Adapter](workpacks/09-android-background-location-and-geofence-adapter.md): 11 open of 14 boxes.
- [WP02 Current Tracking Snapshot And Gap Map](workpacks/02-current-tracking-snapshot-and-gap-map.md): 10 open of 10 boxes.
- [WP08 Android Foreground Location Adapter](workpacks/08-android-foreground-location-adapter.md): 10 open of 13 boxes.
- [WP10 Android Battery Connectivity And Status Adapter](workpacks/10-android-battery-connectivity-and-status-adapter.md): 6 open of 13 boxes.
- [WP11 iOS Core Location Foreground Adapter](workpacks/11-ios-core-location-foreground-adapter.md): 6 open of 15 boxes.
- [WP12 iOS Background Region Significant-Change Adapter](workpacks/12-ios-background-region-significant-change-adapter.md): 6 open of 15 boxes.
- [WP15 Geofence Transition Engine](workpacks/15-geofence-transition-engine.md): 6 open of 11 boxes.
- [WP16 Expected-Place Schedule Engine](workpacks/16-expected-place-schedule-engine.md): 6 open of 11 boxes.
- [WP17 Parent Acknowledgement And Exception Model](workpacks/17-parent-acknowledgement-and-exception-model.md): 6 open of 11 boxes.
- [WP18 Child Check-In Flow](workpacks/18-child-check-in-flow.md): 6 open of 11 boxes.
- [WP20 Google Places And POI Provider Adapter](workpacks/20-google-places-and-poi-provider-adapter.md): 6 open of 12 boxes.
- [WP03 Contract Boundary And Effect Schemas](workpacks/03-contract-boundary-and-effect-schemas.md): 5 open of 11 boxes.
- [WP07 Retention And Custody Model](workpacks/07-retention-and-custody-model.md): 5 open of 23 boxes.
- [WP13 Desktop Location And Presence Hint Model](workpacks/13-desktop-location-and-presence-hint-model.md): 5 open of 11 boxes.
- [WP19 Nearby-Place Provider Abstraction](workpacks/19-nearby-place-provider-abstraction.md): 5 open of 11 boxes.
- [WP23 AI Location Safety Analysis Contracts](workpacks/23-ai-location-safety-analysis-contracts.md): 5 open of 11 boxes.
- [WP24 AI Provider Routing](workpacks/24-ai-provider-routing.md): 5 open of 11 boxes.
- [WP26 Alert Severity And Notification Model](workpacks/26-alert-severity-and-notification-model.md): 5 open of 23 boxes.
- [WP31 Platform Extension Checklists And Proof Routing](workpacks/31-platform-extension-checklists-and-proof-routing.md): 5 open of 24 boxes.

## PR readiness guard

A PR-ready slice should close a named workpack or clearly explain the exact remaining rows. Do not create a tiny PR that only updates one proof note while leaving the assigned workpack/checklist state stale.

Before reporting `DONE` or `PR_READY`, update the workpack, checklist row(s), proof reference(s), and feature/product docs if product status changed.
