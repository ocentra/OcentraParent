<!-- agent-capsule -->

> Agent Capsule
> Plan: `network-plan`
> Doc: `Network Workpack Families`
> Kind: owner-path classifier for selected workpacks.
> Read when: only after `WORKPACK_INDEX.md` selects or names a workpack and the owner/proof family is unclear.
> Stop rule: classify the selected workpack only; do not use this file as permission to scan every workpack in the family.
> Proves: routing and owner-path classification only.
> Does not prove: network implementation, live capture, platform action readiness, mobile authority, product action readiness, private content proof, or PR readiness.
> Proof rule: if this file changes route/status claims, update `AGENTS.md`, `PLAN_STATE.md`, and any affected selected workpack route.

<!-- /agent-capsule -->

# Network Workpack Families

Use this file to classify a selected workpack before opening source. This plan owns network metadata/evidence proof and network handoff boundaries. It does not own browser exact-URL truth, screen analysis, AI runtime, policy authority, platform action execution, LAN/family-hub delivery, data custody policy, or portal UI ownership.

## Foundation contracts and eventing family

```text
Workpacks:
WP01 Foundation Contracts And Eventing

Owners:
schema-domain for canonical shared TypeScript network contracts
network-domain only as package metadata/proof-consumer unless explicit exports exist
ocentra-network-evidence for Rust evidence/proof helpers
agent-protocol/agent-service/core when selected
eventing-plan for reusable event bus semantics only

Rule:
Foundation proof must separate schemas, Rust parity, evidence grade, policy handoff, and eventing handoff. Network evidence can inform policy but cannot directly execute product actions.
```

## Passive capture, parser, and fixture replay family

```text
Workpacks:
WP02 Passive Capture And Parsing

Owners:
ocentra-network-evidence packet, DNS, host, TLS/SNI, replay, flow, and live-capture proof helpers
agent-core/agent-service only when selected for runtime capture/service path

Rule:
Fixture replay proves parser behavior only. Live capture proof requires platform/device artifacts, permission state, capture limits, redaction, and manual-required blockers where applicable.
```

## Classification and correlation family

```text
Workpacks:
WP03 Classification And Correlation

Owners:
ocentra-network-evidence classifier/category/domain/process/browser/app/screen correlation helpers
browser-plan for exact URL/browser authority
screen-plan for screen fallback
app-game/browser/tracking owners for their evidence sources

Rule:
Classification proof must carry evidence refs and ambiguity states. Network-only evidence must not become exact URL, exact video, private message, or application foreground truth.
```

## Cascade and parent surface family

```text
Workpacks:
WP04 Cross Slice Cascade And Parent Surface

Owners:
ocentra-network-evidence cascade/bundle/local AI queue/readiness helpers
agent-service/agent-core for service/runtime chain when selected
portal-domain/apps/portal for projection only
AI, policy, notification, data-custody owners through typed handoff only

Rule:
Cascade proof routes evidence to other slices; it does not prove AI runtime, policy decisioning, notification delivery, data custody, or portal-owned network truth.
```

## Platform action gate family

```text
Workpacks:
WP05 Intervention Adapter Proof Gates

Owners:
ocentra-network-evidence platform action gate helpers
v0-8-enforcement-control-plan for action authority and execution policy
platform owners when physical/device/entitlement proof is required

Rule:
Adapter proof must distinguish capability, authority, reversible lab action, rollback, unavailable state, audit event, and production no-claim. Contract/replay proof is not production action readiness.
```

## Analyzer, AI audit, and risk budget family

```text
Workpacks:
WP06 Analyzer AI Audit And Risk Budget

Owners:
ocentra-network-evidence AI audit/detection/risk budget helpers
ai-plan for AI runtime/provider execution
policy-control-plane-plan for policy decision semantics

Rule:
AI audit/risk proof can evaluate fixtures and produce bounded recommendations. It does not prove local model execution, remote provider execution, product actions, or final parent-facing decisions.
```

## Performance, security, and rollout family

```text
Workpacks:
WP07 Performance Security Rollout

Owners:
ocentra-network-evidence performance/security proof helpers
selected runtime/platform owners only when their proof roots are named
network-plan proof docs for rollout aggregation and no-claim boundaries

Rule:
Rollout proof may aggregate only accepted roots or exact carried blockers. It cannot convert checklist count, skeleton proof, catalog existence, or lab proof into production readiness.
```

## Control catalog reference-routing family

```text
Workpacks:
WP08 Control Catalog Reference Routing

Owners:
network control catalog docs as reference material only
selected WP01-WP07 or sibling plan when a specific control becomes implementation work

Rule:
The large control catalog, schema proposal, and settings inventory are source/reference material, not implementation proof. Open only the exact section/search hit needed for the selected control.
```
