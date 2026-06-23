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

All AI input, output, runtime, queue, route, memory, graph, explanation, and remote assistant shapes that cross package, crate, app, or plan boundaries are Effect Schema backed and owned by `packages/schema-domain` or another explicitly neutral shared boundary.

## Where We Are

Historical notes referenced `packages/parent-domain` as the AI contract home. That is stale for current central-schema direction. Current routing is:

```text
packages/schema-domain:
  canonical shared AI shapes and parsers.
packages/ai-domain:
  helper/projection/focused validation only.
crates/child-ai-core / crates/screen-ai-core / crates/agent-protocol:
  Rust runtime/parity/wire consumers only when selected.
```

Do not add new cross-plan canonical AI contracts to `parent-domain`, `browser-domain`, `app-game-domain`, `screen-domain`, or portal packages. If those owners need the same shape, promote it to `schema-domain` or consume it from `schema-domain`.

## Owner Path

```text
Primary owner: packages/schema-domain
Allowed consumers: ai-domain, child-ai-core, screen-ai-core, agent-protocol, agent-service, portal-domain/apps/portal when selected
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
npm run build --workspace @ocentra-parent/schema-domain
npm run test --workspace @ocentra-parent/schema-domain -- ai
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
- A helper package cannot re-own a canonical shared shape that belongs in `schema-domain`.

## No-Claim Boundary

This workpack can prove contract/schema readiness for the selected shape family only. It does not prove runtime model execution, provider mesh, local model packaging, evidence capture, policy execution, portal UX, remote assistant readiness, or PR_READY.
