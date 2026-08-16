# Tracking UI/UX Requirements Guide

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `Tracking UI/UX Requirements Guide`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

Tracking UI must reduce parent uncertainty without shaming the child or
overstating weak evidence.

## Required Screens

- Tracking Dashboard
- Child Location Card
- Map/List View
- Location Evidence Drawer
- Geofence Editor
- Expected Place Schedule Editor
- Holiday/Exception Editor
- Alert Inbox
- Parent Acknowledgement Modal
- Child Check-In Request
- Temporary Live Tracking Panel
- Missing Device Mode
- Retention/Delete Settings
- Platform Capability Matrix

## Required Badges

- Live
- Recent
- Stale
- Last known
- Offline
- Permission denied
- Location disabled
- Battery throttled
- Low accuracy
- Ambiguous place
- Expected
- Not expected
- Acknowledged
- Exception active
- Escalating
- Critical

## Parent Dashboard Requirements

The parent dashboard needs:

- children map/list;
- last known location;
- freshness badge;
- accuracy circle;
- source and permission label;
- custody and retention label;
- battery and connectivity;
- expected place now;
- geofence status;
- recent transitions;
- alerts needing acknowledgement;
- exceptions/holiday mode;
- temporary live tracking;
- missing-device mode;
- platform capability matrix.

## Alert Card Requirements

Alert cards must show:

- child profile;
- evidence-backed headline;
- expected place;
- reported location or location class;
- accuracy;
- last update;
- source;
- custody;
- risk/severity;
- reason codes;
- recommended parent action;
- evidence drill-in;
- acknowledgement/exception controls.

Example shape:

```text
Aarav may not be at school

Expected: School, 8:30 AM - 3:20 PM
Reported location: near Cinema / Mall area
Accuracy: 42m
Last update: 2 minutes ago
Risk: Medium
Recommendation: Check in or acknowledge if expected

Actions:
[It is fine]
[Ask child to check in]
[Start live tracking 15 min]
[Call child]
[Create exception]
[View evidence]
```

## High Concern Card

```text
Aarav's device is near a hospital

Expected: Home
Reported location: near Lakeshore Hospital
Accuracy: 28m
Risk: Urgent

Actions:
[Call child]
[Ask check-in]
[Notify other parent]
[Acknowledge safe]
[Start live tracking]
```

The heading says the device is near a hospital. It does not claim the child is
inside the hospital unless the evidence and rule can prove that exact claim.

## Child Check-In UX

Child copy must be short, calm, and non-shaming.

```text
Your parent is asking you to check in.
Are you safe?

[I'm safe]
[Need help]
[Share current location]
[Call parent]
```

Avoid:

```text
You are in trouble.
AI thinks you are lying.
You are at a bad place.
```

## Copy Rule

Use:

```text
device reported near...
may not be where expected
possible urgent situation
```

Do not use:

```text
child is inside...
skipping school
emergency confirmed
```

## Empty And Degraded States

The UI needs first-class states for:

- tracking disabled;
- permission required;
- background permission missing;
- location service disabled;
- child device offline;
- battery-throttled updates;
- provider unavailable;
- parent cache only;
- remote sync disabled;
- remote AI disabled;
- unsupported platform;
- manual setup required;
- deleted by retention.

## States To Snapshot

Playwright/UI proof must snapshot:

- tracking off;
- permission required;
- background permission missing;
- live location;
- recent location;
- stale location;
- offline last-known only;
- low accuracy;
- ambiguous nearby place;
- expected place normal;
- not where expected;
- parent acknowledgement pending;
- exception active;
- child check-in requested;
- temporary live tracking active and expired;
- missing-device mode;
- retention-deleted history;
- unsupported/manual-required platform.

## UI Done Signal

UI work is done only when parent and child surfaces render every required state,
the screenshots are stored under the assigned proof root, accessibility checks
pass, deleted or stale data cannot be presented as live, and the feature doc or
checklist reflects the remaining product gaps.

## Accessibility And Safety

- Map/list must have equivalent status content for screen readers.
- Do not rely only on color for severity.
- Accuracy circle must have a text equivalent.
- Stale/offline points must be visibly distinct from live points.
- Deleted history must not remain visible in cached UI.
- Sensitive coordinates can be approximate when parent setting requires it.
- Provider notification previews should minimize sensitive location detail.
