# WP140 - Timer parent preference setup child-runtime handoff command-result visibility

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP140 - Timer parent preference setup child-runtime handoff command-result visibility`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Render the accepted parent preference setup request result as parent-safe
command-result details in the portal, including action-result persistence,
mutation receipt, and child-runtime handoff refs/status.

This continues WP139 by making the handoff readiness visible in the App/Game
Sessions command-result flow. It does not create a separate app-only or
game-only control path.

## Implementation

- `packages/portal-domain` parses the accepted setup result event through the
  existing agent-protocol-domain schema and projects only parent-safe detail
  rows.
- `apps/portal` renders those detail rows above the raw command-result payload
  when the selected event is the app/game parent preference setup request
  result.
- The focused portal test covers action-result persistence refs, mutation
  receipt refs, child-runtime handoff refs/status, and explicit no-claim
  delivery/adapter/platform states.

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

- Child-runtime handoff readiness is rendered as readiness, not actual child
  runtime delivery.
- Provider delivery, provider receipt ingestion, and durable production outbox
  storage are not claimed.
- Adapter dispatch, broad blocking, and platform enforcement are not claimed.
- Raw private source rows, raw target values, and private diagnostics are not
  exposed.
- `docs/product-capability-checklist.md` is intentionally untouched.
