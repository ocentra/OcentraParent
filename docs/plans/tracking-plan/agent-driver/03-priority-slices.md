# Tracking Agent Driver — Priority Implementation Slices

## Purpose

This file gives Codex a practical selection model for the next tracking work. It does not replace hub assignment. If the hub assigns a workpack, follow the hub assignment.

If no specific workpack is assigned, use this file to choose the smallest useful implementation slice.

## Slice selection rule

Prefer slices that add real source behavior and close a named workpack row.

Reject slices that only:

```text
- regenerate proof JSON
- update checklist status
- reword a workpack
- add another aggregate proof script
- add another artifact inventory without source behavior
```

unless explicitly assigned as proof-routing-only.

## Highest-value slice classes

### Class A — source truth/reconciliation

Workpacks:

```text
WP01 Source Index And Repo Reconciliation
WP02 Current Tracking Snapshot And Gap Map
```

Use when:

```text
- docs are stale against current source
- workpack proof paths disagree
- source ownership is unclear
- checklist claims conflict with implementation
```

Output must be reconciliation, not product claims.

Required behavior:

```text
- inspect exact source/docs named by the workpack
- update source-index/current snapshot only when source truth changed
- preserve manual-required/product-not-ready boundaries
- do not mark runtime behavior complete
```

### Class B — Rust runtime decision behavior

Workpacks:

```text
WP15 Geofence Transition Engine
WP16 Expected-Place Schedule Engine
WP17 Parent Acknowledgement And Exception Model
WP18 Child Check-In Flow
WP28 Temporary Live Tracking Mode
WP29 Missing-Device Mode
```

Use when:

```text
- platform-neutral tracking logic needs implementation or tests
- behavior can live in crates/tracking-core
- no physical-device proof is required for the slice
```

Preferred files:

```text
crates/tracking-core/src/geofence.rs
crates/tracking-core/src/expected_place.rs
crates/tracking-core/src/parent_acknowledgement.rs
crates/tracking-core/src/child_check_in.rs
crates/tracking-core/src/temporary_live.rs
crates/tracking-core/src/missing_device.rs
crates/tracking-core/src/runtime_flow.rs
crates/tracking-core/src/read_model.rs
```

Required tests:

```text
cargo test -p ocentra-tracking-core <focused-test>
```

### Class C — service/read-model integration

Workpacks:

```text
WP32 Journal SQLite And Read-Model Proof
WP30 Parent And Child UI/UX Surfaces
WP07 Retention And Custody Model
```

Use when:

```text
- service command path must expose a read model
- portal must render service-backed rows
- local durable state must be read back
```

Preferred files:

```text
crates/agent-service/src/*tracking*
crates/tracking-core/src/read_model*.rs
packages/agent-protocol-domain/src/*tracking*
apps/portal/src/**
packages/portal-domain/src/**
```

Required tests:

```text
cargo test -p ocentra-parent-agent-service <focused-test>
npm run test --workspace @ocentra-parent/portal -- <focused-test>
```

### Class D — Android local/emulator/platform routing

Workpacks:

```text
WP08 Android Foreground Location Adapter
WP09 Android Background Location And Geofence Adapter
WP10 Android Battery Connectivity And Status Adapter
WP31 Platform Extension Checklists And Proof Routing
```

Use when:

```text
- Android local proof scripts need to classify evidence correctly
- app-owned local listener evidence must stay separate from Android system delivery
- emulator/physical/manual tiers must remain explicit
```

Do not claim:

```text
Android system geofence delivery
physical-device geofence delivery
authority-enrolled behavior
product-ready Android tracking
```

unless the required proof artifact exists and the workpack explicitly allows it.

### Class E — provider/AI/policy boundary

Workpacks:

```text
WP19 Nearby-Place Provider Abstraction
WP20 Google Places And POI Provider Adapter
WP21 Place-Risk Taxonomy And Ambiguity Model
WP23 AI Location Safety Analysis Contracts
WP24 AI Provider Routing
WP25 Policy Compiler For Tracking Rules
WP26 Alert Severity And Notification Model
WP27 Escalation Engine
```

Use when:

```text
- ambiguity, provider status, AI-evidence-only, or policy authority needs correction
- provider execution must remain separate from fixture/provider-contract proof
- alert/escalation authority must stay parent-policy-owned
```

Do not allow:

```text
AI publishes policy/enforcement/notification directly
nearby-place evidence makes accusation/alert directly
provider unavailable becomes silent success
```

## Recommended default next work if unassigned

If no workpack is assigned and the goal is implementation progress, prefer Class B or C over Class A.

Default order:

```text
1. WP15 geofence transition engine if runtime behavior/tests are incomplete
2. WP16 expected-place schedule engine if runtime behavior/tests are incomplete
3. WP17 parent acknowledgement/exception model
4. WP18 child check-in flow
5. WP32 service read-model integration if service path needs coverage
6. WP30 portal UI rendering only after service read model is stable
```

Use WP01/WP02 only if the task is explicitly source reconciliation or current-state correction.

## Slice template

Every selected slice must fill this before editing:

```text
Slice id:
Assigned workpack:
Rows targeted:
Owner layer: TS contract | Rust core | protocol | service | portal | proof-routing
Files expected:
Tests expected:
Proof root:
No-claim boundaries:
Reason this is not proof-only churn:
```

## Example: Rust runtime slice

```text
Slice id: tracking-geofence-transition-runtime
Assigned workpack: WP15 Geofence Transition Engine
Rows targeted: geofence transition rules + ambiguous/stale/manual-required cases
Owner layer: Rust core
Files expected:
  crates/tracking-core/src/geofence.rs
  crates/tracking-core/src/runtime_flow.rs
  crates/tracking-core/tests/geofence_transition_tests.rs
Tests expected:
  cargo test -p ocentra-tracking-core geofence
Proof root:
  output/tracking-plan-proof/15-geofence-transition-engine/
No-claim boundaries:
  no Android/iOS physical geofence claim; platform proof remains manual-required
Reason not proof-only:
  modifies Rust transition behavior and tests before proof/docs
```

## Example: service read-model slice

```text
Slice id: tracking-read-model-service-command
Assigned workpack: WP32 Journal SQLite And Read-Model Proof
Rows targeted: tracking read-model command + replay/idempotency rows
Owner layer: service + Rust core
Files expected:
  crates/tracking-core/src/read_model.rs
  crates/agent-service/src/*tracking*
  crates/agent-protocol/src/*tracking*
Tests expected:
  cargo test -p ocentra-parent-agent-service tracking_read_model
Proof root:
  output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/
No-claim boundaries:
  service read-model proof only; no physical-device/product-ready claim
Reason not proof-only:
  adds or fixes actual service command/read-model behavior
```

## Example: portal slice

```text
Slice id: tracking-portal-service-backed-render
Assigned workpack: WP30 Parent And Child UI/UX Surfaces
Rows targeted: hosted route renders service-backed tracking state
Owner layer: portal
Files expected:
  apps/portal/src/**
  packages/portal-domain/src/**
Tests expected:
  npm run test --workspace @ocentra-parent/portal -- <focused-test>
Proof root:
  output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/
No-claim boundaries:
  hosted UI only; no platform runtime or physical-device claim
Reason not proof-only:
  modifies portal rendering and tests before screenshot/proof updates
```

## Do not split into tiny proof PRs

A valid PR-ready unit should close a named workpack slice or clearly explain remaining rows.

Avoid:

```text
- one proof note only
- one checklist tick only
- proof artifact refresh without behavior
- doc-only wording change that claims progress
```

Prefer:

```text
- one runtime behavior + tests + proof + workpack update
- one service path + tests + proof + workpack update
- one portal rendering state + tests/screenshot + proof + workpack update
```
