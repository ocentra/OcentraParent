use super::NestedClaimViolationKind;
use serde_json::{json, Value};

impl NestedClaimViolationKind {
    pub(super) fn value(self) -> Value {
        match self {
            Self::Artifact => json!({
                "schemaVersion": 1, "artifactReferenceId": "artifact-1", "sourceResultId": "result-1",
                "targetDomain": "native-app", "childReasonReferenceIds": ["reason-1"],
                "childStatusReferenceIds": ["status-1"], "childDeliveryClaimed": false,
                "notificationDeliveryClaimed": false, "adapterDispatchClaimed": false,
                "platformEnforcementClaimed": false, "rawPrivateSourceRowsIncluded": false
            }),
            Self::Intent => json!({
                "schemaVersion": 1, "parentSurfaceIntentReferenceId": "intent-1", "sourceResultId": "result-1",
                "sourceArtifactReferenceId": "artifact-1", "targetDomain": "native-app",
                "historyVisibility": "history-row-visible", "parentSurfaceStatus": "manual-action-required",
                "preferenceVisibility": "preference-setup-required", "drillInReferenceIds": [],
                "manualProofReferenceIds": [], "sensitiveDetailIncluded": false,
                "parentNotificationUiRendered": false, "parentPreferenceMutationClaimed": false,
                "providerDeliveryClaimed": false, "childDeliveryClaimed": false,
                "adapterDispatchClaimed": false, "platformEnforcementClaimed": false,
                "rawPrivateSourceRowsIncluded": false
            }),
            Self::Preference => json!({
                "schemaVersion": 1, "parentPreferenceSetupReferenceId": "setup-1",
                "sourceParentSurfaceIntentReferenceId": "intent-1", "sourceResultId": "result-1",
                "sourceArtifactReferenceId": "artifact-1", "targetDomain": "native-app",
                "draftStatus": "draft-ready", "parentPreferenceSetupRequestStatus": "request-ready",
                "parentPreferenceSetupRequestReferenceIds": [], "drillInReferenceIds": [],
                "manualProofReferenceIds": [], "parentPreferenceUiRendered": false,
                "parentFrequencyControlUiRendered": false, "parentPreferenceMutationClaimed": false,
                "notificationRuleMutationClaimed": false, "providerDeliveryClaimed": false,
                "childDeliveryClaimed": false, "adapterDispatchClaimed": false,
                "platformEnforcementClaimed": false, "rawPrivateSourceRowsIncluded": false
            }),
        }
    }

    pub(super) fn record_name(self) -> &'static str {
        match self {
            Self::Artifact => "childUxLocalHandoffArtifactRecords",
            Self::Intent => "childUxParentSurfaceIntentRecords",
            Self::Preference => "childUxParentPreferenceSetupRecords",
        }
    }
}
