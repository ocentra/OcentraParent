# V0.8 Policy Enforcement Handoff Plan

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
