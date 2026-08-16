# 06 Windows Installed App/Game Inventory Adapter

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `06 Windows Installed App/Game Inventory Adapter`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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
