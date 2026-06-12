# WP194 App/Game Android Accessibility Overlay Preflight

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP194 App/Game Android Accessibility Overlay Preflight`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Turn physical Android Accessibility settings into a redacted app/game overlay
preflight for warning, block, request, and usage-context overlay actions.

This proves that Accessibility overlay actions have explicit readiness rows and
remain blocked before adapter dispatch until an enabled service, overlay
runtime proof, and child delivery proof exist.

## Implementation

- Added
  `packages/parent-domain/src/app-game-android-accessibility-overlay-preflight.ts`.
- Added focused tests for disabled-service and enabled-service-count states,
  plus rejection of raw service names, overlay execution, adapter dispatch, and
  platform enforcement overclaims.
- Added
  `scripts/test/app-game-android-accessibility-overlay-preflight-proof.mjs`.

## Validation

Focused validation for this workpack:

```powershell
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-android-accessibility-overlay-preflight
cmd /c node scripts/test/app-game-android-accessibility-overlay-preflight-proof.mjs
```

## Proof

- `test-results/app-game-android-accessibility-overlay-preflight-proof/proof.json`
- `output/app-game-plan-proof/194-app-game-android-accessibility-overlay-preflight/proof.json`

## Boundaries

Proved:

- Android Accessibility overlay actions have explicit preflight rows.
- The physical Android target can provide redacted Accessibility settings
  evidence without storing service/component names.
- Overlay actions stay blocked before adapter dispatch until service enablement,
  overlay runtime, and child delivery proof exist.

Not proved:

- Ocentra Accessibility service implementation or enablement.
- Warning, blocking, request, or usage-context overlay execution.
- Adapter dispatch, platform enforcement, provider delivery, child-device
  delivery, raw Accessibility service/component names, or raw overlay content.
