# WP150 - Timer parent preference setup durable local outbox runtime

## Scope

Extend the accepted app/game parent preference setup request path from
service-local receipt ingestion to a durable local outbox record. This is a
runtime storage step for the setup path, not provider delivery.

## Implementation

- `packages/agent-protocol-domain` adds durable local outbox refs/status fields
  to the accepted parent preference setup request result schema.
- `crates/agent-protocol` mirrors the result fields and adds durable outbox
  status, suffix, and JSONL file-extension constants.
- `crates/agent-service` appends a parent-safe JSONL durable outbox record after
  the setup ActivityStore rows persist.
- `packages/portal-domain` and `apps/portal` render and test durable local
  outbox refs/status in accepted command-result details.
- App/game docs record that this is a local outbox runtime only.

## Validation

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

- Durable local outbox recording is not provider delivery.
- Provider receipt ingestion is not claimed.
- Adapter dispatch, broad blocking, and platform enforcement are not claimed.
- Raw private source rows, raw target values, and private diagnostics remain
  excluded.
- `docs/product-capability-checklist.md` is intentionally untouched.
