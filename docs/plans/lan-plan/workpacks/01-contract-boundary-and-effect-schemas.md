# 01 Rust-Owned Contract Boundary And Bridge Validation

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `01 Rust-Owned Contract Boundary And Bridge Validation`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../v0-9-lan-discovery-20-step-plan.md),
[test blueprint](../v0-9-lan-discovery-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

Current V0.9 work already has partial Rust-owned LAN route snapshots, signed
discovery proof, and service/runtime read models. Older notes still mention
TypeScript contract packages and Effect Schema ownership, but those are legacy
edge-consumer terms only, not authoritative product truth. Any legacy Effect
Schema references that still appear at untrusted TS edges are compatibility
artifacts only. The broader production discovery model still needs one
Rust-owned contract boundary for interface evidence, scan evidence, merge
decisions, classification, child-agent hello, heartbeat, assignment, ignore,
rename, revocation, UI state, and proof summaries.

## Where We Want To Be

The workpack name is historical. Effect Schema may still appear at untrusted TS
edges, but it is not the product contract owner. Every LAN discovery value that
crosses service, runtime, route-snapshot, or bridge boundaries must be owned by
Rust first. Prefer the current shared Rust schema owner for cross-plan shapes,
then the owning Rust protocol/service/runtime crate for behavior. TS remains
presentation-only and may consume generated bridge DTOs or edge decoders; it
must not own contracts, read models, or business logic.

## Requirement Checklist

- [x] Define or extend Rust-owned contract families before service, runtime, or
      UI consumers rely on them.
- [x] Keep TS limited to generated bridge types, thin adapters, and
      presentation-only mapping; do not add TS business rules or TS-owned
      contract catalogs.
- [x] Use explicit Rust enums/newtypes plus serde/drift validation at the
      owning contract boundary. TS edge decoders are allowed only at untrusted
      or presentation edges.
- [x] Keep API paths, command names, event names, fields, and display tokens
      out of app/runtime source.
- [x] Define versioned Rust-owned contract families for discovery evidence,
      device record, merge result, child hello, heartbeat, event stream,
      route-snapshot UI state, and proof summary.
- [x] Add invalid payload and drift coverage in proper contract test folders;
      do not rely on inline source-owned tests for closure.

## Acceptance And Proof

- Rust contract tests accept valid payloads and reject malformed,
  missing-field, wrong-version, wrong-family, and future enum payloads safely.
- Generated bridge output or TS edge decoders stay in sync with the Rust-owned
  contract family before presentation consumers rely on the shape.
- Focused Rust contract tests and schema-boundary validation pass.

Current local proof: `output/lan-plan-proof/01-contract-boundary-and-effect-schemas/01-local-validation.md`

Current local closure for this slice:

- `crates/agent-protocol` now fail-closes LAN schema-version drift for the
  selected contract families instead of accepting future versions silently.
- Signed child hello or heartbeat envelopes, mDNS advertisement contracts,
  discovery evidence, event-history rows, and browser add-device route-snapshot
  surfaces all have explicit Rust-owned payload coverage in `tests/contract/`.
- The selected contract slice keeps TS presentation-only; no TS-owned LAN
  contract catalog or business rule was reintroduced.

## Parallel Ownership Notes

This workpack blocks most implementation work. Workers may split by contract
family, but they must not create competing Rust schema owners, duplicate event
names, or hand-written TS mirrors that drift away from Rust truth.
