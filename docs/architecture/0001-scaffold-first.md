<!-- agent-capsule -->

> Agent Capsule
> Doc: Decision 0001: Scaffold First
> Kind: architecture/reference documentation; read only when selected by plan route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Decision 0001: Scaffold First

## Status

Accepted.

## Context

Ocentra Parent will become a local-first parental-control system with a
Windows-first agent and parent-owned portal surfaces. The foundation has to be
reliable before capture, local AI safety evaluation, policy decisions,
enforcement, notifications, parent-owned sync, or remote relay/report flows are
implemented.

The first service target is a Rust Windows desktop agent with local loopback and
LAN control surfaces. The shared contracts should still be platform-neutral so
later desktop, mobile, parent-owned portal, storage connector, and Cloudflare
surfaces can reuse the same domain language.

## Decision

Start with repository scaffold, validation infrastructure, and a minimal local visibility loop only.

The first commit should establish:

- workspace layout
- domain packages
- Rust crate boundaries
- Rust local API and WebSocket smoke endpoints
- explicit LAN dev mode with bind and origin guardrails
- Vite dev portal smoke surface
- command/event protocol contracts
- Effect Schema guardrails
- no-Zod enforcement
- branded-string enforcement
- lint, type-check, test, and Rust gates
- CI
- architecture docs

It must not implement recorder logic, product portal UI, capture hooks, policy rules, local AI model runtime, blocking, notification delivery, or Cloudflare runtime behavior.

## Consequences

Future feature work has a place to land without inventing structure during implementation.
