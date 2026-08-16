# 08 Windows Process Runtime Evidence Adapter

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `08 Windows Process Runtime Evidence Adapter`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Windows process snapshots, starts, exits, and observed runtime rows produce typed
app/game runtime evidence.

## Scope

- Process id, parent process id, process name, executable path ref, start/exit
  time, observed time, publisher/signature/hash where available.
- Unknown process state.
- Launcher process state.
- Permission-limited metadata state.

## Tests And Proof

- Process appears and creates runtime evidence.
- Same process persists and session can continue.
- Process exits and session can close.
- Unknown process remains unknown.
- Runtime evidence does not claim foreground.

## Done Signal

Runtime evidence can feed sessionization and policy preview without becoming
foreground or content proof.

Use the standard checklist in [workpacks README](README.md).
