# 47 - Performance Resource Battery Proof Lane

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
