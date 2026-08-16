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

## Execution Detail

Minimum context:

- `docs/plans/app-plan/workpacks/32-live-process-snapshot-source.md`
- `docs/plans/app-plan/workpacks/33-live-process-journal-sqlite-bridge.md`
- `docs/plans/app-game-plan/workpacks/34-service-capture-app-game-live-process-bridge.md`

Owner boundary:

- This workpack proves service capture path only.
- It does not prove recurring freshness, foreground, policy readiness, portal rendering, or adapter authority.

Required output:

- Service capture source and store boundary.
- Proof that runtime-only rows remain runtime-only.
- Explicit open gaps for policy, portal, and platform action.

Expected tests/proof names:

- `app-plan.wp34.service-capture-row`
- `app-plan.wp34.runtime-only-boundary`
- `app-plan.wp34.query-store-proof`
- `app-plan.wp34.no-adapter-claim`

Failure conditions:

- A service capture row is treated as foreground, policy, UI, or enforcement proof.
