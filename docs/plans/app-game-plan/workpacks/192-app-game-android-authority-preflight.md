# WP192 App/Game Android Authority Preflight

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP192 App/Game Android Authority Preflight`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Turn the physical Android policy-state proof into a machine-readable authority
preflight for package policy actions.

This proves that Android hide, suspend, uninstall-block, lock-task, and managed
configuration actions are explicitly blocked before adapter dispatch on the
current physical phone because Device Owner/Profile Owner proof is absent.

## Implementation

- Added `packages/parent-domain/src/app-game-android-authority-preflight.ts`.
- Added focused tests for the current not-enrolled physical device,
  `not-proved` owner-state handling, and rejection of raw data, dispatch, and
  enforcement overclaims.
- Added `scripts/test/app-game-android-authority-preflight-proof.mjs`.

## Validation

Focused validation for this workpack:

```powershell
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-android-authority-preflight
cmd /c node scripts/test/app-game-android-authority-preflight-proof.mjs
```

## Proof

- `test-results/app-game-android-authority-preflight-proof/proof.json`
- `output/app-game-plan-proof/192-app-game-android-authority-preflight/proof.json`

## Boundaries

Proved:

- Android package policy actions have explicit authority preflight rows.
- The current physical Android target remains blocked before adapter dispatch
  because Device Owner/Profile Owner proof is absent.
- `not-proved` policy states do not count as owner proof.

Not proved:

- Device Owner/Profile Owner enrollment.
- Android hide, suspend, uninstall block, lock task, or managed configuration
  execution.
- Adapter dispatch, platform enforcement, provider delivery, child-device
  delivery, raw package names, or raw device serial custody.
