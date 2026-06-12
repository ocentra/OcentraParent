# 35 Service App/Game Recurring Freshness

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `35 Service App/Game Recurring Freshness`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

The agent-service startup capture path keeps bounded live app/game runtime rows
fresh by repeating the existing activity-capture journal/store flow on a
protocol-owned cadence. The proof stays limited to runtime evidence freshness;
it does not claim foreground capture, policy consumption, portal polish, or
adapter execution.

## Scope

- Move Windows startup activity capture from a one-shot background task to a
  recurring bounded capture loop.
- Keep the cadence in `agent-protocol` constants so service code does not own a
  local runtime literal.
- Prove two service capture cycles append two app/game runtime rows into the
  encrypted journal and ActivityStore.
- Prove foreground state remains not-claimed across recurring runtime capture.
- Preserve opaque executable path refs, unknown-process classification,
  no-content, no-policy-consumer, and no adapter execution boundaries.

## Tests And Proof

- `cargo test -p ocentra-parent-agent-service activity_capture`
- `output/app-game-plan-proof/35-service-app-game-recurring-freshness`

## Done Signal

Service startup now keeps the existing app/game runtime read-model path fresh on
a bounded cadence. Foreground source proof, richer process start/exit
subscriptions, portal source freshness polish, policy consumption, and adapter
execution remain follow-up work.
