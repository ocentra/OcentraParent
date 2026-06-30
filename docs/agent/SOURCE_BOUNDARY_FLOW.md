<!-- agent-capsule -->

> Agent Capsule
> Doc: Source Boundary Flow
> Kind: agent flow documentation; read only when selected by root AGENTS or TASK_ROUTER.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Source Boundary Flow

Use this whenever you touch shared contracts, protocol shapes, TypeScript or
Rust source, tests, route IDs, event names, policy IDs, logging shapes, or app
runtime strings.

## Rule router

Before coding, read `.ocentra-ai/rules/ocentra-parent-rules.mdc`. When changing
multiple layers, use `.ocentra-ai/skills/ocentra-parent-rule-router/SKILL.md`
instead of loading every rule file at once.

Before claiming validation, use `docs/agent/TEST_PROOF_DECISION_MATRIX.md` to
select the required contract, schema, protocol, auth, persistence, security,
performance, AI, observability, or platform proof rows for the touched source.

When the assignment concerns Rust-first parent architecture, read
`docs/agent/RUST_FIRST_PARENT_ARCHITECTURE.md` before using older plan wording.
That document is the current authority when stale docs still describe older
schema-domain or web/WebSocket ownership wording.

## Universal logging/proof-chain gate

For every new or edited runtime, service, domain behavior, app command path,
proof helper, test, script, or Rust crate participating in a command/event/request/read-model chain, read:

- `.ocentra-ai/rules/ocentra-parent-logging-redaction.mdc`
- `docs/repo-audits/event-driven-proof-architecture/LOGGER_USAGE_PATTERN_STANDARD.md`
- `docs/repo-audits/event-driven-proof-architecture/LOGGED_PROOF_CHAIN_STANDARD.md`

This is universal, not Cloudflare-only. Pure schemas, constants, brands, generated data, and static docs are exempt unless they define log/proof shapes.

## Non-negotiable boundaries

- Shared API paths, route ids, event names, log shapes, policy ids, and device
  identifiers do not belong directly in app or crate code.
- Canonical cross-boundary contracts, DTOs, route snapshots, actions, read
  models, and product schema truth live in `crates/schema` or the owning Rust
  domain/runtime crate. `apps/portal/generated/parent-ui-bridge.ts` is checked-
  in generated output from Rust-owned bridge schema, not TS authority.
- `@ocentra-parent/schema-domain`, `@ocentra-parent/agent-protocol-domain`, and
  feature TS domain packages are migration surfaces. They may keep temporary
  edge decoders, generated validation adapters, or pure presentation helpers;
  they must not become new product contract or behavior owners.
- If two TypeScript packages, or one TypeScript package and one Rust boundary,
  need the same product shape, add or move the canonical shape to
  `crates/schema` or the owning Rust crate first. TypeScript consumes generated
  DTOs or temporary edge decoders from that source.
- `crates/agent-protocol` remains a Rust protocol surface for parent/child or
  service wire concerns, but it is not a reason to invent TS-owned product
  contracts.
- Generated TypeScript and any temporary TS edge decoder must preserve the Rust
  encoded field names, discriminants, nullability, enum values, and version
  semantics. Drift coverage is required through Rust serialization tests,
  generated artifact checks, fixtures, or equivalent parity tests.
- Use Effect Schema only at untrusted TypeScript edges or generated validation
  edges. Do not add Zod.
- Do not create new hand-written product brands or DTO authorities in
  TypeScript.
- Runtime source must not contain inline string literals for text, ids, routes,
  fields, commands, or events; canonical contract owners own those.
- App/runtime TypeScript source must not annotate product values as raw
  `string`; use generated DTO types or keep external input as `unknown` until
  parsed at an edge.
- Rust runtime strings live in `crates/schema`, the owning Rust domain crate, or
  Rust protocol constants for transport-only boundaries.
- Do not use mocks, fakes, stubs, spies, MSW, Nock, Sinon, `vi.mock`, `vi.fn`, or
  equivalent test doubles.
- Cross-responsibility behavior must be command/event/request/read-model driven.
  Direct imports are allowed for schemas, constants, brands, and typed
  contracts from their canonical owner; direct imports are not allowed for
  another owner's runtime behavior.

## Package responsibility router

| Package                                 | Owns                                                             |
| --------------------------------------- | ---------------------------------------------------------------- |
| `crates/schema`                         | Canonical cross-boundary Rust DTOs, route snapshots, action/result shapes, generated TypeScript bridge artifacts, and encoded-shape tests. |
| `crates/parent-runtime-core`            | Parent app facade: accepts UI actions, dispatches Rust commands/events, and builds route snapshots. |
| Rust domain/runtime crates              | Product logic, projections, read models, security decisions, policy decisions, and runtime state for their domain. |
| `crates/ocentra-eventing`               | Rust event backbone and internal business-event decoupling.      |
| `apps/parent-desktop/src-tauri`         | Product Tauri command/event host into Rust.                      |
| `apps/portal` / future `apps/parent-ui` | Presentation only: TSX, CSS, assets, generated bridge DTO imports, host/dev adapters, and visual state. |
| `@ocentra-parent/schema-domain`         | Transitional TS edge decoders or generated validation adapters only; no new product authority. |
| `@ocentra-parent/agent-protocol-domain` | Transitional dev/protocol adapters only until Rust/generated consumers replace them. |
| `@ocentra-parent/portal-domain`         | Pure presentation helpers only; no product contracts, route snapshots, or business logic. |
| `@ocentra-parent/text-domain`           | Pure presentation text only, if it does not encode product state or policy. |
| `@ocentra-parent/logging-domain`        | Narrow dev/proof/UI-edge helpers only; Rust owns product proof logs. |
