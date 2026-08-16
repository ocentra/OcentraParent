# WP40 - App/Game Boundary Read-Model Event

Native app WP40 cross-records the shared app/game dedicated backend boundary
read-model event.

The native app plan now has a service-backed command/event for staged app/game
evidence claim, identity, approval authority/action-result, platform authority
matrix/rows, and AI classifier result counts plus citation refs.

This does not add native app classifier execution, policy consumption, portal
rows, adapter execution, broad blocking, or platform support.

## Execution Detail

Minimum context:

- `docs/plans/app-plan/AGENTS.md`
- `docs/plans/app-game-plan/AGENTS.md`
- `docs/features/app-game-control.md`
- `docs/features/child-agent-local-service.md`

Owner boundary:

- `app-plan` owns native app/runtime substrate, service/read-model handoff, and app-only safety against overclaim.
- `app-game-plan` owns product-level app/game classification, game/addiction semantics, launcher/game sessions, and parent-facing app/game policy.
- This workpack is a cross-record. It must not create a second app/game truth.

Required output:

- State which read-model event is produced/consumed.
- State which app-only claims remain false.
- Record the consumer handoff and proof path.
- Update checklist/proof rows only for this exact boundary.

Expected tests/proof names:

- `app-plan.wp40.boundary-event-shape`
- `app-plan.wp40.no-classifier-execution-claim`
- `app-plan.wp40.app-game-owner-handoff`
- `app-plan.wp40.read-model-event-proof`

Failure conditions:

- Future agent claims game classification, policy enforcement, portal readiness, or platform blocking from this read-model boundary alone.
- App and app-game plans diverge on event meaning or source authority.
