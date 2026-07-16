<!-- agent-capsule -->

> Agent Capsule
> Doc: Local Dev Ports
> Kind: agent flow documentation; read only when selected by root AGENTS or TASK_ROUTER.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Local Dev Ports

Default local ports:

- Rust agent: `127.0.0.1:4477`
- Dev portal: `127.0.0.1:4478`

Worker lanes that need visible demos may set:

```bash
export OCENTRA_PARENT_AGENT_PORT=4677
export OCENTRA_PARENT_PORTAL_PORT=4678
npm run dev
# or npm run dev:agent / npm run dev:portal / npm run dev:lan
```

LAN dev uses the same selected ports with explicit `npm run dev:lan` binding and
origin allowlists. Use managed scripts; they reclaim only stale Ocentra Parent
processes and must not take over Ocentra Games editor ports.
