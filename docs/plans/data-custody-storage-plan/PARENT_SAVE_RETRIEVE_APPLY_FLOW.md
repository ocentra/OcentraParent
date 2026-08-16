<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `PARENT_SAVE_RETRIEVE_APPLY_FLOW.md`
> Kind: flow contract.
> Read when: When a workpack needs the save, retrieve, preview, and apply flow.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Flow claims must match the bundle, UI, and event models.

<!-- /agent-capsule -->

# Data Custody Storage Plan Parent Save Retrieve Apply Flow

## Flow stages

### Save or export

- Parent chooses a destination and data classes.
- The product snapshots the selected source data.
- The bundle is classified, encrypted, and hashed.
- The manifest is written before or with the payload, never after as a silent mismatch.
- The result is queued or written with an explicit status.

### Retrieve or import preview

- The product fetches the bundle or provider file.
- The manifest and payload are validated.
- The UI shows a preview with no mutation to local truth.
- Wrong-household, wrong-key, corrupt, or unsupported bundles are rejected.

### Apply or restore

- The parent explicitly confirms the apply step.
- The product applies only the allowed sections.
- Local truth and tombstones remain authoritative.
- The result records what changed, what stayed, and what was rejected.

## Reject states

| State | Meaning | Action |
| --- | --- | --- |
| `wrongHousehold` | Household binding failed | Reject |
| `wrongKey` | Bundle cannot be unlocked | Reject |
| `schemaUnsupported` | Bundle version is not supported | Reject or preview only |
| `bundleCorrupt` | Integrity check failed | Reject |
| `tombstoneConflict` | Deleted data cannot be safely restored | Reject or partial restore only |
| `providerRevoked` | Provider access is missing | Reject retrieve or mark manual-required |
| `manualRequired` | Safe auto-action is not available | Stop and escalate |

## Partial restore

- Partial restore is allowed only when the product can state exactly what was accepted and what was rejected.
- Partial restore must preserve tombstones and source-of-truth ordering.
- Partial restore must not create duplicate household, child, or device truth.

## Rollback and manual-required

- If apply fails after any state change, the result must become manual-required and be recorded.
- Rollback is not optional when the bundle protocol says state is reversible.
- Human review is required when conflicts, tombstones, or unsupported sections block a safe apply.

## Event families

- `data-custody.bundle.*`
- `data-custody.sync.*`
- `data-custody.retention.*`
- `data-custody.support.*`

## Proof anchors

- `data-custody.export.bundle-contract`
- `data-custody.import.verify-restore`
- `data-custody.import.corrupt-bundle-negative`
- `data-custody.import.wrong-household-negative`
- `data-custody.backup.partial-restore`
- `data-custody.export.redacted-summary-proof`

