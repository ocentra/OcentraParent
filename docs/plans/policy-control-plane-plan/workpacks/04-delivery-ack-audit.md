# Workpack 04: Delivery Ack Audit

Goal: define policy delivery, acknowledgement, conflict, retry, and audit lifecycle.

Expected shape:

- Policy updates are idempotent, ordered, replay-safe, and observable.
- Offline children receive pending/degraded state, not fake success.
- Every applied, rejected, superseded, or rolled-back policy has audit evidence.

Expected proof:

- Duplicate/out-of-order/replay tests.
- Offline/retry/ack proof.
- Rollback proof.
- Audit and redacted log proof.
- Expiry, superseded policy, and stale child state proof.
- Cross-domain delivery status proof.
- Parent-visible degraded/manual-required state proof.

Failure: policy marked active before child/device/domain acknowledgement or explicit degraded state.

## Decision Tree

| If delivery touches...                          | Required route                               |
| ----------------------------------------------- | -------------------------------------------- |
| Event/queue semantics                           | eventing-plan selected workpack              |
| App/game/browser/network/tracking/screen target | owning domain plan                           |
| Enforcement action/rollback                     | v0-8-enforcement-control-plan                |
| Parent UI status                                | portal-ux-household-surfaces-plan            |
| Offline/remote device                           | remote/lan/account route proof as applicable |

## Execution Detail

Minimum context:

- `docs/plans/eventing-plan/AGENTS.md`
- `docs/plans/app-plan/AGENTS.md`
- `docs/plans/v0-8-enforcement-control-plan/AGENTS.md`

Required lifecycle:

- Drafted.
- Previewed.
- Confirmed.
- Queued.
- Delivered.
- Acknowledged.
- Applied.
- Rejected.
- Superseded.
- Rolled back.
- Degraded/offline/manual-required.
- Expired before delivery.
- Partially applied across domains.
- Blocked by permission/platform/account state.

Rules:

- Active policy status must distinguish parent intent from device acknowledgement.
- Offline child devices keep pending/degraded state.
- Duplicate/out-of-order events are safe.
- Every transition has audit evidence.
- Rollback must reference previous known state and failure reason.
- Parent UI must show pending/degraded/manual-required instead of fake success.
- Delivery state is per child/device/domain, not only global.

Expected tests/proof names:

- `policy-delivery.idempotent`
- `policy-delivery.out-of-order-safe`
- `policy-delivery.offline-degraded`
- `policy-delivery.ack-required`
- `policy-delivery.rollback-audited`
- `policy-delivery.expired-before-delivery`
- `policy-delivery.partial-domain-apply`
- `policy-delivery.permission-loss-blocked`
- `policy-delivery.parent-visible-state`

Proof artifact expectations:

- Delivery state machine.
- Audit event examples.
- Retry/backoff expectations.
- Per-domain/device status examples.
- Redacted log samples and denied replay/duplicate cases.

## Failure Conditions

- Do not mark policy active globally when only one domain/device acknowledged.
- Do not hide offline/manual-required delivery behind success UI.
- Do not omit audit refs for rejected, superseded, rollback, or expired states.
