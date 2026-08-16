# 28 - TabAgent Model Lifecycle Cache Reuse Candidate

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `28 - TabAgent Model Lifecycle Cache Reuse Candidate`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

TabAgent model lifecycle/cache ideas are adapted into Ocentra model artifact,
runtime status, progress, cache integrity, and unavailable state contracts.

## Where We Are

TabAgent has model manager, model service, inference service, IndexedDB cache,
and Rust model route references. Ocentra already has llama/GGUF runtime status
and cache status foundations.

## Checklist

- [ ] Map model list/load/unload/generate states.
- [ ] Map download/progress/cache stats.
- [ ] Separate model cache from evidence storage.
- [ ] Add checksum/version/license fields.
- [ ] Add corruption and resume states.

## Proof

- Model cache/evidence separation test.
- Cache corruption proof.
- Runtime status proof.
