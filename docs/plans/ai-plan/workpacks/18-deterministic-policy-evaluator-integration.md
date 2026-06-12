# 18 - Deterministic Policy Evaluator Integration

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `18 - Deterministic Policy Evaluator Integration`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Policy consumes schema-valid AI evidence and explicit parent rules. AI never
becomes policy authority.

## Where We Are

Policy dry-run evaluator exists in `crates/agent-core`. Enforcement dispatch
proof pieces exist. AI policy integration needs full result/journal/explanation
proof.

## Checklist

- [ ] Feed valid AI result into policy evaluator.
- [ ] Include parent-rule refs and policy version.
- [ ] Apply stricter-rule precedence.
- [ ] Map low confidence to safe policy state.
- [ ] Journal AI and policy result.

## Proof

- Policy tests for allow/warn/time-limit/ask-parent/block/unknown.
- AI cannot override stricter rule.
- AI result with missing refs is rejected before policy.
