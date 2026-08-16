# Workpack 08: Policy Event Model

Purpose: define event families, aggregate keys, idempotency keys, causation/correlation, replay behavior, delivery events, audit events, and rollback events for policy control-plane state.

Owns: event family registry, idempotency and replay safety, delivery events, audit events, rollback linkage, and dead-letter/manual-required handling.

## Required event families

```text
policy.draft.created
policy.preview.requested
policy.preview.generated
policy.confirmed
policy.version.superseded
policy.compiler.requested
policy.compiler.completed
policy.delivery.queued
policy.delivery.sent
policy.delivery.acknowledged
policy.delivery.rejected
policy.delivery.expired
policy.delivery.retry-scheduled
policy.domain.applied
policy.domain.partial
policy.rollback.requested
policy.rollback.applied
policy.ask-parent.requested
policy.ask-parent.approved
policy.ask-parent.denied
policy.override.created
policy.override.expired
policy.audit.recorded
```

## Required proof IDs

- `policy-event.event-family-registry`
- `policy-event.aggregate-key-stable`
- `policy-event.idempotency-key-stable`
- `policy-event.causation-correlation-present`
- `policy-event.replay-safe`
- `policy-event.out-of-order-safe`
- `policy-event.audit-recorded`
- `policy-event.rollback-linked`
- `policy-event.dead-letter-manual-required`
- `policy-event.no-sensitive-log-payload`

## Required behavior

- Every event has stable aggregate and idempotency keys.
- Causation and correlation are explicit.
- Replay is safe.
- Out-of-order events cannot overwrite newer state.
- Rollback can find the prior version.
- Dead-letter/manual-required paths remain visible.
- Audit and log payloads are redacted.

## Negative cases

```text
event has no aggregate key
event has no idempotency key
replay mutates policy twice
out-of-order event overwrites newer state
rollback cannot find prior version
dead-letter hidden from parent or admin state
logs contain child private details or raw policy payload
```

## Required proof artifacts

```text
docs/proof/policy-control-plane-plan/08-event-family-registry-proof.md
docs/proof/policy-control-plane-plan/08-event-idempotency-proof.md
docs/proof/policy-control-plane-plan/08-event-replay-ordering-proof.md
docs/proof/policy-control-plane-plan/08-rollback-event-linkage-proof.md
docs/proof/policy-control-plane-plan/08-event-redaction-proof.md
```
