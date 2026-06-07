# WP109 Timer Parent-Surface Service Read Model

## Scope

- Add a dedicated app/game timer parent-surface service command/event:
  `agent.activity.app-game.timer-parent-surface.read-model.get` and
  `agent.activity.app-game.timer-parent-surface.read-model.reported`.
- Keep one shared app/game evidence spine from the existing ActivityStore
  app-game service model, while preserving native app versus native game product
  meaning in row `targetDomain`.
- Report only parent-surface readiness/blocker rows. Do not claim runtime timer
  scheduling, scheduler persistence, durable storage, audit/rollback runtime,
  adapter dispatch, child delivery, platform enforcement, or raw private source
  row access.

## Implementation

- `@ocentra-parent/agent-protocol-domain` defines and exports the timer
  parent-surface read-model parser, command, event, defaults key, and contract
  tests.
- `ocentra-parent-agent-protocol` mirrors the command/event and read-model
  structs for Rust service transport.
- `ocentra-parent-agent-service` maps the existing app-game ActivityStore
  service read model into timer parent-surface rows and routes the command
  through the websocket command dispatcher.

## Validation

- `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- app-game-timer-parent-surface-read-model contracts`
- `cargo test -p ocentra-parent-agent-protocol app_game_timer_parent_surface --quiet`
- `cargo test -p ocentra-parent-agent-service app_game_timer_parent_surface --quiet`

## Remaining Gaps

- Portal rendering and parent UI consumption remain unimplemented for this
  timer parent-surface read model.
- Runtime persistence, live timer scheduling, scheduler persistence, durable
  scheduler storage, durable audit logs, rollback execution, adapter dispatch,
  child delivery, platform enforcement, and raw private source rows remain
  unclaimed.
