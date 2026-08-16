# 10 - Evidence Reference Normalization

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `10 - Evidence Reference Normalization`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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
