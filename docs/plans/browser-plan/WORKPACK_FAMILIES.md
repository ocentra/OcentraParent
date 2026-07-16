<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `Browser Workpack Families`
> Kind: owner-path classifier for selected workpacks.
> Read when: only after `WORKPACK_INDEX.md` selects or names a workpack and the owner path is unclear.
> Stop rule: classify the selected workpack only; do not use this file as permission to scan every workpack in the family.
> Proves: routing and owner-path classification only.
> Does not prove: workpack completion, implementation correctness, product readiness, or PR readiness.
> Proof rule: if this file changes route/status claims, update `AGENTS.md`, `PLAN_STATE.md`, and any affected selected workpack route.

<!-- /agent-capsule -->

# Browser Workpack Families

Use this file to classify a selected workpack before opening source. This plan has large reference inventories; do not open reference/control inventories unless the selected workpack names that exact file.

## Route/status/reconciliation family

```text
Examples:
01 contract boundary and Effect schemas
02 source index and doc reconciliation
23 E2E and manual proof artifacts
24 rollout checklist and PR gate

Owners:
docs/plans/browser-plan
output/browser-plan-proof/<workpack>/
feature/product docs only when a browser product claim changes

Rule:
No source edit unless the selected workpack names the source owner. Route normalization and reference cleanup do not prove runtime support.
```

## Canonical schema and browser contract family

```text
Examples:
browser family/channel
capability status
managed session status
active tab state
browser tab evidence
browser read model
browser intervention rows/read model
browser control identifiers/catalog values
policy value/update contracts

Owners:
packages/schema-domain first when shapes cross package/crate/app/plan boundaries
packages/browser-domain only as helper/projection/focused validation surface
crates/agent-protocol only when Rust/wire parity is selected

Rule:
Do not re-create shared browser shapes in sibling feature domains. Direct imports from policy/enforcement/notification/portal are not a contract-sharing strategy; move shared shapes through schema-domain or another neutral boundary.
```

## Browser inventory and support-matrix family

```text
Examples:
03 browser inventory model
04 Windows browser inventory adapter
05 cross-platform inventory matrix
browser control coverage matrix
browser control settings inventory
browser policy settings catalog

Owners:
browser-plan for browser support/source meaning
browser-domain/schema-domain for contract/projection shapes when selected
browser-core/platform proof when source observation is selected

Rule:
Inventory proves detectable browser/support state, not managed URL capture, exact active tab evidence, policy readiness, or enforcement readiness.
```

## Managed profile, launcher, and bridge custody family

```text
Examples:
06 managed profile store
07 managed Chromium launcher
08 bridge custody and security
21 extension and native host boundary
managed/unmanaged browser reference workpack

Owners:
browser-plan for managed browser profile/session/bridge contracts
browser-core/agent-service only when runtime/service proof is selected
schema-domain for cross-boundary bridge/profile/custody shapes

Rule:
Managed profile/bridge proof must include custody, redaction, permission, restart, stale, degraded, and native-host/extension validation where applicable. It does not prove policy action or enforcement.
```

## CDP, target, tab, URL, and active-tab evidence family

```text
Examples:
09 CDP version and target adapter
10 tab evidence mapper
11 active-tab proof model
browser URL/video evidence rows
browser-game URL shape parser rows
runtime signal detector rows
metadata extractor rows

Owners:
browser-plan for exact URL/tab source boundaries
browser-core/agent-service when source or service proof is selected
schema-domain for evidence/read-model shapes
AI consumes only stored evidence or structured digests

Rule:
Target lists are not active-tab proof. Process/window/network evidence is not exact URL proof. URL/title/domain evidence must be source-labeled, journaled, and redacted/custody-gated before portal, AI, or policy consumes it.
```

## Journal, SQLite, service read-model family

```text
Examples:
12 journal and SQLite browser ingest
13 browser read models and service events
browser-game journal/SQLite read-model rows
service/browser intervention read models

Owners:
browser-core for browser event production when selected
agent-service for service/read API projection when selected
schema-domain/agent-protocol for shared row and wire shapes

Rule:
Staged journal/read-model proof is not live browser-source proof. Service rows must carry evidence refs, source/custody labels, and freshness/degraded states instead of raw private source payloads.
```

## Policy authoring and target compiler family

```text
Examples:
15 browser policy authoring manifest
16 policy target compiler
browser policy questionnaire forest
browser policy settings catalog
browser-game parent policy compiler rows

Owners:
policy-control-plan/policy-domain for policy compiler/evaluator semantics
browser-plan for browser source evidence and target readiness facts
schema-domain for shared policy/browser handoff shapes

Rule:
Policy authoring is not intervention execution. Candidate policy rows must stay dry-run/manual-required until source readiness, actor authority, service, intervention/action, and audit proof exist.
```

## Managed intervention and block-page family

```text
Examples:
17 managed intervention and block page
browser intervention rows/read model
browser-game managed hold/block adapter rows
child-facing delivery status rows when selected

Owners:
browser-plan for managed-browser intervention surfaces
policy/enforcement own deterministic decision/action authority
portal/child UX surfaces render status only when selected

Rule:
Managed intervention harness proof is not product-level warning/block readiness unless typed policy decision refs, action refs, audit refs, child delivery state, and portal proof are present.
```

## Unmanaged fallback and OS control family

```text
Examples:
18 unmanaged browser detection
19 unmanaged fallback UX and actions
20 Windows AppLocker and App Control proof
unmanaged process terminate/warn proof paths

Owners:
browser-plan for unmanaged browser detection/source status
v0-8-enforcement-control-plan or platform-specific owners for enforcement/AppLocker/App Control execution
agent-service/child-runtime only through typed handoff when selected

Rule:
Unmanaged process detection is not exact URL evidence. OS-block/AppLocker/App Control readiness needs real platform artifacts, permissions, rollback, negative cases, and manual-required states.
```

## Browser-game, social/video, and cloud-gaming family

```text
Examples:
browser-game URL shape parser
runtime signal detector
metadata extractor
hidden analysis profile safety
portal pattern library
cloud-gaming pattern library
educational classifier rows
social/video control rows

Owners:
browser-plan for browser-game/source evidence patterns
AI plan for model/provider/classifier runtime where selected
policy/enforcement consume only validated evidence/result handoffs
schema-domain for shared digest/result/pattern shapes

Rule:
Browser-game rows must reject raw page bodies, raw frame/audio/gamepad payloads, child cookie/session reuse, final policy decisions, native game control, and enforcement unless a selected proof explicitly owns those boundaries.
```

## Portal/status surface family

```text
Examples:
14 portal browser status surfaces
portal recent browser activity view
browser intervention portal proof
browser settings/control UI surfaces

Owners:
portal-domain and apps/portal for rendering/projection only
browser-plan provides source/status/read-model inputs
agent-service when service data is selected

Rule:
Portal can show browser evidence/status/action-result refs. Portal does not capture browser state, infer exact URLs, classify content, run policy, or enforce.
```

## Performance and service health family

```text
Examples:
22 performance and service health
100-tab CDP target mapping
10000-event SQLite replay
rapid bridge reconnect
portal 100-tab render
local AI queue timeout

Owners:
selected runtime/service/proof root
browser-core/agent-service when runtime/service paths are selected

Rule:
Fixture-backed measured proof must stay separate from manual-required future/runtime paths. Performance proof does not prove source custody, policy, or intervention readiness by itself.
```

## Rollout and PR gate family

```text
Examples:
23 E2E and manual proof artifacts
24 rollout checklist and PR gate
broad browser readiness claims

Owners:
selected proof root plus plan status docs
feature/product docs only when product status changes

Rule:
No PR_READY from settings inventory/reference rows, managed-intervention harness alone, CDP target list alone, unmanaged detection as exact URL evidence, portal UI alone, or policy authoring alone.
```
