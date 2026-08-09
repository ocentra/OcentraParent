# Tracking Plan Workpack Index

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `Tracking Plan Workpack Index`
> Kind: workpack selector; use before opening any workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

Use this index to open exactly one assigned workpack. Do not read every file in `workpacks/`.

Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear.

| Status  | Workpack                                                                                                                  |   Size | Boxes                              |
| ------- | ------------------------------------------------------------------------------------------------------------------------- | -----: | ---------------------------------- |
| open    | [WP25 Policy Compiler For Tracking Rules](workpacks/25-policy-compiler-for-tracking-rules.md)                           |  3,656 | 11/11 checked; audit reopen        |
| open    | [WP27 Escalation Engine](workpacks/27-escalation-engine.md)                                                              |  4,200 | 11/11 checked; audit reopen        |
| open    | [WP28 Temporary Live Tracking Mode](workpacks/28-temporary-live-tracking-mode.md)                                        |  4,048 | 11/11 checked; audit reopen        |
| open    | [WP29 Missing-Device Mode](workpacks/29-missing-device-mode.md)                                                          |  3,631 | 11/11 checked; audit reopen        |
| open    | [WP33 Proof Gates Fixtures Rollout And PR Gate](workpacks/33-proof-gates-fixtures-rollout-and-pr-gate.md)               | 22,994 | 65/65 checked; proof rerun blocked |
| open    | [WP01 Source Index And Repo Reconciliation](workpacks/01-source-index-and-repo-reconciliation.md)                         |  3,032 | 0/11 checked; 11 open |
| open    | [WP02 Current Tracking Snapshot And Gap Map](workpacks/02-current-tracking-snapshot-and-gap-map.md)                       |  2,902 | 0/10 checked; 10 open |
| open    | [WP03 Contract Boundary And Effect Schemas](workpacks/03-contract-boundary-and-effect-schemas.md)                         |  3,476 | 6/11 checked; 5 open  |
| open    | [WP04 Location Evidence Model](workpacks/04-location-evidence-model.md)                                                   |  3,498 | 6/10 checked; 4 open  |
| open    | [WP05 Device Status Model](workpacks/05-device-status-model.md)                                                           |  3,236 | 6/10 checked; 4 open  |
| open    | [WP06 Permission And Capability Status Model](workpacks/06-permission-and-capability-status-model.md)                     |  3,446 | 6/10 checked; 4 open  |
| open    | [WP07 Retention And Custody Model](workpacks/07-retention-and-custody-model.md)                                           |  7,236 | 18/23 checked; 5 open |
| open    | [WP08 Android Foreground Location Adapter](workpacks/08-android-foreground-location-adapter.md)                           |  4,407 | 3/13 checked; 10 open |
| open    | [WP09 Android Background Location And Geofence Adapter](workpacks/09-android-background-location-and-geofence-adapter.md) |  4,584 | 3/14 checked; 11 open |
| open    | [WP10 Android Battery Connectivity And Status Adapter](workpacks/10-android-battery-connectivity-and-status-adapter.md)   |  5,010 | 7/13 checked; 6 open  |
| open    | [WP11 iOS Core Location Foreground Adapter](workpacks/11-ios-core-location-foreground-adapter.md)                         |  5,789 | 9/15 checked; 6 open  |
| open    | [WP12 iOS Background Region Significant-Change Adapter](workpacks/12-ios-background-region-significant-change-adapter.md) |  6,259 | 9/15 checked; 6 open  |
| open    | [WP13 Desktop Location And Presence Hint Model](workpacks/13-desktop-location-and-presence-hint-model.md)                 |  3,311 | 6/11 checked; 5 open  |
| open    | [WP14 Geofence Rule Model](workpacks/14-geofence-rule-model.md)                                                           |  3,274 | 6/10 checked; 4 open  |
| open    | [WP15 Geofence Transition Engine](workpacks/15-geofence-transition-engine.md)                                             |  3,584 | 5/11 checked; 6 open  |
| open    | [WP16 Expected-Place Schedule Engine](workpacks/16-expected-place-schedule-engine.md)                                     |  3,618 | 5/11 checked; 6 open  |
| open    | [WP17 Parent Acknowledgement And Exception Model](workpacks/17-parent-acknowledgement-and-exception-model.md)             |  3,708 | 5/11 checked; 6 open  |
| open    | [WP18 Child Check-In Flow](workpacks/18-child-check-in-flow.md)                                                           |  3,557 | 5/11 checked; 6 open  |
| open    | [WP19 Nearby-Place Provider Abstraction](workpacks/19-nearby-place-provider-abstraction.md)                               |  3,362 | 6/11 checked; 5 open  |
| open    | [WP20 Google Places And POI Provider Adapter](workpacks/20-google-places-and-poi-provider-adapter.md)                     |  4,374 | 6/12 checked; 6 open  |
| open    | [WP21 Place-Risk Taxonomy And Ambiguity Model](workpacks/21-place-category-taxonomy-and-ambiguity-model.md)               |  3,422 | 6/10 checked; 4 open  |
| open    | [WP22 Local Parent-Defined Place Database](workpacks/22-local-parent-defined-place-database.md)                           |  4,425 | 6/10 checked; 4 open  |
| open    | [WP23 AI Location Safety Analysis Contracts](workpacks/23-ai-location-safety-analysis-contracts.md)                       |  3,297 | 6/11 checked; 5 open  |
| open    | [WP24 AI Provider Routing](workpacks/24-ai-provider-routing.md)                                                           |  3,276 | 6/11 checked; 5 open  |
| open    | [WP26 Alert Severity And Notification Model](workpacks/26-alert-severity-and-notification-model.md)                       | 10,266 | 18/23 checked; 5 open |
| open    | [WP30 Parent And Child UI/UX Surfaces](workpacks/30-parent-and-child-ui-ux-surfaces.md)                                   | 24,016 | 72/74 checked; 2 open |
| open    | [WP31 Platform Extension Checklists And Proof Routing](workpacks/31-platform-extension-checklists-and-proof-routing.md)   |  7,398 | 19/24 checked; 5 open |
| open    | [WP32 Journal SQLite And Read-Model Proof](workpacks/32-journal-sqlite-and-read-model-proof.md)                           | 21,929 | 73/76 checked; 3 open |
| open    | [WP34 Tracking Event Contracts And Protocol Constants](workpacks/34-tracking-event-contracts-and-protocol-constants.md)   |  4,248 | 0/0 checked; Rust event catalog and focused schema tests exist; runtime/audit proof remains open |
| open    | [WP35 Parent Tracking Config Command Event Flow](workpacks/35-parent-tracking-config-command-event-flow.md)               |  3,281 | 0/0 checked; on-disk audit open    |
| open    | [WP36 Tracking Detection Cascade Event Flow](workpacks/36-tracking-detection-cascade-event-flow.md)                       |  4,015 | 0/0 checked; on-disk audit open    |
| open    | [WP37 Tracking Event Journal Replay And Projection](workpacks/37-tracking-event-journal-replay-and-projection.md)        |  2,550 | 0/0 checked; on-disk audit open    |
| open    | [WP38 Tracking Notification And Escalation Event Flow](workpacks/38-tracking-notification-and-escalation-event-flow.md)   |  2,980 | 0/0 checked; on-disk audit open    |
| open    | [WP39 Tracking Portal Event Read-Model Proof](workpacks/39-tracking-portal-event-read-model-proof.md)                     |  2,908 | 0/0 checked; on-disk audit open    |

| open | [Device Location Tracking Capability Guide](workpacks/device-location-tracking-capability-guide.md) | 33,263 | 0/0 checked; 0 open |
| open | [Device Location Tracking Schema Proposal](workpacks/device-location-tracking-schema-proposal.md) | 46,695 | 0/0 checked; 0 open |
| open | [Tracking Control Settings Inventory](workpacks/tracking-control-settings-inventory.md) | 326,337 | 0/0 checked; 0 open |

Audit note: `WP25`, `WP27`, `WP28`, `WP29`, and `WP33` were reopened by the 2026-06-16 source/test/proof audit. `WP34-WP39` exist on disk and belong in the active index; earlier generated summaries omitted them.

## Selection rules

- Choose exactly one workpack.
- If owner/proof family is unclear, classify through `WORKPACK_FAMILIES.md`; do not scan every family.
- Do not use checked boxes as proof when a workpack is audit-reopened.
- Do not omit WP34-WP39 from scope.
- Cross-boundary schemas must cite `schema-domain` or a neutral protocol/event/evidence owner.
- Tracking-local schemas are private helpers only.
