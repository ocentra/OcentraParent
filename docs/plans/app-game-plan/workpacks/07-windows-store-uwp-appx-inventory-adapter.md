# 07 Windows Store/UWP/AppX/MSIX Inventory Adapter

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `07 Windows Store/UWP/AppX/MSIX Inventory Adapter`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Microsoft Store, UWP, AppX, and MSIX app/game identity is represented separately
from Win32 executable identity.

## Scope

- Package family/name.
- AppUserModelId.
- Store id where available.
- Installed state and source refs.
- Store game package rows.
- Permission-limited source states.

## Tests And Proof

- Store app row decodes.
- Store game row decodes.
- Store package and runtime process merge only through deterministic identity.
- AppUserModelId target can be compiled by later policy work.

## Done Signal

Store/UWP/AppX inventory is first-class evidence and does not collapse into
Win32 display-name matching.

Use the standard checklist in [workpacks README](README.md).
