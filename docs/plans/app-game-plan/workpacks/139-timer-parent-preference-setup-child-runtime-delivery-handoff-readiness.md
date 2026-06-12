# WP139 - Timer parent preference setup child-runtime delivery handoff readiness

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP139 - Timer parent preference setup child-runtime delivery handoff readiness`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Extend the accepted parent preference setup request result with a parent-safe
child-runtime delivery handoff readiness state and persist that readiness into
the local ActivityStore.

This continues the WP138 mutation receipt handoff. It keeps native app and
native game control on one shared evidence/control spine and does not introduce
separate app-only or game-only child delivery semantics.

## Implementation

- `packages/agent-protocol-domain` extends the accepted setup request result
  with child-runtime delivery handoff id/ref/status fields plus explicit
  no-claim booleans for receipt ingestion, child runtime delivery, broad
  blocking, raw private rows, raw target values, and private diagnostics.
- `crates/agent-protocol` mirrors those fields and shared handoff constants for
  Rust consumers.
- `crates/agent-service` persists the manual-required action-result row, the
  local mutation receipt event, and a local child-runtime delivery handoff-ready
  audit event before reporting handoff readiness.

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

- Child-runtime delivery handoff readiness is not actual child runtime delivery.
- Provider delivery and provider receipt ingestion are not claimed.
- Durable production outbox storage is not claimed.
- Adapter dispatch, broad blocking, and platform enforcement are not claimed.
- Raw private source rows, raw target values, and private diagnostics are not
  exposed through this handoff.
- `docs/product-capability-checklist.md` is intentionally untouched because
  another lane owns the current central checklist churn.
