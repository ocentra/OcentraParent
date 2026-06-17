# app-plan Event Architecture Instruction

## Owns

- app-plan truth overlay and native-app proof routing where distinct from app-game;
- cross-platform app evidence route docs and proof contracts when not delegated.

## Must not own

- app-game domain/runtime truth that belongs to app-game-plan;
- policy source truth;
- enforcement runtime authority;
- stale activity-domain or parent-domain app-game paths.

## Required chain

```text
app route/proof requirement
-> delegate to app-game or platform owner when behavior is not app-plan-owned
-> consume typed proof/read-model artifact
-> publish app-plan aggregate only after owner proof exists
```

## Logging/proof

Log delegation decision, owner proof reference, platform status, and no-claim rows. App-plan proof should not regenerate sibling-owned behavior under a shadow owner.

## Tests

App-plan should test overlays, routing, and aggregate proof shape. Runtime tests belong to app-game, policy, enforcement, service, or platform owner.

## First architecture slice

Do app-plan truth repair only. Then decide which rows are delegated to app-game before any source implementation expands.
