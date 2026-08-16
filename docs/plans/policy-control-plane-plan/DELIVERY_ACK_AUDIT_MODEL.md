# Delivery Ack Audit Model

Policy delivery is per child profile, child device, domain, and policy version.

## Required delivery dimensions

```text
policy version
target child profile
target child device
domain
delivery route
queued at
sent at
acknowledged at
applied at
rejected reason
retry count
expiry
rollback ref
audit refs
```

## Required states

```text
drafted
previewed
confirmed
queued
delivering
delivered
acknowledged
applied
rejected
superseded
rolledBack
expiredBeforeDelivery
offlineQueued
retryScheduled
partialDomainApply
blockedByPermission
blockedByCapability
manualRequired
```

## Required behavior

- Delivery is idempotent.
- Delivery is ordered.
- Delivery is replay-safe.
- Delivery is visible to the parent.
- Delivery tracks each device and domain separately.
- Active status requires ack or explicit degraded/manualRequired state.
- Partial domain apply must stay visible as partial, not active.

## Negative cases

```text
duplicate delivery creates duplicate active policy
out-of-order stale update overwrites newer policy
offline child shown as active
policy active globally after one domain ack
rejected state lacks audit ref
rollback lacks previous state ref
raw child or policy data appears in logs
```

## Proof expectation

The delivery model is closed only when the proof inventory shows the state machine, idempotency/replay safety, offline retry, per-device-domain ack, rollback audit, redacted logs, and parent-visible status.
