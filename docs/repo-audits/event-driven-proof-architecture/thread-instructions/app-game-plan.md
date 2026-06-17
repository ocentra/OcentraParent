# app-game-plan Event Architecture Instruction

## Owns

- app/game source freshness, policy preview, timer, notification request, child-facing UX, adapter readiness, platform proof status;
- app-game domain contracts and runtime chain.

## Must not own

- generic app-plan overlay truth;
- enforcement final action authority;
- notification provider runtime unless assigned;
- parent-domain wrapper ownership.

## Required chain

```text
app/game source observation
-> app-game owner evaluates source freshness and policy/timer state
-> app-game event/read model records request or manual-required state
-> enforcement/notification/portal consumers react through typed contracts
```

## Logging/proof

Log source observation, freshness decision, policy/timer decision, notification request, adapter readiness, execution preflight, and platform no-claim boundary.

## Tests

Domain tests stay in app-game-domain. Rust runtime/service tests move to crate `tests/`. Product e2e belongs in service/portal/proof runner and must verify logs/events/read models.

## First architecture slice

Run truth ownership cleanup: remove or replace parent-domain app-game facade ownership, normalize source index, and establish proof-root shape before platform proof.
