# 17 Unknown App/Game Approval Flow

## Target State

New, unknown, portable, installer, launcher-game candidate, and game-like
executables can request parent approval or remain report-only/manual-required
where proof is missing.

## Scope

- Approval request contracts.
- Evidence refs and child reason/status refs.
- Parent responses: allow once, allow this app/game, allow category, ask child
  why, block if supported, report only.
- Expiry, denial, override, and audit states.

## Tests And Proof

- New inventory app creates candidate.
- Unknown process creates launched candidate.
- Unknown game-like executable stays unknown/possible game.
- Parent approval includes evidence refs.
- Block remains manual-required without adapter proof.
- Approval survives restart where storage exists.

## Done Signal

Unknown apps and games can be handled without auto-promoting weak evidence or
pretending unsupported blocks exist.

Use the standard checklist in [workpacks README](README.md).
