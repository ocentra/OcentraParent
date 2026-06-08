# WP154 - Timer parent preference setup provider preflight durable outbox

## Branch

`codex/app-game-control-product-completion`

## Scope

WP154 extends the app/game parent preference setup durable local outbox so it
persists the provider adapter and provider credential/manual-proof preflight
requirement IDs/statuses from the accepted setup request result.

## Runtime Boundary

- Durable setup outbox JSONL rows now include provider adapter requirement ID
  and status.
- Durable setup outbox JSONL rows now include provider credential/manual-proof
  requirement ID and status.
- The agent-service persistence test asserts the JSONL outbox fields match the
  real service result.

## No-Claim Boundaries

- No provider delivery execution.
- No provider receipt ingestion.
- No adapter dispatch.
- No broad blocking.
- No platform enforcement.
- No raw private source rows, raw target values, or private diagnostics.
- `docs/product-capability-checklist.md` intentionally remains untouched.

## Validation

See `10-validation-commands.log`.
