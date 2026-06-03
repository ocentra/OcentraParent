# 22 - Short-Window Recent Activity Memory

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
