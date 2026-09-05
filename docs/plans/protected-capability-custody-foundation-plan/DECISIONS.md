# Protected Capability Custody Decisions

This plan records architectural decisions in `adr/` using the `ADR-PCC-NNN`
identifier. An accepted decision is an ownership and routing contract, not
implementation, test, proof, runtime, READY, or DONE evidence. The selected
shape is one raw FFI crate plus private Windows adapter modules inside the
existing core; the broker keeps its core/protocol dependency boundary.

| ID | Decision | Status | Canonical scope |
| --- | --- | --- | --- |
| ADR-PCC-002 | One Windows front-door custody module with one raw FFI crate and private safe core modules; TPM2 NV/TBS owns monotonic generation without public authority construction. | Accepted for implementation-only routing; runtime blocked. | [ADR-PCC-002](adr/ADR-PCC-002.md) |
