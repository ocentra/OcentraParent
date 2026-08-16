# Workpack 04: Delivery Ack Audit

Goal: define policy delivery, acknowledgement, conflict, retry, and audit lifecycle.

Owns: event-driven delivery lifecycle, per-child/device/domain status, offline degradation, retry, rollback, and parent-visible audit evidence.

Handoff: eventing and enforcement plans own runtime mechanics; this workpack defines the policy delivery contract and proof.

## Current implementation boundary

- [x] The policy-owned delivery contract covers explicit lifecycle states, audit and rollback linkage, fail-closed receipt-required transitions, receipt evidence validation, and degraded parent-visible behavior.
- [x] No production execution-authority entry is exposed; caller-supplied receipt fields are evidence only, cannot advance delivery state, and cannot generically hydrate schema-v2 acknowledged, applied, or rolled-back history.
- [x] Schema-v1 receiptless acknowledged and rolled-back history is retained as explicitly unverified compatibility data and surfaces `manualRequired` rather than active success.
- [x] Until a trusted adapter exists, the child-policy handoff converts acknowledged and applied requests into typed `manualRequired` state rather than fabricating active success.
- [ ] A trusted domain- or enforcement-owned adapter must perform the real side effect, emit the required inspectable execution trace, and provide non-forgeable execution authority before acknowledged, applied, or rolled-back advancement can be proven at runtime.

Status: contract checked; runtime blocked on the domain/enforcement handoff. The current public policy surface fails closed and cannot advance receipt-required acknowledged, applied, or rolled-back state.

## Production-code audit — 2026-08-16

The exact missing composition point is `crates/child-policy-core/src/policy_control_delivery_handoff.rs::apply_trusted_adapter_delivery_handoff`: it accepts a public `PolicyDeliveryExecutionReceipt`, but no domain- or enforcement-owned capability issues that receipt or supplies an inspectable execution trace. `crates/agent-core/src/enforcement_adapter.rs` currently returns an enforcement outcome and optional rollback token; no production bridge connects that outcome to policy delivery authority, execution identity, or rollback trace. The smallest legal implementation slice belongs to the v0-8 enforcement-control/domain adapter owner, followed by the child-policy handoff. This workpack remains runtime-blocked here; caller receipts remain evidence only.

## Ownership boundary

```text
policy-control-plane-plan owns delivery contract, per-child/device/domain status, ack requirement, retry/degraded states, rollback refs, and audit proof.
eventing-plan owns reusable event bus, idempotency, replay, journal, and request/response mechanics.
domain plans own runtime apply behavior after typed handoff.
v0-8-enforcement-control-plan owns enforcement authority and action execution.
portal plan owns rendered parent-visible state.
```

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

## Required proof fields

The selected proof must name, at minimum:

```text
delivery_id
source_policy_version
child_profile_ref
child_device_ref
domain_target
delivery_state
ack_state
retry_state
offline_state
partial_apply_state
rejected_state
superseded_state
rollback_state
previous_state_ref
audit_ref
redaction_state
parent_visible_state
enforcement_authority_state
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

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

Do not mark policy active globally when only one domain or device acknowledged, and do not hide offline/manualRequired delivery behind success UI. Do not claim enforcement authority from delivery proof.
