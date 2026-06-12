# 15 Encrypted Temporary Image Queue

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `15 Encrypted Temporary Image Queue`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Encrypted image refs, TTL, retry, digest, source refs, status, deletion required, and redacted paths are implemented.

## Current State

Partial queue foundation exists in `crates/agent-core/src/screen_evidence_queue.rs`.

## Checklist

- [ ] Confirm encryption boundary.
- [ ] Add queue metadata contract.
- [ ] Add bounded capacity/backpressure.
- [ ] Add tamper/unreadable tests.
- [ ] Redact raw paths outside child agent.
- [ ] Report queue health.

## Proof

- Queue encryption tests.
- Queue tamper/retry/TTL tests.
