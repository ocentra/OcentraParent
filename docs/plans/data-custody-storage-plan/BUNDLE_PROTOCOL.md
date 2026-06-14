<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `BUNDLE_PROTOCOL.md`
> Kind: bundle and restore protocol.
> Read when: When a workpack needs export, import, backup, or apply-back rules.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Bundle protocol claims must match the key, event, and UI docs.

<!-- /agent-capsule -->

# Data Custody Storage Plan Bundle Protocol

## Bundle contract

| Field | Required | Notes |
| --- | --- | --- |
| `bundleId` | Yes | Stable identifier for export, backup, import, or support bundle |
| `schemaVersion` | Yes | Explicit protocol version |
| `productVersion` | Yes | Product build or release reference |
| `createdAt` | Yes | Creation time in UTC |
| `sourceHouseholdId` | Yes | Household binding |
| `sourceDeviceId` | Yes | Device binding when present |
| `bundleType` | Yes | Export, backup, import preview, restore, support |
| `dataClasses` | Yes | Declared data classes included |
| `encryptionMode` | Yes | Must describe encrypted payload behavior |
| `keyRef` | Yes | Key reference, not raw key material |
| `payloadHashes` | Yes | Integrity hashes or signatures |
| `tombstoneCursor` | Yes when relevant | Used to prevent resurrection |
| `retentionNotes` | Yes | Retention and expiry context |
| `proofTier` | Yes | Expected proof strength for the bundle |
| `redactionSummary` | Yes | Safe summary for humans/support |

## Required operations

1. Save or export.
2. Retrieve or import preview.
3. Apply or restore.
4. Reject or partially accept.
5. Record rollback or manual-required state when apply fails.

## Bundle states

| State | Meaning | Default action |
| --- | --- | --- |
| `bundleQueued` | Export or restore work is scheduled | Wait |
| `bundleWritten` | Local encrypted bundle exists | Verify |
| `bundleVerified` | Hashes or signatures matched | Preview or apply |
| `bundlePreviewOnly` | Retrieved bundle is read-only pending confirmation | Show preview |
| `bundleApplyPending` | Parent confirmation is required | Wait for confirm |
| `bundleApplied` | Approved data is applied | Record event |
| `bundleRejected` | Validation or custody rules failed | Keep source truth unchanged |
| `bundleCorrupt` | Integrity check failed | Reject |
| `bundleWrongHousehold` | Household binding failed | Reject |
| `bundleWrongKey` | Required key could not unlock bundle | Reject |
| `bundleManualRequired` | Automatic action is unsafe or unsupported | Escalate |

## Restore rules

- Retrieval must not mutate local truth.
- Preview must show source, created time, product version, schema version, household or device match, data classes, conflicts, rejected sections, retention or tombstone impact, and manual confirmation requirement.
- Apply must state what will change, what will not change, which local data remains source of truth, and which tombstones stay preserved.
- Partial restore must explicitly list accepted and rejected sections.
- A bundle that cannot be verified, decrypted, or household-bound must not auto-apply.
- Support cannot read encrypted payloads by default.

## Proof anchors

- `data-custody.export.bundle-contract`
- `data-custody.export.encrypted-payload-proof`
- `data-custody.import.verify-restore`
- `data-custody.import.corrupt-bundle-negative`
- `data-custody.import.wrong-household-negative`
- `data-custody.import.migration-rollback`
- `data-custody.backup.partial-restore`
- `data-custody.export.redacted-summary-proof`

