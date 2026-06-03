# Tracking Plan Implementation Checklist

A checkbox may be marked `[x]` only after the referenced proof pack exists
under:

```text
output/tracking-plan-proof/<workpack-id>/
```

Every checked item must cite one or more proof artifacts.

## Fill Rules

- Leave a checkbox unchecked until proof exists.
- Add the proof artifact path next to the item in the assigned workpack before
  reporting DONE or PR-ready.
- Do not mark platform behavior complete from docs, mocks, screenshots of
  settings only, or simulator-only evidence unless the workpack explicitly
  allows simulator proof.
- Do not mark UI complete without screenshot states and accessibility proof.
- Do not mark product docs complete unless the feature doc and capability
  checklist status/gap text were updated or explicitly justified as unchanged.

## Main Execution Gates

- [ ] No precise location is inferred from LAN/IP/pairing.
- [ ] Every location sample has source, timestamp, accuracy/hint quality,
      freshness, custody, retention, permission state, confidence, and reason
      codes.
- [ ] Geofence transitions cite location evidence and geofence rule refs.
- [ ] Expected-place decisions cite schedule and expected-place rule refs.
- [ ] Nearby-place evidence includes query radius, distance, provider,
      category, confidence, and ambiguity state.
- [ ] AI location safety result is evidence only and cannot alert/escalate
      directly.
- [ ] Parent policy is the only authority for notification/action/escalation.
- [ ] Parent acknowledgement and exceptions can suppress or modify alerts
      according to rules.
- [ ] Critical alerts cannot be suppressed by generic holiday/exception unless
      explicitly configured.
- [ ] Retention/delete/export behavior is implemented and tested.
- [ ] Remote sync and remote AI are disabled by default.
- [ ] Android background claims have real device permission/background proof.
- [ ] iOS background/region claims have real device permission/background
      proof.
- [ ] Desktop LAN/IP/Wi-Fi presence is labelled hint-only unless OS location
      proof exists.

## Base Workpacks

| Step | Workpack                                         | Status                      | Required proof                                                            |
| ---- | ------------------------------------------------ | --------------------------- | ------------------------------------------------------------------------- |
| 01   | Source index and repo reconciliation             | [x] Planning folder created | Source index and coverage audit.                                          |
| 02   | Current tracking snapshot and gap map            | [x] Planning folder created | Snapshot and gap map.                                                     |
| 03   | Contract boundary and Effect schemas             | [ ] Not started             | TypeScript schema tests and no-claim negative tests.                      |
| 04   | Location evidence model                          | [ ] Not started             | Valid/invalid schema tests and source/accuracy/freshness gates.           |
| 05   | Device status model                              | [ ] Not started             | Heartbeat, battery, connectivity, pending upload, degraded state tests.   |
| 06   | Permission and capability status model           | [ ] Not started             | Permission/platform/capability matrix tests.                              |
| 07   | Retention and custody model                      | [ ] Not started             | Deletion, tombstone, export, custody, no-default-remote tests.            |
| 08   | Android foreground location adapter              | [ ] Not started             | Android permission, fused/current/last known, and foreground proof.       |
| 09   | Android background location and geofence adapter | [ ] Not started             | Background permission, enter/exit/dwell, and active geofence limit proof. |
| 10   | Android battery connectivity and status adapter  | [ ] Not started             | Battery saver, offline, app killed, pending upload proof.                 |
| 11   | iOS Core Location foreground adapter             | [ ] Not started             | Authorization/current sample proof.                                       |
| 12   | iOS background region significant-change adapter | [ ] Not started             | Background/Always/region/significant-change/visit proof.                  |
| 13   | Desktop location and presence hint model         | [ ] Not started             | Hint-only negative tests for LAN/Wi-Fi/IP.                                |
| 14   | Geofence rule model                              | [ ] Not started             | Geometry, accuracy, grace, schedule, audit tests.                         |
| 15   | Geofence transition engine                       | [ ] Not started             | Transition, stale, ambiguity, evidence/rule refs tests.                   |
| 16   | Expected-place schedule engine                   | [ ] Not started             | School/home/activity/calendar/temporary schedule tests.                   |
| 17   | Parent acknowledgement and exception model       | [ ] Not started             | Acknowledgement, holiday, false-alarm, still-alert tests.                 |
| 18   | Child check-in flow                              | [ ] Not started             | Child copy, response, optional sample, audit, resolve-alert tests.        |
| 19   | Nearby-place provider abstraction                | [ ] Not started             | Provider abstraction, ambiguity, retention, degradation tests.            |
| 20   | Google Places and POI provider adapter           | [ ] Not started             | Field mask, bounded radius, response mapping, provider failure tests.     |
| 21   | Place-risk taxonomy and ambiguity model          | [ ] Not started             | Category, ambiguity, low-accuracy, no-accusation tests.                   |
| 22   | Local parent-defined place database              | [ ] Not started             | Parent-owned place CRUD, audit, export/delete tests.                      |
| 23   | AI location safety analysis contracts            | [ ] Not started             | AI input/output schema, source refs, confidence, no-final-action tests.   |
| 24   | AI provider routing                              | [ ] Not started             | Local/default, family hub, parent-approved remote, remote-disabled tests. |
| 25   | Policy compiler for tracking rules               | [ ] Not started             | Policy action compile, capability, exception, AI candidate tests.         |
| 26   | Alert severity and notification model            | [ ] Not started             | Severity, evidence refs, safe copy, action state tests.                   |
| 27   | Escalation engine                                | [ ] Not started             | Acknowledgement-aware escalation and provider minimization tests.         |
| 28   | Temporary live tracking mode                     | [ ] Not started             | Authorization, duration, disclosure, auto-expiry, retention tests.        |
| 29   | Missing-device mode                              | [ ] Not started             | Last known, offline, battery, connectivity, parent actions tests.         |
| 30   | Parent and child UI/UX surfaces                  | [ ] Not started             | Playwright states, accessibility, deleted-history, no-overclaim tests.    |
| 31   | Platform extension checklists and proof routing  | [ ] Not started             | Android/iOS/desktop extension proof routing.                              |
| 32   | Journal SQLite and read-model proof              | [ ] Not started             | Journal/replay/query/delete/read-model proof.                             |
| 33   | Proof gates fixtures rollout and PR gate         | [ ] Not started             | Full proof pack, blockers, docs/checklist/roadmap update discipline.      |

## Proof Pack Requirements

Each applicable proof pack must include:

- [ ] `00-source-snapshot.md`
- [ ] `01-contract-proof.log`
- [ ] `02-platform-permission-proof.md`
- [ ] `03-runtime-location-evidence.json`
- [ ] `04-device-status-proof.json`
- [ ] `05-geofence-transition-proof.json`
- [ ] `06-expected-place-proof.json`
- [ ] `07-nearby-place-proof.json`
- [ ] `08-ai-analysis-proof.json`
- [ ] `09-policy-alert-proof.json`
- [ ] `10-journal-sqlite-proof.json`
- [ ] `11-ui-snapshots/`
- [ ] `12-playwright-proof.log`
- [ ] `13-security-negative-proof.log`
- [ ] `14-retention-delete-proof.json`
- [ ] `15-manual-platform-proof.md`
- [ ] `16-validation-commands.log`

## Documentation Update Rule

Every implementation workpack must update, or explicitly justify not updating:

- `docs/features/location-geofence-device-status.md`;
- `docs/product-capability-checklist.md`;
- the assigned workpack doc;
- this checklist;
- expectation docs if acceptance contracts change;
- roadmap only if milestone scope, order, or completion changes.

## Progress Reconciliation

- [x] Feature doc exists.
- [x] Expectation doc exists.
- [x] Capability guide exists.
- [x] Schema proposal exists.
- [x] Raw tracking settings inventory exists.
- [x] Tracking plan folder exists.
- [ ] Location evidence contracts are not product-complete.
- [ ] Geofence transition runtime proof is not product-complete.
- [ ] Expected-place schedule engine is not product-complete.
- [ ] Nearby-place/AI safety analysis is not product-complete.
- [ ] Parent acknowledgement/exception system is not product-complete.
- [ ] Android background permission proof is not complete.
- [ ] iOS background/region proof is not complete.
- [ ] Retention/delete/export proof is not complete.
- [ ] Tracking UI/UX is not product-complete.

## UI Snapshot Gates

- [ ] Parent dashboard snapshots cover live, stale, offline, permission
      denied, low accuracy, ambiguous nearby place, alert, acknowledgement,
      exception, temporary live, missing device, and retention-deleted states.
- [ ] Child snapshots cover check-in, disclosure, safe/help responses, and
      location-share consent.
- [ ] Screenshots are stored under the assigned proof root.
- [ ] Accessibility output is stored with the UI proof.

## Evidence Quality Gates

- [ ] Every location-derived UI or alert cites evidence refs.
- [ ] Every geofence transition cites rule refs and source evidence refs.
- [ ] Every nearby-place result carries radius, provider, category, distance,
      confidence, and ambiguity state.
- [ ] Every AI result carries source refs and no final action.
- [ ] Every alert carries policy decision refs.
- [ ] Every retention/delete/export claim has before/after proof.

## Fixture And Manual Gates

- [ ] Fixtures cover fresh, stale, offline, denied, low accuracy, ambiguous,
      exception, acknowledged, check-in, temporary-live-expired,
      missing-device, retention-deleted, remote-sync-disabled, and
      remote-AI-disabled states.
- [ ] Android/iOS/manual desktop claims include real-device or explicitly
      approved manual proof.
- [ ] Unsupported platforms render manual-required state instead of fake
      capability.

## Explicit Merge-Blocking Checklist

Block PR-ready or merge-ready status when any of these are true:

- precise location is inferred from LAN/IP/pairing;
- stale data is displayed as live;
- nearby POI is displayed as exact child place without proof;
- AI triggers notification/escalation without policy decision;
- remote sync or remote AI runs by default;
- Android/iOS background behavior is claimed without manual proof;
- retention delete leaves visible cached history;
- workpack proof root or validation log is missing.

## Worker Report Template

```text
DONE <workpack-id> <branch> <commit>
Touched files:
Validation:
Proof artifacts:
Product docs/checklist updated:
Known gaps/manual-required:
```
