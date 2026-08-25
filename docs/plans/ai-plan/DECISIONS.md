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
`6318d5e3d`. The complete contract family now lives in the neutral
`crates/ai-contracts` / `ocentra-ai-contracts` leaf, including the journal and
result digest modules. The existing `crates/schema` exporter is the only current
consumer, producing the generated `packages/schema-domain` edge. This is
implementation-only reviewed evidence: no general `agent-protocol` or
`agent-service` consumer/provider-owner composition is present, and the three
expected contract/parity test roots, focused validation, proof, and completion
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

This ADR records ownership and routing; the source-preserving move is separately
reviewed at `6318d5e3d`, and no tests are added by this decision. The graph keeps
WP03's normal lifecycle in validation while recording its bounded
implementation-phase evidence and exposes WP04 only through the reviewed
implementation gate. No graph state may mark WP03 or WP04 READY/DONE from this
decision or source evidence alone. The three test roots, general caller and
provider-owner composition, focused gates, proof, checklist, PR, CI, and
promotion remain later gates.
