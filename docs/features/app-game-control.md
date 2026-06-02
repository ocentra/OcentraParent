# App And Game Control

## Parent Outcome

Parents can see which apps and games are active, understand time spent, set app
or category limits, approve exceptions, and enforce supported limits.

## Ocentra Requirement

App/game control starts with evidence: inventory, identity, running time,
foreground time, category candidates, and confidence. Blocking or time limits
require typed policy decisions and platform adapter proof.

## Roadmap And Expectations

- Roadmap: V0.5.2 app/game evidence, V0.8 enforcement, V5 policy product.
- Expectations: [app/game evidence](../expectations/app-game-evidence.md),
  [policy](../expectations/policy.md),
  [enforcement](../expectations/enforcement.md).
- Supporting docs: [app settings inventory](../app-control-settings-inventory.md)
  and [game settings inventory](../game-control-settings-inventory.md).
- Modules: `packages/activity-domain`, `packages/parent-domain`,
  `crates/agent-core`, `crates/agent-service`.

## Competitor Pressure

See [Competitor Capability Map](../competitor-capability-map.md), especially
app inventory, app block/app limits, screen time schedules, and install
approval/purchases.

Google, Apple, Microsoft, Bark, Qustodio, Norton, Net Nanny, and Kidslox expose
app visibility, limits, and blocking. Ocentra must provide comparable parent
control with better evidence and local audit.

## Current Ocentra State

- App/game session contracts and read-model proof exist.
- Package/process identity and owned-process time-limit proof are in progress.
- The V0.8 product-control spine separates app time-limit and scoped
  owned-process control from broad installed-app blocking, so downstream policy
  and device surfaces can show time-limit/control-capable versus report-only or
  manual-required states.
- The V0.8 product-control runtime path now exposes app time-limit and scoped
  owned-process states through a Rust service WebSocket read model and typed
  agent-protocol adapter while broad app blocking stays manual-required.
- The V0.8 policy-dispatch proof now validates parent actor, target device,
  policy decision, schedule, app/game session evidence refs, adapter capability,
  timer state, approval state, audit refs, and child reason codes before
  dispatch-ready app/game time-limit states.
- Broad app blocking remains manual-required or unproved by platform.
- Raw app and game control setting inventories are preserved as design inputs,
  not product-complete implementation proof.

## Current Gap

Parent-visible app catalog, category quality, install/approval linkage, child
request flow, broad blocking, and cross-platform parity are incomplete. Broad
installed-app blocking remains manual-required beyond the scoped
owned-process/app-session proof.

## Checklist

- [ ] App/game inventory and identity.
- [ ] Running and foreground session evidence.
- [ ] Category and unknown-state handling.
- [ ] App/category schedule and time-budget rules.
- [ ] Ask-parent and bonus-time flow.
- [ ] Child-facing reason/status.
- [ ] Adapter capability status per platform.
- [ ] Blocking/time-limit proof before done claim.

## Next AI Instructions

Do not equate session evidence with blocking. Keep app identity quality,
category confidence, policy decisions, and adapter results as separate typed
states.
