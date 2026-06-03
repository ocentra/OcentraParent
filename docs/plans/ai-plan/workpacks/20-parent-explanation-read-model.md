# 20 - Parent Explanation Read Model

## Target State

Parents can see what was decided, why, what evidence and rules were used, and
what AI/model state was involved.

## Where We Are

Parent assistant and portal runtime details exist. AI explanation needs a
dedicated read model that cites evidence and parent rules without exposing raw
child data unnecessarily.

## Checklist

- [ ] Define explanation read model.
- [ ] Include policy action and AI evidence result.
- [ ] Include evidence refs and parent-rule refs.
- [ ] Include confidence/degraded state.
- [ ] Include model/runtime and prompt refs.
- [ ] Render portal explanation UI.

## Proof

- Explanation contract tests.
- Portal Playwright proof.
- UI screenshots for normal, degraded, and unavailable states.
