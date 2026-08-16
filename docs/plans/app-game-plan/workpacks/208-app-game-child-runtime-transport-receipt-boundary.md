# WP208 App/Game Child Runtime Transport Receipt Boundary

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP208 App/Game Child Runtime Transport Receipt Boundary`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Convert child-device runtime writer envelope rows into an explicit child runtime
transport and receipt boundary.

This is still a non-executing boundary: it records which transport and receipt
contracts are required before runtime child delivery can be claimed.

## Implementation

- Added
  `packages/parent-domain/src/app-game-child-facing-ux-child-runtime-transport-receipt-boundary.ts`.
- Added
  `packages/parent-domain/tests/app-game-child-facing-ux-child-runtime-transport-receipt-boundary.test.ts`.
- Added `scripts/test/app-game-child-runtime-transport-receipt-boundary-proof.mjs`.

## Validation

Focused validation for this workpack:

```powershell
cmd /c node --check scripts/test/app-game-child-runtime-transport-receipt-boundary-proof.mjs
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-child-facing-ux-child-runtime-transport-receipt-boundary
cmd /c node scripts/test/app-game-child-runtime-transport-receipt-boundary-proof.mjs
```

## Proof

- `test-results/app-game-child-runtime-transport-receipt-boundary-proof/proof.json`
- `output/app-game-plan-proof/208-app-game-child-runtime-transport-receipt-boundary/proof.json`
- `output/app-game-plan-proof/208-app-game-child-runtime-transport-receipt-boundary/00-source-snapshot.md`
- `output/app-game-plan-proof/208-app-game-child-runtime-transport-receipt-boundary/10-validation-commands.log`

## Boundaries

Proved:

- Writer-envelope-ready rows can become child-runtime-transport-required
  receipt boundary rows.
- Manual-required and unavailable writer rows remain blocked before transport
  execution.
- Required receipt contract refs can be represented without claiming receipt
  ingestion.
- Runtime transport execution, receipt ingestion, provider delivery, platform
  delivery channel, adapter dispatch, platform enforcement, and raw private
  source rows remain unclaimed.

Not proved:

- Child runtime transport execution.
- Child runtime receipt ingestion.
- Provider delivery execution.
- Platform delivery channel execution.
- Adapter dispatch or platform enforcement.
