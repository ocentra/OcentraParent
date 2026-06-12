# WP143 - Timer parent preference setup child-runtime delivery dispatch readiness

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP143 - Timer parent preference setup child-runtime delivery dispatch readiness`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Extend the accepted app/game parent preference setup request result with
service-local child-runtime delivery dispatch refs/status after the WP141 queue
row is persisted.

This advances the queued setup handoff toward a local child-runtime dispatch
seam without claiming actual child receipt, provider delivery, durable
production outbox runtime, adapter dispatch, broad blocking, or platform
enforcement.

## Implementation

- `packages/agent-protocol-domain` adds dispatch refs/status fields to the
  accepted parent preference setup request result schema and parser test.
- `crates/agent-protocol` mirrors the dispatch fields and constants for Rust.
- `crates/agent-service` persists a fifth local audit event for dispatch
  readiness after action-result, mutation receipt, handoff, and queue events.
- App-game feature/checklist/workpack docs record the dispatch-readiness scope
  and no-claim boundaries.

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

- Dispatch readiness is still not actual child runtime delivery or receipt.
- Provider delivery and provider receipt ingestion are not claimed.
- Durable production outbox storage/runtime is not claimed.
- Adapter dispatch, broad blocking, and platform enforcement are not claimed.
- Raw private source rows, raw target values, and private diagnostics remain
  excluded.
- `docs/product-capability-checklist.md` is intentionally untouched.
