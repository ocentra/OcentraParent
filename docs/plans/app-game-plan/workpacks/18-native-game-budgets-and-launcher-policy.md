# 18 Native Game Budgets And Launcher Policy

## Target State

Native games have game-specific budgets and rules that can count known games and
explicit launcher-game candidates without counting launcher-only rows as play.

## Scope

- Game budget targets.
- Candidate inclusion policy.
- Launcher-only exclusion/default posture.
- Game category/rating/multiplayer/UGC/purchase inputs.
- Budget dry-run and parent preview.

## Tests And Proof

- Known game session counts toward game budget.
- Launcher-only session does not count by default.
- Launcher-game candidate counts only when parent policy allows candidate state.
- Rating/UGC/multiplayer/purchase signals do not enforce directly.

## Done Signal

Game budgets are useful and honest without treating every launcher row as play.

Use the standard checklist in [workpacks README](README.md).
