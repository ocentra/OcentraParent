# WP207 App/Game Child-Device Runtime Writer

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP207 App/Game Child-Device Runtime Writer`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Convert app/game child-device delivery readiness rows into runtime-writer
envelope rows while keeping runtime execution unclaimed.

This is a boundary contract for the eventual child-device delivery path; it is
not a transport implementation.

## Implementation

- Added
  `packages/parent-domain/src/app-game-child-facing-ux-child-device-runtime-writer.ts`.
- Added
  `packages/parent-domain/tests/app-game-child-facing-ux-child-device-runtime-writer.test.ts`.
- Added `scripts/test/app-game-child-device-runtime-writer-proof.mjs`.

## Validation

Focused validation for this workpack:

```powershell
cmd /c node --check scripts/test/app-game-child-device-runtime-writer-proof.mjs
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-child-facing-ux-child-device-runtime-writer
cmd /c node scripts/test/app-game-child-device-runtime-writer-proof.mjs
```

## Proof

- `test-results/app-game-child-device-runtime-writer-proof/proof.json`
- `output/app-game-plan-proof/207-app-game-child-device-runtime-writer/proof.json`
- `output/app-game-plan-proof/207-app-game-child-device-runtime-writer/00-source-snapshot.md`
- `output/app-game-plan-proof/207-app-game-child-device-runtime-writer/10-validation-commands.log`

## Boundaries

Proved:

- Transport-required delivery readiness rows can become writer-envelope-ready
  rows.
- Manual-required and unavailable rows remain non-executable.
- Writer rows carry parent-safe target and audit refs only.
- Runtime writer execution, child runtime transport, receipt ingestion, provider
  delivery execution, platform delivery channel, adapter dispatch, platform
  enforcement, and raw private source rows remain unclaimed.

Not proved:

- Runtime writer process execution.
- Child runtime transport attachment.
- Child runtime receipt ingestion.
- Provider delivery execution.
- Platform delivery channel execution.
- Adapter dispatch or platform enforcement.
