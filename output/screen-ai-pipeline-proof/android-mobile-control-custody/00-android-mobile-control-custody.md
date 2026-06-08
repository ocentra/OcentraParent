# Screen AI Android Mobile Control Custody Proof

Generated: 2026-06-08T00:43:31.624Z

Status: android-mobile-control-custody-artifact-written-final-execution-blocked

## Apply

```json
{
  "state": "not-executed-manual-required",
  "requestedAction": "block",
  "screenDerived": true,
  "mobileControlAttempted": false,
  "liveAndroidDeviceOwnerOrProfileProofProved": false,
  "physicalDeviceProofRef": null,
  "refusalReason": "manual-artifact-required",
  "requiredBeforeExecution": [
    "device-owner or managed-profile artifact",
    "UsageStats artifact",
    "accessibility or VPN/DNS artifact"
  ]
}
```

## Rollback

```json
{
  "state": "not-executed-no-mobile-apply",
  "rollbackAttempted": false,
  "rollbackRequiredBeforeProductComplete": true,
  "rollbackReferenceState": "manual-required"
}
```

## Audit

```json
{
  "state": "custody-recorded-not-executed",
  "auditReferenceState": "manual-required",
  "auditRef": "screen-ai-android-mobile-control-custody-audit",
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
    "output/screen-plan-proof/android/proof-summary.json",
    "output/screen-plan-proof/android-mediaprojection/proof-summary.json"
  ],
  "rawImageRetained": false,
  "rawImageDeletedBeforeAdapter": true
}
```

## Closure

```json
{
  "screenDerivedBlockDecisionPreserved": true,
  "androidEmulatorCapturePrerequisitePresent": true,
  "androidRawImageDeletionPreserved": true,
  "androidMobileApplyCustodyRecorded": true,
  "androidMobileApplyExecuted": false,
  "androidRollbackExecutionRecorded": false,
  "androidAuditCustodyRecorded": true,
  "finalAdapterCompletionClaimed": false,
  "productCompleteAdapterRowStillOpen": true
}
```
