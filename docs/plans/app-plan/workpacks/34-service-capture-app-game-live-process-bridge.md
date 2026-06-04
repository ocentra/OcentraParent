# 34 Service Capture App/Game Live Process Bridge

## Target State

The native app plan cross-records shared app/game WP34: service activity capture
can store bounded live native app/game runtime rows in the encrypted journal and
ActivityStore without product-complete foreground, policy, portal, or adapter
claims.

## Scope

- Reuse the shared app/game service capture bridge.
- Prove a real service capture stores a live process runtime row.
- Prove the existing app/game service read model can query that row.
- Keep recurring freshness, foreground source proof, policy consumption, portal
  polish, and adapter execution as explicit gaps.

## Tests And Proof

- `cargo test -p ocentra-parent-agent-core app_game_windows_process_source`
- `cargo test -p ocentra-parent-agent-service activity_capture`
- `output/app-plan-proof/34-service-capture-app-game-live-process-bridge`

## Done Signal

Native app runtime evidence can be captured through the service journal/store
path and queried as runtime-only app/game evidence. Product status remains in
progress until recurring service freshness, foreground capture, policy
consumption, portal source status, and platform action proof are added.
