<!-- agent-capsule -->

> Agent Capsule
> Doc: Local AI Runtime Provider Proof - 2026-05-30
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Local AI Runtime Provider Proof - 2026-05-30

Branch: `codex/local-ai-runtime-provider-proof`

Scope:

- one local `ai-provider` role per physical device
- shared parent/child same-device provider contract
- one local model runtime lane per physical device
- child-safety scheduler priority over parent-assistant work
- queued, degraded, and unavailable provider lifecycle states
- parent-assistant local-provider submission boundary
- duplicate local model load refusal

Proof command:

```powershell
node scripts/test/local-ai-runtime-provider-proof.mjs
```

Expected generated evidence:

```text
test-results/local-ai-runtime-provider-proof/proof.json
```

Product truth boundary:

- This proves same-device local provider/scheduler contracts and service state.
- This does not prove LAN AI provider pooling, cross-device AI job routing, remote/API provider authorization, model quality, or child-safety classifier accuracy.
- Portal remains a read/render surface for this proof; runtime ownership stays in the Rust service/child-agent path.
