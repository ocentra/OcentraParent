# 32 Live Process Snapshot Source

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `32 Live Process Snapshot Source`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

The Rust core can read the local process table through the existing `sysinfo`
dependency and produce app/game runtime evidence records without upgrading that
evidence into foreground, content, policy, or adapter authority.

## Scope

- Add a real process snapshot source module in `agent-core`.
- Feed the existing `WindowsProcessRuntimeRecord` shape.
- Use opaque executable-path refs instead of raw path strings.
- Keep unknown process classification and catalog/launcher/inventory refs empty
  until deterministic catalog or launcher proof exists.
- Prove the current process is captured from the real system process table.

## Tests And Proof

- `cargo test -p ocentra-parent-agent-core app_game_windows_process`
- Current-process snapshot produces one runtime row.
- Runtime rows remain `runtimeRunning` and `foregroundNotClaimed`.
- Executable paths become `path-ref-sha256-*` refs and are not raw paths.

## Done Signal

Live process snapshot records can feed the staged app/game runtime adapter while
remaining source evidence only. Journal subscription, service events, portal
source freshness, policy evaluation, foreground evidence, and adapter execution
remain separate proof-gated work.
