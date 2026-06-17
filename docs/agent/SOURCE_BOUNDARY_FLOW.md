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
- Shared TypeScript contracts live under `packages/*-domain`.
- Rust-facing protocol shapes go under `crates/agent-protocol` only after the
  TypeScript contract is explicit and test-backed.
- Use Effect Schema for TypeScript runtime validation. Do not add Zod.
- Do not create manual branded-string aliases. Use Effect Schema brands and
  decode helpers.
- Runtime source must not contain inline string literals for text, ids, routes,
  fields, commands, or events; domain packages own those.
- App/runtime TypeScript source must not annotate values as raw `string`; use a
  branded domain type or keep external input as `unknown` until parsed.
- Rust runtime strings live in `crates/agent-protocol` constants.
- Do not use mocks, fakes, stubs, spies, MSW, Nock, Sinon, `vi.mock`, `vi.fn`, or
  equivalent test doubles.
- Cross-responsibility behavior must be command/event/request/read-model driven.
  Direct imports are allowed for schemas, constants, brands, and typed contracts;
  direct imports are not allowed for another owner's runtime behavior.

## Package responsibility router

| Package                                 | Owns                                                             |
| --------------------------------------- | ---------------------------------------------------------------- |
| `@ocentra-parent/schema-domain`         | Shared Effect Schema wrappers and decode helpers.                |
| `@ocentra-parent/endpoint-domain`       | API path, route id, header, query, endpoint brands.              |
| `@ocentra-parent/agent-protocol-domain` | WebSocket command/event contracts shared by portal and Rust.     |
| `@ocentra-parent/text-domain`           | Schema-backed display text tokens.                               |
| `@ocentra-parent/portal-domain`         | Portal routes, DOM constants, dev command button contracts.      |
| `@ocentra-parent/parent-domain`         | Parent/family/device product contracts.                          |
| `@ocentra-parent/activity-domain`       | Device activity event schemas and query contracts.               |
| `@ocentra-parent/logging-domain`        | Universal structured logging, redaction, proof-chain, app-log, and test-log contracts shared by TS and Rust-facing code. |
