# Workpack 04: Delivery Ack Audit

Goal: define policy delivery, acknowledgement, conflict, retry, and audit lifecycle.

Owns: event-driven delivery lifecycle, per-child/device/domain status, offline degradation, retry, rollback, and parent-visible audit evidence.

Handoff: eventing and enforcement plans own runtime mechanics; this workpack defines the policy delivery contract and proof.

## Required lifecycle

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

- Policy updates are idempotent, ordered, replay-safe, and observable.
- Delivery is per child profile, child device, and domain.
- Offline children receive pending/degraded state, not fake success.
- Every applied, rejected, superseded, or rolled-back policy has audit evidence.
- Active status requires ack or explicit degraded/manualRequired state.
- Parent UI must show pending, degraded, or manualRequired instead of fake success.

## Required proof IDs

- `policy-delivery.state-machine`
- `policy-delivery.idempotent`
- `policy-delivery.out-of-order-safe`
- `policy-delivery.replay-rejected`
- `policy-delivery.offline-degraded`
- `policy-delivery.retry-safe`
- `policy-delivery.ack-required`
- `policy-delivery.partial-domain-apply`
- `policy-delivery.expired-before-delivery`
- `policy-delivery.superseded-before-ack`
- `policy-delivery.permission-loss-blocked`
- `policy-delivery.rollback-audited`
- `policy-delivery.redacted-log-proof`
- `policy-delivery.parent-visible-state`
- `policy-delivery.per-device-domain-status`

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

## Failure

Do not mark policy active globally when only one domain or device acknowledged, and do not hide offline/manualRequired delivery behind success UI.
