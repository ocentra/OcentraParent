# Tracking Agent Driver — Current State Audit

## Purpose

This file is the compact audit for the tracking plan execution layer.

The existing `docs/plans/tracking-plan/` folder is already token-aware and workpack-based. The missing layer is an execution driver that prevents agents from doing proof-only churn, scanning giant docs, or editing random tracking files without a selected workpack.

## Current routing surface

Default route is already present:

```text
README.md
  -> AGENTS.md
  -> PLAN_STATE.md
  -> NEXT_ACTIONS.md
  -> WORKPACK_INDEX.md
  -> selected workpack only
  -> exact checklist/proof rows only
```

Do not replace that route. This agent-driver pack strengthens it.

## Current plan state

Observed from current docs:

```text
tracking-plan has 33 indexed workpacks
28 workpacks have open checkboxes
5 workpacks are fully checked
implementation-checklist reports 111 rows, 79 checked, 32 unchecked
```

The highest-open workpacks are currently:

```text
WP01 Source Index And Repo Reconciliation              11 open
WP09 Android Background Location And Geofence Adapter  11 open
WP02 Current Tracking Snapshot And Gap Map             10 open
WP08 Android Foreground Location Adapter               10 open
WP10 Android Battery Connectivity And Status Adapter    6 open
WP11 iOS Core Location Foreground Adapter               6 open
WP12 iOS Background Region Adapter                      6 open
WP15 Geofence Transition Engine                         6 open
WP16 Expected-Place Schedule Engine                     6 open
WP17 Parent Acknowledgement And Exception Model         6 open
WP18 Child Check-In Flow                                6 open
WP20 Google Places / POI Provider Adapter               6 open
```

## Key current rule

The plan already states the correct implementation order:

```text
PLAN -> CODE -> TEST -> RUN/FIX -> PROOF -> DOC
```

This is now mandatory for every new implementation slice.

## Current repository implementation shape

TypeScript package:

```text
packages/tracking-domain
```

Current package description:

```text
Tracking location, geofence, expected-place, retention, and read-model contracts.
```

Current exports include:

```text
tracking
tracking-primitives
tracking-evidence
tracking-geofence
tracking-local-place-store-schemas
tracking-local-place-store
tracking-evidence-quality-gate
tracking-read-model
tracking-retention-runtime
tracking-runtime
```

Rust crate:

```text
crates/tracking-core
package: ocentra-tracking-core
```

Current Rust modules include:

```text
ai_boundary
alerting
child_check_in
expected_place
geofence
local_place
location_validation
missing_device
nearby_place
parent_acknowledgement
read_model
read_model_guard
retention_settings
runtime_flow
status
temporary_live
```

Service surface:

```text
crates/agent-service
```

Protocol surface:

```text
crates/agent-protocol
packages/agent-protocol-domain
```

Portal surface:

```text
apps/portal
packages/portal-domain
```

## Main risk

The branch contains a lot of proof accounting. That is useful, but not sufficient.

Reject work that only changes:

```text
proof harnesses
proof JSON
proof inventory docs
checklist status
claim-gate summaries
```

unless the selected workpack is explicitly proof-routing-only.

For implementation work, require real source behavior plus tests before proof/doc updates.

## Current technical smell to fix deliberately

`packages/tracking-domain/src/tracking.ts` is an aggregate re-export file:

```text
export * from './tracking-primitives';
export * from './tracking-evidence';
...
```

If the repo-level no-barrel/no-reexport rule is active, this file is a violation candidate. Do not add more aggregate re-export files. When changing exports, prefer explicit package export entries and direct imports.

Do not remove or rewrite this file casually; first check all consumers and package export expectations.

## What this agent-driver pack adds

This folder defines:

```text
00-current-state-audit.md       current state and risks
01-workpack-execution-rules.md  exact execution protocol
02-runtime-boundary-map.md      TypeScript/Rust/service/portal ownership
03-priority-slices.md           implementation slice selection
04-validation-proof-enforcement.md validation/proof gates
```

## Acceptance for using this pack

Before editing tracking code, an agent must state:

```text
Assigned workpack:
Implementation slice:
Source files expected to change:
Tests expected to change:
Proof artifact expected:
No-claim boundaries preserved:
```

If any field is unknown, stop and inspect the assigned workpack only. Do not scan every workpack.
