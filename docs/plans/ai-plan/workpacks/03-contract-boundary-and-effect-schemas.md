# 03 - Contract Boundary And Effect Schemas

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `03 - Contract Boundary And Effect Schemas`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only canonical AI contract/schema boundary after focused tests and proof exist.
> Does not prove: AI runtime, provider readiness, memory readiness, policy readiness, remote assistant readiness, product status, PR readiness, or broad DONE.
> Proof rule: Before DONE, apply `workpacks/00-owner-boundary-proof-gate.md`, select tests in TEST_PROOF_EXPECTATIONS.md, and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

All AI input, output, runtime, queue, route, memory, graph, explanation, and remote assistant shapes that cross package, crate, app, or plan boundaries are Rust-owned in `crates/schema` or another explicitly neutral Rust boundary. TypeScript may keep generated validation or temporary edge decoders only where migration is still incomplete.

## Where We Are

### Current source checkpoint — 2026-08-25

The Rust-owned contract source is integrated at canonical `83382d67b`. The
packet includes the journal and result digest bindings in
`crates/schema/src/ai_contracts/journal/digest.rs` and
`crates/schema/src/ai_contracts/result/digest.rs`, the exporter, and the
generated `packages/schema-domain` edge surface. Independent source review is
implementation-only: no general production caller is mapped and the expected
test source is absent at:

- `crates/ai-contracts/tests/contract/ai_contracts.rs`
- `crates/ai-contracts/tests/contract/ai_contracts_negative.rs`
- `packages/schema-domain/tests/contract/ai-contracts.test.ts`

ADR-AI-001 (`docs/plans/ai-plan/DECISIONS.md`) selects the source-preserving
neutral leaf crate `crates/ai-contracts` / `ocentra-ai-contracts`. The move,
schema/protocol dependency update, explicit WP04 adapter, tests, caller,
focused validation, proof, CI, READY, and DONE remain open. Do not treat the
current schema source or the review at `83382d67b` as completion.

Historical notes referenced `packages/parent-domain` as the AI contract home. That is stale for current central-schema direction. Current routing is:

```text
crates/ai-contracts (`ocentra-ai-contracts`), after the source-preserving move:
  canonical shared AI shapes and parsers in a neutral leaf crate.
crates/schema:
  current reviewed source during migration, then a direct consumer only.
packages/schema-domain:
  temporary generated-validation or edge-decoder surface only where TypeScript still needs one during migration.
packages/ai-domain:
  helper/projection/focused validation only.
crates/child-ai-core / crates/screen-ai-core / crates/agent-protocol:
  Rust runtime/parity/wire consumers only when selected.
```

Do not add new cross-plan canonical AI contracts to `parent-domain`, `browser-domain`, `app-game-domain`, `screen-domain`, or portal packages. If those owners need the same shape, promote it to `crates/schema` or consume it from the relevant Rust owner. Use `schema-domain` only as a temporary generated-validation or edge-decoder surface while migration is still incomplete.

## Owner Path

```text
Primary owner: crates/ai-contracts (`ocentra-ai-contracts`) after migration
Allowed consumers: crates/schema and crates/agent-protocol as direct consumers,
  schema-domain as generated parity/edge validation, ai-domain, child-ai-core,
  screen-ai-core, agent-service, portal-domain/apps/portal when selected
Forbidden owner drift: browser/screen/tracking/network/app-game/policy/enforcement/portal runtime packages defining their own AI contract copies
```

## Checklist

- [ ] Complete local AI input/result contracts in the canonical shared schema layer.
- [ ] Complete runtime/provider/queue/route contracts in the canonical shared schema layer.
- [ ] Complete context-builder request/result contracts with evidence/source refs and custody labels.
- [ ] Complete memory and graph reference contracts with source evidence requirements.
- [ ] Complete AI result journal and explanation contracts.
- [ ] Keep remote assistant contracts separate from child safety and outside the default blocking path.

## Required Proof

```text
proof root: output/ai-plan-proof/03-contract-boundary-and-effect-schemas/
required files:
  00-scope-summary.md
  01-negative-case-proof.md
  02-no-claim-boundary.md
  16-validation-commands.log
```

Focused proof should include:

```bash
cargo test -p ocentra-schema
cargo lint-architecture crates/schema
npm run build --workspace @ocentra-parent/schema-domain
npm run type-check --workspace @ocentra-parent/schema-domain
npm run lint:architecture -- --files packages/schema-domain packages/ai-domain docs/plans/ai-plan
```

If Rust/wire consumers are touched, add the focused Rust/protocol commands from `TEST_PROOF_EXPECTATIONS.md`.

## Negative Cases

- Invalid confidence is rejected.
- Unsourced memory or graph reference is rejected.
- Missing rule/evidence refs block a content-understanding claim.
- Model/provider output cannot become policy/enforcement authority.
- Remote assistant shape cannot enter the normal child safety blocking path.
- A helper package cannot re-own a canonical shared shape that belongs in `crates/schema` or the owning Rust crate.

## No-Claim Boundary

This workpack can prove contract/schema readiness for the selected shape family only. It does not prove runtime model execution, provider mesh, local model packaging, evidence capture, policy execution, portal UX, remote assistant readiness, or PR_READY.

## Graph ownership correction — 2026-08-25

WP03 is the sole owner of the shared TypeScript parity test
`packages/schema-domain/tests/contract/ai-contracts.test.ts`, alongside the
canonical Rust AI contract source and generated schema-domain edge source
listed in `code-map.json`. The next implementation owner is the
source-preserving `ocentra-ai-contracts` leaf migration; it must not add a
public re-export or move authority constructors into the new crate. AI WP04
owns its Rust protocol contract test and explicit wire adapter and is an
explicit consumer of this parity packet (`WP04 -> WP03`); it must not claim the
shared TypeScript test or duplicate WP03 schema ownership. This is a routing
correction with implementation-only evidence; it does not add tests, proof, or
completion.
