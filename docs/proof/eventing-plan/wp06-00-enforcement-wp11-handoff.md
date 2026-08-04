# Eventing WP06 To Enforcement WP11 Handoff

## Scope

This hand-authored durable manifest records only reusable local Eventing
journal, replay, idempotency, and topology mechanics. It does not implement an
enforcement adapter, choose an enforcement action, invoke a platform effect, or
prove enforcement dispatch.

## Typed mechanics available to the enforcement owner

| Need | Eventing-owned typed surface | Proven local behavior |
| --- | --- | --- |
| Stored record | `StoredEventEnvelope` through `EventJournal::append` / `append_phase` | NDJSON append emits one record per line; the optional hash chain is checked on reopen and replay. |
| Dispatch timing | `JournalDispatchPhase` with explicit `JournalMode` values | Before/after and selected journaling policy are focused-test covered. |
| Duplicate protection | journal idempotent append and typed event/idempotency identities | Reopen and retry do not append duplicate records; reuse for a different event is rejected. |
| Safe replay | `ReplayFilter`, `ReplayCursor`, and `ReplayMode::ProjectionOnly` | Projection replay has no authority to invoke handlers; action delivery requires an explicit action-mode read. |
| Topology review | `EventTopologyManifest` from contracts, publishers, subscribers, family variants, and accepted one-sided entries | Covered, no-publisher, no-subscriber, and accepted-one-sided states are explicit and deterministic. |

## Consumer boundary

Enforcement WP11 may consume these generic mechanics only after it owns a
typed enforcement event contract, an adapter boundary, authorization, audit
storage, and action/rollback proof. This handoff is `local-bus-only`; it does
not prove cross-process delivery, policy authority, retention/deletion, or
platform side effects.

## Required use constraints

- Use an explicit journal policy and a typed event contract.
- Keep ordinary replay projection-only; an action replay is an
  enforcement-owned exception requiring separate authority and rollback proof.
- Preserve event id, idempotency key, correlation id, schema version, event
  type, aggregate key, and journal phase in consumer proof.

## Evidence

- [Journal/replay proof](wp06-01-journal-replay-proof.md)
- [Topology/lineage proof](wp06-02-topology-lineage-proof.md)
- [Validation commands](wp06-16-validation-commands.md)

## Handoff status

The generic Eventing prerequisite is evidenced. Enforcement WP11/WP04 still
own adapter authority, action execution, authorization, audit, rollback, and
platform-side-effect proof.
