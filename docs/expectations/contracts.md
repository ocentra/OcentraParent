<!-- agent-capsule -->

> Agent Capsule
> Doc: Contract Feature Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Contract Feature Expectations

Contract features define meaning shared across runtimes. V0.2 through V0.5
should use contracts to make evidence, capture, query, and portal behavior hard
to fake and easy to validate.

Current parent architecture is Rust-first. For product surfaces, canonical
contracts, schema truth, actions, route snapshots, read models, and
cross-boundary DTOs belong in `crates/schema`, `crates/parent-runtime-core`, or
the owning Rust domain/runtime crate. TypeScript consumes generated DTOs, thin
adapters, presentation helpers, or temporary edge decoders; it must not become
the product contract owner.

## Outcome Bar

Parent outcome:

- Parent-visible claims map to explicit contracts instead of ad hoc strings,
  hidden browser state, or runtime-only assumptions.
- A later implementation agent can name the evidence, capture, portal, and
  failure shapes before writing product code.

Child-device outcome:

- The agent accepts, emits, stores, replays, queries, and reports activity using
  Rust-owned schema-versioned shapes consumed by generated TypeScript DTOs or
  temporary edge decoders.
- Invalid requests, records, and service payloads fail at the boundary.

Platform scope:

- Contracts should be platform-neutral unless they represent a
  platform-specific capability state or adapter observation.
- Windows-specific process/window details can be adapter-specific internally,
  but emitted activity and status contracts should preserve a cross-platform
  path where possible.

Data scope:

- In scope for V0.2 through V0.5: journal record metadata, activity event
  envelopes, source ids, adapter ids, capture status, query status, ingest
  status, recent activity summaries, Rust-owned parent UI action/query
  envelopes, service health, copy/debug payloads, and typed failure reasons.
- Out of scope: content-inspection contracts, blocking/enforcement contracts,
  stealth/anti-tamper contracts, and local AI decision contracts until the
  roadmap reaches the matching policy or dry-run AI milestone.

Trust boundary:

- Shared contracts define what crosses runtime boundaries.
- Storage internals, OS adapter internals, and portal rendering state are not
  contracts unless another runtime or persisted evidence path consumes them.
- The UI must validate untrusted edge payloads; Rust must validate HostBridge
  actions and service/runtime payloads; journal replay must validate persisted
  events.

## Expected Deliverables

- Rust-owned contract in `crates/schema` or the owning Rust domain/runtime
  crate.
- Rust newtypes/branded primitives for domain-bearing text.
- Generated TypeScript DTOs or explicit temporary edge decoders where
  TypeScript consumes untrusted data.
- Exact valid and invalid TypeScript tests only for edge decoders or generated
  validation edges.
- Rust protocol struct when transport-specific Rust code sends or receives the
  shape.
- Rust serialization/round-trip test with exact field names and values.
- Rust-owned constants for intents, events, fields, ids, and stable strings.
- Versioning strategy for persisted records and service payloads.
- Typed failure/degraded states for unavailable platform capability, validation
  failure, journal corruption, ingest failure, query-store failure, and stale
  HostBridge/dev transport connection.

## Acceptance

- Invalid payloads fail at the boundary.
- Valid payloads parse into branded/domain types.
- Rust-owned schemas and generated TypeScript/edge consumers agree on schema
  version, field names, and enum values.
- The contract does not leak implementation-specific storage details unless
  that is the contract's purpose.
- Runtime code consumes the contract instead of inventing local equivalents.
- Unknown, partial, unsupported, degraded, and unavailable states are explicit
  enum or tagged-union cases, not magic strings in app code.
- Tests would fail if a parent-visible label, field name, schema version, or
  persisted enum changes incompatibly.

## V0.2 Through V0.5 Contract Expectations

V0.2 evidence storage:

- Journal record metadata, event envelopes, ingest status, replay status, query
  status, and recent activity summaries are shared contracts.
- Rebuild and tamper failures have typed reasons.

V0.3 process/window capture:

- Process/window observations have source ids, adapter ids, timestamps,
  observation mode, platform capability state, and clear unsupported/degraded
  reasons.
- Contracts distinguish process/window evidence from URL, page content, and
  network/domain evidence.

V0.4 network/domain observation:

- Network/domain events, when implemented, record attribution confidence and
  unknown attribution as explicit states.
- Contracts do not imply packet capture, payload decryption, or content
  inspection.

V0.5 live activity portal:

- Portal health, recent activity, storage status, capture status, and copy/debug
  output use typed query or event contracts.
- Portal copy/debug contracts define redaction expectations and must not expose
  secrets or raw private content.

## Non-Goals

- Do not add runtime behavior just because a contract exists.
- Do not add broad future fields without a concrete expected use.
- Do not create parallel contracts for the same concept in multiple packages.
- Do not create content-inspection, blocking, stealth, or local AI decision
  contracts before the matching roadmap milestone.
- Do not encode product claims in docs or UI text before the contracts can prove
  the claim.

## Validation Gates

- Rust serialization and round-trip tests for every new or changed product
  contract.
- TypeScript valid and invalid parser tests only for generated validation or
  untrusted edge decoders.
- Rust serialization/parity tests when Rust sends, receives, stores, or replays
  the shape.
- Schema-boundary and no-naked-string checks.
- Service integration tests for request/event contracts when a runtime boundary
  is touched.
- Portal Playwright or component-level validation only after the UI consumes the
  contract through the real service path.

## Done Signal

The contract can be used from Rust and generated TypeScript/edge consumers
without string guessing, and tests would fail if a field name, enum value,
schema version, parent-visible claim, persisted payload, or domain primitive
changed incorrectly.
