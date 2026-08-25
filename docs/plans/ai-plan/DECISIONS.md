# AI Plan Decisions

<!-- agent-capsule -->

> Plan: `ai-plan`
> Doc: `AI Plan Decisions`
> Kind: architecture routing decision; it does not prove implementation, tests, proof, READY, or DONE.
> Read when: selecting the canonical owner for the shared AI contract family.
> Stop rule: use the selected workpack and graph mapping for implementation scope; do not infer completion from this decision.

<!-- /agent-capsule -->

## ADR-AI-001: neutral leaf owner for the shared AI contract family

**Status:** accepted routing decision, 2026-08-25. No source move is included in
this documentation packet.

### Context

AI WP03's source packet is real and integrated in the canonical consolidation at
`83382d67b`. It currently lives under `crates/schema`, including the journal and
result digest modules, the exporter, and the generated
`packages/schema-domain/src/generated-ai-contracts.ts` edge. That source is
implementation-only reviewed evidence: the three expected contract/parity test
roots, a general production caller, focused validation, proof, and completion
remain open.

Keeping the whole AI family inside the general schema crate would make the
schema and protocol ownership boundary harder to evolve and risks a dependency
cycle when `agent-protocol` consumes the family. The family needs a neutral leaf
owner rather than a schema or protocol-owned copy.

### Decision

The canonical owner target is a new Rust leaf crate at
`crates/ai-contracts` with package name `ocentra-ai-contracts`. The source
packet must be moved source-preservingly from the current `crates/schema`
AI-contract family. The move must preserve encoded names, discriminants,
validation, digest binding, and generated output semantics. `crates/schema`
and `crates/agent-protocol` may depend directly on the leaf crate; neither may
re-own the family or introduce a public re-export/barrel shim.

The generated TypeScript artifact remains owned by the Rust exporter and
`packages/schema-domain` remains the generated/edge parity consumer during the
migration. AI WP03 owns the two Rust contract tests in the leaf crate and the
shared `packages/schema-domain/tests/contract/ai-contracts.test.ts` parity test.
AI WP04 owns only `crates/agent-protocol/tests/contract/ai_contracts.rs` and
its explicit wire adapter; it consumes the leaf contract and does not duplicate
WP03's tests or schema ownership. A real WP04 caller remains required after the
adapter exists.

### No-claim boundary and migration gate

This ADR does not create the crate, move source, change Cargo manifests, or add
tests. The graph therefore keeps WP03 in validation and exposes only the
source-migration roots plus the WP04 adapter as implementation work. No graph
state may mark WP03 or WP04 READY/DONE from this decision, source presence, or
the `83382d67b` review alone. The test roots, caller, focused gates, proof,
checklist, PR, CI, and promotion remain later gates.
