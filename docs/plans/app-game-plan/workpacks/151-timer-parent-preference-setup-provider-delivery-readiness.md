# WP151 - Timer parent preference setup provider delivery readiness

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP151 - Timer parent preference setup provider delivery readiness`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Extend the accepted app/game parent preference setup request path from durable
local outbox recording to provider-delivery readiness visibility. This is a
manual-required readiness audit step, not provider delivery.

## Implementation

- `packages/agent-protocol-domain` adds provider-delivery readiness refs/status
  fields to the accepted parent preference setup request result schema.
- `crates/agent-protocol` mirrors the result fields and adds provider-delivery
  readiness status and suffix constants.
- `crates/agent-service` persists a provider-delivery manual-required audit row
  after the durable local outbox JSONL append succeeds.
- `packages/portal-domain` and `apps/portal` render and test provider-delivery
  readiness refs/status in accepted command-result details.
- App/game docs record that this is provider-delivery readiness only.

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

- Provider-delivery readiness is not provider delivery.
- Provider receipt ingestion is not claimed.
- Adapter dispatch, broad blocking, and platform enforcement are not claimed.
- Raw private source rows, raw target values, and private diagnostics remain
  excluded.
- `docs/product-capability-checklist.md` is intentionally untouched.
