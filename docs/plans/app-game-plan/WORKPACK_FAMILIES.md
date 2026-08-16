<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `App + Game Workpack Families`
> Kind: owner-path classifier for selected workpacks.
> Read when: only after `WORKPACK_INDEX.md` selects or names a workpack and the owner path is unclear.
> Stop rule: classify the selected workpack only; do not use this file as permission to scan every workpack in the family.
> Proves: routing and owner-path classification only.
> Does not prove: workpack completion, implementation correctness, product readiness, or PR readiness.
> Proof rule: if this file changes route/status claims, update `AGENTS.md`, `PLAN_STATE.md`, and any affected selected workpack route.

<!-- /agent-capsule -->

# App + Game Workpack Families

Use this file to classify a selected workpack before opening source. It exists because `WORKPACK_INDEX.md` is large and contains checked rows, possible-done rows, reference rows, generated handoff rows, platform proof rows, timer chains, notification chains, and parent/child UX chains.

## Route/status/reconciliation family

```text
Examples:
01 contract/source/index style rows
02 source reconciliation
03 current snapshot/gap map
28 E2E/manual proof/rollout/PR gate
176 blocking/time-limit done gate

Owners:
docs/plans/app-game-plan
output/app-game-plan-proof/<workpack>/
feature/product docs only when a product claim changes

Rule:
No source edit unless the selected workpack names the source owner. These rows cannot prove runtime readiness by themselves.
```

## Canonical schema and contract family

```text
Examples:
identity model
inventory evidence model
runtime evidence model
foreground evidence model
launcher evidence/candidate model
AI classifier digest boundary
approval authority/action-result contracts
platform authority matrix
policy target/compiler contracts
time budget/session/child UX contracts
Android/platform proof contract rows

Owners:
the owning Rust crate first when shapes cross package/crate/app/plan boundaries
packages/schema-domain only as a generated-validation or edge-decoder surface
crates/agent-protocol for canonical wire/evidence contracts
crates/agent-core, app-game-core, agent-service, parent-runtime-core, portal, or
Android Java only when the selected workpack maps that exact runtime layer

Rule:
Do not re-create removed App/Game TypeScript owners. Direct imports from
policy/enforcement/notification/portal are not the way to share contracts; use
the owning Rust boundary and generated edges only where needed.
```

## Adapter and source-observation family

```text
Examples:
Windows installed inventory adapter
Windows Store/UWP/AppX/MSIX inventory adapter
Windows process runtime evidence adapter
Windows foreground app/game evidence adapter
Android UsageEvents capability/runtime/package proof
Apple/Linux platform preflight rows
live process snapshot source
service capture bridge
recurring freshness rows

Owners:
crates/app-game-core
selected platform adapter/proof file when named
agent-service only for service bridge work explicitly selected

Rule:
Adapters observe source facts and capability states. They do not call AI, policy, enforcement, notification, or portal internals. Store observations before downstream consumers act.
```

## Journal, SQLite, service read-model family

```text
Examples:
journal and SQLite ingest
journal/SQLite authority/classifier storage
read models and service events
service app/game read model rows
policy readiness service read model
notification service read model
source-gated service readiness/read API rows

Owners:
crates/app-game-core for app/game runtime event production when selected
agent-service for service/read API projection when selected
crates/schema and agent-protocol for shared row and wire shapes; schema-domain only as temporary generated-validation or edge-decoder surface

Rule:
Staged journal/read-model proof is not live source subscription proof. Service rows must carry evidence refs and source/freshness status instead of raw private source state.
```

## AI classifier/digest handoff family

```text
Examples:
AI classifier digest boundary
app/game unknown classifier lane
category/risk route rows
AI classifier result rows
browser/game/cloud-game bridge rows when assigned to this plan

Owners:
app-game owns source evidence and digest inputs
aI-plan owns model/provider/classifier runtime behavior
crates/schema or the owning Rust crate owns shared digest/result shapes; schema-domain is transitional validation only
policy/enforcement consume only validated results through handoff

Rule:
AI consumes stored app/game evidence or structured digests. AI does not scan the OS, launcher, process table, windows, files, or device state directly. AI output is evidence/classification input, not enforcement authority.
```

## Policy, timer, budget, and source-readiness family

```text
Examples:
native game budgets and launcher policy
policy target compiler for app/game rules
time budget, schedule, bonus-time integration
source-gated policy preview timer rows
policy readiness read models
runtime readiness rows
scheduler persistence rows
audit rollback rows

Owners:
policy-control-plan/policy-domain for policy compiler/evaluator semantics
app-game-plan for app/game source evidence and readiness facts
crates/schema or the owning Rust crate for shared policy-readiness/app-game handoff shapes; schema-domain only as transitional validation if still needed

Rule:
Policy consumes app/game source readiness and session evidence. Policy dry-run proof is not enforcement proof. Stale, missing, unavailable, permission-limited, manual-required, and not-claimed source rows must block preview/compile claims.
```

## Authority, approval, and action-result family

```text
Examples:
unknown app/game approval flow
approval authority/action-result rows
platform authority matrix
remote/view/control authority refs when named
export/delete or billing owner refs when named

Owners:
account-identity-family-plan owns actor/household/role/device/session authority
app-game-plan owns app/game target evidence and source status
policy/enforcement own policy decision/action execution
notification owns delivery

Rule:
Approval proof must include expiry, replay, stale actor/device, wrong household, and manual-required states when applicable. App/game may emit or consume authority refs but must not re-own account authority.
```

## Enforcement and adapter execution family

```text
Examples:
owned-process terminate proof
broad blocking proof gates
adapter dispatch preflight/result/readiness rows
block/terminate/time-limit/ask-parent rows
rollback/teardown rows

Owners:
enforcement-control plan and child-enforcement-core for enforcement semantics/adapters
app-game-core for app/game target/evidence readiness when selected
agent-service/child-runtime only through typed delivery/receipt handoff when selected

Rule:
Manual-required, observe-only, dry-run, unsupported, unavailable, permission-limited, and adapter-error states are first-class results. Do not claim block/terminate readiness from UI, schema, or policy dry-run alone.
```

## Parent portal, parent-surface, and child UX family

```text
Examples:
parent portal app/game dashboard surfaces
policy readiness portal renderer
notification parent-surface intent/renderer/read-model rows
child-facing warning/request UX
child UX local outbox/receipt/action-card rows
portal platform limitation rows

Owners:
portal-domain and apps/portal for rendering/projection only
notification-domain for notification intent/outbox/scheduler/provider status
child-runtime/child UX plans for child-side delivery/receipt when selected
app-game-plan provides source/status/read-model inputs

Rule:
Portal can show evidence/status/action result refs. Portal does not observe OS state, classify apps, run timers, enforce, or make a stale source ready.
```

## Notification, outbox, and scheduler family

```text
Examples:
notification intent contract
notification service read model
local outbox bridge
scheduler bridge
audit-history bridge
provider/preference preflight/status rows
parent-surface notification rows

Owners:
notification-domain/notification plan for notification semantics
app-game-plan for app/game trigger/source status
crates/schema or the owning Rust crate for shared handoff shapes; schema-domain only as transitional validation if still needed

Rule:
Notification delivery is a handoff. It does not prove app/game source readiness, policy readiness, enforcement execution, or parent UI readiness unless those proof roots are also present.
```

## Platform proof and preflight family

```text
Examples:
Apple CI platform proof preflight
Linux Docker host preflight
Android UsageEvents capability/runtime/count/package proof
Android accessibility proof rows
platform authority matrix

Owners:
app-game-plan for app/game-specific platform evidence expectations
platform-specific runtime/adapter plan or crate when named
crates/schema or the owning Rust crate for shared platform proof shapes; schema-domain only as transitional validation if still needed

Rule:
Platform preflight is not platform parity. Real platform readiness needs device/OS/version, permission state, adapter output, cleanup/rollback, negative cases, and manual-required notes where capability is absent.
```

## Rollout and PR gate family

```text
Examples:
rollout checklist and PR gate
broad blocking proof gates
E2E/manual proof gates
performance and service health
security/privacy gates

Owners:
selected proof root plus plan status docs
product/feature docs only when product status changes

Rule:
No PR_READY from checked generated handoff rows alone. No PR_READY from staged proof alone. No PR_READY from portal rows without source/service/protocol/runtime proof. No PR_READY from policy dry-run without adapter/manual-required proof.
```
