<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-plan`
> Doc: `Native Apps Workpack Families`
> Kind: owner-path classifier for selected workpacks.
> Read when: only after `WORKPACK_INDEX.md` selects or names a workpack and the owner path is unclear.
> Stop rule: classify the selected workpack only; do not use this file as permission to scan every workpack in the family.
> Proves: routing and owner-path classification only.
> Does not prove: workpack completion, implementation correctness, product readiness, or PR readiness.
> Proof rule: if this file changes route/status claims, update `AGENTS.md`, `ROUTE_INDEX.md`, `PLAN_STATE.md`, and any affected selected workpack route.

<!-- /agent-capsule -->

# Native Apps Workpack Families

Use this file to classify a selected workpack before opening source. This plan is app-only and legacy/reconciliation-oriented; shared native app/game implementation usually belongs to `app-game-plan` unless the selected workpack explicitly narrows an app-only slice here.

## Route/status/reconciliation family

```text
Examples:
source index and doc reconciliation
current app snapshot and gap map
rollout checklist and PR gate
legacy app-plan reconciliation rows

Owners:
docs/plans/app-plan
output/app-plan-proof/<workpack>/
feature/product docs only when an app-specific product claim changes

Rule:
No source edit unless the selected workpack names the source owner. Route normalization does not prove runtime support.
```

## Canonical schema and contract family

```text
Examples:
contract boundary and Effect schemas
app identity model
installed app inventory model
app category and risk taxonomy
app AI classifier digest boundary
policy target/compiler contracts
source freshness/readiness contracts
child-facing app status contracts

Owners:
packages/schema-domain first when shapes cross package/crate/app/plan boundaries
selected helper/projection package only when the workpack names it
crates/agent-protocol only when Rust/wire parity is selected

Rule:
Do not invent an `app-domain` owner unless a selected implementation creates it. Shared native-app shapes should move through schema-domain or another neutral shared boundary.
```

## Rust observation and event family

```text
Examples:
app observation intent
app evidence-recorded event
app AI-analysis-requested event
app policy-evaluation-requested event
Rust protocol evidence identity parity
Rust protocol authority classifier parity

Owners:
crates/app-core
crates/agent-protocol only when wire parity is selected
crates/agent-service only when service proof is selected

Rule:
App observation emits events and requests. It does not import AI, policy, enforcement, notification, or portal runtime internals.
```

## Installed inventory source family

```text
Examples:
installed app inventory model
Windows installed app inventory adapter
Windows Store/UWP/AppX/MSIX inventory adapter
live Windows inventory source
Windows store package source
Windows registry inventory source
service inventory capture bridges

Owners:
app-plan for app-only inventory meaning
app-game-plan for shared native app/game evidence spine when the selected row is shared
app-core/platform adapter proof when source observation is selected

Rule:
Inventory proves app presence/source status, not app usage, foreground activity, policy readiness, or enforcement readiness.
```

## Runtime/process and foreground family

```text
Examples:
Windows process runtime evidence adapter
Windows foreground app evidence adapter
live process snapshot source
service capture app/game live process bridge
live foreground window source
service foreground capture bridge
recurring freshness rows

Owners:
app-core/platform adapter proof for app-only observation
app-game-plan for shared runtime/foreground chains
agent-service only for selected service projection work

Rule:
Runtime is not foreground. Foreground is not content. Window/process evidence must become stored evidence/read-model rows before AI, policy, portal, or enforcement consumes it.
```

## Journal, SQLite, and service read-model family

```text
Examples:
journal and SQLite app ingest
journal/SQLite authority classifier storage
app read models and service events
backend source freshness read model
source-gated service readiness/read API rows

Owners:
app-game-plan for shared evidence spine unless the selected row is app-only
agent-service for service/read API projection when selected
schema-domain/agent-protocol for shared row and wire shapes

Rule:
Staged journal/read-model proof is not live source subscription proof. Service rows must preserve evidence/source refs and manual-required states.
```

## Policy, timer, and source-readiness family

```text
Examples:
policy target compiler for app rules
time budget schedule bonus-time integration
source freshness policy consumption
source freshness preview gate
source-gated policy preview timer rows
policy readiness portal renderer

Owners:
policy-control-plan/policy-domain for policy compiler/evaluator semantics
app-plan/app-game-plan for app source evidence and readiness facts
schema-domain for shared handoff shapes

Rule:
Policy dry-run is not enforcement. Stale, missing, unavailable, permission-limited, manual-required, and not-claimed source rows must block preview/compile claims.
```

## Approval, install, uninstall, and store handoff family

```text
Examples:
new app and unknown app approval flow
install and uninstall approval handoff
install/package handoff proof
store/package handoff rows

Owners:
setup-install-provisioning-plan or payment/subscription when install/purchase flow owns the transaction
app-plan/app-game-plan for app evidence and inventory deltas
account-identity-family-plan for actor/household/role authority
notification/portal for their own handoff surfaces

Rule:
Approval proof must include expiry, replay, stale actor/device, wrong household, and manual-required states when applicable. App evidence does not own install/purchase transaction authority.
```

## Enforcement and adapter execution family

```text
Examples:
Windows owned process terminate time-limit proof
broad blocking proof gates
adapter dispatch/preflight/action-result rows
rollback/teardown rows

Owners:
v0-8-enforcement-control-plan and child-enforcement-core for enforcement semantics/adapters
app-core/app-game-core for app target/evidence readiness when selected
agent-service/child-runtime through typed delivery/receipt handoff only when selected

Rule:
Manual-required, observe-only, dry-run, unsupported, unavailable, permission-limited, and adapter-error states are first-class. Do not claim block/terminate readiness from schema, portal, or policy dry-run alone.
```

## Portal, parent-surface, child UX, and notification family

```text
Examples:
parent portal app inventory/running/session surfaces
portal source freshness surface
child-facing app warning/block/request UX
notification intent/service/read-model/outbox/scheduler/provider/preference rows

Owners:
portal-domain/apps/portal for rendering/projection only
notification-domain/notification plan for notification semantics
child-runtime/child UX surfaces when selected
app-plan/app-game-plan provides source/status/read-model inputs

Rule:
Portal and notification can show app evidence/status/action-result refs. They do not observe OS state, classify apps, run timers, enforce, or make stale source ready.
```

## Platform proof and preflight family

```text
Examples:
platform extension checklist and proof routing
Windows app source proof
Android/iOS/macOS/Linux capability or limitation rows
platform authority matrix

Owners:
app-plan for app-specific platform evidence expectations
app-game-plan for shared app/game platform matrices
platform-specific runtime/adapter plan or crate when named
schema-domain for shared platform proof shapes

Rule:
Platform preflight is not platform parity. Real readiness needs OS/version, permission state, adapter output, cleanup/rollback, negative cases, and manual-required notes where capability is absent.
```

## Rollout and PR gate family

```text
Examples:
e2e and manual proof artifacts
rollout checklist and PR gate
performance and service health
broad blocking proof gates

Owners:
selected proof root plus plan status docs
feature/product docs only when product status changes

Rule:
No PR_READY from route normalization alone. No PR_READY from app-game-plan proof unless this plan names the app-only handoff. No PR_READY from package preview/scaffold, portal row, policy dry-run, or platform preflight alone.
```
