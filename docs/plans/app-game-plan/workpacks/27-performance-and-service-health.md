# 27 Performance And Service Health

## Target State

Inventory, runtime polling, journaling, replay, policy, and portal rendering stay
bounded at realistic app/game scale.

## Scope

- Inventory scan bounds.
- Runtime polling cadence.
- Journal/SQLite write volume.
- Session replay cost.
- Policy compile cost.
- Portal row virtualization or pagination where needed.
- Health/degraded status.

## Tests And Proof

- Render 500 app/game rows without layout failure.
- Compile 1,000 app/game rules within budget.
- Replay large evidence fixture within budget.
- Degraded state appears when adapters fail or data is stale.

## Done Signal

The app/game subsystem has performance limits, health states, and proof for
large-enough parent households.

Use the standard checklist in [workpacks README](README.md).
