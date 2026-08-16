# 41 - Llama GGUF Runtime Packaging Lane

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `41 - Llama GGUF Runtime Packaging Lane`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

llama.cpp/GGUF runtime packaging is product-grade, visible, configurable, and
safe to degrade.

## Where We Are

Rust service has llama-related runtime config/status/cache/generation pieces.
Packaging, artifact integrity, and UI proof need hardening.

## Checklist

- [ ] Verify runtime binary path and version.
- [ ] Verify GGUF artifact path and checksum.
- [ ] Verify acceleration settings.
- [ ] Add install/repair/uninstall states.
- [ ] Add generation smoke proof.
- [ ] Add unavailable/degraded proof.

## Proof

- Runtime install/status tests.
- Generation proof script.
- Portal runtime screenshot.
