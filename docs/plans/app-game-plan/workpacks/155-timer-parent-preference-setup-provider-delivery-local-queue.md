# WP155 - Timer parent preference setup provider delivery local queue

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP155 - Timer parent preference setup provider delivery local queue`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Extend the accepted app/game parent preference setup request path from provider
preflight requirements into a local provider-delivery queue seam.

This queues the future provider-delivery handoff in the typed setup result,
durable setup outbox, service audit rows, and parent command-result visibility.
It does not send a provider notification, ingest provider receipts, dispatch an
adapter, or claim platform enforcement.

## Implementation

- `packages/agent-protocol-domain` adds provider-delivery local queue
  refs/status and claimed flag to the accepted parent preference setup request
  result schema.
- `crates/agent-protocol` mirrors those fields and adds provider queue
  constants for result status, suffix, event payload, and durable outbox field
  names.
- `crates/agent-service` derives provider queue refs from the credential
  preflight requirement, marks the local queue as queued after persistence,
  serializes the queue ID/status into the durable setup outbox, and records a
  provider queue audit row.
- `packages/portal-domain` and `apps/portal` render and test provider-delivery
  queue refs/status in accepted command-result details.
- App/game docs record that this remains local queue visibility only.

## Validation

- `cmd /c npm run build --workspace @ocentra-parent/agent-protocol-domain`
- `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- --run tests/app-game-timer-parent-preference-setup-request.test.ts`
- `cargo test -p ocentra-parent-agent-service app_game_timer_parent_preference_setup_request -- --nocapture`
- `cmd /c npm run build --workspace @ocentra-parent/portal-domain`
- `cmd /c npm run test --workspace @ocentra-parent/portal -- --run tests/app-game-timer-parent-surface-panel.test.ts`
- `cargo fmt --all --check`
- `git diff --check`
- `node scripts/check-no-test-doubles.mjs`
- `node scripts/check-source-shape.mjs`
- `cmd /c npm run lanes:guard`
- `cmd /c npm run hub:guard`

## No-Claim Boundaries

- Provider-delivery queue status is a local queue seam; it is not provider
  delivery execution.
- Provider receipt ingestion is not claimed.
- Adapter dispatch is not claimed.
- Broad blocking and platform enforcement are not claimed.
- Raw private source rows, raw target values, and private diagnostics remain
  excluded.
- `docs/product-capability-checklist.md` is intentionally untouched.
