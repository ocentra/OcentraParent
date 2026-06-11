# WP147 - Timer parent preference setup child-runtime delivery receipt-pending seam

## Scope

Extend the accepted app/game parent preference setup request result with
service-local child-runtime delivery receipt-pending refs/status after the
WP145 receipt-required row is persisted.

This records that the local path is awaiting a future child-runtime receipt
before delivery can be claimed, without claiming actual child runtime delivery,
child runtime receipt, provider delivery, durable production outbox runtime,
adapter dispatch, broad blocking, or platform enforcement.

## Implementation

- `packages/agent-protocol-domain` adds receipt-pending refs/status fields to
  the accepted parent preference setup request result schema and parser test.
- `crates/agent-protocol` mirrors the receipt-pending fields and constants for
  Rust.
- `crates/agent-service` persists a seventh local audit event for
  receipt-pending after action-result, mutation receipt, handoff, queue,
  dispatch, and receipt-required rows.
- The portal command-result fixture is updated so stricter accepted-result
  parsing remains honest while rendering receipt-pending visibility stays a
  later UI slice.
- App-game feature/checklist/workpack docs record the receipt-pending scope and
  no-claim boundaries.

## Validation

- `cmd /c npm run build --workspace @ocentra-parent/agent-protocol-domain`
- `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- --run tests/app-game-timer-parent-preference-setup-request.test.ts`
- `cargo test -p ocentra-parent-agent-service app_game_timer_parent_preference_setup_request -- --nocapture`
- `cargo fmt --all --check`
- `git diff --check`
- `node scripts/check-no-test-doubles.mjs`
- `node scripts/check-source-shape.mjs`
- `cmd /c npm run lanes:guard`
- `cmd /c npm run hub:guard`

## No-Claim Boundaries

- Receipt-pending is still not actual child runtime delivery or receipt.
- Provider delivery and provider receipt ingestion are not claimed.
- Durable production outbox storage/runtime is not claimed.
- Adapter dispatch, broad blocking, and platform enforcement are not claimed.
- Raw private source rows, raw target values, and private diagnostics remain
  excluded.
- Package/crate README updates are deferred because E-D owns those locks.
- `docs/product-capability-checklist.md` is intentionally untouched.
