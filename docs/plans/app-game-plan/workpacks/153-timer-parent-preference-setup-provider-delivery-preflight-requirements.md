# WP153 - Timer parent preference setup provider delivery preflight requirements

## Scope

Extend the accepted app/game parent preference setup request path from a
provider-delivery attempt handoff to explicit provider adapter and provider
credential/manual-proof preflight requirements. This remains manual-required
preflight only; it does not execute provider delivery or dispatch an adapter.

## Implementation

- `packages/agent-protocol-domain` adds provider adapter and provider
  credential/manual-proof requirement refs/status and claimed flags to the
  accepted parent preference setup request result schema.
- `crates/agent-protocol` mirrors the result fields and adds provider
  adapter/credential requirement status and suffix constants.
- `crates/agent-service` persists provider adapter and provider
  credential/manual-proof requirement audit rows after provider-delivery
  attempt persists.
- `packages/portal-domain` and `apps/portal` render and test provider adapter
  and credential/manual-proof requirement refs/status in accepted command-result
  details.
- App/game docs record that these are preflight blockers only.

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

- Provider adapter requirement status is manual-required; it is not adapter
  dispatch.
- Provider credential/manual-proof requirement status is manual-required; it is
  not provider delivery execution.
- Provider receipt ingestion is not claimed.
- Broad blocking and platform enforcement are not claimed.
- Raw private source rows, raw target values, and private diagnostics remain
  excluded.
- `docs/product-capability-checklist.md` is intentionally untouched.
