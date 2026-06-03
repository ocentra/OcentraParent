# 06 Windows Installed App/Game Inventory Adapter

## Target State

Windows registry, Start Menu, known path, metadata, signature, hash, launcher,
and parent catalog sources populate partial app/game inventory.

## Scope

- Uninstall registry.
- Start Menu shortcuts.
- Known install paths.
- Executable metadata/signature/hash where available.
- Native game launcher entries where available.
- Permission-limited and weak identity states.

## Tests And Proof

- Registry app detected.
- Start Menu app/game detected.
- Same app from registry and shortcut deduplicates by strong identity.
- Same display name alone does not merge.
- Inventory adapter never marks use.

## Done Signal

Windows installed inventory produces typed app/game inventory evidence with
source refs and no-use guards.

Use the standard checklist in [workpacks README](README.md).
