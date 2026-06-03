# 07 Capture Trigger Model

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
