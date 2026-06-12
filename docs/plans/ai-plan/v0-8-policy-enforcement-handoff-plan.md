# V0.8 Policy Enforcement Handoff Plan

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `V0.8 Policy Enforcement Handoff Plan`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Goal

Allow enforcement adapters to consume deterministic policy decisions that may
include AI evidence, without letting AI enforce directly.

## Handoff Shape

```text
AI result
  -> schema validation
  -> deterministic policy evaluator
  -> policy decision
  -> audit event
  -> enforcement adapter
```

## Required Policy Decision Fields

- policy decision id;
- policy version;
- child/device refs;
- target refs;
- action;
- timer/expiry where relevant;
- evidence refs;
- AI result refs when used;
- parent-rule refs;
- confidence/degraded state;
- rollback/unavailable behavior;
- enforcement capability refs.

## Rejections

- AI result contains direct block/kill/limit command.
- AI result has no parent-rule refs.
- AI result has no evidence refs.
- AI result conflicts with stricter parent rule.
- AI result is low confidence and policy lacks safe degraded mapping.
- Enforcement adapter receives raw model output.

## Validation

- Policy integration tests for allow, warn, time-limit, ask-parent, block, and
  unknown.
- AI cannot override stricter parent rule.
- Enforcement consumes policy only.
- Adapter unavailable produces explicit unavailable/audit state.
- Restart/replay preserves decision trace.
