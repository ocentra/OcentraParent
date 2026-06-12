# 03 Current App/Game Snapshot And Gap Map

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `03 Current App/Game Snapshot And Gap Map`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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
