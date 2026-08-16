<!-- agent-capsule -->

> Agent Capsule
> Doc: Policy, Schedules, And Approvals
> Kind: feature documentation; read only when selected by FEATURE_ROUTE_INDEX, PLAN_INDEX, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

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
- Browser/social approval planning:
  [social platform account feed and gating plan](../plans/browser-plan/v0-5-social-platform-account-feed-gating-plan.md).
- Browser-game approval planning:
  [browser games/cloud gaming gating plan](../plans/browser-plan/v0-5-browser-games-cloud-gaming-gating-plan.md).
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
- App/game policy target compiler contracts now validate parent-authored
  app/game target compile requests against identity, unknown-state, category,
  schedule, capability, authority, device, local-user, and freshness proof,
  while keeping unproved block-launch in manual-required dry-run output.
- App/game time-budget contracts now tie stored session refs, schedule proof,
  bonus-time approval/audit refs, ask-parent/manual-required dry-run states,
  and restart-recovered timer refs to parent-visible budget decisions.
- Child-facing app/game UX contracts now represent ask-parent and more-time
  request states with approval, evidence, child reason, and child status refs
  while keeping manual-required and unavailable states separate from adapter
  execution.
- Complete nontechnical authoring UX is not done.

## Current Gap

Profiles, schedule builder, rule templates, conflict UX, ask-parent flow, bonus
time, approvals, and full audit history are incomplete. The dispatch, app/game
target compiler, and app/game time-budget proofs type approval/override state,
bonus-time refs, audit refs, target proof, timer recovery proof, and
manual-required dry-run boundaries, but they do not claim notification
delivery, finished parent approval UX, portal authoring, or runtime adapter
execution. The child-facing UX proof types request and child-status boundaries
and provides text tokens, but it does not claim live child UI, notification
delivery, service persistence, or platform execution.

## Checklist

- [ ] Family/child/device policy scope.
- [ ] Rule targets across app/site/category/social/video/location. App/game
      target compiler contracts now cover app/game target proof and dry-run
      manual-required output; portal authoring and runtime evaluator execution
      remain.
- [ ] Schedule and time-budget builder. App/game time-budget contracts now
      require schedule evidence and stored session refs before representing
      budget decisions; portal authoring and runtime evaluator execution
      remain.
- [ ] Conflict resolution.
- [ ] Dry-run preview. Exceeded app/game budget outcomes now remain in
      observe/warn/ask-parent/time-limit-dry-run/manual-required contract
      states; service execution and adapter dispatch remain.
- [ ] Ask-parent request/approval/expiry. Contract proof now requires approval
      refs for pending/approved states and audit refs for resolved states.
      Child-facing app/game UX contracts now require approval, evidence, child
      reason, and child status refs before request actions; notification
      delivery and finished request UX remain.
- [ ] Override and bonus-time flow. Contract proof now prevents bonus-time
      extension without approval and audit refs; parent override UI and runtime
      execution remain.
- [ ] Audit and report integration.

## Next AI Instructions

Keep policy evaluation out of the browser. Add contracts and evaluator proof
before UI convenience. Never let AI output directly become enforcement. Social
account creation, secondary-account, feed, short-video, messaging-route, and
unknown-social ask-parent flows must use typed approval and policy decision
contracts rather than browser-only gates.
Browser-game unknown-start, cloud-gaming, game account, purchase, loot-box,
download, unblocked-site, and educational-game approval/time-budget flows must
also use typed policy and approval contracts.
