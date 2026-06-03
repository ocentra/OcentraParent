# 41 - Llama GGUF Runtime Packaging Lane

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
