# 15 Read Models And Service Events

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `15 Read Models And Service Events`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

The service emits typed app/game inventory, running, foreground, launcher,
session, approval, capability, and policy read models.

## Scope

- Rust protocol parity for service-facing payloads.
- Service read-model routes/events.
- Typed fields for stale, degraded, manual-required, unavailable, and
  not-claimed rows.
- Portal-compatible DTOs without private raw content.

## Tests And Proof

- TypeScript/Rust field names and enum values match.
- Read models preserve no-claim states.
- Service event payloads serialize exactly.
- Private paths are redacted or represented as refs for UI.

## Done Signal

Portal and policy consumers receive app/game read models from the service rather
than inventing state.

Use the standard checklist in [workpacks README](README.md).

## Completion Notes - 2026-06-03

Completed on branch `codex/app-game-read-model-service-events` with proof under
`output/app-game-plan-proof/15-read-models-and-service-events/`.

Implemented:

- TypeScript activity-surface app-use and games rows now carry app/game
  projection state: product kind, classification, inventory, runtime,
  foreground, capability, last-observed timestamp, source row counts, and
  evidence refs.
- Rust protocol now exposes `AppGameServiceReadModel` for staged journal/SQLite
  projection groups and expands activity-surface app-use/games row structs.
- `agent-core` now returns the app-game service read model over the existing
  staged encrypted-journal SQLite projection.
- `agent-service` now builds app-use and games read models from that service
  projection instead of older summary/report shapes.
- Focused TypeScript, Rust protocol, Rust core, and Rust service tests cover
  serialization, parsing, no-claim state separation, and projection mapping.

Still not claimed:

- Dedicated portal app/game dashboard UI and screenshots.
- Unknown approval and policy-action read models.
- Live OS source subscriptions or platform authority changes.
- Broad app/game blocking or launcher/game budget enforcement.
