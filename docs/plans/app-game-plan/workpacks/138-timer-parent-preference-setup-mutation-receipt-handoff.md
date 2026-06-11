# WP138 - Timer parent preference setup mutation receipt handoff

## Scope

Extend the accepted parent preference setup request result with a parent-safe
mutation receipt handoff and persist that receipt into the local ActivityStore.

This is the next step from the WP137 action-result persistence slice. It keeps
native app and native game control on one shared evidence/control spine and
does not introduce a separate app-only or game-only preference path.

## Implementation

- `packages/agent-protocol-domain` extends the accepted setup request result
  with mutation receipt id/ref/status fields and a receipt-claimed boolean.
- `crates/agent-protocol` mirrors those fields and shared receipt constants for
  Rust consumers.
- `crates/agent-service` persists the manual-required action-result row and a
  local parent preference setup mutation receipt event in one ActivityStore
  write path before reporting receipt persistence.
- The receipt refs are parent-safe and reuse the accepted setup/request refs.

## Validation

- `cmd /c npm run build --workspace @ocentra-parent/agent-protocol-domain`
- `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- --run tests/app-game-timer-parent-preference-setup-request.test.ts`
- `cargo test -p ocentra-parent-agent-service app_game_timer_parent_preference_setup_request --quiet`
- `cargo fmt --all --check`
- `git diff --check`
- `node scripts/check-no-test-doubles.mjs`
- `node scripts/check-source-shape.mjs`
- `cmd /c npm run lanes:guard`
- `cmd /c npm run hub:guard`

## No-Claim Boundaries

- The receipt is not a durable parent preference mutation.
- Notification rule mutation is not claimed.
- Provider delivery, receipt ingestion, child runtime delivery, durable outbox
  storage, adapter dispatch, broad blocking, and platform enforcement are not
  claimed.
- Raw private source rows, raw target values, and private diagnostics are not
  exposed through this receipt handoff.
- `docs/product-capability-checklist.md` is intentionally untouched because
  another lane owns the current central checklist churn.
