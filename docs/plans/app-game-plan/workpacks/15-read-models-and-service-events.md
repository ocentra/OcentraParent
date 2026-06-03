# 15 Read Models And Service Events

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
