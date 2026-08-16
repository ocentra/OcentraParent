use super::*;

pub(super) fn field_descriptors() -> Vec<ProtocolLiteralDescriptor<&'static str>> {
    let mut descriptors = Vec::new();
    descriptors.extend(field_descriptors_core());
    descriptors.extend(field_descriptors_browser_social());
    descriptors.extend(field_descriptors_browser_runtime_action());
    descriptors.extend(field_descriptors_browser_runtime_stream());
    descriptors.extend(field_descriptors_lan());
    descriptors.extend(field_descriptors_local_ai());
    descriptors.extend(field_descriptors_network());
    descriptors.extend(field_descriptors_parent_assistant());
    descriptors.extend(field_descriptors_misc());
    descriptors
}

fn field_descriptors_core() -> Vec<ProtocolLiteralDescriptor<&'static str>> {
    vec![
        field_descriptor("ActivityDigest", field::ACTIVITY_DIGEST),
        field_descriptor("ActivityFamilySources", field::ACTIVITY_FAMILY_SOURCES),
        field_descriptor("ActivityReadModel", field::ACTIVITY_READ_MODEL),
        field_descriptor("ActivityReadModelKind", field::ACTIVITY_READ_MODEL_KIND),
        field_descriptor("ActivityReportDocument", field::ACTIVITY_REPORT_DOCUMENT),
        field_descriptor("ActivityReportFrequency", field::ACTIVITY_REPORT_FREQUENCY),
        field_descriptor("ActivityReportId", field::ACTIVITY_REPORT_ID),
        field_descriptor("ActivityReports", field::ACTIVITY_REPORTS),
        field_descriptor("ActivitySurfaceState", field::ACTIVITY_SURFACE_STATE),
        field_descriptor(
            "ActivityTrackingRetentionSettingsWriteResult",
            field::ACTIVITY_TRACKING_RETENTION_SETTINGS_WRITE_RESULT,
        ),
        field_descriptor("ClaimBoundary", field::CLAIM_BOUNDARY),
        field_descriptor("DeviceId", field::DEVICE_ID),
        field_descriptor("EventRef", field::EVENT_REF),
        field_descriptor("EventType", field::EVENT_TYPE),
        field_descriptor("FamilyId", field::FAMILY_ID),
        field_descriptor("Origin", field::ORIGIN),
        field_descriptor("Payload", field::PAYLOAD),
        field_descriptor("StartedAt", field::STARTED_AT),
        field_descriptor("StaleAt", field::STALE_AT),
    ]
}

fn field_descriptors_browser_social() -> Vec<ProtocolLiteralDescriptor<&'static str>> {
    vec![
        field_descriptor(
            "BrowserSocialAlertReportReadModel",
            field::BROWSER_SOCIAL_ALERT_REPORT_READ_MODEL,
        ),
        field_descriptor(
            "BrowserSocialAlertReportParentSurfaceReadModel",
            field::BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_READ_MODEL,
        ),
        field_descriptor(
            "BrowserSocialDashboardReadModel",
            field::BROWSER_SOCIAL_DASHBOARD_READ_MODEL,
        ),
        field_descriptor(
            "BrowserSocialParentNotificationDeliveryReadModel",
            field::BROWSER_SOCIAL_PARENT_NOTIFICATION_DELIVERY_READ_MODEL,
        ),
    ]
}

fn field_descriptors_browser_runtime_action() -> Vec<ProtocolLiteralDescriptor<&'static str>> {
    vec![
        field_descriptor(
            "BrowserRuntimeActionIntentAdapterExecutions",
            field::BROWSER_RUNTIME_ACTION_INTENT_ADAPTER_EXECUTIONS,
        ),
        field_descriptor(
            "BrowserRuntimeActionIntentCandidates",
            field::BROWSER_RUNTIME_ACTION_INTENT_CANDIDATES,
        ),
        field_descriptor(
            "BrowserRuntimeActionIntentChildAcceptedEventRefs",
            field::BROWSER_RUNTIME_ACTION_INTENT_CHILD_ACCEPTED_EVENT_REFS,
        ),
        field_descriptor(
            "BrowserRuntimeActionIntentChildAcceptedRows",
            field::BROWSER_RUNTIME_ACTION_INTENT_CHILD_ACCEPTED_ROWS,
        ),
        field_descriptor(
            "BrowserRuntimeActionIntentChildCommandRefs",
            field::BROWSER_RUNTIME_ACTION_INTENT_CHILD_COMMAND_REFS,
        ),
        field_descriptor(
            "BrowserRuntimeActionIntentChildInterventionExecutions",
            field::BROWSER_RUNTIME_ACTION_INTENT_CHILD_INTERVENTION_EXECUTIONS,
        ),
        field_descriptor(
            "BrowserRuntimeActionIntentDispatchAttempts",
            field::BROWSER_RUNTIME_ACTION_INTENT_DISPATCH_ATTEMPTS,
        ),
        field_descriptor(
            "BrowserRuntimeActionIntentEnforcementExecutions",
            field::BROWSER_RUNTIME_ACTION_INTENT_ENFORCEMENT_EXECUTIONS,
        ),
        field_descriptor(
            "BrowserRuntimeActionIntentHandoffCandidates",
            field::BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_CANDIDATES,
        ),
        field_descriptor(
            "BrowserRuntimeActionIntentHandoffOutboxRefs",
            field::BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_OUTBOX_REFS,
        ),
        field_descriptor(
            "BrowserRuntimeActionIntentHandoffRefs",
            field::BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_REFS,
        ),
        field_descriptor(
            "BrowserRuntimeActionIntentParentReadModelRefs",
            field::BROWSER_RUNTIME_ACTION_INTENT_PARENT_READ_MODEL_REFS,
        ),
    ]
}

fn field_descriptors_browser_runtime_stream() -> Vec<ProtocolLiteralDescriptor<&'static str>> {
    vec![
        field_descriptor(
            "BrowserRuntimeEventChainStream",
            field::BROWSER_RUNTIME_EVENT_CHAIN_STREAM,
        ),
        field_descriptor(
            "BrowserRuntimeExactUrlRows",
            field::BROWSER_RUNTIME_EXACT_URL_ROWS,
        ),
        field_descriptor(
            "BrowserRuntimeFailedRows",
            field::BROWSER_RUNTIME_FAILED_ROWS,
        ),
        field_descriptor(
            "BrowserRuntimeInterventionCommandEvents",
            field::BROWSER_RUNTIME_INTERVENTION_COMMAND_EVENTS,
        ),
        field_descriptor(
            "BrowserRuntimeManualRequiredRows",
            field::BROWSER_RUNTIME_MANUAL_REQUIRED_ROWS,
        ),
        field_descriptor(
            "BrowserRuntimeObservedRows",
            field::BROWSER_RUNTIME_OBSERVED_ROWS,
        ),
        field_descriptor(
            "BrowserRuntimeReadModelProjectionEvents",
            field::BROWSER_RUNTIME_READ_MODEL_PROJECTION_EVENTS,
        ),
        field_descriptor(
            "BrowserRuntimeSocialProviderAttemptRefs",
            field::BROWSER_RUNTIME_SOCIAL_PROVIDER_ATTEMPT_REFS,
        ),
        field_descriptor(
            "BrowserRuntimeSocialProviderDispatchRequiredRows",
            field::BROWSER_RUNTIME_SOCIAL_PROVIDER_DISPATCH_REQUIRED_ROWS,
        ),
        field_descriptor(
            "BrowserRuntimeSocialProviderDurableResultRefs",
            field::BROWSER_RUNTIME_SOCIAL_PROVIDER_DURABLE_RESULT_REFS,
        ),
        field_descriptor(
            "BrowserRuntimeSocialProviderDurableRows",
            field::BROWSER_RUNTIME_SOCIAL_PROVIDER_DURABLE_ROWS,
        ),
        field_descriptor(
            "BrowserRuntimeSocialProviderDurableStoreRefs",
            field::BROWSER_RUNTIME_SOCIAL_PROVIDER_DURABLE_STORE_REFS,
        ),
        field_descriptor(
            "BrowserRuntimeSocialProviderManualReceiptRequiredRows",
            field::BROWSER_RUNTIME_SOCIAL_PROVIDER_MANUAL_RECEIPT_REQUIRED_ROWS,
        ),
        field_descriptor(
            "BrowserRuntimeSocialProviderReadModelRefs",
            field::BROWSER_RUNTIME_SOCIAL_PROVIDER_READ_MODEL_REFS,
        ),
        field_descriptor(
            "BrowserRuntimeSocialProviderReceiptBoundaryRows",
            field::BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_BOUNDARY_ROWS,
        ),
        field_descriptor(
            "BrowserRuntimeSocialProviderReceiptProofRefs",
            field::BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_PROOF_REFS,
        ),
        field_descriptor(
            "BrowserRuntimeSocialProviderSupportStatusRefs",
            field::BROWSER_RUNTIME_SOCIAL_PROVIDER_SUPPORT_STATUS_REFS,
        ),
        field_descriptor(
            "BrowserRuntimeStreamedEvents",
            field::BROWSER_RUNTIME_STREAMED_EVENTS,
        ),
    ]
}

fn field_descriptors_lan() -> Vec<ProtocolLiteralDescriptor<&'static str>> {
    vec![
        field_descriptor("LanAiJobId", field::LAN_AI_JOB_ID),
        field_descriptor("LanAiJobState", field::LAN_AI_JOB_STATE),
        field_descriptor("LanAiJobStatus", field::LAN_AI_JOB_STATUS),
        field_descriptor(
            "LanAiProviderCustodyLabel",
            field::LAN_AI_PROVIDER_CUSTODY_LABEL,
        ),
        field_descriptor(
            "LanAiProviderRoutingState",
            field::LAN_AI_PROVIDER_ROUTING_STATE,
        ),
        field_descriptor(
            "LanControllerLeaseExpiresAt",
            field::LAN_CONTROLLER_LEASE_EXPIRES_AT,
        ),
        field_descriptor("LanControllerLeaseId", field::LAN_CONTROLLER_LEASE_ID),
        field_descriptor(
            "LanControllerLeaseIssuedAt",
            field::LAN_CONTROLLER_LEASE_ISSUED_AT,
        ),
        field_descriptor("LanCanonicalDeviceId", field::LAN_CANONICAL_DEVICE_ID),
        field_descriptor("LanChildDeviceId", field::LAN_CHILD_DEVICE_ID),
        field_descriptor("LanControllerDeviceId", field::LAN_CONTROLLER_DEVICE_ID),
        field_descriptor(
            "LanHouseholdActionId",
            lan_pairing::HOUSEHOLD_ACTION_ID_FIELD,
        ),
        field_descriptor(
            "LanHouseholdActionKind",
            lan_pairing::HOUSEHOLD_ACTION_KIND_FIELD,
        ),
        field_descriptor(
            "LanHouseholdActionChildProfileId",
            lan_pairing::HOUSEHOLD_ACTION_CHILD_PROFILE_ID_FIELD,
        ),
        field_descriptor(
            "LanHouseholdActionDisplayName",
            lan_pairing::HOUSEHOLD_ACTION_DISPLAY_NAME_FIELD,
        ),
        field_descriptor(
            "LanHouseholdActionDeviceKind",
            lan_pairing::HOUSEHOLD_ACTION_DEVICE_KIND_FIELD,
        ),
        field_descriptor(
            "LanHouseholdActionRevokedAt",
            lan_pairing::HOUSEHOLD_ACTION_REVOKED_AT_FIELD,
        ),
        field_descriptor("LanIntentId", field::LAN_INTENT_ID),
        field_descriptor("LanIntentKind", field::LAN_INTENT_KIND),
        field_descriptor("LanPairingId", field::LAN_PAIRING_ID),
        field_descriptor("LanParentAuthority", field::LAN_PARENT_AUTHORITY),
        field_descriptor("LanParentActorId", field::LAN_PARENT_ACTOR_ID),
        field_descriptor("LanParentDeviceId", field::LAN_PARENT_DEVICE_ID),
        field_descriptor("LanProofDigest", field::LAN_PROOF_DIGEST),
        field_descriptor("LanRouteId", field::LAN_ROUTE_ID),
        field_descriptor("LoadState", field::LOAD_STATE),
    ]
}

fn field_descriptors_local_ai() -> Vec<ProtocolLiteralDescriptor<&'static str>> {
    vec![
        field_descriptor(
            "LocalAiAdapterReadinessState",
            field::LOCAL_AI_ADAPTER_READINESS_STATE,
        ),
        field_descriptor("LocalAiCapabilityFlags", field::LOCAL_AI_CAPABILITY_FLAGS),
        field_descriptor("LocalAiDegradedState", field::LOCAL_AI_DEGRADED_STATE),
        field_descriptor("LocalAiExecutionState", field::LOCAL_AI_EXECUTION_STATE),
        field_descriptor("LocalAiModelId", field::LOCAL_AI_MODEL_ID),
        field_descriptor("LocalAiPrivacyMode", field::LOCAL_AI_PRIVACY_MODE),
        field_descriptor("LocalAiProviderId", field::LOCAL_AI_PROVIDER_ID),
        field_descriptor("LocalAiProviderSource", field::LOCAL_AI_PROVIDER_SOURCE),
        field_descriptor("LocalAiResourceClass", field::LOCAL_AI_RESOURCE_CLASS),
        field_descriptor(
            "LocalAiRuntimeReferenceId",
            field::LOCAL_AI_RUNTIME_REFERENCE_ID,
        ),
        field_descriptor(
            "LocalAiUnavailableReason",
            field::LOCAL_AI_UNAVAILABLE_REASON,
        ),
        field_descriptor("Message", field::MESSAGE),
    ]
}

fn field_descriptors_network() -> Vec<ProtocolLiteralDescriptor<&'static str>> {
    vec![
        field_descriptor(
            "NetworkAndroidVpnServiceGateStatus",
            network_flow::FIELD_NETWORK_ANDROID_VPN_SERVICE_GATE_STATUS,
        ),
        field_descriptor(
            "NetworkAppleNetworkExtensionGateStatus",
            network_flow::FIELD_NETWORK_APPLE_NETWORK_EXTENSION_GATE_STATUS,
        ),
        field_descriptor(
            "NetworkLinuxNftablesLabStatus",
            network_flow::FIELD_NETWORK_LINUX_NFTABLES_LAB_STATUS,
        ),
        field_descriptor(
            "NetworkLiveCaptureStatus",
            network_flow::FIELD_NETWORK_LIVE_CAPTURE_STATUS,
        ),
        field_descriptor(
            "NetworkRuntimeDeadLetters",
            field::NETWORK_RUNTIME_DEAD_LETTERS,
        ),
        field_descriptor(
            "NetworkRuntimeDeliveredRows",
            field::NETWORK_RUNTIME_DELIVERED_ROWS,
        ),
        field_descriptor(
            "NetworkRuntimeEnforcementCommandEvents",
            field::NETWORK_RUNTIME_ENFORCEMENT_COMMAND_EVENTS,
        ),
        field_descriptor(
            "NetworkRuntimeEventChainStream",
            field::NETWORK_RUNTIME_EVENT_CHAIN_STREAM,
        ),
        field_descriptor(
            "NetworkRuntimeFailedRows",
            field::NETWORK_RUNTIME_FAILED_ROWS,
        ),
        field_descriptor(
            "NetworkRuntimeManualRequiredRows",
            field::NETWORK_RUNTIME_MANUAL_REQUIRED_ROWS,
        ),
        field_descriptor(
            "NetworkRuntimeObservedRows",
            field::NETWORK_RUNTIME_OBSERVED_ROWS,
        ),
        field_descriptor(
            "NetworkRuntimePublishReports",
            field::NETWORK_RUNTIME_PUBLISH_REPORTS,
        ),
        field_descriptor(
            "NetworkRuntimeStoredEvents",
            field::NETWORK_RUNTIME_STORED_EVENTS,
        ),
        field_descriptor(
            "NetworkRuntimeStreamedEvents",
            field::NETWORK_RUNTIME_STREAMED_EVENTS,
        ),
        field_descriptor(
            "NetworkRemoteDeliveryStatus",
            field::NETWORK_REMOTE_DELIVERY_STATUS,
        ),
        field_descriptor(
            "NetworkWindowsFirewallLabStatus",
            network_flow::FIELD_NETWORK_WINDOWS_FIREWALL_LAB_STATUS,
        ),
        field_descriptor(
            "NetworkWindowsWfpGateStatus",
            network_flow::FIELD_NETWORK_WINDOWS_WFP_GATE_STATUS,
        ),
        field_descriptor("Online", field::ONLINE),
    ]
}

fn field_descriptors_parent_assistant() -> Vec<ProtocolLiteralDescriptor<&'static str>> {
    vec![
        field_descriptor(
            "ParentAssistantAnswerState",
            field::PARENT_ASSISTANT_ANSWER_STATE,
        ),
        field_descriptor(
            "ParentAssistantApiAuthorizationState",
            field::PARENT_ASSISTANT_API_AUTHORIZATION_STATE,
        ),
        field_descriptor(
            "ParentAssistantApiCustodyLabel",
            field::PARENT_ASSISTANT_API_CUSTODY_LABEL,
        ),
        field_descriptor(
            "ParentAssistantApiDeletionState",
            field::PARENT_ASSISTANT_API_DELETION_STATE,
        ),
        field_descriptor(
            "ParentAssistantApiProviderBoundary",
            field::PARENT_ASSISTANT_API_PROVIDER_BOUNDARY,
        ),
        field_descriptor(
            "ParentAssistantApiRetentionState",
            field::PARENT_ASSISTANT_API_RETENTION_STATE,
        ),
        field_descriptor(
            "ParentAssistantCitationCount",
            field::PARENT_ASSISTANT_CITATION_COUNT,
        ),
        field_descriptor(
            "ParentAssistantEvidenceSummary",
            field::PARENT_ASSISTANT_EVIDENCE_SUMMARY,
        ),
        field_descriptor(
            "ParentAssistantProviderRoute",
            field::PARENT_ASSISTANT_PROVIDER_ROUTE,
        ),
        field_descriptor(
            "ParentAssistantRequestId",
            field::PARENT_ASSISTANT_REQUEST_ID,
        ),
        field_descriptor(
            "ParentAssistantQuickActionId",
            field::PARENT_ASSISTANT_QUICK_ACTION_ID,
        ),
        field_descriptor(
            "ParentAssistantPromptTemplateId",
            field::PARENT_ASSISTANT_PROMPT_TEMPLATE_ID,
        ),
        field_descriptor(
            "ParentAssistantStarterCategory",
            field::PARENT_ASSISTANT_STARTER_CATEGORY,
        ),
        field_descriptor(
            "ParentAssistantInputText",
            field::PARENT_ASSISTANT_INPUT_TEXT,
        ),
        field_descriptor(
            "ParentAssistantInputSource",
            field::PARENT_ASSISTANT_INPUT_SOURCE,
        ),
    ]
}

fn field_descriptors_misc() -> Vec<ProtocolLiteralDescriptor<&'static str>> {
    vec![
        field_descriptor("RangeEnd", field::RANGE_END),
        field_descriptor("RangeStart", field::RANGE_START),
        field_descriptor("Reason", field::REASON),
        field_descriptor("RequestedAt", field::REQUESTED_AT),
        field_descriptor("Returned", field::RETURNED),
        field_descriptor("ScopeKind", field::SCOPE_KIND),
        field_descriptor("Transport", field::TRANSPORT),
    ]
}
