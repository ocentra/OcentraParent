# WP212 App/Game Android Child Runtime Local Receipt Proof

> **Current status (2026-08-29): package-local behavior covered; runtime
> receipt BLOCKED.** Canonical `0505bdd61` adds real Robolectric filesystem,
> receiver-action, tamper, and failure coverage. These tests deliberately do
> not treat same-app files or broadcasts as authenticated service receipts.

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP212 App/Game Android Child Runtime Local Receipt Proof`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Add Android child-app package-local receipt append and readback proof for the
app/game child runtime receipt path.

This moves the Android child package beyond store availability by proving that
it can write and read back a parent-safe internal receipt marker, while still
not claiming service ingestion, provider delivery, platform delivery channel
execution, adapter dispatch, or platform enforcement.

## Implementation

Current Java behavior is package-local proof only. The legacy
`packages/parent-domain` files and proof runner below are retired, and
`MainActivity` is not the launched child runtime. WP211's authenticated receipt
ingress remains the implementation dependency.

- Extended
  `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidChildRuntimeTransportReceiptProof.java`
  with internal receipt append/readback proof states.
- Updated Android `MainActivity` to render local receipt append and readback
  states.
- Added
  `packages/parent-domain/src/app-game-android-child-runtime-local-receipt-proof.ts`.
- Added
  `packages/parent-domain/tests/app-game-android-child-runtime-local-receipt-proof.test.ts`.
- Added `scripts/test/app-game-android-child-runtime-local-receipt-proof.mjs`.

## Validation

Focused validation for this workpack:

```powershell
cmd /c node --check scripts/test/app-game-android-child-runtime-local-receipt-proof.mjs
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-android-child-runtime-local-receipt-proof
cmd /c node scripts/test/app-game-android-child-runtime-local-receipt-proof.mjs
```

## Proof

- `test-results/app-game-android-child-runtime-local-receipt-proof/proof.json`
- `output/app-game-plan-proof/212-app-game-android-child-runtime-local-receipt-proof/proof.json`
- `output/app-game-plan-proof/212-app-game-android-child-runtime-local-receipt-proof/00-source-snapshot.md`
- `output/app-game-plan-proof/212-app-game-android-child-runtime-local-receipt-proof/10-validation-commands.log`

## Boundaries

Proved:

- The Android child app package compiles with package-local internal receipt
  append and readback code.
- The Android activity renders parent-safe receipt append and readback states.
- Parent-domain accepts the proof only when package-local write/readback and UI
  evidence are present.

Not proved:

- Physical child runtime transport execution.
- Service receipt ingestion.
- Provider delivery execution.
- Platform delivery channel execution.
- Adapter dispatch or platform enforcement.
- Raw private source row custody.
