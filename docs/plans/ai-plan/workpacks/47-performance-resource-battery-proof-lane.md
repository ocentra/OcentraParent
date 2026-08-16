# 47 - Performance Resource Battery Proof Lane

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `47 - Performance Resource Battery Proof Lane`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

AI jobs respect resource budgets, queue backpressure, foreground safety,
battery/thermal state, and portal responsiveness.

## Where We Are

Runtime acceleration settings exist. Product-grade AI needs bounded resource
proof before claiming always-on local intelligence.

## Checklist

- [ ] Add AI job concurrency limits.
- [ ] Add CPU/GPU/RAM fit checks.
- [ ] Add battery/thermal degraded state where platform supports it.
- [ ] Add queue backpressure behavior.
- [ ] Add portal/service responsiveness proof.
- [ ] Add provider class eligibility: desktop-preferred, laptop-preferred,
      mobile-dormant, mobile-fallback.
- [ ] Add foreground/user-active degraded state where relevant.
- [ ] Add school/out-of-LAN minimal local AI fallback policy.

## Proof

- Queue stress proof.
- Resource degraded-state tests.
- Portal remains responsive screenshot/proof.
- Mobile dormant proof.
- Mobile low-battery rejection proof.
- Desktop provider preferred proof.
