# Policy, Schedules, And Approvals

## Parent Outcome

Parents can set household rules, schedules, time budgets, bonus time,
exceptions, ask-parent approvals, and overrides without editing files or
understanding device internals.

## Ocentra Requirement

Parent-authored policy is the authority. The portal authors intents. The
child-device agent validates rules, evaluates schedules, resolves conflicts,
records decisions, and executes only supported enforcement actions.

## Roadmap And Expectations

- Roadmap: V0.6 policy contracts, V0.7 dry-run evaluator, V5 parent policy
  product.
- Expectations: [policy](../expectations/policy.md),
  [family setup](../expectations/family-setup.md),
  [enforcement](../expectations/enforcement.md).
- Modules: `packages/parent-domain`, `packages/portal-domain`,
  `packages/agent-protocol-domain`, `crates/agent-service`.

## Competitor Pressure

See [Competitor Capability Map](../competitor-capability-map.md), especially
screen time schedules, app block/app limits, install approval/purchases, and AI
assistant setup/control.

Google, Apple, Microsoft, Qustodio, Bark, Kidslox, and others expose schedules,
downtime, app limits, school/bedtime modes, and exception requests. Ocentra must
match this usability while keeping policy local and auditable.

## Current Ocentra State

- Policy and dry-run decision contracts exist.
- Policy-preview service/API/read-model paths exist.
- Complete nontechnical authoring UX is not done.

## Current Gap

Profiles, schedule builder, rule templates, conflict UX, ask-parent flow, bonus
time, approvals, and full audit history are incomplete.

## Checklist

- [ ] Family/child/device policy scope.
- [ ] Rule targets across app/site/category/social/video/location.
- [ ] Schedule and time-budget builder.
- [ ] Conflict resolution.
- [ ] Dry-run preview.
- [ ] Ask-parent request/approval/expiry.
- [ ] Override and bonus-time flow.
- [ ] Audit and report integration.

## Next AI Instructions

Keep policy evaluation out of the browser. Add contracts and evaluator proof
before UI convenience. Never let AI output directly become enforcement.
