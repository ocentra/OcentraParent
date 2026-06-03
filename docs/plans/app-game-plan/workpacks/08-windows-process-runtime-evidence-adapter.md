# 08 Windows Process Runtime Evidence Adapter

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
