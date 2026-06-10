# Screen AI Linux Host Adapter Custody Proof

Generated: 2026-06-08T00:27:46.915Z

Status: linux-host-custody-artifact-written-final-execution-blocked

## Apply

```json
{
  "state": "not-executed-target-unavailable",
  "requestedAction": "block",
  "screenDerived": true,
  "hostMutationAttempted": false,
  "liveLinuxHostMutationProved": false,
  "nativeSessionProofRef": null,
  "refusalReason": "target-unavailable",
  "requiredBeforeExecution": [
    "Linux service manager artifact",
    "Linux permission artifact",
    "Linux rollback artifact"
  ]
}
```

## Rollback

```json
{
  "state": "not-executed-no-host-apply",
  "rollbackAttempted": false,
  "rollbackRequiredBeforeProductComplete": true,
  "rollbackReferenceState": "unavailable"
}
```

## Audit

```json
{
  "state": "custody-recorded-not-executed",
  "auditReferenceState": "unavailable",
  "auditRef": "screen-ai-linux-host-adapter-custody-audit",
  "sourceEvidenceReferences": [
    {
      "evidenceReferenceId": "screen-analysis-evidence-bypass-tool",
      "kind": "activity-event",
      "observedAt": "2026-06-04T08:53:32.027Z"
    }
  ],
  "custodyRefs": [
    "output/screen-ai-pipeline-proof/block-action-dispatch/00-screen-block-source.json",
    "output/screen-ai-pipeline-proof/adapter-readiness/read-model.json",
    "output/screen-plan-proof/linux/proof-summary.json",
    "output/screen-plan-proof/linux-wslg/proof-summary.json"
  ],
  "rawImageRetained": false,
  "rawImageDeletedBeforeAdapter": true
}
```

## Closure

```json
{
  "screenDerivedBlockDecisionPreserved": true,
  "linuxCapturePrerequisitePresent": true,
  "linuxRawImageDeletionPreserved": true,
  "linuxHostApplyCustodyRecorded": true,
  "linuxHostApplyExecuted": false,
  "linuxRollbackExecutionRecorded": false,
  "linuxAuditCustodyRecorded": true,
  "finalAdapterCompletionClaimed": false,
  "productCompleteAdapterRowStillOpen": true
}
```
