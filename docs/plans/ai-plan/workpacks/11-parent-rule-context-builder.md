# 11 - Parent Rule Context Builder

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
