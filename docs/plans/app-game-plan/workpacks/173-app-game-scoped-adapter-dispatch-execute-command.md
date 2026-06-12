# WP173 App/Game Scoped Adapter Dispatch Execute Command

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP173 App/Game Scoped Adapter Dispatch Execute Command`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Add an explicit parent/dev command for executing the already scoped app/game
adapter dispatch handoff:
`agent.activity.app-game.adapter-dispatch.execute`.

This command is limited to the single scoped Windows owned-process app/game
timer row that WP168 through WP172 already proved eligible, command-ready,
audited, bridged to `agent.enforcement.execute`, and store-backed. The existing
read-model command remains side-effect-free and must not auto-execute through
portal overview polling.

## Implementation

- Add TypeScript agent-protocol command/event contracts for
  `agent.activity.app-game.adapter-dispatch.execute` and
  `agent.activity.app-game.adapter-dispatch.executed`.
- Mirror the command/event names in the Rust protocol.
- Route the agent-service command through the existing
  `agent.enforcement.execute` path only for the scoped Windows owned-process
  row, then return a parent-safe executed result payload.
- Expose the execute command as a manual portal command button while keeping it
  out of the overview auto-poll command list.
- Keep the read-model command available for side-effect-free store readback.

## Validation

- `node scripts/test/app-game-scoped-adapter-dispatch-execute-command-proof.mjs`
- `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- app-game-adapter-dispatch-result contracts`
- `cmd /c npm run test --workspace @ocentra-parent/text-domain -- portal-dev`
- `cmd /c npm run test --workspace @ocentra-parent/portal-domain -- contracts app-game-adapter-dispatch-result-panel`
- `cargo test -p ocentra-parent-agent-protocol app_game_adapter_dispatch_execute_command_and_event_names_serialize_to_contract_shape`
- `cargo test -p ocentra-parent-agent-service app_game_adapter_dispatch_execute`
- `git diff --check`
- `npm run lanes:guard`
- `npm run hub:guard`

## Non-Claims

- Broad installed-app blocking execution is not implemented.
- macOS, Linux, Android, and iOS enforcement remain manual-required,
  unavailable, unsupported, or not claimed according to existing proof rows.
- Provider delivery, provider receipt ingestion, and child-device runtime
  delivery are not implemented by this workpack.
- Raw private source rows, raw target values, and private diagnostics remain out
  of parent-facing payloads.
- The portal overview command list does not auto-run the execute command.
