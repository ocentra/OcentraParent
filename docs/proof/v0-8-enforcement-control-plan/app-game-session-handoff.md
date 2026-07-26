# WP05 App/Game Session Handoff Evidence

Branch: `codex/enforcement-wp06-managed-browser-adapter`

The deterministic, ignored proof receipt is kept at
`output/v0-8-enforcement-control-plan-proof/05-app-game-session-handoff/`.
It is intentionally not versioned: repository policy reserves `output/` for
generated proof artifacts.

## Scope proved locally

The app/game timer handoff binds only to persisted SQLite runtime and
sessionization evidence. The focused fixture seeds process observation,
foreground observation, and typed app/game runtime evidence. The resulting
binding carries session and runtime evidence identifiers, process identity,
known classification, timestamp, and running/foreground durations. Timer
expiry revalidates that stored evidence.

## Rejection proof

- Changed process identity is rejected as a mismatch.
- Missing session evidence is rejected before dispatch.
- An unresolved runtime evidence identifier is rejected.
- An unknown runtime classification is rejected; weak identity is not upgraded
  into execution authority.

## Validation receipt

- `cargo test -p ocentra-parent-agent-service --test enforcement_runtime app_game_timer_session_evidence --quiet` — 2 passed.
- `cargo test -p ocentra-parent-agent-service --test app_game_activity_read_models app_game_adapter_dispatch_execute --quiet` — 3 passed.
- Protocol, core, and service `cargo clippy --all-targets -- -D warnings` were run for this packet; the final CI-equivalent rerun is recorded with the PR check state.

## No-claim boundary

This evidence does not claim broad process blocking, mobile parity,
notification delivery, AI authority, portal timer authority, or a completed
parent-visible expiry workflow. Those remain manual-required or follow-on
work. WP05 stays open until its full checklist and CI evidence are complete.
