# 07 Capture Trigger Model

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `07 Capture Trigger Model`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Cadence, foreground app change, managed browser URL change, app/game foreground start, unusual network, policy ambiguity, and manual parent test triggers are modeled.

## Current State

Trigger ideas exist in architecture. Scheduler proof is open.

## Checklist

- [ ] Define trigger enum.
- [ ] Define cadence bounds.
- [ ] Define trigger debounce.
- [ ] Require opt-in before trigger runs.
- [ ] Require capability ready before queueing.
- [ ] Record capture reason in queue job.

## Proof

- Scheduler tests.
- Queue job proof for each enabled trigger.
