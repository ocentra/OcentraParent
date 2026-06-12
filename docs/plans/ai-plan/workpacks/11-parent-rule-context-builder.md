# 11 - Parent Rule Context Builder

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `11 - Parent Rule Context Builder`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

AI context includes only explicit parent rules, schedules, approvals, policy
versions, child/device refs, and effective windows.

## Where We Are

Policy and policy preview contracts exist. AI context must cite exact parent
rule refs so policy can explain the final action.

## Checklist

- [ ] Select rules by child/device/target/time.
- [ ] Include policy version and effective window.
- [ ] Include parent approvals and overrides.
- [ ] Include stricter-rule precedence.
- [ ] Reject context without rule refs when policy action depends on rules.

## Proof

- Parent rule context tests.
- Policy conflict tests.
- Explanation cites parent rule refs.
