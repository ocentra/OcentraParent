# v0-8-enforcement-control-plan Event Architecture Instruction

## Owns

- enforcement product-control spine;
- owned-process action boundary;
- managed-browser intervention boundary;
- integrity/tamper status bridge;
- enforcement proof router.

## Must not own

- app-game readiness/preflight owner logic;
- policy source authority;
- portal visual polish;
- Apple privileged enforcement proof without Apple host.

## Required chain

```text
policy/app/browser runtime emits eligible enforcement request
-> enforcement owner validates adapter/preflight/manual-required state
-> enforcement action or no-op event is recorded
-> audit/read model updates
-> portal renders service-backed status
```

## Logging/proof

Log adapter class, target class, preflight result, action/no-op/manual-required result, audit entry, child-facing reason, and broad-target no-claim boundary.

## Tests

Rust enforcement/protocol/service tests must move into crate `tests/` categories where counted. Portal proof must be service-backed Playwright. App-game readiness proof must come from app-game owner unless temporarily transferred.

## First architecture slice

Run proof-router truth after duplicate writer claims are resolved. Then Windows/browser/integrity boundaries. Delay app-game service bridge until app-game readiness/preflight ownership is settled.
