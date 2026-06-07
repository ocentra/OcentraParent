# WP112 Timer Parent-Surface Active-State Portal Visibility

## Scope

WP112 updates the parent portal timer parent-surface intent so WP111 active
timer state-store visibility is displayed instead of hidden behind stale
no-runtime copy.

## Implementation Evidence

- `packages/portal-domain/src/app-game-timer-parent-surface-panel.ts` displays
  timer runtime, scheduler persistence, durable scheduler storage, audit
  runtime, and rollback runtime details from the service read model.
- `packages/agent-protocol-domain/src/app-game-timer-parent-surface-read-model.ts`
  accepts service-reported timer runtime, scheduler persistence, and durable
  scheduler storage flags while keeping audit, rollback, adapter, child,
  platform, and raw private source flags guarded as unclaimed.
- `apps/portal/tests/app-game-timer-parent-surface-panel.test.ts` verifies both
  inactive state-store rows and active state-store rows.
- `packages/agent-protocol-domain/tests/app-game-timer-parent-surface-read-model.test.ts`
  verifies active state-store flags parse and audit overclaims remain rejected.
- `packages/text-domain/src/portal-dev.ts` keeps the no-claim copy precise:
  active timer state-store is shown only when the service reports it.

## Claim Boundary

- Visible when service reports it: timer runtime active-state visibility,
  scheduler persistence active-state visibility, durable scheduler state-store
  visibility.
- Not claimed: live scheduling execution, audit runtime/log storage, rollback
  execution, adapter dispatch, child delivery, broad blocking, platform
  enforcement, raw private source rows.

## Validation

- `cmd /c "npm run build --workspace @ocentra-parent/agent-protocol-domain && npm run build --workspace @ocentra-parent/text-domain && npm run build --workspace @ocentra-parent/portal-domain"`
- `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- app-game-timer-parent-surface-read-model`
- `cmd /c npm run test --workspace @ocentra-parent/portal -- app-game-timer-parent-surface-panel`
- `cmd /c npx prettier --check ...`
- `cmd /c node scripts/check-no-test-doubles.mjs`
- `cmd /c node scripts/check-source-shape.mjs`
- `git diff --check`
- `cmd /c npm run lanes:guard`
- `cmd /c npm run hub:guard`
