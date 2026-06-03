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
- Shared native app/game planning:
  [app + game plan](../plans/app-game-plan/README.md).
- Native app planning:
  [native apps plan](../plans/app-plan/README.md).
- Browser-game planning:
  [browser games/cloud gaming gating plan](../plans/browser-plan/v0-5-browser-games-cloud-gaming-gating-plan.md).
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
- Stored app/game observation rows now derive deterministic running,
  foreground, background, stale-gap, process-exit, replay-stable, and daily
  rollup duration proof without upgrading inventory, launcher, or foreground
  evidence into content or enforcement authority.
- Staged encrypted journal-file replay now persists typed inventory, runtime,
  foreground, and launcher evidence through SQLite and projects inventory,
  running-now, foreground-now, launcher, and daily rollup rows while preserving
  the no-use/no-content/no-launcher-promotion boundaries.
- The service now exposes those staged app/game journal and SQLite projections
  through typed app-use and games activity-surface read models, including
  inventory, runtime, foreground, launcher/source-count, daily rollup,
  capability, and evidence-ref fields.
- The parent portal App/Game Sessions surface now renders those service-backed
  app-use and games read-model rows in a dedicated dashboard intent and SVG
  surface with separate inventory, running, foreground, launcher-only,
  unknown-review, manual-required capability, game-budget gap, and evidence
  counts.
- App/game unknown approval contracts now represent new inventory apps, unknown
  runtime processes, portable/installer candidates, launcher-game candidates,
  unknown game-like executables, child status/reason refs, parent response
  scope, expiry, audit-backed replay state, and manual-required block outcomes
  without dispatching unsupported adapters.
- Native game budget policy contracts now represent game budget targets,
  running/foreground duration modes, launcher-only exclusion, parent-approved
  launcher-game candidate inclusion, advisory rating/UGC/multiplayer/purchase
  signals, dry-run preview decisions, and no-enforcement handoff guards.
- App/game policy target compiler contracts now require identity,
  unknown-state, category, schedule, capability, authority, device, local-user,
  and freshness proof before accepting app/game rule compile requests, and keep
  unproved block-launch in manual-required dry-run output.
- App/game time-budget contracts now consume stored app/game session refs,
  schedule evidence, bonus-time approval/audit refs, dry-run/manual-required
  handoff state, and restart-recovered timer refs before representing exceeded
  budget decisions.
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
- The V0.8 broad-adapter proof now exposes a service-backed runtime read model
  that keeps owned-process/app timer support as implemented-boundary while
  broad installed-app blocking remains manual-required.
- The V0.8 supported-adapter runtime proof now narrows the implemented app/game
  claim to Windows owned-process time-limit support with evidence, timer,
  rollback, and audit references while keeping package-wide app blocking
  manual-required.
- The V0.8 enforcement integrity runtime audit now exposes supported app/game
  time-limit success, expiry, rollback, parent override/supersede, dry-run,
  stale decision rejection, wrong-device rejection, child-status refs, timer
  refs, rollback refs, audit refs, and permission/dependency unavailable states
  through TypeScript/Rust/service proof without upgrading broad installed-app
  blocking.
- Launcher evidence and launcher-game candidate contracts now exist with Rust
  protocol parity and staged Windows launcher parser proof. Launcher-only,
  launcher foreground, launcher-game candidate, and proved child-game states
  are separated so launcher evidence cannot become fake known-game proof.
- Cross-platform app/game authority matrix contracts now represent platform,
  action, authority tier, setup state, proof state, parent-visible limitation,
  proof needed to claim, and no-execute guards for Windows, macOS, Linux,
  Android, and iOS/iPadOS hard-control rows.
- App/game category-risk taxonomy contracts now represent native app
  categories, native game categories, risk candidates, game context signals,
  source kind, source ref, confidence, reason code, evidence refs, parent
  display override, AI digest refs, policy-candidate action, and a
  `notEnforcement` guard.
- Broad app blocking remains manual-required or unproved by platform.
- Raw app and game control setting inventories are preserved as design inputs,
  not product-complete implementation proof.

## Current Gap

Parent-visible app catalog/dashboard UI now has an initial service-backed
App/Game Sessions surface, but category quality, unknown approvals, game budget
policy, live launcher crawling, install/approval linkage, child request flow,
broad blocking, and cross-platform parity are incomplete. Broad installed-app
blocking remains manual-required beyond the scoped
owned-process/app-session proof, including in the broad-adapter and
supported-adapter runtime service proofs. The integrity runtime audit proves
typed timer/rollback/child-status visibility for scoped app/game outcomes, but
does not prove package-wide app blocking, polished child request UX, install
approval, or runtime cross-platform parity. The authority matrix is contract
proof only; it does not prove AppLocker/App Control, MDM, Endpoint Security,
Device Owner/Profile Owner, FamilyControls/ManagedSettings, cgroup/systemd, or
kiosk/single-app adapter behavior. The category/risk taxonomy is contract proof
only; it does not prove live catalog enrichment, local AI classifier quality,
policy compiler routing, portal category UI, or runtime app/game category
decisions.
The unknown approval proof is still contract-level: it does not yet provide
finished parent/child approval UI, notification delivery, persisted service
read models, live candidate creation from platform adapters, or platform hard
blocking.
The native game budget proof is also contract-level: it does not yet provide a
policy target compiler, live game budget authoring UI, service persistence,
budget notifications, or adapter execution.
The app/game policy target compiler proof is contract-level: it does not yet
provide runtime service evaluation, Rust/WebSocket parity, portal rule
authoring, timer integration, notifications, rollback, or adapter execution.
The app/game time-budget proof is contract-level: it does not yet provide
runtime service evaluation, Rust/WebSocket parity, portal budget authoring,
notification delivery, child request UX, service persistence, adapter
execution, or platform timer/rollback execution.

## Checklist

- [ ] App/game inventory and identity.
- [ ] Running and foreground session evidence. Contract and local
      SQLite-row session-duration proof now exists, and staged journal-file
      replay proof now covers typed fixture rows; service app-use/games
      read-model DTOs now expose those projected rows; the parent portal now
      has a dedicated app/game dashboard surface for those rows; live source
      adapters and policy integration remain.
- [ ] Category and unknown-state handling. Unknown approval contracts now keep
      weak app/game evidence in review/report-only/manual-required states with
      evidence refs, child status refs, expiry, and audit-backed persistence
      fields; live candidate production and parent/child UX remain.
- [ ] App/category schedule and time-budget rules. Native game budget dry-run
      contracts now exist for known-game counts, launcher-only exclusion,
      parent-approved candidate inclusion, and advisory signal boundaries.
      App/game policy target compiler contracts now validate identity,
      unknown-state, category, schedule, capability, authority, device,
      local-user, and freshness proof before dry-run decisions; live evaluator,
      authoring UI, persistence, timers, and enforcement remain. App/game
      time-budget contracts now consume stored session refs, schedule refs,
      bonus approval/audit refs, and timer recovery refs before dry-run or
      manual-required decisions; live evaluator, UI, notification, persistence,
      and adapter execution remain.
- [ ] Ask-parent and bonus-time flow. Contract proof now requires approval and
      audit refs before bonus time extends a budget and keeps ask-parent/manual
      states dry-run only; notification delivery and finished parent/child UX
      remain.
- [ ] Child-facing reason/status is referenced in the runtime audit; finished
      child request/status UX remains.
- [ ] Adapter capability status per platform.
- [ ] Blocking/time-limit proof before done claim.

## Next AI Instructions

Do not equate session evidence with blocking. Keep app identity quality,
category confidence, policy decisions, and adapter results as separate typed
states. Treat
`scripts/test/v0-8-enforcement-integrity-runtime-audit.mjs` as scoped
app/game time-limit audit proof only, not broad installed-app blocking proof.
Browser-game and cloud-gaming web surfaces belong in the browser plan when the
source is managed browser evidence; native games, launchers, process/session
duration, and broad app blocking stay in this app/game feature.
