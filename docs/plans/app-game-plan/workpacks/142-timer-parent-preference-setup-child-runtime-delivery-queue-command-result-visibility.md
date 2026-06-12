# WP142 - Timer parent preference setup child-runtime delivery queue command-result visibility

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP142 - Timer parent preference setup child-runtime delivery queue command-result visibility`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Render the WP141 service-local child-runtime delivery queue refs/status in the
parent portal command-result details for accepted app/game parent preference
setup request results.

This keeps the queue handoff visible to the parent surface while preserving the
line between service-local queue readiness and actual child device delivery.

## Implementation

- `packages/portal-domain` adds parent-safe queue refs/status detail rows to
  the accepted setup command-result projection.
- `apps/portal` focused test fixtures include the WP141 queue schema fields and
  assert the queue rows render beside action-result persistence, mutation
  receipt, and child-runtime handoff details.
- Portal and portal-domain README notes clarify that queue readiness is not
  delivery or platform enforcement.

## Validation

- `cmd /c npm run build --workspace @ocentra-parent/portal-domain`
- `cmd /c npm run type-check --workspace @ocentra-parent/portal`
- `cmd /c npm run test --workspace @ocentra-parent/portal -- --run tests/app-game-timer-parent-surface-panel.test.ts`
- `git diff --check`
- `node scripts/check-no-test-doubles.mjs`
- `node scripts/check-source-shape.mjs`
- `cmd /c npm run lanes:guard`
- `cmd /c npm run hub:guard`

## No-Claim Boundaries

- Service-local queue readiness is not actual child runtime delivery.
- Provider delivery and provider receipt ingestion are not claimed.
- Durable production outbox storage is not claimed.
- Adapter dispatch, broad blocking, and platform enforcement are not claimed.
- Raw private source rows, raw target values, and private diagnostics remain
  excluded.
- `docs/product-capability-checklist.md` is intentionally untouched.
