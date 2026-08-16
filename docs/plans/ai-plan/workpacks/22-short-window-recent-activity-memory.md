# 22 - Short-Window Recent Activity Memory

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `22 - Short-Window Recent Activity Memory`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Recent activity memory helps local safety decisions with bounded time windows
without becoming permanent surveillance memory.

## Where We Are

Activity stores and read models exist for browser, app/game, network, screen,
and policy preview. Recent memory needs a safe selection/index layer.

## Checklist

- [ ] Define recent activity window.
- [ ] Select source-cited evidence summaries.
- [ ] Include expiry and invalidation.
- [ ] Keep scope child/device/policy bounded.
- [ ] Feed context builder through memory refs only.

## Proof

- Recent-memory selection tests.
- Expiry tests.
- Context builder uses memory refs with source citations.
