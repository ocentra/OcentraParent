# WP04 Queue Idempotency Dead Letter

Scope: prove local queue policy, duplicate handling, idempotency, TTL/deadline, retry, overflow, and dead-letter behavior.

Source rows: `05-implementation-workpacks.md` rows 25-30.

Read next:

- `../05-implementation-workpacks.md` rows 25-30 only
- `../04-tests-proof-and-validation.md`
- `../TEST_PROOF_EXPECTATIONS.md`

Expected outcome:

- No-subscriber behavior, bounded capacity, overflow policy, TTL/deadline, retry, in-flight duplicate guard, idempotency registry, and dead-letter records are explicit.
- Dead-letter output is a typed event with reason, custody, source, retry state, and correlation metadata.
- Queue behavior is deterministic under clock control and does not require real sleeps for tests.

Expected tests/proof:

- `eventing.queue.no-subscriber-policy`
- `eventing.queue.capacity-overflow`
- `eventing.queue.ttl-deadline.manual-clock`
- `eventing.idempotency.duplicate-inflight`
- `eventing.dead-letter.created`
- `eventing.retry-storm.guard`
- Proof includes saturation case, duplicate case, expiry boundary, replay of dead-letter metadata, and skipped-risk note if persistence is deferred.

Failure conditions:

- Do not claim durable queue persistence unless journal/storage proof exists.
- Do not treat duplicate suppression as policy authority.
- Do not hide dropped events without typed reason and observable proof.
