<!-- agent-capsule -->

> Agent Capsule
> Doc: API AI Provider Authorization Proof
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# API AI Provider Authorization Proof

Date: 2026-05-31
Branch: `codex/parent-assistant-api-ai-authorization-custody-runtime-proof`
Focused proof command: `node scripts/test/api-ai-provider-authorization-proof.mjs`

## Scope

This checkpoint records the Parent Assistant optional API AI provider boundary
for explicit parent authorization, custody labels, retention/deletion terms,
and evidence citations.

## Proof Boundary

- Environment flags alone do not authorize API AI provider use.
- Authorized/degraded API AI provider state requires explicit parent payload
  terms for authorization, custody, retention, and deletion.
- The API provider remains unavailable/degraded metadata for Parent Assistant
  only; it does not answer child-safety, timing, ask-parent, blocking, or
  enforcement decisions.
- Parent Assistant answers continue to use local/LAN provider priority and fail
  closed when the local provider is unavailable.
- Evidence citations remain required before any API provider boundary can expose
  an authorized state.

## Non-Claims

- This branch does not implement a remote/API AI adapter or send evidence to a
  remote provider.
- This branch does not change child-device enforcement, timing, blocking,
  policy writes, or ask-parent decisions.
- This branch does not add portal UI for API provider authorization.

## Evidence

- `packages/parent-domain/src/parent-assistant.ts`
- `packages/agent-protocol-domain/src/defaults.ts`
- `crates/agent-protocol/src/constants/field.rs`
- `crates/agent-protocol/src/constants/parent_assistant.rs`
- `crates/agent-service/src/parent_assistant_api/api_boundary.rs`
- `crates/agent-service/src/parent_assistant_runtime.rs`
- `scripts/test/api-ai-provider-authorization-proof.mjs`

Generated proof evidence is the focused script result:

```text
api-ai-provider-authorization-proof-ok
```
