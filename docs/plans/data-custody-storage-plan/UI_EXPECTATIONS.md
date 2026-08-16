<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `UI_EXPECTATIONS.md`
> Kind: UI expectations and copy contract.
> Read when: When a workpack needs the parent-facing storage and restore UI contract.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: UI claims must be supported by proof states, screenshots, traces, or manual-required notes.

<!-- /agent-capsule -->

# Data Custody Storage Plan UI Expectations

## Required screens

- Data and Backup
- Storage destination
- Encryption and key status
- Data classes included
- Backup and sync health
- Restore and import preview
- Retention and delete status
- Provider account or folder
- Manual-required gaps
- Support and export safety

## Required current mode labels

- Local only
- Local plus encrypted backup
- Local plus encrypted provider sync
- Provider disconnected
- Provider error
- Manual required
- Disabled

## Required state classes

| State | Meaning |
| --- | --- |
| `providerNotConfigured` | No provider selected |
| `providerAuthExpired` | Provider auth must be renewed |
| `providerPermissionMissing` | Selected scope or folder access is missing |
| `providerRevoked` | Provider access was removed |
| `providerQuotaExceeded` | Provider storage quota blocked the write |
| `providerUnavailable` | Provider API or network is down |
| `localStoreUnavailable` | Local source of truth is unavailable |
| `keyUnavailable` | Required key is not present |
| `keyRevoked` | Key was intentionally removed |
| `wrongHousehold` | Bundle or provider content does not match household |
| `wrongDevice` | Device binding does not match |
| `schemaUnsupported` | Bundle version is not supported |
| `bundleCorrupt` | Bundle integrity failed |
| `tombstoneConflict` | Delete protection blocks apply |
| `manualRequired` | Automatic action is unsafe or unsupported |
| `offlineQueued` | Local work is queued for later sync |
| `syncDisabled` | Sync is intentionally off |
| `remoteDisabled` | Remote custody is disabled |
| `ocentraHostedStorageNotUsed` | Ocentra-hosted storage is not the selected path |

## Copy requirements

- The UI must explain the custody boundary before the parent picks a destination.
- The UI must show which data classes are included and excluded.
- The UI must show the last successful backup or sync and the last failure.
- The UI must show encryption status and key status separately.
- The UI must show restore preview before apply.
- The UI must separate disconnect from delete.
- The UI must not promise a completed action without proof.

## Required warning copy

> Ocentra does not store child activity data by default.
> Your selected storage provider may see encrypted file metadata such as file name, size, and modified time.
> Sensitive data is encrypted before upload.
> If you lose your recovery key or device keys, Ocentra may not be able to recover encrypted child activity data.
> Disconnecting a provider stops future sync but does not automatically delete files already written there unless you request deletion and proof succeeds.
> Deleting local data may require tombstones so old backups do not restore deleted evidence.

## No-claim language

The UI must never say:

- Backed up
- Synced
- Restored
- Deleted
- Encrypted
- Private
- Safe
- Complete

unless the corresponding proof state exists.

Use precise status language:

- Backup queued
- Encrypted bundle written
- Provider upload pending
- Provider upload failed
- Provider upload confirmed
- Import preview passed
- Apply requires confirmation
- Tombstone propagation pending
- Deleted locally; provider delete pending
- Provider disconnected; existing files may remain
- Manual proof required

## Restore preview flow

1. Retrieve bundle.
2. Validate and preview import.
3. Confirm apply.

Preview must show:

- bundle source
- created at
- product version
- schema version
- household or device match
- data classes
- retention or tombstone impact
- conflicts
- rejected sections
- partial restore status
- required parent confirmation

Apply must show:

- what will change
- what will not change
- which local data remains source of truth
- which tombstones will be preserved
- which conflicts need manual review
- rollback or manual-required behavior

## Delete and disconnect flow

- Delete local child evidence.
- Delete parent portal cache.
- Delete generated report.
- Delete provider backup copy.
- Disconnect provider.
- Delete support bundle.
- Delete Ocentra account or control-plane metadata.

These are separate actions and must not be merged into one vague "cloud sync" control.

## Proof names

- `data-custody.ui.state-proof`
- `data-custody.ui.restore-preview-flow`
- `data-custody.ui.delete-disconnect-flow`
- `data-custody.ui.no-claim-language`

## Adjacent handoff

- `portal-ux-household-surfaces-plan` owns the final portal surface implementation.
- `parent-client-runtime-distribution-plan` owns desktop runtime packaging and host wiring.
- `account-identity-family-plan` owns identity and household authority when storage access depends on it.
