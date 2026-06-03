# 15 Encrypted Temporary Image Queue

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
