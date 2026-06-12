# WP137 - Timer parent preference setup action-result persistence

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP137 - Timer parent preference setup action-result persistence`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Persist the accepted parent preference setup request action-result handoff into
the shared app/game local ActivityStore when a service store is available.

This keeps native app and native game control on one low-level evidence and
journal spine. The slice does not create an app-only or game-only persistence
path.

## Implementation

- `packages/agent-protocol-domain` allows the accepted setup request result to
  report `actionResultPersistenceStatus` as `persisted` or `unavailable` and to
  claim persistence only when the service actually stores the result.
- `crates/agent-protocol` mirrors the persistence status field and shared
  status constants for Rust consumers.
- `crates/agent-service` writes a replayable manual-required
  `AppGameControlActionResult` journal row through the real `ActivityStore`
  before claiming action-result persistence.
- The persisted row uses parent-safe setup/action/result refs and avoids raw
  private source rows or raw target values.

## Validation

- `cmd /c npm run build --workspace @ocentra-parent/agent-protocol-domain`
- `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- --run tests/app-game-timer-parent-preference-setup-request.test.ts`
- `cargo test -p ocentra-parent-agent-service app_game_timer_parent_preference_setup_request --quiet`
- `cargo fmt --all --check`

## No-Claim Boundaries

- Parent preference mutation is not claimed.
- Notification rule mutation is not claimed.
- Provider delivery, receipt ingestion, child runtime delivery, durable outbox
  storage, adapter dispatch, broad blocking, and platform enforcement are not
  claimed.
- The persisted action-result row is manual-required and replayable; it is not
  proof that an enforcement adapter executed.
- Raw private source rows, raw target values, and private diagnostics are not
  exposed through this persistence handoff.
- `docs/product-capability-checklist.md` is intentionally untouched because
  another lane owns the current central checklist churn.
