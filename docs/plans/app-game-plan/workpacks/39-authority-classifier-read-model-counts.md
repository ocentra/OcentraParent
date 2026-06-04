# WP39 - Authority Classifier Read-Model Counts

## Scope

Expose explicit staged app/game boundary row counts in the existing service
app-use and games read-model payloads.

The counts cover evidence claim, identity, approval authority, approval
action-result, platform authority matrix, platform authority matrix rows, and
AI classifier result rows already present in `AppGameServiceReadModel`.

This workpack does not add live classifier/provider execution, dedicated
authority/classifier event streams, portal rows, policy consumption, adapter
execution, broad blocking, or platform support claims.

## Implementation

- Extend the shared activity-surface app-use and games read-model schemas with
  staged boundary count fields.
- Mirror the additive fields in the Rust `agent-protocol` app-use and games
  read-model row structs.
- Add an `agent-service` helper that counts staged boundary rows from
  `AppGameServiceReadModel`.
- Populate the counts on both app-use and games read-model rows.
- Keep recent-summary fallback app-use rows at zero because they do not carry
  the app/game staged boundary read model.
- Add focused TypeScript, Rust protocol, and service projection assertions for
  the new fields.

## Proof

- `cmd /c npm exec --workspace @ocentra-parent/activity-domain -- vitest run tests/activity-surface.test.ts`
- `cmd /c npm run build --workspace @ocentra-parent/activity-domain`
- `cmd /c npm exec --workspace @ocentra-parent/agent-protocol-domain -- vitest run tests/activity-surface-adapter.test.ts`
- `cargo test -p ocentra-parent-agent-protocol activity_app_use_read_model_serializes_app_game_projection_state`
- `cargo test -p ocentra-parent-agent-protocol activity_games_read_model_serializes_launcher_source_counts`
- `cargo test -p ocentra-parent-agent-service app_game_boundary_evidence`
- `cargo fmt --check`

Proof artifacts live in:

```text
output/app-game-plan-proof/39-authority-classifier-read-model-counts
```

## No-Claim Boundaries

- Count fields are read-model payload metadata only.
- They do not prove live model/provider execution.
- They do not create a dedicated authority/classifier event stream.
- They do not render portal authority/classifier rows.
- They do not trigger policy decisions, adapter execution, broad app blocking,
  or platform support.

## Product Doc Decision

`docs/product-capability-checklist.md` is intentionally unchanged. WP39 exposes
existing staged boundary row counts in app-use/games service payloads only;
product status should not move until live classifier/provider execution, policy
consumption, portal rows, adapter proof, and platform support exist.
