# 18 - Deterministic Policy Evaluator Integration

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
