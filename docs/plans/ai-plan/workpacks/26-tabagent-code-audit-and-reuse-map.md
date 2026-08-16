# 26 - TabAgent Code Audit And Reuse Map

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `26 - TabAgent Code Audit And Reuse Map`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

TabAgent reference code is mapped to Ocentra-owned contracts before anything is
copied, extracted, or adapted.

## Where We Are

Local TabAgent and TabAgentServer files were inspected and indexed in
`tabagent-source-index.md`.

## Checklist

- [ ] Confirm current local TabAgent file list.
- [ ] Map native bridge code to Ocentra command/status contracts.
- [ ] Map model lifecycle code to Ocentra runtime/provider contracts.
- [ ] Map cache code to Ocentra model artifact/cache contracts.
- [ ] Map graph code to Ocentra source-cited memory/graph contracts.
- [ ] List non-reused UI/persona/remote/string ids.
- [ ] Add extraction-specific Rust parity, route unavailable/timeout/invalid
      payload proof, and license/ownership notes before any TabAgent code is
      copied or adapted.

## Current Reuse Map

- Native bridge: study TabAgent persistent native connection, route metadata,
  request ids, queueing, and reconnect state. Ocentra reuse belongs behind
  `@ocentra-parent/agent-protocol-domain`, `crates/agent-protocol`, and
  child-agent service command/status contracts.
- Model lifecycle: study load/unload/progress/generation/halt behavior. Ocentra
  reuse belongs behind local runtime/provider status, provider scheduler,
  runtime access lane, unavailable/degraded state, prompt/template, and result
  parser contracts already covered by local AI proof rows.
- Model cache: study manifest, quantization, chunked cache, and cache status
  ideas. Ocentra reuse belongs behind model artifact manifest/cache contracts;
  model cache corruption proof remains open and must not touch evidence
  storage.
- Memory/graph: study graph node/edge/index patterns. Ocentra reuse belongs
  only behind source-cited recent-memory and graph reference contracts; derived
  memory cannot replace encrypted journal or SQLite source truth.
- Not reused: TabAgent UI, dashboard/persona, broad agent workflows, remote/API
  behavior, string route ids, model ids, provider names, and any browser
  behavior that would redefine Ocentra child-safety policy or custody.

## Proof

- Reuse map updated.
- No extracted code without contract and license note.
- No copied TabAgent string ids in app/runtime source.
