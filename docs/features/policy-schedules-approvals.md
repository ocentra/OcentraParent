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
- The V0.8 product-control spine now carries dry-run preview, ask-parent,
  approval/override audit, restart recovery, and rollback/audit states as
  separate typed parent-visible actions.
- The V0.8 product-control runtime path now exposes those policy/audit states
  through a Rust service WebSocket read model and typed agent-protocol adapter
  that keep dry-run preview separate from adapter execution.
- The V0.8 policy-dispatch proof now validates parent-authored dispatch intents
  with actor, device, policy decision, schedule/time-budget reference, evidence
  refs, approval state, override/audit refs, route/source state, and timer
  recovery state before reporting dispatch-ready or rejected states.
- Complete nontechnical authoring UX is not done.

## Current Gap

Profiles, schedule builder, rule templates, conflict UX, ask-parent flow, bonus
time, approvals, and full audit history are incomplete. The new dispatch proof
types approval/override state and audit refs but does not claim notification
delivery or finished parent approval UX.

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
