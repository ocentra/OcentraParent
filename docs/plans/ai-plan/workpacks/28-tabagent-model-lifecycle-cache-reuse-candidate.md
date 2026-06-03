# 28 - TabAgent Model Lifecycle Cache Reuse Candidate

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
