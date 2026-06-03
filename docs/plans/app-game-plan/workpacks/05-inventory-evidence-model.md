# 05 Inventory Evidence Model

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
