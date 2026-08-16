<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-ai-pipeline-plan`
> Doc: `Screen AI Pipeline Workpack Families`
> Kind: owner-path classifier for selected workpacks.
> Read when: only after `WORKPACK_INDEX.md` selects or names a workpack and owner/proof family is unclear.
> Stop rule: classify the selected workpack only; do not use this file as permission to scan every workpack.
> Proves: routing and owner-path classification only.
> Does not prove: capture readiness, AI readiness, policy readiness, enforcement readiness, custody readiness, live-operator readiness, or PR readiness.
> Proof rule: if this file changes route/status claims, update `AGENTS.md`, `PLAN_STATE.md`, and any affected selected workpack route.

<!-- /agent-capsule -->

# Screen AI Pipeline Workpack Families

Use this file to classify a selected workpack before opening source. This plan owns cross-hop integration proof from screen evidence to AI result to policy/action boundary; it does not own sibling domain runtime surfaces.

## Prerequisite and route gate family

```text
Workpacks:
WP01 Prerequisite Merge And Branch Gate

Owners:
screen-ai-pipeline-plan for prerequisite truth, proof-root availability, architecture-gate status, and branch/merge blocker state
owning source plans for any blocked source surfaces

Rule:
Prerequisite proof is route hygiene only. It does not prove trigger, capture, AI, policy, action, custody, or live-operator behavior.
```

## Trigger-to-capture family

```text
Workpacks:
WP02 Real Trigger To Capture Gate

Owners:
screen-ai-pipeline-plan for trigger-to-capture scenario proof and structured skip proof
screen-plan for capture primitives, protected surfaces, disclosure, and screen settings
browser/app-game/network/tracking/domain owners for source-trigger truth when selected

Rule:
Trigger proof must produce a real capture job or structured skip. It is not AI analysis, policy, action, or product completion proof.
```

## Capture-to-analysis family

```text
Workpacks:
WP03 Capture To AI Analysis Gate

Owners:
screen-ai-pipeline-plan for capture-ref to AI-context/router/result integration proof
screen-plan for capture/evidence source and protected-surface boundary
ai-plan/schema-domain for AI context/result contracts and provider/runtime behavior

Rule:
AI analysis proof must preserve evidence refs, route decisions, degradation states, redaction, and queue deletion. It cannot claim policy authority.
```

## AI result to policy family

```text
Workpacks:
WP04 AI Result To Policy Gate

Owners:
screen-ai-pipeline-plan for schema-valid AI result to deterministic policy handoff proof
policy-control-plane-plan for policy source truth, parent-rule precedence, and policy decision semantics
ai-plan/schema-domain for AI result schema and confidence/degraded semantics

Rule:
Only schema-valid AI results with evidence refs and parent-rule refs can reach policy. AI never writes policy or enforcement state directly.
```

## Policy dry-run and action boundary family

```text
Workpacks:
WP05 Policy Action Dry-Run Gate

Owners:
screen-ai-pipeline-plan for observe/allow/warn/ask-parent/time-limit/block dry-run proof and action-boundary no-claims
policy-control-plane-plan for policy authority and preview/dry-run semantics
v0-8-enforcement-control-plan for actual adapter execution and rollback

Rule:
Dry-run/action handoff proof is not enforcement runtime proof. Unsupported browser/category/network/mobile/broad adapters remain explicit non-claims until their owners prove them.
```

## Journal/read-model/portal family

```text
Workpacks:
WP06 Journal Read Model And Portal Gate

Owners:
screen-ai-pipeline-plan for journal/read-model integration proof and route shape
agent-service/agent-protocol/portal-domain/apps/portal for selected service/protocol/projection seams
portal-ux-household-surfaces-plan for rendered parent-visible UI proof

Rule:
Read-model proof is not raw capture, model runtime, policy authority, or portal UX completion unless the selected proof root proves that tier.
```

## Custody/deletion family

```text
Workpacks:
WP07 Deletion Retention And Custody Gate

Owners:
screen-ai-pipeline-plan for pipeline raw image deletion and scenario custody proof
data-custody-storage-plan for retention/export/delete/privacy policy
screen-plan for screenshot/capture custody settings

Rule:
Raw images are temporary and local by default. Retention or upload claims require explicit custody proof and opt-in state.
```

## Live operator family

```text
Workpacks:
WP08 Live Operator Proof Gate

Owners:
screen-ai-pipeline-plan for manifest-driven real URL/app operator proof and artifact-gate non-claims
browser/app-game/screen/AI/policy/domain owners for source truth when their trigger/runtime behavior is selected

Rule:
Live operator proof can prove operator-supplied scenarios only. Artifact-gate proof rechecks retained outputs; it does not rerun capture.
```

## Performance, cadence, and backpressure family

```text
Workpacks:
WP09 Performance Cadence And Backpressure Gate

Owners:
screen-ai-pipeline-plan for cadence, queue, backpressure, deletion, and degradation proof
agent-service/agent-core for selected runtime queue behavior

Rule:
Cadence proof must include queue pressure, disabled/no-new-work state, deletion/custody, and degraded/manual-required behavior. It is not classification or policy proof by itself.
```

## Final rollout family

```text
Workpacks:
WP10 Final Rollout And PR Gate

Owners:
selected proof roots under `output/screen-ai-pipeline-proof/`
PLAN_STATE, WORKPACK_INDEX, NEXT_ACTIONS, PROOF_INDEX, TEST_PROOF_EXPECTATIONS, PLAN_HEALTH, and selected workpacks when state changes

Rule:
Final rollout may aggregate only retained proof roots or exact carried blockers. Missing proof root, missing manifest, mock-only proof, source-only proof, or proof-shape drift blocks PR_READY.
```
