# WP144 - Timer parent preference setup child-runtime delivery dispatch command-result visibility

## Scope

Render the WP143 service-local child-runtime delivery dispatch refs/status in
the parent portal command-result details for accepted app/game parent preference
setup request results.

This keeps the local dispatch seam parent-visible without claiming actual child
runtime receipt, provider delivery, receipt ingestion, durable production outbox
runtime, adapter dispatch, broad blocking, or platform enforcement.

## Implementation

- `packages/portal-domain` adds dispatch refs/status detail rows to accepted
  parent preference setup request command-result details.
- `apps/portal` updates the parent-surface panel test fixture and assertions so
  the command result proves the WP143 dispatch fields render as parent-safe
  detail rows.
- Portal and feature docs record that dispatch readiness is visible but remains
  distinct from delivery, provider, outbox, adapter, broad-blocking, platform,
  raw-source, raw-target, and private-diagnostic claims.

## Validation

- `cmd /c npm run build --workspace @ocentra-parent/portal-domain`
- `cmd /c npm run type-check --workspace @ocentra-parent/portal`
- `cmd /c npm run test --workspace @ocentra-parent/portal -- --run tests/app-game-timer-parent-surface-panel.test.ts`
- `git diff --check`
- `node scripts/check-no-test-doubles.mjs`
- `node scripts/check-source-shape.mjs`
- `cmd /c npm run lanes:guard`
- `cmd /c npm run hub:guard`

## No-Claim Boundaries

- Dispatch readiness is still not actual child runtime delivery or receipt.
- Provider delivery and provider receipt ingestion are not claimed.
- Durable production outbox storage/runtime is not claimed.
- Adapter dispatch, broad blocking, and platform enforcement are not claimed.
- Raw private source rows, raw target values, and private diagnostics remain
  excluded.
- `docs/product-capability-checklist.md` is intentionally untouched.
