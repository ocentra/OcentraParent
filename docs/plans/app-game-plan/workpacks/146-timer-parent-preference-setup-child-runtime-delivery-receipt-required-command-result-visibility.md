# WP146 - Timer parent preference setup child-runtime delivery receipt-required command-result visibility

## Scope

Render the WP145 service-local child-runtime delivery receipt-required refs/status
in the parent portal command-result details for accepted app/game parent
preference setup request results.

This keeps the future receipt requirement parent-visible without claiming actual
child runtime delivery, child runtime receipt, provider delivery, durable
production outbox runtime, adapter dispatch, broad blocking, or platform
enforcement.

## Implementation

- `packages/portal-domain` adds receipt-required refs/status detail rows to
  accepted parent preference setup request command-result details.
- `apps/portal` updates the parent-surface panel assertions so the command
  result proves the WP145 receipt-required fields render as parent-safe detail
  rows.
- Portal and feature docs record that receipt-required visibility is not actual
  child delivery, child receipt, provider delivery, durable outbox, adapter, or
  platform enforcement proof.

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

- Receipt-required is still not actual child runtime delivery or receipt.
- Provider delivery and provider receipt ingestion are not claimed.
- Durable production outbox storage/runtime is not claimed.
- Adapter dispatch, broad blocking, and platform enforcement are not claimed.
- Raw private source rows, raw target values, and private diagnostics remain
  excluded.
- `docs/product-capability-checklist.md` is intentionally untouched.
