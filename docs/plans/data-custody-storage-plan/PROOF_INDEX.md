<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `Data Custody Storage Plan Proof Index`
> Kind: proof artifact router.
> Read when: selected workpack needs proof paths or PR_READY/DONE proof validation.
> Stop rule: use only the proof root for the selected workpack.
> Proves: proof location routing only.
> Does not prove: implementation completion by itself.
> Proof rule: proof artifacts are valid only after focused commands run or precise blockers are recorded.

<!-- /agent-capsule -->

# Data Custody Storage Plan Proof Index

## Proof roots

```text
output/data-custody-storage-plan-proof/01-custody-source-of-truth/
output/data-custody-storage-plan-proof/02-encryption-key-custody/
output/data-custody-storage-plan-proof/03-parent-owned-cloud-sync/
output/data-custody-storage-plan-proof/04-retention-delete-tombstone/
output/data-custody-storage-plan-proof/05-export-import-backup-recovery/
output/data-custody-storage-plan-proof/06-report-query-custody/
output/data-custody-storage-plan-proof/08-parent-storage-settings-apply-flow/
output/data-custody-storage-plan-proof/07-rollout-proof-and-route-gate/
```

## Required proof files per workpack

### WP01 Custody Source Of Truth

```text
00-data-classification-matrix-proof.md
01-source-of-truth-proof.md
02-no-default-hosted-private-activity-proof.md
03-account-control-plane-separation-proof.md
04-redaction-boundary-proof.md
16-validation-commands.log
```

### WP02 Encryption Key Custody

```text
00-key-custody-model-proof.md
01-platform-key-wrapper-matrix-proof.md
02-wrong-key-negative-proof.md
03-revoked-device-negative-proof.md
04-no-universal-ocentra-key-proof.md
05-recovery-mode-proof.md
16-validation-commands.log
```

### WP03 Parent Owned Cloud Sync

```text
00-provider-capability-matrix-proof.md
01-encrypted-before-upload-proof.md
02-provider-revoked-state-proof.md
03-quota-conflict-corruption-proof.md
04-offline-retry-partial-outage-proof.md
05-tombstone-propagation-proof.md
06-no-automatic-ocentra-fallback-proof.md
16-validation-commands.log
```

### WP04 Retention Delete Tombstone

```text
00-retention-matrix-proof.md
01-delete-state-machine-proof.md
02-tombstone-idempotency-proof.md
03-offline-retry-proof.md
04-derived-output-boundary-proof.md
05-wrong-role-denied-proof.md
06-retention-expiry-boundary-proof.md
07-restore-cannot-revive-deleted-state-proof.md
16-validation-commands.log
```

### WP05 Export Import Backup Recovery

```text
00-export-bundle-contract-proof.md
01-encrypted-payload-proof.md
02-import-preview-non-mutating-proof.md
03-wrong-household-key-bundle-proof.md
04-tombstone-preserved-proof.md
05-restore-apply-idempotent-proof.md
06-partial-restore-proof.md
16-validation-commands.log
```

### WP06 Report Query Custody

```text
00-derived-source-matrix-proof.md
01-deleted-expired-not-returned-proof.md
02-query-cursor-pagination-proof.md
03-query-rate-limit-proof.md
04-notification-payload-allow-deny-proof.md
05-portal-cache-custody-proof.md
06-assistant-allowed-citation-proof.md
07-stale-conflict-state-proof.md
16-validation-commands.log
```

### WP08 Parent Storage Settings Apply Flow

```text
00-parent-storage-choice-state-machine-proof.md
01-export-status-proof.md
02-import-preview-proof.md
03-apply-confirmation-proof.md
04-provider-disconnect-proof.md
05-provider-delete-proof.md
06-no-automatic-fallback-proof.md
07-portal-cache-status-proof.md
16-validation-commands.log
```

### WP07 Rollout Proof And Route Gate

```text
00-rollout-proof-pack.md
01-route-index-sync-proof.md
02-privacy-language-review-proof.md
03-manual-required-gap-register.md
04-adjacent-handoff-proof.md
16-validation-commands.log
```

## Command log format

Every proof root must include:

```text
command: <exact command>
exit: <code>
result: pass | fail | blocked
artifact: <path or n/a>
notes: <short note>
```

If blocked:

```text
blocker:
required environment:
why this does not prove completion:
next command:
```

## No-claim language

Do not claim:

```text
hosted storage ready
sync ready
export ready
restore ready
delete ready
report/query custody ready
assistant custody ready
parent storage settings ready
PR_READY
```

unless the selected workpack proof root proves the claim and WP07 aggregates it when broad readiness is claimed.
