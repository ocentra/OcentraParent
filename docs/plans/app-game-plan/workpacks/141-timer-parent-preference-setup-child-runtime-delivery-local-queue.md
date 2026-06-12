# WP141 - Timer parent preference setup child-runtime delivery local queue

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP141 - Timer parent preference setup child-runtime delivery local queue`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Extend the accepted app/game parent preference setup request result with a
service-local child-runtime delivery queue record. This continues the WP139
handoff and WP140 parent-visible result work by adding a persisted local queue
audit event after the action-result, mutation receipt, and handoff rows.

The queue is a local service seam only. It does not claim actual child device
delivery, provider delivery, provider receipt ingestion, adapter dispatch, or
platform enforcement.

## Implementation

- `packages/agent-protocol-domain` adds schema-backed
  `childRuntimeDeliveryQueue*` result fields with `queued` and `unavailable`
  states.
- `crates/agent-protocol` mirrors those fields and constants for Rust service
  serialization.
- `crates/agent-service` derives queue refs from the parent preference setup
  reference and persists a fourth ActivityStore audit event only after the
  local write succeeds.
- Focused TypeScript and Rust tests assert queue ids/status while preserving
  the existing no-claim delivery/platform boundaries.

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

- The queue row is service-local and not actual child runtime delivery.
- Provider delivery and provider receipt ingestion are not claimed.
- Durable production outbox storage is not claimed.
- Adapter dispatch, broad blocking, and platform enforcement are not claimed.
- Raw private source rows, raw target values, and private diagnostics remain
  excluded.
- `docs/product-capability-checklist.md` is intentionally untouched.
