# 32 Live Process Snapshot Source

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
