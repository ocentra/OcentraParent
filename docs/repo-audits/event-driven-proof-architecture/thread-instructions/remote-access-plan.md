# remote-access-plan Event Architecture Instruction

## Owns

- remote access route, capability, standing grant, session, revoke/remove-device, audit, and view-only contracts.

## Must not own

- account/session authority;
- device-trust semantics;
- LAN route truth;
- screen live-view primitive ownership;
- logging/proof infrastructure.

## Required chain

```text
parent remote request
-> remote owner validates route/capability/grant/session
-> child/runtime gate consumes typed remote grant
-> service emits remote session/audit event
-> portal renders view-only/degraded/revoked state
```

## Logging/proof

Log grant source, actor/device scope, transport route, view-only boundary, revoke/remove-device outcome, child disclosure state, and relay/degraded result.

## Tests

Remote-domain/core own contract and parity tests. Protocol/service/portal proof starts after contract parity. Cross-platform e2e belongs in service/portal/proof runner.

## First architecture slice

Hold until account/device/LAN basics are sequenced. Then run RA-01 contract parity and test repair. Do not build portal remote UI before protocol/service contracts exist.
