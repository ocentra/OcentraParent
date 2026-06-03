# 10 - Evidence Reference Normalization

## Target State

Browser, app/game, network, screen, tracking, LAN, parent-action, policy, AI, and
memory refs share a common reference envelope.

## Where We Are

Each slice has emerging evidence contracts. AI needs one common evidence ref
shape for context, result, policy, journal, explanation, and memory use.

## Checklist

- [ ] Define common evidence ref fields.
- [ ] Add source/custody labels.
- [ ] Add freshness/degraded/unknown fields.
- [ ] Add confidence kind where probabilistic.
- [ ] Add source evidence refs for derived summaries.

## Proof

- Contract tests for every evidence kind.
- Derived summary without source refs is rejected.
- Missing confidence on probabilistic claims degrades or rejects.
