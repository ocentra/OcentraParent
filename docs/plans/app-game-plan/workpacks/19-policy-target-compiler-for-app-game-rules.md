# 19 Policy Target Compiler For App/Game Rules

## Target State

App/game rules compile only with identity, category, evidence, schedule,
approval, authority, and capability proof.

## Scope

- App targets: specific app, package, bundle, AppUserModelId, desktop entry,
  executable hash, publisher, category, unknown, new, portable, risk, all
  non-system.
- Game targets: specific game, launcher game id, store id, category, unknown,
  new, launcher-game candidate, multiplayer, UGC, purchase-capable, mature,
  all games.
- Dry-run before enforcement.

## Tests And Proof

- Specific target requires identity ref.
- Unknown target compiles from unknown state.
- Block launch returns manual-required without proof.
- Wrong device/local user/stale evidence is rejected.
- Policy output carries evidence and capability refs.

## Done Signal

Parent rules compile into typed decisions without inventing evidence or adapter
authority.

Use the standard checklist in [workpacks README](README.md).
