# Eventing WP06 To Enforcement WP11 Handoff

## Scope

This hand-authored durable manifest records reusable local Eventing journal,
replay, idempotency, and topology mechanics plus the narrow production consumer
handoff in `agent-service`. It does not implement an enforcement adapter, choose
an enforcement action, invoke a platform effect, or prove enforcement dispatch.

## Typed mechanics available to the enforcement owner

| Need | Eventing-owned typed surface | Proven local behavior |
| --- | --- | --- |
| Stored record | `StoredEventEnvelope` through `EventJournal::append` / `append_phase` | NDJSON append emits one record per line; the optional hash chain is checked on reopen and replay. |
| Dispatch timing | `JournalDispatchPhase` with explicit `JournalMode` values | Before/after and selected journaling policy are focused-test covered. |
| Duplicate protection | journal idempotent append and typed event/idempotency identities | Reopen and retry do not append duplicate records; reuse for a different event is rejected. |
| Safe replay | `ReplayFilter`, `ReplayCursor`, and `ReplayMode::ProjectionOnly` | Projection replay has no authority to invoke handlers; action delivery requires an explicit action-mode read. |
| Topology review | `EventTopologyManifest` from contracts, publishers, subscribers, family variants, and accepted one-sided entries | Covered, no-publisher, no-subscriber, and accepted-one-sided states are explicit and deterministic. |

## Implemented consumer handoff

`EnforcementAuditJournalEvent` is the redacted typed Eventing payload. It keeps
only audit/action/result identifiers, typed audit/result/adapter/capability
statuses, and observed time; it does not copy actor, target, evidence, policy,
or rollback payloads into the generic NDJSON journal.

The production call path is:

```text
build_enforcement_audit_report_with_paths
  -> execute_enforcement_command
  -> record_eventing_enforcement_audit
  -> append_enforcement_audit_journal_event
  -> hash-chained idempotent Eventing NDJSON append
  -> existing activity audit write
  -> adapter outcome / final audit summary
```

The before-action summary is written before the existing activity audit write
and before adapter selection. The final summary is written before its matching
activity audit write. The Eventing event id remains the audit-summary id, while
its correlation and recorded time are the stable command `message_id` and
`sent_at`. That makes a command retry byte-identical at the Eventing boundary,
so it returns the original journal sequence instead of appending a duplicate.
The append path creates the parent directory before it opens the sidecar;
replay is read only through `ReplayMode::ProjectionOnly`.

## Consumer boundary

Enforcement WP11 consumes the typed audit-summary handoff while retaining its
adapter boundary, authorization, encrypted activity audit storage, and
action/rollback proof. This handoff is `local-bus-only`; it does not prove
cross-process delivery, policy authority, retention/deletion, or platform side
effects.

## Required use constraints

- Use an explicit journal policy and a typed event contract.
- Keep ordinary replay projection-only; an action replay is an
  enforcement-owned exception requiring separate authority and rollback proof.
- Preserve event id, idempotency key, correlation id, schema version, event
  type, aggregate key, and journal phase in consumer proof.
- Correlate the Eventing sidecar to the transport command message id, not to
  an audit-summary id.

## Evidence

- [Journal/replay proof](wp06-01-journal-replay-proof.md)
- [Topology/lineage proof](wp06-02-topology-lineage-proof.md)
- [Validation commands](wp06-16-validation-commands.md)

## Handoff status

The generic Eventing prerequisite and narrow typed audit-summary consumer
handoff are evidenced. Enforcement WP11/WP04 still own adapter authority,
action execution, authorization, activity audit custody, rollback, and
platform-side-effect proof.
