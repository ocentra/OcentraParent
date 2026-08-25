# Protected Capability Custody Decisions

This plan records architectural decisions in `adr/` using the `ADR-PCC-NNN`
identifier. An accepted decision is an ownership and routing contract, not
implementation, test, proof, runtime, READY, or DONE evidence.

| ID | Decision | Status | Canonical scope |
| --- | --- | --- | --- |
| ADR-PCC-002 | One Windows front-door custody module with a raw FFI package and a safe in-process adapter; TPM2 NV/TBS owns monotonic generation. | Accepted for implementation-only routing; runtime blocked. | [ADR-PCC-002](adr/ADR-PCC-002.md) |

