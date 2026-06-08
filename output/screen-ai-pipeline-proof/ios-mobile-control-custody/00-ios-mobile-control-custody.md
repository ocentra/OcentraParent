# Screen AI iOS Mobile Control Custody Proof

Generated: 2026-06-08T00:52:32.781Z

Status: ios-mobile-control-custody-artifact-written-final-execution-blocked

## Apply

```json
{
  "state": "not-executed-manual-required",
  "requestedAction": "block",
  "screenDerived": true,
  "mobileControlAttempted": false,
  "liveIosFamilyControlsOrDeviceActivityProofProved": false,
  "physicalDeviceProofRef": null,
  "refusalReason": "manual-artifact-required",
  "requiredBeforeExecution": [
    "Family Controls entitlement artifact",
    "DeviceActivity artifact",
    "Network Extension artifact"
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
  "auditRef": "screen-ai-ios-mobile-control-custody-audit",
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
    "output/screen-plan-proof/ios/proof-summary.json"
  ],
  "rawImageRetained": false,
  "rawImageDeletedBeforeAdapter": true
}
```

## Closure

```json
{
  "screenDerivedBlockDecisionPreserved": true,
  "iosReplayKitSourceDocPrerequisitePresent": true,
  "iosRawImageRetentionBlocked": true,
  "iosMobileApplyCustodyRecorded": true,
  "iosMobileApplyExecuted": false,
  "iosRollbackExecutionRecorded": false,
  "iosAuditCustodyRecorded": true,
  "finalAdapterCompletionClaimed": false,
  "productCompleteAdapterRowStillOpen": true
}
```
