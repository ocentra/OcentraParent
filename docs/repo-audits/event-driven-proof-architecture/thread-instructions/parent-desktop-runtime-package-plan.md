# parent-desktop-runtime-package-plan Event Architecture Instruction

## Owns

- parent-client runtime/package proof routing;
- parent web/desktop/Android/iOS distribution boundary;
- artifact identity, update, rollback, release proof where assigned.

## Must not own

- setup producer contract;
- child package proof;
- billing entitlement semantics;
- network/runtime behavior beyond parent-client consumption.

## Required chain

```text
parent package/proof command
-> artifact identity and build output recorded
-> launch/update/readiness proof emitted
-> setup or portal consumer cites typed artifact/readiness proof
```

## Logging/proof

Log artifact identity, build target, install/launch result, update/rollback result, setup handoff state, and host-limited platform rows.

## Tests

Parent-domain package/runtime tests remain targeted. Desktop/mobile package proof belongs in release/proof scripts plus canonical output roots. Apple proof remains external unless an Apple host is assigned.

## First architecture slice

Create proof root and parent-web distribution proof. Then desktop Windows package proof. Setup handoff waits setup-plan producer contract.
