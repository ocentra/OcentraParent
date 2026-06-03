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

## Proof

- Queue stress proof.
- Resource degraded-state tests.
- Portal remains responsive screenshot/proof.
