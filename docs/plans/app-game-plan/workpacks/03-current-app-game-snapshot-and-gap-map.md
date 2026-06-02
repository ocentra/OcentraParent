# 03 Current App/Game Snapshot And Gap Map

## Target State

Current TypeScript, Rust, service, portal, proof-script, and docs state is
captured before implementation changes.

## Scope

- Update `current-app-game-snapshot.md`.
- Record existing proof versus missing proof.
- Keep scoped proof described as scoped proof, not product completion.
- Track manual-required and not-claimed states.
- Record worker handoff notes after implementation changes.

## Tests And Proof

- Snapshot links to real source paths.
- Gaps name exact missing contract/runtime/storage/UI/platform proof.
- Old checkpoint or pasted wording does not override current feature docs.
- `git diff --check` passes.

## Done Signal

The snapshot tells the next worker what exists, what is scoped, what is missing,
and what cannot be claimed.

Use the standard checklist in [workpacks README](README.md).
