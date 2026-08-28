use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use serde_json::Value;

fn parse_parent_ui_bridge_text_identifier(value: impl Into<String>) -> Option<String> {
    let value = value.into();
    if value.trim().is_empty() {
        None
    } else {
        Some(value)
    }
}

macro_rules! parent_ui_bridge_text_identifier {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Option<Self> {
                parse_parent_ui_bridge_text_identifier(value).map(Self)
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

parent_ui_bridge_text_identifier!(ParentChildDeviceId);
parent_ui_bridge_text_identifier!(ParentContractReferenceId);
parent_ui_bridge_text_identifier!(ParentEvidenceId);
parent_ui_bridge_text_identifier!(ParentEvidenceReferenceId);
parent_ui_bridge_text_identifier!(ParentLanActionId);
parent_ui_bridge_text_identifier!(ParentLanAddressRef);
parent_ui_bridge_text_identifier!(ParentLanCanonicalDeviceId);
parent_ui_bridge_text_identifier!(ParentLanChallengeId);
parent_ui_bridge_text_identifier!(ParentLanDeviceId);
parent_ui_bridge_text_identifier!(ParentLanDiscoveryEventId);
parent_ui_bridge_text_identifier!(ParentLanPairingId);
parent_ui_bridge_text_identifier!(ParentLanRouteId);
parent_ui_bridge_text_identifier!(ParentLanScanSessionId);
parent_ui_bridge_text_identifier!(ParentLanWorkpackId);
parent_ui_bridge_text_identifier!(ParentParentActorId);
parent_ui_bridge_text_identifier!(ParentPolicyApprovalId);
parent_ui_bridge_text_identifier!(ParentPolicyAuditReferenceId);
parent_ui_bridge_text_identifier!(ParentPolicyDecisionActionId);
parent_ui_bridge_text_identifier!(ParentPolicyDecisionId);
parent_ui_bridge_text_identifier!(ParentPolicyOverrideId);
parent_ui_bridge_text_identifier!(ParentPolicyPreviewId);
parent_ui_bridge_text_identifier!(ParentPolicyReasonCodes);
parent_ui_bridge_text_identifier!(ParentPolicyReplayApprovalId);
parent_ui_bridge_text_identifier!(ParentPolicyRuleContextRefIds);
parent_ui_bridge_text_identifier!(ParentPolicyRuleIds);
parent_ui_bridge_text_identifier!(ParentPolicyTargetId);
parent_ui_bridge_text_identifier!(ParentPortalClipboardText);
parent_ui_bridge_text_identifier!(ParentPortalDetailValue);
parent_ui_bridge_text_identifier!(ParentPortalShellStatusCardId);
parent_ui_bridge_text_identifier!(ParentRouteAdapterId);
parent_ui_bridge_text_identifier!(ParentRouteEventId);
parent_ui_bridge_text_identifier!(ParentRouteEventCorrelationId);
parent_ui_bridge_text_identifier!(ParentRoutePeerId);
parent_ui_bridge_text_identifier!(ParentRuntimeEventType);
parent_ui_bridge_text_identifier!(ParentSubjectId);
parent_ui_bridge_text_identifier!(ParentTrackingStatusProofArtifact);
parent_ui_bridge_text_identifier!(ParentUserActorId);
parent_ui_bridge_text_identifier!(ParentUserLocalAiResultId);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ParentRoutePeerRole {
    #[serde(rename = "portal")]
    Portal,
    #[serde(rename = "agent-service")]
    AgentService,
    #[serde(rename = "cloud-relay")]
    CloudRelay,
}

pub const PARENT_UI_BRIDGE_SCHEMA_VERSION: u16 = 1;
pub const PARENT_BRIDGE_COMMAND_LOAD_ROUTE: &str = "parent_load_route";
pub const PARENT_BRIDGE_COMMAND_DISPATCH: &str = "parent_dispatch";
pub const PARENT_BRIDGE_COMMAND_SUBSCRIBE: &str = "parent_subscribe_route";
pub const PARENT_BRIDGE_COMMAND_UNSUBSCRIBE: &str = "parent_unsubscribe_route";
pub const PARENT_DEV_BRIDGE_ROUTE_LOAD_ROUTE: &str = "load-route";
pub const PARENT_DEV_BRIDGE_ROUTE_DISPATCH: &str = "dispatch";
pub const PARENT_DEV_BRIDGE_LOAD_ROUTE_PATH: &str = "/api/parent-ui/load-route";
pub const PARENT_DEV_BRIDGE_DISPATCH_PATH: &str = "/api/parent-ui/dispatch";
pub const PARENT_ROUTE_HASH_PREFIX: &str = "#";
pub const PARENT_ROUTE_HASH_QUERY_SEPARATOR: &str = "?";
pub const PARENT_ROUTE_SUBSCRIPTION_EVENT_PREFIX: &str = "parent-route-subscription-";
pub const PARENT_ROUTE_SUBSCRIPTION_POLL_INTERVAL_MS: u64 = 1000;
pub const PARENT_DEV_BRIDGE_REQUEST_TIMEOUT_MS: u64 = 5000;
pub const PARENT_SCREEN_SETTINGS_COMMAND_SCHEMA_VERSION: u16 = 1;
pub const PARENT_SCREEN_SETTINGS_REQUEST_ID_PREFIX: &str = "screen-settings-request-";
pub const PARENT_SCREEN_SETTINGS_UPDATE_KIND_GET: &str = "get";
pub const PARENT_SCREEN_SETTINGS_UPDATE_KIND_REPLACE: &str = "replace";
pub const PARENT_SCREEN_SETTINGS_UPDATE_STATUS_ACCEPTED: &str = "accepted";
pub const PARENT_SCREEN_SETTINGS_UPDATE_STATUS_REJECTED: &str = "rejected";
pub const PARENT_SCREEN_OPTIONAL_VISIBILITY_CAPABILITY_PROOF_GENERATED_AT: &str =
    "2026-06-07T05:55:00Z";
pub const PARENT_SCREEN_EVIDENCE_SETTINGS_WRITABLE_UI_PROOF_JSON: &str = r#"{
  "title": "Writable screen settings proof",
  "note": "Parent Settings can build a schema-valid local screen-summary intent and submit it to the child service command path.",
  "intentLegend": "Intent",
  "draftHeading": "Draft mode",
  "draftTriggerHeading": "Triggers and custody",
  "retentionHeading": "Remote boundary",
  "serviceCommandHeading": "Service command",
  "serviceApplyActionLabel": "Save selected screen setting",
  "serviceRefreshActionLabel": "Refresh persisted screen setting",
  "servicePendingStatus": "waiting for service response",
  "serviceAcceptedStatus": "service accepted persisted setting",
  "serviceRejectedStatus": "service rejected setting",
  "serviceDisconnectedStatus": "service command unavailable while disconnected",
  "serviceNoResponseStatus": "no service settings response yet",
  "validationStatusLabel": "Parser status",
  "validationStatusValue": "schema-valid local parent intent",
  "defaultIntentKey": "disabledLocalSummary",
  "intents": [
    {
      "intentKey": "disabledLocalSummary",
      "label": "Keep screen analysis disabled",
      "detail": "No cadence capture, trigger capture, strict mode, or policy use can run while disabled.",
      "setting": {
        "schemaVersion": 1,
        "screenAnalysisEnabled": false,
        "analysisMode": "observeOnly",
        "cadenceCaptureEnabled": false,
        "cadenceSeconds": 300,
        "strictModeEnabled": false,
        "triggerCaptureEnabled": false,
        "enabledTriggers": [],
        "allowedCaptureScope": "unsupported",
        "ocrTextEnabled": false,
        "ocrTextSnippetLimit": 0,
        "redactionMode": "disabled",
        "ocrTextRetentionMode": "disabled",
        "credentialSuppressionEnabled": true,
        "piiRedactionEnabled": false,
        "temporaryImageTtlSeconds": 300,
        "maxRetryCount": 0,
        "deleteAfterSuccess": true,
        "deleteAfterExpiry": true,
        "retainRawImage": false,
        "policyUseEnabled": false,
        "changedByParentRef": "screen-settings-ui-parent-disabled",
        "changedAt": "2026-06-04T23:50:00Z",
        "settingVersion": 1,
        "reason": "parent kept local screen summaries disabled"
      },
      "remoteBoundarySetting": {
        "schemaVersion": 1,
        "parentSettingRef": "screen-settings-ui-parent-disabled",
        "settingVersion": 1,
        "rawScreenshotRetentionMode": "disabled",
        "liveViewMode": "disabled",
        "rawScreenshotRemoteUploadEnabled": false,
        "remoteSummaryMode": "disabled",
        "remoteSummaryRedactedOnly": true,
        "parentApprovedRemoteSummary": false,
        "remoteSummaryApprovalRef": null,
        "remoteSummaryDestinationCustodyState": "unavailable",
        "changedByParentRef": "screen-settings-ui-parent-disabled",
        "changedAt": "2026-06-04T23:50:00Z",
        "reason": "local screen summary settings do not enable raw retention or live view"
      }
    },
    {
      "intentKey": "observeOnlyLocalSummary",
      "label": "Enable observe-only summaries",
      "detail": "Five-minute local summaries can be reviewed by the parent, but policy handoff remains disabled.",
      "setting": {
        "schemaVersion": 1,
        "screenAnalysisEnabled": true,
        "analysisMode": "observeOnly",
        "cadenceCaptureEnabled": true,
        "cadenceSeconds": 300,
        "strictModeEnabled": false,
        "triggerCaptureEnabled": true,
        "enabledTriggers": ["foregroundAppChange", "policyAmbiguity"],
        "allowedCaptureScope": "activeWindow",
        "ocrTextEnabled": true,
        "ocrTextSnippetLimit": 3,
        "redactionMode": "localSensitiveText",
        "ocrTextRetentionMode": "redactedSnippets",
        "credentialSuppressionEnabled": true,
        "piiRedactionEnabled": true,
        "temporaryImageTtlSeconds": 300,
        "maxRetryCount": 2,
        "deleteAfterSuccess": true,
        "deleteAfterExpiry": true,
        "retainRawImage": false,
        "policyUseEnabled": false,
        "changedByParentRef": "screen-settings-ui-parent-observe",
        "changedAt": "2026-06-04T23:50:00Z",
        "settingVersion": 2,
        "reason": "parent enabled observe-only local screen summaries"
      },
      "remoteBoundarySetting": {
        "schemaVersion": 1,
        "parentSettingRef": "screen-settings-ui-parent-observe",
        "settingVersion": 2,
        "rawScreenshotRetentionMode": "disabled",
        "liveViewMode": "disabled",
        "rawScreenshotRemoteUploadEnabled": false,
        "remoteSummaryMode": "disabled",
        "remoteSummaryRedactedOnly": true,
        "parentApprovedRemoteSummary": false,
        "remoteSummaryApprovalRef": null,
        "remoteSummaryDestinationCustodyState": "unavailable",
        "changedByParentRef": "screen-settings-ui-parent-observe",
        "changedAt": "2026-06-04T23:50:00Z",
        "reason": "local screen summary settings do not enable raw retention or live view"
      }
    },
    {
      "intentKey": "strictDryRunLocalSummary",
      "label": "Enable strict dry-run review",
      "detail": "One-minute cadence, selected triggers, local OCR, redaction, and policy dry-run become explicit parent intent.",
      "setting": {
        "schemaVersion": 1,
        "screenAnalysisEnabled": true,
        "analysisMode": "policyDryRun",
        "cadenceCaptureEnabled": true,
        "cadenceSeconds": 60,
        "strictModeEnabled": true,
        "triggerCaptureEnabled": true,
        "enabledTriggers": [
          "foregroundAppChange",
          "managedBrowserUrlChange",
          "appGameForegroundStart",
          "policyAmbiguity"
        ],
        "allowedCaptureScope": "activeWindow",
        "ocrTextEnabled": true,
        "ocrTextSnippetLimit": 5,
        "redactionMode": "localSensitiveText",
        "ocrTextRetentionMode": "redactedSnippets",
        "credentialSuppressionEnabled": true,
        "piiRedactionEnabled": true,
        "temporaryImageTtlSeconds": 300,
        "maxRetryCount": 2,
        "deleteAfterSuccess": true,
        "deleteAfterExpiry": true,
        "retainRawImage": false,
        "policyUseEnabled": true,
        "changedByParentRef": "screen-settings-ui-parent-strict",
        "changedAt": "2026-06-04T23:50:00Z",
        "settingVersion": 3,
        "reason": "parent enabled strict local screen summary dry run"
      },
      "remoteBoundarySetting": {
        "schemaVersion": 1,
        "parentSettingRef": "screen-settings-ui-parent-strict",
        "settingVersion": 3,
        "rawScreenshotRetentionMode": "disabled",
        "liveViewMode": "disabled",
        "rawScreenshotRemoteUploadEnabled": false,
        "remoteSummaryMode": "disabled",
        "remoteSummaryRedactedOnly": true,
        "parentApprovedRemoteSummary": false,
        "remoteSummaryApprovalRef": null,
        "remoteSummaryDestinationCustodyState": "unavailable",
        "changedByParentRef": "screen-settings-ui-parent-strict",
        "changedAt": "2026-06-04T23:50:00Z",
        "reason": "local screen summary settings do not enable raw retention or live view"
      }
    },
    {
      "intentKey": "approvedRawRetentionLocalTtl",
      "label": "Approve local short-TTL retention",
      "detail": "Parent-approved local raw screenshot retention uses a short TTL and keeps delete-after-success and delete-after-expiry required.",
      "setting": {
        "schemaVersion": 1,
        "screenAnalysisEnabled": true,
        "analysisMode": "policyDryRun",
        "cadenceCaptureEnabled": true,
        "cadenceSeconds": 60,
        "strictModeEnabled": true,
        "triggerCaptureEnabled": true,
        "enabledTriggers": [
          "foregroundAppChange",
          "managedBrowserUrlChange",
          "appGameForegroundStart",
          "policyAmbiguity"
        ],
        "allowedCaptureScope": "activeWindow",
        "ocrTextEnabled": true,
        "ocrTextSnippetLimit": 5,
        "redactionMode": "localSensitiveText",
        "ocrTextRetentionMode": "redactedSnippets",
        "credentialSuppressionEnabled": true,
        "piiRedactionEnabled": true,
        "temporaryImageTtlSeconds": 120,
        "maxRetryCount": 2,
        "deleteAfterSuccess": true,
        "deleteAfterExpiry": true,
        "retainRawImage": true,
        "policyUseEnabled": true,
        "changedByParentRef": "screen-settings-ui-parent-raw-retention-local-ttl",
        "changedAt": "2026-06-04T23:50:00Z",
        "settingVersion": 4,
        "reason": "parent approved local short TTL raw screenshot retention"
      },
      "remoteBoundarySetting": {
        "schemaVersion": 1,
        "parentSettingRef": "screen-settings-ui-parent-raw-retention-local-ttl",
        "settingVersion": 4,
        "rawScreenshotRetentionMode": "parentApprovedLocalShortTtl",
        "liveViewMode": "disabled",
        "rawScreenshotRemoteUploadEnabled": false,
        "remoteSummaryMode": "disabled",
        "remoteSummaryRedactedOnly": true,
        "parentApprovedRemoteSummary": false,
        "remoteSummaryApprovalRef": null,
        "remoteSummaryDestinationCustodyState": "unavailable",
        "changedByParentRef": "screen-settings-ui-parent-raw-retention-local-ttl",
        "changedAt": "2026-06-04T23:50:00Z",
        "reason": "parent approved local short TTL raw screenshot retention without raw remote upload"
      }
    }
  ]
}"#;
pub const PARENT_SCREEN_CONTROL_SETTINGS_PORTAL_PROOF_JSON: &str = r#"{
  "title": "Screen settings and capability proof",
  "note": "Read-only Settings proof from the Screen control catalog; child runtime owns capture, queue, local analysis, policy handoff, and audit.",
  "metrics": [
    {
      "label": "Catalog settings",
      "value": "474",
      "detail": "Screen settings parsed from the current capability guide and schema proposal."
    },
    {
      "label": "Catalog tabs",
      "value": "11",
      "detail": "Parent-facing Screen categories available for read-only rendering."
    },
    {
      "label": "Proof-required controls",
      "value": "68",
      "detail": "Strict behavior requires platform capture, local analysis, deletion, or policy proof before use."
    },
    {
      "label": "Unavailable sensitive modes",
      "value": "9",
      "detail": "Raw retention, hosted processing, hidden capture, continuous recording, and unsupported sensitive states fail closed."
    }
  ],
  "gates": [
    {
      "label": "Allow Ocentra-hosted processing of child screen images?",
      "status": "unavailable",
      "statusText": "unavailable / unavailable",
      "capabilityState": "unavailable",
      "runtimeOwner": "parent-owned-storage",
      "detail": "Disable or reject this state; do not retain raw capture or use hosted child screen processing by default.",
      "sourceDocument": "docs/screen-evidence-analysis-schema-proposal.md"
    },
    {
      "label": "Show raw screenshots in parent reports by default?",
      "status": "unavailable",
      "statusText": "unavailable / unavailable",
      "capabilityState": "unavailable",
      "runtimeOwner": "portal-only",
      "detail": "Disable or reject this state; do not retain raw capture or use hosted child screen processing by default.",
      "sourceDocument": "docs/screen-evidence-analysis-schema-proposal.md"
    },
    {
      "label": "Retain raw screenshots or recordings?",
      "status": "unavailable",
      "statusText": "unavailable / unavailable",
      "capabilityState": "unavailable",
      "runtimeOwner": "parent-owned-storage",
      "detail": "Disable or reject this state; do not retain raw capture or use hosted child screen processing by default.",
      "sourceDocument": "docs/screen-evidence-analysis-schema-proposal.md"
    },
    {
      "label": "Allow screen summaries to be used by policy?",
      "status": "proof-required",
      "statusText": "proof-required / available",
      "capabilityState": "available",
      "runtimeOwner": "os-adapter",
      "detail": "Require validated summary, evidence refs, deletion proof, and deterministic policy before policy use.",
      "sourceDocument": "docs/screen-evidence-analysis-schema-proposal.md"
    },
    {
      "label": "Use local OCR/vision returns schema-valid output;?",
      "status": "needs-effect-wiring",
      "statusText": "needs-effect-wiring / available",
      "capabilityState": "available",
      "runtimeOwner": "local-ai-runtime",
      "detail": "Portal renders authored intent; child agent owns capture gating, queue, analysis, compile, and audit.",
      "sourceDocument": "docs/screen-evidence-analysis-capability-guide.md"
    }
  ]
}"#;
pub const PARENT_SCREEN_OPTIONAL_VISIBILITY_CAPABILITY_STATUS_PROOF_JSON: &str = r#"{
  "schemaVersion": 1,
  "generatedAt": "2026-06-07T05:55:00Z",
  "proofId": "screen-optional-visibility-capability-status-proof",
  "rows": [
    {
      "schemaVersion": 1,
      "checkedAt": "2026-06-07T05:55:00Z",
      "capabilityKind": "rawScreenshotRetention",
      "parentSettingRef": "screen-parent-retention-capability-disabled",
      "readinessState": "disabled",
      "rawRetentionSetting": {
        "schemaVersion": 1,
        "settingId": "screen-retention-capability-disabled",
        "parentSettingRef": "screen-parent-retention-capability-disabled",
        "settingVersion": 1,
        "changedAt": "2026-06-07T05:55:00Z",
        "mode": "disabled",
        "explicitParentApproval": false,
        "approvalRef": null,
        "disclosureState": "notRequired",
        "auditRef": null,
        "ttlSeconds": null,
        "custodyState": "unavailable",
        "exportRef": null,
        "sourceLabel": "unavailable",
        "retentionBehavior": "noRawRetention",
        "deleteAfterTtl": false,
        "deleteOnParentDisable": true,
        "deleteProofRequired": false,
        "rawScreenshotRemoteUploadEnabled": false,
        "reason": "raw screenshot retention is disabled by default"
      },
      "liveViewSetting": null,
      "liveViewPermissionGate": null,
      "runtimeProofRef": null,
      "deletionProofRef": null,
      "transportProofRef": null,
      "childDisclosureReady": false,
      "childDeviceCapabilityReady": false,
      "productModeReady": false,
      "rawFramesRetained": false,
      "rawRemoteUploadAllowed": false,
      "remoteInputAllowed": false,
      "reason": "raw screenshot retention is disabled by default"
    },
    {
      "schemaVersion": 1,
      "checkedAt": "2026-06-07T05:55:00Z",
      "capabilityKind": "rawScreenshotRetention",
      "parentSettingRef": "screen-parent-retention-capability-local-ttl",
      "readinessState": "manualRequired",
      "rawRetentionSetting": {
        "schemaVersion": 1,
        "settingId": "screen-retention-capability-local-ttl",
        "parentSettingRef": "screen-parent-retention-capability-local-ttl",
        "settingVersion": 1,
        "changedAt": "2026-06-07T05:55:00Z",
        "mode": "localShortTtl",
        "explicitParentApproval": true,
        "approvalRef": "screen-retention-capability-approval",
        "disclosureState": "requiredShown",
        "auditRef": "screen-retention-capability-audit",
        "ttlSeconds": 300,
        "custodyState": "child-device-temp-queue",
        "exportRef": null,
        "sourceLabel": "rawScreenshotRetention",
        "retentionBehavior": "deleteAfterTtl",
        "deleteAfterTtl": true,
        "deleteOnParentDisable": true,
        "deleteProofRequired": true,
        "rawScreenshotRemoteUploadEnabled": false,
        "reason": "parent approved local short TTL raw screenshot retention"
      },
      "liveViewSetting": null,
      "liveViewPermissionGate": null,
      "runtimeProofRef": null,
      "deletionProofRef": null,
      "transportProofRef": null,
      "childDisclosureReady": false,
      "childDeviceCapabilityReady": false,
      "productModeReady": false,
      "rawFramesRetained": false,
      "rawRemoteUploadAllowed": false,
      "remoteInputAllowed": false,
      "reason": "raw screenshot retention needs runtime and deletion proof before product readiness"
    },
    {
      "schemaVersion": 1,
      "checkedAt": "2026-06-07T05:55:00Z",
      "capabilityKind": "rawScreenshotRetention",
      "parentSettingRef": "screen-parent-retention-capability-local-ttl-runtime",
      "readinessState": "ready",
      "rawRetentionSetting": {
        "schemaVersion": 1,
        "settingId": "screen-retention-capability-local-ttl-runtime",
        "parentSettingRef": "screen-parent-retention-capability-local-ttl-runtime",
        "settingVersion": 2,
        "changedAt": "2026-06-07T05:55:00Z",
        "mode": "localShortTtl",
        "explicitParentApproval": true,
        "approvalRef": "screen-retention-runtime-approval",
        "disclosureState": "requiredShown",
        "auditRef": "screen-retention-runtime-audit",
        "ttlSeconds": 120,
        "custodyState": "child-device-temp-queue",
        "exportRef": null,
        "sourceLabel": "rawScreenshotRetention",
        "retentionBehavior": "deleteAfterTtl",
        "deleteAfterTtl": true,
        "deleteOnParentDisable": true,
        "deleteProofRequired": true,
        "rawScreenshotRemoteUploadEnabled": false,
        "reason": "parent approved local short TTL raw screenshot retention with runtime and deletion proof"
      },
      "liveViewSetting": null,
      "liveViewPermissionGate": null,
      "runtimeProofRef": "output/screen-plan-proof/screen-settings-service-command/proof-summary.json",
      "deletionProofRef": "output/screen-plan-proof/screen-service-deletion-event-producer/proof-summary.json",
      "transportProofRef": null,
      "childDisclosureReady": true,
      "childDeviceCapabilityReady": true,
      "productModeReady": true,
      "rawFramesRetained": false,
      "rawRemoteUploadAllowed": false,
      "remoteInputAllowed": false,
      "reason": "raw screenshot retention is ready only with parent approval, runtime proof, deletion proof, child disclosure, and child device readiness"
    },
    {
      "schemaVersion": 1,
      "checkedAt": "2026-06-07T05:55:00Z",
      "capabilityKind": "liveView",
      "parentSettingRef": "screen-parent-live-capability-disabled",
      "readinessState": "disabled",
      "rawRetentionSetting": null,
      "liveViewSetting": {
        "schemaVersion": 1,
        "settingId": "screen-live-capability-disabled",
        "parentSettingRef": "screen-parent-live-capability-disabled",
        "settingVersion": 1,
        "changedAt": "2026-06-07T05:55:00Z",
        "liveViewMode": "disabled",
        "transportMode": "none",
        "explicitParentApproval": false,
        "approvalRef": null,
        "disclosureState": "notRequired",
        "viewerAuditRef": null,
        "platformProofState": "notRequired",
        "platformProofRef": null,
        "custodyState": "unavailable",
        "sourceLabel": "unavailable",
        "frameRetentionBehavior": "noFrameRetention",
        "cacheRawFrames": false,
        "sessionRecordingAllowed": false,
        "remoteInputControlAllowed": false,
        "stopOrRevokeAuditRequired": true,
        "reason": "live view is disabled by default"
      },
      "liveViewPermissionGate": null,
      "runtimeProofRef": null,
      "deletionProofRef": null,
      "transportProofRef": null,
      "childDisclosureReady": false,
      "childDeviceCapabilityReady": false,
      "productModeReady": false,
      "rawFramesRetained": false,
      "rawRemoteUploadAllowed": false,
      "remoteInputAllowed": false,
      "reason": "live view is disabled by default"
    },
    {
      "schemaVersion": 1,
      "checkedAt": "2026-06-07T05:55:00Z",
      "capabilityKind": "liveView",
      "parentSettingRef": "screen-parent-live-capability-lan",
      "readinessState": "blocked",
      "rawRetentionSetting": null,
      "liveViewSetting": {
        "schemaVersion": 1,
        "settingId": "screen-live-capability-lan",
        "parentSettingRef": "screen-parent-live-capability-lan",
        "settingVersion": 1,
        "changedAt": "2026-06-07T05:55:00Z",
        "liveViewMode": "lanOnlyView",
        "transportMode": "lanMutualAuth",
        "explicitParentApproval": true,
        "approvalRef": "screen-live-capability-approval",
        "disclosureState": "requiredShown",
        "viewerAuditRef": "screen-live-capability-audit",
        "platformProofState": "operatorVerified",
        "platformProofRef": "screen-live-capability-platform-proof",
        "custodyState": "live-lan-child-agent",
        "sourceLabel": "liveView",
        "frameRetentionBehavior": "noFrameRetention",
        "cacheRawFrames": false,
        "sessionRecordingAllowed": false,
        "remoteInputControlAllowed": false,
        "stopOrRevokeAuditRequired": true,
        "reason": "parent approved LAN live view but capture-only evidence is insufficient"
      },
      "liveViewPermissionGate": {
        "schemaVersion": 1,
        "checkedAt": "2026-06-07T05:55:00Z",
        "platform": "android-mediaprojection",
        "liveViewMode": "lanOnlyView",
        "transportMode": "lanMutualAuth",
        "permissionEvidenceKind": "screen-capture-only",
        "platformProofState": "operatorVerified",
        "platformProofRef": "screen-live-capability-capture-only-proof",
        "viewerAuditRef": "screen-live-capability-audit",
        "sourceLabel": "liveView",
        "custodyState": "live-lan-child-agent",
        "frameRetentionBehavior": "noFrameRetention",
        "liveTransportProofRef": null,
        "explicitViewerDisclosure": true,
        "cacheRawFrames": false,
        "sessionRecordingAllowed": false,
        "remoteInputControlAllowed": false,
        "productLiveViewReady": false,
        "reason": "capture-only permission cannot satisfy live-view readiness"
      },
      "runtimeProofRef": null,
      "deletionProofRef": null,
      "transportProofRef": null,
      "childDisclosureReady": false,
      "childDeviceCapabilityReady": false,
      "productModeReady": false,
      "rawFramesRetained": false,
      "rawRemoteUploadAllowed": false,
      "remoteInputAllowed": false,
      "reason": "live view remains blocked until live-view permission and transport proof are present"
    }
  ],
  "nonClaims": [
    "This proof proves raw screenshot retention readiness only after explicit parent approval, runtime proof, deletion proof, child disclosure readiness, and child device readiness.",
    "This proof does not enable live-view transport, relay, cache, or remote input.",
    "This proof does not satisfy privacy/legal approval or physical platform live-view prompt screenshots."
  ]
}"#;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParentRouteId {
    Overview,
    Assistant,
    Start,
    Activity,
    Browser,
    BrowserSettings,
    Policy,
    PolicyApps,
    PolicyGames,
    PolicyScreen,
    PolicyNetwork,
    PolicyTracking,
    PolicyRemoteScreen,
    RuleManagement,
    Schedules,
    Approvals,
    Enforcement,
    PrivacyDesign,
    Memory,
    MemorySettings,
    AiGuide,
    AiRuntime,
    ApiProviders,
    ReportsGuide,
    ScreenAnalysis,
    AppGameSessions,
    NetworkActivity,
    Devices,
    LanPairing,
    CapabilityStatus,
    Notifications,
    NotificationChannels,
    DriveConnections,
    ExportRetention,
    RemoteAccess,
    ReportCompiler,
    AuditHistory,
    Subscription,
    Entitlements,
    PlatformsInstall,
    InstallUpdates,
    Diagnostics,
    ProofPanels,
    SettingsRules,
    AppLayout,
    FrameTuner,
    Commands,
    Events,
    Logs,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParentBridgeConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Error,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParentRouteDataSource {
    HostBridge,
    RustReadModel,
    DevDiagnostics,
    Unavailable,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParentPortalTone {
    Cyan,
    Gold,
    Purple,
    Red,
    Muted,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParentPortalParentAccessState {
    ActiveController,
    ObserverOnly,
    Unauthenticated,
    ProofMissing,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentRouteContext {
    pub selected_child_device_id: Option<ParentChildDeviceId>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentPortalRowSnapshot {
    pub label: String,
    pub order: u16,
    pub signal_score: u16,
    pub ready_count: u16,
    pub gap_count: u16,
    pub primary_area: String,
    pub trend: String,
    pub tone: ParentPortalTone,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentPortalShellStatusCardSnapshot {
    pub id: ParentPortalShellStatusCardId,
    pub label: String,
    pub value: String,
    pub detail: String,
    pub tone: ParentPortalTone,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentPortalShellStatusSnapshot {
    pub route_label: String,
    pub parent_access_state: ParentPortalParentAccessState,
    pub global_connection_state: String,
    pub route_capability_state: String,
    pub data_source_label: String,
    pub cards: Vec<ParentPortalShellStatusCardSnapshot>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParentServiceHealthState {
    Ready,
    Degraded,
    Unavailable,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParentServiceHealthRoute {
    Localhost,
    LocalNetwork,
    CloudRelay,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParentServiceHealthTransport {
    #[serde(rename = "websocket")]
    WebSocket,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParentServiceHealthAuthenticationState {
    Authenticated,
    Unauthenticated,
    Unavailable,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParentServiceHealthReason {
    Ready,
    TransportUnavailable,
    RouteDependencyUnavailable,
    ResponseSchemaMismatch,
    ResponseIdentityMismatch,
    ResponsePayloadMismatch,
    ResponseNonceMismatch,
    ResponseEventIdMismatch,
    ResponseTimestampMissing,
    ResponseTimestampStale,
    ServiceVersionMissing,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentServiceHealthTraceSnapshot {
    pub request_id: Option<String>,
    pub correlation_id: Option<String>,
    pub response_event_id: Option<String>,
    pub request_sent_at: Option<String>,
    pub response_sent_at: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentServiceHealthSnapshot {
    pub state: ParentServiceHealthState,
    pub route: Option<ParentServiceHealthRoute>,
    pub protocol_schema_version: Option<u16>,
    pub service_version: Option<String>,
    pub transport: Option<ParentServiceHealthTransport>,
    pub authentication_state: ParentServiceHealthAuthenticationState,
    pub reason: ParentServiceHealthReason,
    pub trace: ParentServiceHealthTraceSnapshot,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentCommandResultDetailSnapshot {
    pub label: String,
    pub value: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentCommandResultProjectionSnapshot {
    pub projection_kind: String,
    pub details: Vec<ParentCommandResultDetailSnapshot>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentRouteEventSnapshot {
    pub event: Option<String>,
    pub event_id: Option<ParentRouteEventId>,
    pub correlation_id: Option<ParentRouteEventCorrelationId>,
    pub sent_at: Option<String>,
    pub source_peer_id: Option<ParentRoutePeerId>,
    pub source_role: Option<ParentRoutePeerRole>,
    pub target_peer_id: Option<ParentRoutePeerId>,
    pub target_role: Option<ParentRoutePeerRole>,
    pub severity: Option<String>,
    pub payload: Option<Value>,
    pub snapshot: Option<Value>,
    pub command_result_projection: Option<ParentCommandResultProjectionSnapshot>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentLanAddDeviceScanSummarySnapshot {
    pub schema_version: u16,
    pub source_labels: Vec<String>,
    pub scanned_device_count: u32,
    pub agent_device_count: u32,
    pub passive_device_count: u32,
    pub infrastructure_device_count: u32,
    pub unsupported_device_count: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentLanPairingDeviceRefSnapshot {
    pub device_id: ParentLanDeviceId,
    pub child_profile_id: Option<ParentContractReferenceId>,
    pub label: String,
    pub platform: String,
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub hostname: Option<String>,
    pub network_interface: Option<String>,
    pub agent_status: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentLanServiceIdentityProbeEvidenceSnapshot {
    pub evidence_kind: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentLanBrowserAddDeviceDiscoveryDeviceSnapshot {
    pub schema_version: u16,
    pub discovered_at: String,
    pub child_device: ParentLanPairingDeviceRefSnapshot,
    pub agent_peer_id: ParentRoutePeerId,
    pub route_id: ParentLanRouteId,
    pub network_mode: String,
    pub reachability: String,
    pub address_ref: ParentLanAddressRef,
    pub discovery_status: String,
    pub discovery_state: String,
    pub evidence_sources: Vec<String>,
    pub service_identity_probe_evidence: Vec<ParentLanServiceIdentityProbeEvidenceSnapshot>,
    pub hint_sources: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentLanBrowserAddDevicePairingRequestSnapshot {
    pub schema_version: u16,
    pub challenge_id: ParentLanChallengeId,
    pub child_device_id: ParentLanDeviceId,
    pub parent_device_id: ParentLanDeviceId,
    pub route_id: ParentLanRouteId,
    pub origin: String,
    pub pairing_state: String,
    pub rejection_reason: Option<String>,
    pub issued_at: String,
    pub expires_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentLanDiscoveryEvidenceRecordSnapshot {
    pub schema_version: u16,
    pub evidence_id: ParentEvidenceId,
    pub source: String,
    pub evidence_kind: String,
    pub device_id: ParentLanDeviceId,
    pub value: String,
    pub normalized_value: String,
    pub first_seen_at: String,
    pub last_seen_at: String,
    pub expires_at: Option<String>,
    pub confidence: String,
    pub merge_key: String,
    pub note: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentLanCanonicalHouseholdNetworkIdentitySnapshot {
    pub hostname: Option<String>,
    pub ip_addresses: Vec<String>,
    pub mac_address: Option<String>,
    pub mac_vendor: Option<String>,
    pub network_interfaces: Vec<String>,
    pub reachability: String,
    pub confidence: String,
    pub stale_at: Option<String>,
    pub offline_at: Option<String>,
    pub evidence_records: Vec<ParentLanDiscoveryEvidenceRecordSnapshot>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentLanChildAgentInventoryPacketSnapshot {
    pub device_name: String,
    pub platform: String,
    pub os: String,
    pub cpu_model: Option<String>,
    pub cpu_cores: Option<String>,
    pub memory_total: Option<String>,
    pub gpu_model: Option<String>,
    pub gpu_driver: Option<String>,
    pub gpu_memory: Option<String>,
    pub nvidia_smi: Option<String>,
    pub network_interfaces: Vec<String>,
    pub capabilities: Vec<String>,
    pub role_state: String,
    pub route_state: String,
    pub pairing_trust_state: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentLanCanonicalHouseholdDeviceSnapshot {
    pub schema_version: u16,
    pub canonical_device_id: ParentLanCanonicalDeviceId,
    pub display_name: String,
    pub classification: String,
    pub role_badges: Vec<String>,
    pub enrollable: bool,
    pub discovery_state: String,
    pub trust_state: String,
    pub route_id: Option<ParentLanRouteId>,
    pub route_state: String,
    pub network_mode: String,
    pub source_labels: Vec<String>,
    pub network_identity: ParentLanCanonicalHouseholdNetworkIdentitySnapshot,
    pub child_agent_inventory: Option<ParentLanChildAgentInventoryPacketSnapshot>,
    pub policy_target_surfaces: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentLanTrustedDeviceRegistryEntrySnapshot {
    pub schema_version: u16,
    pub pairing_id: ParentLanPairingId,
    pub child_device: ParentLanPairingDeviceRefSnapshot,
    pub parent_device: ParentLanPairingDeviceRefSnapshot,
    pub route_id: ParentLanRouteId,
    pub origin: String,
    pub proof_digest: String,
    pub trust_state: String,
    pub trusted_at: String,
    pub expires_at: String,
    pub revoked_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentLanHouseholdDeviceDecisionSnapshot {
    pub schema_version: u16,
    pub action_id: ParentLanActionId,
    pub action_kind: String,
    pub canonical_device_id: ParentLanCanonicalDeviceId,
    pub child_profile_id: Option<ParentContractReferenceId>,
    pub display_name: Option<String>,
    pub device_kind: Option<String>,
    pub parent_actor_id: ParentParentActorId,
    pub decided_at: String,
    pub revoked_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentLanSignedDiscoveryRelayAdapterRowSnapshot {
    pub schema_version: u16,
    pub adapter: String,
    pub discovery_state: String,
    pub proof_state: String,
    pub source_confidence: String,
    pub custody_label: String,
    pub runtime_owner: String,
    pub evidence_label: String,
    pub required_artifact_summary: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentLanSignedDiscoveryRelaySignedProofRowSnapshot {
    pub schema_version: u16,
    pub check: String,
    pub discovery_state: String,
    pub response_state: String,
    pub rejection_reason: Option<String>,
    pub proof_state: String,
    pub runtime_owner: String,
    pub evidence_label: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentLanSignedDiscoveryRelayRouteSafetyRowSnapshot {
    pub schema_version: u16,
    pub check: String,
    pub route_id: Option<ParentLanRouteId>,
    pub discovery_state: String,
    pub response_state: String,
    pub rejection_reason: Option<String>,
    pub proof_state: String,
    pub runtime_owner: String,
    pub custody_label: String,
    pub evidence_label: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentLanSignedDiscoveryRelayCacheRowSnapshot {
    pub schema_version: u16,
    pub check: String,
    pub decision_state: String,
    pub discovery_state: String,
    pub proof_state: String,
    pub runtime_owner: String,
    pub custody_label: String,
    pub evidence_label: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentLanSignedDiscoveryRelaySpineSummarySnapshot {
    pub schema_version: u16,
    pub generated_at: String,
    pub adapter_rows: Vec<ParentLanSignedDiscoveryRelayAdapterRowSnapshot>,
    pub signed_proof_rows: Vec<ParentLanSignedDiscoveryRelaySignedProofRowSnapshot>,
    pub route_safety_rows: Vec<ParentLanSignedDiscoveryRelayRouteSafetyRowSnapshot>,
    pub relay_cache_rows: Vec<ParentLanSignedDiscoveryRelayCacheRowSnapshot>,
    pub manual_proof_required: Vec<String>,
    pub not_implemented: Vec<String>,
    pub claims_proved: Vec<String>,
    pub claims_not_proved: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentLanSelectedDeviceReadinessSnapshot {
    pub schema_version: u16,
    pub selected_child_device_id: Option<ParentChildDeviceId>,
    pub route_id: Option<ParentLanRouteId>,
    pub pairing_id: Option<ParentLanPairingId>,
    pub trust_state: String,
    pub reachability: String,
    pub ready_for_control: bool,
    pub stale_at: Option<String>,
    pub offline_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentLanDiscoveryEventRowSnapshot {
    pub schema_version: u16,
    pub event_id: ParentLanDiscoveryEventId,
    pub event_kind: String,
    pub occurred_at: String,
    pub previous_event_id: Option<ParentLanDiscoveryEventId>,
    pub scan_session_id: Option<ParentLanScanSessionId>,
    pub affected_device_id: Option<ParentLanDeviceId>,
    pub evidence_id: Option<ParentEvidenceId>,
    pub summary: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentLanDiscoveryEventHistorySnapshot {
    pub schema_version: u16,
    pub generated_at: String,
    pub state: String,
    pub latest_event_id: Option<ParentLanDiscoveryEventId>,
    pub latest_observed_at: Option<String>,
    pub rows: Vec<ParentLanDiscoveryEventRowSnapshot>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentLanDiscoverySourceMatrixWorkpackRowSnapshot {
    pub workpack_id: ParentLanWorkpackId,
    pub title: String,
    pub discovery_state: String,
    pub proof_state: String,
    pub runtime_owner: String,
    pub status: String,
    pub read_model_visible: bool,
    pub required_artifact_summary: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentLanDiscoverySourceMatrixSourceRowSnapshot {
    pub source: String,
    pub workpack_id: ParentLanWorkpackId,
    pub status: String,
    pub authority: String,
    pub runtime_path: String,
    pub ui_surface: String,
    pub can_confirm_child_agent: bool,
    pub can_assign_child_profile: bool,
    pub can_control_route: bool,
    pub requires_selected_interface: bool,
    pub persists_across_restart: bool,
    pub evidence_label: String,
    pub required_artifact_summary: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentLanDiscoverySourceMatrixSnapshot {
    pub schema_version: u16,
    pub generated_at: String,
    pub workpack_rows: Vec<ParentLanDiscoverySourceMatrixWorkpackRowSnapshot>,
    pub source_rows: Vec<ParentLanDiscoverySourceMatrixSourceRowSnapshot>,
    pub claims_proved: Vec<String>,
    pub claims_not_proved: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentLanAddDeviceReadModelSnapshot {
    pub schema_version: u16,
    pub generated_at: String,
    pub discovery_source: String,
    pub add_device_state: String,
    pub local_service_discovery_state: String,
    pub physical_household_lan_state: String,
    pub cloud_relay_state: String,
    pub scan_summary: ParentLanAddDeviceScanSummarySnapshot,
    pub discovered_devices: Vec<ParentLanBrowserAddDeviceDiscoveryDeviceSnapshot>,
    pub discovery_event_history: ParentLanDiscoveryEventHistorySnapshot,
    pub canonical_household_devices: Vec<ParentLanCanonicalHouseholdDeviceSnapshot>,
    pub pairing_requests: Vec<ParentLanBrowserAddDevicePairingRequestSnapshot>,
    pub trusted_device_registry: Vec<ParentLanTrustedDeviceRegistryEntrySnapshot>,
    pub household_device_decisions: Vec<ParentLanHouseholdDeviceDecisionSnapshot>,
    pub signed_discovery_relay_spine: Option<ParentLanSignedDiscoveryRelaySpineSummarySnapshot>,
    pub lan_discovery_source_matrix: Option<ParentLanDiscoverySourceMatrixSnapshot>,
    pub trusted_device_ids: Vec<ParentLanDeviceId>,
    pub revoked_device_ids: Vec<ParentLanDeviceId>,
    pub selected_device_readiness: ParentLanSelectedDeviceReadinessSnapshot,
    pub controller_authority: String,
    pub observer_authority: String,
    pub route_requirement_labels: Vec<String>,
    pub audit_check_labels: Vec<String>,
    pub honest_non_claims: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentActivityEvidenceRefSnapshot {
    pub evidence_id: ParentEvidenceId,
    pub kind: String,
    pub digest: Option<String>,
    pub uri: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentActivityNetworkEndpointSnapshot {
    pub ip: Option<String>,
    pub port: Option<u16>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentActivityNetworkFlowCountersSnapshot {
    pub connection_count: u64,
    pub bytes_sent: Option<u64>,
    pub bytes_received: Option<u64>,
    pub first_seen_at: Option<String>,
    pub last_seen_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentActivityNetworkFlowObservationSnapshot {
    pub schema_version: u16,
    pub event_id: ParentRouteEventId,
    pub observed_at: String,
    pub observer: String,
    pub capability_status: String,
    pub adapter_id: ParentRouteAdapterId,
    pub protocol: Option<String>,
    pub tcp_state: Option<String>,
    pub local_endpoint: ParentActivityNetworkEndpointSnapshot,
    pub destination_endpoint: ParentActivityNetworkEndpointSnapshot,
    pub destination_domain: Option<String>,
    pub domain_attribution_status: String,
    pub process_attribution_status: String,
    pub process_id: Option<u64>,
    pub process_name: Option<String>,
    pub counters: ParentActivityNetworkFlowCountersSnapshot,
    pub evidence: Vec<ParentActivityEvidenceRefSnapshot>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentActivityNetworkFlowReadModelSnapshot {
    pub schema_version: u16,
    pub generated_at: String,
    pub custody: String,
    pub limit: u64,
    pub returned: u64,
    pub active_rows: u64,
    pub tombstone_rows: u64,
    pub exportable_rows: u64,
    pub capability_status: String,
    pub latest_event_id: Option<ParentRouteEventId>,
    pub latest_observed_at: Option<String>,
    pub latest_tombstone_event_id: Option<ParentRouteEventId>,
    pub latest_tombstone_observed_at: Option<String>,
    pub deleted_evidence_reference_ids: Vec<ParentEvidenceReferenceId>,
    pub rows: Vec<ParentActivityNetworkFlowObservationSnapshot>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentActivityTrackingReadModelCountSnapshot {
    pub value: String,
    pub count: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentActivityTrackingReadModelRowSnapshot {
    pub schema_version: u16,
    pub event_id: ParentRouteEventId,
    pub observed_at: String,
    pub device_id: ParentLanDeviceId,
    pub platform: String,
    pub observer: String,
    pub kind: String,
    pub subject_kind: String,
    pub subject_id: ParentSubjectId,
    pub subject_display_name: Option<String>,
    pub capability_status: Option<String>,
    pub query_visibility: String,
    pub deleted_at: Option<String>,
    pub evidence_reference_ids: Vec<ParentEvidenceReferenceId>,
    pub deleted_evidence_reference_ids: Vec<ParentEvidenceReferenceId>,
    pub evidence: Vec<ParentActivityEvidenceRefSnapshot>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentActivityTrackingReadModelSnapshot {
    pub schema_version: u16,
    pub generated_at: String,
    pub custody_label: String,
    pub limit: u64,
    pub returned: u64,
    pub active_rows: u64,
    pub tombstone_rows: u64,
    pub capability_status: String,
    pub latest_event_id: Option<ParentRouteEventId>,
    pub latest_observed_at: Option<String>,
    pub latest_active_event_id: Option<ParentRouteEventId>,
    pub latest_active_observed_at: Option<String>,
    pub latest_tombstone_event_id: Option<ParentRouteEventId>,
    pub latest_tombstone_observed_at: Option<String>,
    pub active_kind_counts: Vec<ParentActivityTrackingReadModelCountSnapshot>,
    pub active_device_counts: Vec<ParentActivityTrackingReadModelCountSnapshot>,
    pub active_capability_status_counts: Vec<ParentActivityTrackingReadModelCountSnapshot>,
    pub deleted_evidence_reference_ids: Vec<ParentEvidenceReferenceId>,
    pub rows: Vec<ParentActivityTrackingReadModelRowSnapshot>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParentActivityTrackingReadModelFailureReason {
    WrongEvent,
    MissingJsonField,
    InvalidJson,
    InvalidPayload,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentActivityTrackingReadModelResultSnapshot {
    pub ok: bool,
    pub reason: Option<ParentActivityTrackingReadModelFailureReason>,
    pub value: Option<ParentActivityTrackingReadModelSnapshot>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentNetworkRuntimeEventValueSnapshot {
    pub ai_analysis_ref: Option<ParentContractReferenceId>,
    pub policy_decision_ref: Option<ParentContractReferenceId>,
    pub enforcement_result_ref: Option<ParentContractReferenceId>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentNetworkEvidenceSummarySnapshot {
    pub ai_audit_ref: Option<ParentContractReferenceId>,
    pub policy_decision_ref: Option<ParentContractReferenceId>,
    pub network_evidence_grade: Option<String>,
    pub intervention_result_ref: Option<ParentContractReferenceId>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentNetworkRuntimeEventResultSnapshot {
    pub ok: bool,
    pub reason: Option<String>,
    pub event_type: Option<ParentRuntimeEventType>,
    pub value: Option<ParentNetworkRuntimeEventValueSnapshot>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentNetworkRuntimeEventChainStreamSnapshot {
    pub streamed_event_count: Option<u64>,
    pub events: Vec<ParentNetworkRuntimeEventResultSnapshot>,
    pub invalid_event_count: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentPolicyPreviewConfirmationContext {
    pub request_id: Option<String>,
    pub submission_key: Option<String>,
    pub household_id: Option<String>,
    pub child_profile_id: Option<String>,
    pub device_id: Option<String>,
    pub source_document_id: Option<String>,
    pub policy_version: Option<u64>,
    pub target_reference_id: Option<String>,
    pub rule_id: Option<String>,
    pub requested_at: Option<String>,
    pub expires_at: Option<String>,
    pub assistant_preview_id: Option<String>,
    pub audit_reference_ids: Option<String>,
    pub actor_id: Option<String>,
    pub actor_role: Option<String>,
    pub actor_state: Option<String>,
    pub confirmation_audit_reference_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentPolicyPreviewReadModelSnapshot {
    pub schema_version: Option<String>,
    pub generated_at: Option<String>,
    pub custody: Option<String>,
    pub limit: Option<u64>,
    pub returned: u64,
    pub capability_status: Option<String>,
    pub preview_id: Option<ParentPolicyPreviewId>,
    pub latest_event_id: Option<ParentRouteEventId>,
    pub latest_observed_at: Option<String>,
    pub target_id: Option<ParentPolicyTargetId>,
    pub target_type: Option<String>,
    pub target_value: Option<String>,
    pub evidence_reference_count: Option<u64>,
    pub parent_rule_context_reference_count: Option<u64>,
    pub parent_rule_context_ref_ids: Option<ParentPolicyRuleContextRefIds>,
    pub decision_id: Option<ParentPolicyDecisionId>,
    pub decision_action: Option<ParentPolicyDecisionActionId>,
    pub reason_codes: Option<ParentPolicyReasonCodes>,
    pub rule_ids: Option<ParentPolicyRuleIds>,
    pub local_ai_result_id: Option<ParentUserLocalAiResultId>,
    pub dry_run: Option<bool>,
    pub enforcement_handoff_state: Option<String>,
    pub policy_preview_save_state: Option<String>,
    pub policy_preview_manual_review_state: Option<String>,
    pub policy_preview_target_state: Option<String>,
    pub policy_preview_target_explanation_code: Option<String>,
    pub policy_preview_finding_kinds: Option<String>,
    pub policy_source_status: Option<String>,
    pub policy_source_surface: Option<String>,
    pub policy_request_origin: Option<String>,
    pub policy_assistant_confirmation_state: Option<String>,
    pub policy_request_status: Option<String>,
    pub policy_approval_id: Option<ParentPolicyApprovalId>,
    pub policy_override_id: Option<ParentPolicyOverrideId>,
    pub policy_replay_of_approval_id: Option<ParentPolicyReplayApprovalId>,
    pub policy_reviewed_by_actor_id: Option<ParentUserActorId>,
    pub policy_reviewed_by_actor_role: Option<String>,
    pub policy_reviewed_at: Option<String>,
    pub policy_audit_reference_id: Option<ParentPolicyAuditReferenceId>,
    pub network_evidence_grade: Option<String>,
    pub network_requested_policy_action: Option<String>,
    pub network_mapped_policy_action: Option<String>,
    pub network_policy_mapping_mode: Option<String>,
    pub network_adapter_action_authorized: Option<bool>,
    pub network_enforcement_command_authorized: Option<bool>,
    pub confirmation_context: Option<ParentPolicyPreviewConfirmationContext>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentPolicyPreviewPanelDetailSnapshot {
    pub label: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentPolicyPreviewPanelCardSnapshot {
    pub title: String,
    pub summary: String,
    pub details: Vec<ParentPolicyPreviewPanelDetailSnapshot>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentPolicyPreviewActionSnapshot {
    pub action: ParentUiActionKind,
    pub label: String,
    pub payload: Option<Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentPolicyPreviewAuthoringSnapshot {
    pub target_value: String,
    pub requested_action: String,
    pub stage_action: ParentPolicyPreviewActionSnapshot,
    pub confirm_action: Option<ParentPolicyPreviewActionSnapshot>,
    pub cancel_action: ParentPolicyPreviewActionSnapshot,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentPolicyPreviewPanelSnapshot {
    pub title: String,
    pub body: String,
    pub summary: String,
    pub summary_details: Vec<ParentPolicyPreviewPanelDetailSnapshot>,
    pub cards: Vec<ParentPolicyPreviewPanelCardSnapshot>,
    pub empty_message: String,
    pub product_claim: String,
    pub authoring: Option<ParentPolicyPreviewAuthoringSnapshot>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentAppGamePanelDetailSnapshot {
    pub label: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentAppGamePanelRowSnapshot {
    pub title: String,
    pub details: Vec<ParentAppGamePanelDetailSnapshot>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentAppGamePanelSnapshot {
    pub eyebrow: String,
    pub title: String,
    pub body: String,
    pub load_state: String,
    pub summary_details: Vec<ParentAppGamePanelDetailSnapshot>,
    pub rows: Vec<ParentAppGamePanelRowSnapshot>,
    pub empty_message: String,
    pub product_claim: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentAppGameActionRowSnapshot {
    pub title: String,
    pub details: Vec<ParentAppGamePanelDetailSnapshot>,
    pub action_label: Option<String>,
    pub action_payload: Option<Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentAppGameAdapterDispatchPanelSnapshot {
    pub eyebrow: String,
    pub title: String,
    pub body: String,
    pub preflight_panel: ParentAppGamePanelSnapshot,
    pub result_panel: ParentAppGamePanelSnapshot,
    pub execute_action_label: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentAppGameTimerParentSurfacePanelSnapshot {
    pub eyebrow: String,
    pub title: String,
    pub body: String,
    pub load_state: String,
    pub summary_details: Vec<ParentAppGamePanelDetailSnapshot>,
    pub parent_action_rows: Vec<ParentAppGamePanelRowSnapshot>,
    pub parent_preference_setup_rows: Vec<ParentAppGameActionRowSnapshot>,
    pub rows: Vec<ParentAppGamePanelRowSnapshot>,
    pub empty_message: String,
    pub product_claim: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentAppGameNotificationParentSurfacePanelRowSnapshot {
    pub key: String,
    pub title: String,
    pub details: Vec<ParentAppGamePanelDetailSnapshot>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentAppGameNotificationParentSurfacePanelSnapshot {
    pub eyebrow: String,
    pub title: String,
    pub body: String,
    pub state: String,
    pub summary: String,
    pub product_claim: String,
    pub metrics: Vec<ParentAppGamePanelDetailSnapshot>,
    pub rows: Vec<ParentAppGameNotificationParentSurfacePanelRowSnapshot>,
    pub empty_message: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentScreenSummaryPanelDetailSnapshot {
    pub label: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentScreenSummaryPanelRowSnapshot {
    pub title: String,
    pub details: Vec<ParentScreenSummaryPanelDetailSnapshot>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentScreenSummaryPanelSnapshot {
    pub eyebrow: String,
    pub title: String,
    pub body: String,
    pub load_state: String,
    pub summary_details: Vec<ParentScreenSummaryPanelDetailSnapshot>,
    pub rows: Vec<ParentScreenSummaryPanelRowSnapshot>,
    pub empty_message: String,
    pub product_claim: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentTrackingStatusPanelDetailSnapshot {
    pub label: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentTrackingStatusPanelCardSnapshot {
    pub key: String,
    pub title: String,
    pub details: Vec<ParentTrackingStatusPanelDetailSnapshot>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentTrackingStatusPanelSnapshot {
    pub eyebrow: String,
    pub title: String,
    pub body: String,
    pub summary_cards: Vec<ParentTrackingStatusPanelCardSnapshot>,
    pub cards: Vec<ParentTrackingStatusPanelCardSnapshot>,
    pub empty_message: String,
    pub product_claim: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentSetupFirstRunPanelDetailSnapshot {
    pub label: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentSetupFirstRunPanelCardSnapshot {
    pub title: String,
    pub summary: String,
    pub details: Vec<ParentSetupFirstRunPanelDetailSnapshot>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentSetupFirstRunPanelSnapshot {
    pub eyebrow: String,
    pub title: String,
    pub body: String,
    pub summary_card_title: String,
    pub summary: String,
    pub summary_details: Vec<ParentSetupFirstRunPanelDetailSnapshot>,
    pub cards: Vec<ParentSetupFirstRunPanelCardSnapshot>,
    pub product_claim: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentBrowserPanelDetailSnapshot {
    pub label: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentBrowserPanelRowSnapshot {
    pub key: String,
    pub title: String,
    pub details: Vec<ParentBrowserPanelDetailSnapshot>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentBrowserPanelSnapshot {
    pub eyebrow: String,
    pub title: String,
    pub body: String,
    pub summary: String,
    pub summary_details: Vec<ParentBrowserPanelDetailSnapshot>,
    pub rows: Vec<ParentBrowserPanelRowSnapshot>,
    pub empty_message: String,
    pub product_claim: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentRouteLiveActivitySnapshot {
    pub recent_summary: Option<Value>,
    pub ingest_status: Option<Value>,
    pub activity_screen_read_model: Option<Value>,
    pub activity_app_use_read_model: Option<Value>,
    pub activity_app_game_platform_extension_read_model: Option<Value>,
    pub activity_browser_read_model: Option<Value>,
    pub activity_games_read_model: Option<Value>,
    pub screen_summary_panel: Option<ParentScreenSummaryPanelSnapshot>,
    pub browser_inventory_event: Option<ParentRouteEventSnapshot>,
    pub browser_inventory_read_model: Option<Value>,
    pub browser_evidence_event: Option<ParentRouteEventSnapshot>,
    pub browser_evidence_read_model: Option<Value>,
    pub browser_managed_event: Option<ParentRouteEventSnapshot>,
    pub browser_managed_status: Option<Value>,
    pub local_ai_runtime_status_event: Option<ParentRouteEventSnapshot>,
    pub lan_ai_job_event: Option<ParentRouteEventSnapshot>,
    pub parent_assistant_boundary_event: Option<ParentRouteEventSnapshot>,
    pub activity_memory_graph_read_model: Option<Value>,
    pub network_flow_event: Option<ParentRouteEventSnapshot>,
    pub network_flow_read_model: Option<ParentActivityNetworkFlowReadModelSnapshot>,
    pub network_evidence_summary: Option<ParentNetworkEvidenceSummarySnapshot>,
    pub network_runtime_event_chain_stream: Option<ParentNetworkRuntimeEventChainStreamSnapshot>,
    pub lan_pairing_browser_discovery_event: Option<ParentRouteEventSnapshot>,
    pub lan_add_device_read_model: Option<ParentLanAddDeviceReadModelSnapshot>,
    pub policy_preview_panel: Option<ParentPolicyPreviewPanelSnapshot>,
    pub app_game_notification_parent_surface_panel:
        Option<ParentAppGameNotificationParentSurfacePanelSnapshot>,
    pub app_game_policy_readiness_panel: Option<ParentAppGamePanelSnapshot>,
    pub app_game_platform_proof_status_panel: Option<ParentAppGamePanelSnapshot>,
    pub app_game_child_runtime_transport_receipt_panel: Option<ParentAppGamePanelSnapshot>,
    pub app_game_adapter_dispatch_panel: Option<ParentAppGameAdapterDispatchPanelSnapshot>,
    pub app_game_timer_parent_surface_panel: Option<ParentAppGameTimerParentSurfacePanelSnapshot>,
    pub browser_intervention_event: Option<ParentRouteEventSnapshot>,
    pub browser_intervention_read_model: Option<Value>,
    pub activity_tracking_read_model_event: Option<ParentRouteEventSnapshot>,
    pub activity_tracking_read_model: Option<ParentActivityTrackingReadModelResultSnapshot>,
    pub activity_tracking_panel: Option<ParentTrackingStatusPanelSnapshot>,
    pub activity_tracking_retention_settings_write_result: Option<Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentRouteBrowserPanelsSnapshot {
    pub browser_parent_explanation: Option<ParentBrowserPanelSnapshot>,
    pub social_audit_explanation: Option<ParentBrowserPanelSnapshot>,
    pub social_alert_report: Option<ParentBrowserPanelSnapshot>,
    pub social_alert_report_parent_surface: Option<ParentBrowserPanelSnapshot>,
    pub social_parent_notification_delivery: Option<ParentBrowserPanelSnapshot>,
    pub social_dashboard: Option<ParentBrowserPanelSnapshot>,
    pub browser_action_intent_stream_status: Option<ParentBrowserPanelSnapshot>,
    pub browser_social_provider_receipt_stream_status: Option<ParentBrowserPanelSnapshot>,
    pub browser_social_provider_receipt_ingestion_readiness_status:
        Option<ParentBrowserPanelSnapshot>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentRouteSnapshot {
    pub schema_version: u16,
    pub route: ParentRouteId,
    pub generated_at: String,
    pub season_label: String,
    pub last_updated: String,
    pub connection_state: ParentBridgeConnectionState,
    pub command_enabled: bool,
    pub agent_endpoint: String,
    pub data_source: ParentRouteDataSource,
    pub summary: ParentRouteSummary,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_health: Option<ParentServiceHealthSnapshot>,
    pub diagnostic_panels_enabled: bool,
    pub parent_portal_rows: Option<Vec<ParentPortalRowSnapshot>>,
    pub parent_portal_shell_status: Option<ParentPortalShellStatusSnapshot>,
    pub live_activity: Option<ParentRouteLiveActivitySnapshot>,
    pub browser_panels: Option<ParentRouteBrowserPanelsSnapshot>,
    pub setup_first_run_panel: Option<ParentSetupFirstRunPanelSnapshot>,
    pub screen_settings_service_response: Option<Value>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentRouteSummary {
    pub title: String,
    pub route_capability: String,
    pub parent_access: String,
    pub household: String,
    pub child_device: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParentUiActionKind {
    RefreshRoute,
    Reconnect,
    AgentCommandRequested,
    PolicyPreviewAuthoringDraftStaged,
    PolicyPreviewAuthoringDraftCancelled,
    PolicyRequestAssistantPreviewConfirmRequested,
    PolicyRequestParentResolutionRequested,
    LanPairingBrowserDiscoveryScanRequested,
    NetworkFlowReadModelRefreshRequested,
    TrackingRetentionSettingsWriteRequested,
    ScreenSettingsGetRequested,
    ScreenSettingsReplaceRequested,
    AppGameAdapterDispatchExecuteRequested,
    AppGameTimerParentPreferenceSetupRequested,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentUiAction {
    pub action: ParentUiActionKind,
    pub route: ParentRouteId,
    pub context: Option<ParentRouteContext>,
    pub command: Option<String>,
    pub payload: Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentUiActionResult {
    pub schema_version: u16,
    pub accepted: bool,
    pub connection_state: ParentBridgeConnectionState,
    pub message: String,
    pub snapshot: Option<ParentRouteSnapshot>,
    pub events: Vec<ParentRouteEventSnapshot>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentSubscriptionEvent {
    pub schema_version: u16,
    pub route: ParentRouteId,
    pub snapshot: ParentRouteSnapshot,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<ParentRouteEventSnapshot>>,
}
