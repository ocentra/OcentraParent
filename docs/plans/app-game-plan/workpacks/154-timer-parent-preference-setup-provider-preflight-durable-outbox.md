# WP154 - Timer parent preference setup provider preflight durable outbox

## Scope

Carry the provider adapter and provider credential/manual-proof preflight
requirement IDs/statuses from the accepted app/game parent preference setup
request result into the durable local setup outbox JSONL record.

This is a durable visibility slice. It does not execute provider delivery,
ingest provider receipts, dispatch adapters, or claim platform enforcement.

## Implementation

- `crates/agent-protocol` adds field constants for the provider adapter and
  provider credential/manual-proof requirement IDs/statuses in setup outbox
  records.
- `crates/agent-service` serializes those four preflight fields into the
  parent-safe durable setup outbox record.
- The real agent-service setup request persistence test asserts the JSONL
  outbox record contains the same provider adapter and credential requirement
  IDs/statuses returned by the service result.
- App/game docs record the no-claim boundary and leave the central product
  checklist untouched.

## Validation

- `cargo test -p ocentra-parent-agent-service app_game_timer_parent_preference_setup_request -- --nocapture`
- `cargo fmt --all --check`
- `git diff --check`
- `node scripts/check-no-test-doubles.mjs`
- `node scripts/check-source-shape.mjs`
- `cmd /c npm run lanes:guard`
- `cmd /c npm run hub:guard`

## No-Claim Boundaries

- Provider adapter requirement status remains a manual-required blocker; it is
  not adapter dispatch.
- Provider credential/manual-proof requirement status remains a manual-required
  blocker; it is not provider delivery execution.
- Provider receipt ingestion is not claimed.
- Broad blocking and platform enforcement are not claimed.
- Raw private source rows, raw target values, and private diagnostics remain
  excluded.
- `docs/product-capability-checklist.md` is intentionally untouched.
