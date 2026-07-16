# Policy Lifecycle

Policy lifecycle is a sequence of separate artifacts. No single `active` flag is allowed to hide the state transitions.

## Lifecycle

```text
parent intent
-> draft policy
-> validation
-> dry-run preview
-> parent confirmation
-> versioned policy source document
-> domain compiler outputs
-> delivery queue
-> child/device/domain acknowledgement
-> active / degraded / rejected / partial state
-> audit trail
-> rollback / supersede path
```

## Lifecycle rules

- Parent intent is not yet policy truth.
- Draft and preview are not yet active policy.
- Confirmed source policy exists before any compiled artifact is treated as a release candidate.
- Delivery and acknowledgement are per target, not global.
- Active state is only honest when the required target/domain acknowledgements are known or an explicit degraded/manualRequired state is recorded.

## Required lifecycle artifacts

- Draft policy record.
- Preview record.
- Confirmed versioned source document.
- Domain compiler outputs.
- Delivery queue record.
- Acknowledgement record.
- Audit event record.
- Rollback or supersede record.

## States that must remain distinguishable

```text
draft
previewed
confirmed
queued
delivered
acknowledged
active
partiallyActive
rejected
superseded
rolledBack
stale
expired
manualRequired
```

## Negative cases

```text
preview state treated as active
confirmed source policy skipped before compile
delivery marked complete without ack
global active claim without per-target status
rollback loses prior version reference
audit trail missing for supersede
```

## Proof expectation

The lifecycle closes only when the proof inventory shows each transition, the per-target delivery/ack states, and the rollback/supersede path.
