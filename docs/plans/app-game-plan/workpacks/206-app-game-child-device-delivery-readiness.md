# WP206 App/Game Child-Device Delivery Readiness

## Scope

Carry the existing child-facing app/game UX local outbox provider-status handoff
into a child-device delivery readiness read model.

This keeps native apps and native games on the same child-facing delivery
evidence spine:

- child UX card handoff;
- local child-device artifact handoff;
- parent-owned local outbox bridge;
- scheduler bridge;
- provider preflight;
- provider-status handoff;
- child-device delivery readiness.

## Implementation

- Added
  `packages/parent-domain/src/app-game-child-facing-ux-child-device-delivery-readiness.ts`.
- Added
  `packages/parent-domain/tests/app-game-child-facing-ux-child-device-delivery-readiness.test.ts`.
- Added `scripts/test/app-game-child-device-delivery-readiness-proof.mjs`.

## Validation

Focused validation for this workpack:

```powershell
cmd /c node --check scripts/test/app-game-child-device-delivery-readiness-proof.mjs
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-child-facing-ux-child-device-delivery-readiness app-game-child-facing-ux-local-outbox-provider-status-handoff
cmd /c node scripts/test/app-game-child-device-delivery-readiness-proof.mjs
```

## Proof

- `test-results/app-game-child-device-delivery-readiness-proof/proof.json`
- `output/app-game-plan-proof/206-app-game-child-device-delivery-readiness/proof.json`
- `output/app-game-plan-proof/206-app-game-child-device-delivery-readiness/00-source-snapshot.md`
- `output/app-game-plan-proof/206-app-game-child-device-delivery-readiness/10-validation-commands.log`

## Boundaries

Proved:

- Scheduled child UX provider-status rows become child-transport-required
  readiness rows.
- Manual-required and unavailable rows stay out of transport-required readiness.
- Parent-safe transport references can be carried without raw child payload rows.
- The read model rejects runtime child transport, receipt ingestion, provider
  delivery execution, platform delivery channel, adapter dispatch, platform
  enforcement, and raw private source-row claims.

Not proved:

- Child runtime transport attachment.
- Child runtime receipt ingestion.
- Provider delivery execution.
- Platform push, overlay, notification, or OS-level delivery channel execution.
- Adapter dispatch or platform enforcement.
