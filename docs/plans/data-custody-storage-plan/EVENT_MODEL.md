<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `EVENT_MODEL.md`
> Kind: event model.
> Read when: When a workpack needs the custody event families and payload boundaries.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Event payloads must stay aligned with bundle, retention, report, and UI rules.

<!-- /agent-capsule -->

# Data Custody Storage Plan Event Model

## Event families

| Family | Examples | Allowed payload | Must not include |
| --- | --- | --- | --- |
| `data-custody.storage.*` | destination chosen, connector linked, connector revoked | Mode, provider ref, state, actor role | Raw child evidence |
| `data-custody.key.*` | key provisioned, key revoked, key lost, manual-required | Key ref, household ref, state | Raw key material |
| `data-custody.bundle.*` | export, retrieve, preview, apply, reject | Bundle ref, hashes, proof tier, state | Plaintext sensitive payload |
| `data-custody.sync.*` | queued, syncing, success, conflict, quota, offline, revoked | Provider ref, cursor, retry, state | Unencrypted payload |
| `data-custody.retention.*` | policy changed, delete requested, tombstone written, propagated, expired | Data class, retention class, tombstone ref | Deleted sensitive payload |
| `data-custody.report.*` | generated, stale, redacted, invalidated | Source refs, status, freshness | Private evidence not allowed by custody |
| `data-custody.query.*` | cursor advanced, rate limited, expired | Cursor, page, status | Broad raw evidence dumps |
| `data-custody.notification.*` | routed, redacted, denied | Minimal payload, destination, reason | Child activity details beyond custody boundary |
| `data-custody.support.*` | bundle prepared, bundle shared, support denied | Bundle ref, redaction summary | Raw child activity by default |

## Event invariants

- Events must be idempotent where replay is possible.
- Events must be ordered by explicit sequence or cursor when replay matters.
- Event payloads should reference bundle or source data rather than carrying raw sensitive content.
- Any event that crosses a custody boundary must already be classified and, where needed, encrypted.
- Manual-required is a first-class event outcome, not an error afterthought.

## Required event fields

- `eventId`
- `householdId`
- `deviceId` when relevant
- `actorRole`
- `timestamp`
- `sequence` or `cursor` when ordering matters
- `dataClass`
- `status`
- `reason`
- `proofRef`

## Proof anchors

- `data-custody.events.family-contract`
- `data-custody.events.no-plaintext-payload`
- `data-custody.events.replay-protection`

