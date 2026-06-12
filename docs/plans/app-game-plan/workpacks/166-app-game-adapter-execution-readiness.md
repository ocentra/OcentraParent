# WP166 App/Game Adapter Execution Readiness

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP166 App/Game Adapter Execution Readiness`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Project existing V0.8 supported-adapter runtime proof into an app/game-specific
adapter execution readiness read model. The goal is to move from parent-visible
platform limitation UI toward runtime adapter truth without claiming broad
blocking or platform support that is not proved.

## Implementation

- Add `packages/parent-domain/src/app-game-adapter-execution-readiness.ts`.
- Derive app/game rows from the existing
  `V08SupportedAdapterRuntimeProofReadModel`.
- Include native-app and native-game product meanings on each row.
- Mark only `windows-app-game-owned-process-time-limit` as
  `execution-allowed`.
- Keep broad installed-app blocking, Windows artifact/degraded rows, Linux,
  macOS, Android, and iOS blocked before execution.
- Reject broad blocking, child delivery, platform enforcement, provider
  delivery, and private diagnostics claim upgrades.

## Proof

- `packages/parent-domain/tests/app-game-adapter-execution-readiness.test.ts`
  proves:
  - eight app/game adapter rows are projected from the V0.8 runtime proof;
  - one scoped Windows owned-process time-limit row is execution-allowed;
  - seven rows are blocked before execution;
  - claim upgrades are rejected by the schema.
- `scripts/test/app-game-adapter-execution-readiness-proof.mjs` builds
  contracts, runs the focused parent-domain test, imports the built read model,
  and writes `test-results/app-game-adapter-execution-readiness-proof/proof.json`.

## Non-Claims

- No package export change while another lane owns
  `packages/parent-domain/package.json`.
- No service command, Rust protocol, or portal exposure in this workpack.
- No broad installed-app blocking execution.
- No Linux/macOS/Android/iOS platform enforcement.
- No provider delivery or child-device delivery.
- No private diagnostics exposure.

## Validation

- `cmd /c npm run test --workspace @ocentra-parent/parent-domain -- --run tests/app-game-adapter-execution-readiness.test.ts`
- `node scripts/test/app-game-adapter-execution-readiness-proof.mjs`
- `node --check scripts/test/app-game-adapter-execution-readiness-proof.mjs`
- `git diff --check`
- `node scripts/check-no-test-doubles.mjs`
- `node scripts/check-source-shape.mjs`
- `cmd /c npm run lanes:guard`
- `cmd /c npm run hub:guard`

## Status

Done on `codex/app-game-control-product-completion`.
