# WP149 - Timer parent preference setup child-runtime delivery receipt-ingested boundary

## Scope

Extend the accepted app/game parent preference setup request result from
receipt-pending to a service-local child-runtime delivery receipt-ingested
boundary, and render that boundary in the parent portal command-result details.

This is one combined implementation chunk. It includes the contract/protocol,
service persistence, real tests, and parent-visible details rather than splitting
the service seam and UI visibility into separate tiny commits.

## Implementation

- `packages/agent-protocol-domain` adds receipt-ingested refs/status fields to
  the accepted parent preference setup request result schema.
- `crates/agent-protocol` mirrors the result fields and adds the
  receipt-ingested status/suffix constants.
- `crates/agent-service` creates receipt-ingested refs, marks them only after
  local persistence succeeds, and stores one additional service-local audit row
  after receipt-pending is accepted by the ActivityStore.
- `packages/portal-domain` and `apps/portal` render and test receipt-ingested
  refs/status in the accepted command-result details.
- App/game docs record that this is a local receipt ingestion boundary only.

## Validation

- `cmd /c npm run build --workspace @ocentra-parent/schema-domain`
- `cmd /c npm run build --workspace @ocentra-parent/agent-protocol-domain`
- `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- --run tests/app-game-timer-parent-preference-setup-request.test.ts`
- `cargo test -p ocentra-parent-agent-service app_game_timer_parent_preference_setup_request -- --nocapture`
- `cmd /c npm run build --workspace @ocentra-parent/portal-domain`
- `cmd /c npm run type-check --workspace @ocentra-parent/portal`
- `cmd /c npm run test --workspace @ocentra-parent/portal -- --run tests/app-game-timer-parent-surface-panel.test.ts`
- `cargo fmt --all --check`
- `git diff --check`
- `node scripts/check-no-test-doubles.mjs`
- `node scripts/check-source-shape.mjs`
- `cmd /c npm run lanes:guard`
- `cmd /c npm run hub:guard`

## No-Claim Boundaries

- Receipt-ingested is service-local setup-path evidence, not provider receipt
  ingestion.
- Provider delivery is not claimed.
- Durable production outbox storage/runtime is not claimed.
- Adapter dispatch, broad blocking, and platform enforcement are not claimed.
- Raw private source rows, raw target values, and private diagnostics remain
  excluded.
- `docs/product-capability-checklist.md` is intentionally untouched.
