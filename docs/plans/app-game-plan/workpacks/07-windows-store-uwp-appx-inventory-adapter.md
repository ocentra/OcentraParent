# 07 Windows Store/UWP/AppX/MSIX Inventory Adapter

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
