# app WP68 Notification Live Parent Surface Read Model

Checked at: 2026-06-05T13:52:06.997Z
Implementation commit: ee216cd07a74ed18a5850946ec8e309ef2192562

## Claims Proved
- Portal overview commands request the existing app/game notification readiness service read model
- Portal live state parses the service readiness event before deriving parent-surface rows
- Portal-domain projection maps validated readiness rows into schema-backed manual/unavailable parent-surface setup rows
- Rendered parent-surface rows keep scheduler/outbox runtime refs unreported unless proved by a later service row

## Claims Not Proved
- provider delivery, receipt ingestion, credentials, webhook handling, cloud routing, or production retry workers
- parent preference mutation, frequency controls, quiet-hours editor behavior, or parent notification delivery UI
- scheduler runtime, local outbox runtime, durable production outbox/history storage, or adapter dispatch
- child-device delivery, mobile UI, policy evaluator execution, broad blocking, or platform support

## Evidence
- liveProjection: packages/portal-domain/src/app-game-notification-parent-surface-live-readiness.ts
- overviewCommands: packages/portal-domain/src/commands.ts
- liveActivityState: apps/portal/src/live-activity-state.ts
- commandResultEvents: apps/portal/src/event-results.ts
- portalDomainTest: packages/portal-domain/tests/app-game-notification-parent-surface-panel.test.ts
- portalRouteTest: apps/portal/tests/app-game-notification-parent-surface-panel.test.ts
- proofHarness: scripts/test/app-game-notification-live-parent-surface-proof.mjs
- appGameProofPack: output/app-game-plan-proof/68-notification-live-parent-surface-read-model
- appProofPack: output/app-plan-proof/68-notification-live-parent-surface-read-model
