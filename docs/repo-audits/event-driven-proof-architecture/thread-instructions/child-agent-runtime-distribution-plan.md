# child-agent-runtime-distribution-plan Event Architecture Instruction

## Owns

- child runtime package/build proof contracts;
- Windows/Linux/Android/iOS source/package proof routing;
- child package capability proof and release-gate aggregation.

## Must not own

- setup producer contract;
- device-trust semantics;
- app runtime behavior;
- enforcement/tamper runtime beyond package/distribution evidence.

## Required chain

```text
package/proof command
-> build or source proof emits artifact manifest
-> package capability contract is verified
-> setup/device-trust/enforcement consumers cite artifact by typed proof path
```

## Logging/proof

Log package target, artifact path, checksum, platform capability, install/uninstall state, runtime proof status, and host-limited rows.

## Tests

Move proof-shape tests from `unit` to truthful `contract` categories where appropriate. Package lifecycle proof lives in scripts plus canonical output roots.

## First architecture slice

Create proof-root materializer and test-category normalization. Do not close setup-device-trust handoff until setup and device-trust owner contracts exist.
