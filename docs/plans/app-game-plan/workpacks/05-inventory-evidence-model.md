# 05 Inventory Evidence Model

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `05 Inventory Evidence Model`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Inventory rows include source, confidence, installed/detectable state, product
kind, category candidates, and no-use claim guards.

## Scope

- Model OS-installed, shortcut, store/package, launcher manifest, parent catalog,
  managed-device, and portable-app inventory sources.
- Include app and game rows in one inventory evidence model.
- Keep inventory separate from runtime, foreground, and session evidence.

## Tests And Proof

- Inventory evidence cannot set `running`, `foreground`, or duration.
- Launcher-installed row does not become game-play row.
- Permission-limited and stale inventory states are preserved.
- Inventory source and custody are retained.

## Done Signal

Inventory can feed parent UI and policy previews without pretending the child
used the app/game.

Use the standard checklist in [workpacks README](README.md).
