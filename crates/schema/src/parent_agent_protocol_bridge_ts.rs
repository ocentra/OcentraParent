use crate::schema_result_or_unreachable;
use ocentra_parent_agent_protocol::{
    activity::ActivityEvidenceKind,
    activity_surface::{
        ActivityReadModelState, ActivityReportCustodyLabel, ActivityReportFrequency,
        ActivityReportSectionKind, ActivityReportSourceLabel,
        ActivityReportSourceReachabilityState, ActivitySavedReportState, ActivitySurfaceScopeKind,
    },
    browser::{BrowserCapabilityStatus, BrowserCustodyLabel, BrowserRuntimePhase},
    browser_managed::BrowserQueryVisibilityLabel,
    constants::{
        delimiter, field, lan_pairing, local_ai_runtime, network_flow, peer,
        tracking_retention_settings_write, value,
    },
    network_android_vpn_service_gate_status::{
        NetworkAndroidVpnServiceGateBoundaryReason,
        NetworkAndroidVpnServiceGateCapabilityStatusState,
        NetworkAndroidVpnServiceGateRequiredArtifact, NetworkAndroidVpnServiceGateStatusState,
    },
    network_apple_network_extension_gate_status::{
        NetworkAppleNetworkExtensionGateBoundaryReason,
        NetworkAppleNetworkExtensionGateCapabilityStatusState,
        NetworkAppleNetworkExtensionGateRequiredArtifact,
        NetworkAppleNetworkExtensionGateStatusState, NetworkAppleNetworkExtensionPlatformStatus,
    },
    network_flow::{
        NetworkActivityKind, NetworkAiAdvisoryState, NetworkAuditOutcome,
        NetworkDomainAttributionKind, NetworkEnforcementMode, NetworkEnforcementResultStatus,
        NetworkEvidenceGrade, NetworkLiveCaptureExecutionStatusState,
        NetworkLiveCaptureProofStatusState, NetworkLiveCaptureStatusPlatform,
        NetworkPolicyDecisionAction, NetworkPortalUpdateKind, NetworkRawCaptureStorageStatusState,
        NetworkRemoteDeliveryCrossProcessCustodyReadinessState,
        NetworkRemoteDeliveryExternalCrossProcessTransportState,
        NetworkRemoteDeliveryProviderChildReadinessState, NetworkRemoteDeliveryStatusState,
        NetworkRemoteDeliveryTransportDispatchState, NetworkRuntimePhase,
    },
    network_linux_nftables_lab_status::{
        NetworkLinuxNftablesLabCommandStatusKind, NetworkLinuxNftablesLabStatusState,
    },
    network_windows_firewall_lab_status::{
        NetworkWindowsFirewallLabCommandStatusKind, NetworkWindowsFirewallLabStatusState,
    },
    network_windows_wfp_gate_status::{
        NetworkWindowsWfpGateCapabilityStatusState, NetworkWindowsWfpGateStatusState,
    },
    tracking::{
        config_update_event::{TrackingConfigEffectiveState, TrackingConfigUpdateResponseState},
        retention_settings_write_command::{
            TrackingConfigAckState, TrackingDeleteAfterAlertResolutionState,
            TrackingDurableSettingsPersistenceState, TrackingExecutionClaimState,
            TrackingParentExportState, TrackingRemoteAiState, TrackingRemoteSyncState,
        },
    },
    transport::{
        AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer, AgentPeerRole, AgentRoute,
    },
    ACTIVITY_SURFACE_SCHEMA_VERSION, AGENT_PROTOCOL_SCHEMA_VERSION,
};
use serde::Serialize;

const PARENT_AGENT_MESSAGE_ID_PREFIX: &str = "cmd-";

struct ProtocolLiteralDescriptor<T> {
    key: &'static str,
    value: T,
}

struct ProtocolBridgeNames {
    runtime_const: &'static str,
    payload_value_type: &'static str,
    payload_type: &'static str,
    field_const: &'static str,
    field_type: &'static str,
    delimiter_const: &'static str,
    delimiter_type: &'static str,
    peer_role_const: &'static str,
    peer_role_type: &'static str,
    route_const: &'static str,
    route_type: &'static str,
    peer_type: &'static str,
    target_type: &'static str,
    peer_defaults_const: &'static str,
    target_defaults_const: &'static str,
    command_envelope_type: &'static str,
    command_decoder_fn: &'static str,
    event_envelope_type: &'static str,
    event_decoder_fn: &'static str,
    log_level_const: &'static str,
    log_level_type: &'static str,
    message_id_decoder_fn: &'static str,
    timestamp_decoder_fn: &'static str,
    serialized_message_decoder_fn: &'static str,
    log_text_guard_fn: &'static str,
    command_const: &'static str,
    command_type: &'static str,
    event_const: &'static str,
    event_type: &'static str,
    lan_household_action_kind_const: &'static str,
    lan_household_action_kind_type: &'static str,
    lan_intent_kind_const: &'static str,
    lan_intent_kind_type: &'static str,
    lan_parent_authority_const: &'static str,
    lan_parent_authority_type: &'static str,
    lan_household_device_kind_values_const: &'static str,
    lan_household_device_kind_type: &'static str,
    lan_household_action_device_kind_field_const: &'static str,
    network_remote_delivery_status_refs_const: &'static str,
    network_live_capture_status_refs_const: &'static str,
    network_linux_nftables_lab_status_refs_const: &'static str,
    network_windows_firewall_lab_status_refs_const: &'static str,
    network_windows_wfp_gate_status_refs_const: &'static str,
    network_android_vpn_service_gate_status_refs_const: &'static str,
    network_apple_network_extension_gate_status_refs_const: &'static str,
    tracking_retention_settings_write_defaults_const: &'static str,
    tracking_delete_after_alert_resolution_state_const: &'static str,
    tracking_parent_export_state_const: &'static str,
    tracking_remote_sync_state_const: &'static str,
    tracking_remote_ai_state_const: &'static str,
    tracking_durable_settings_persistence_state_const: &'static str,
    tracking_config_ack_state_const: &'static str,
    tracking_execution_claim_state_const: &'static str,
    tracking_config_update_response_state_const: &'static str,
    tracking_effective_state_const: &'static str,
    tracking_retention_settings_write_result_type: &'static str,
    tracking_retention_settings_write_result_decoder_fn: &'static str,
    tracking_retention_settings_write_result_schema_const: &'static str,
}

pub(crate) fn parent_agent_protocol_bridge_typescript() -> String {
    protocol_bridge_typescript(
        parent_bridge_names(),
        Some(parent_route_event_id_typescript()),
    )
}

pub(crate) fn parent_agent_protocol_domain_contracts_typescript() -> String {
    protocol_bridge_typescript(
        parent_bridge_names(),
        Some(standalone_parent_route_event_id_typescript()),
    )
}

pub(crate) fn generated_portal_agent_protocol_bridge_typescript() -> String {
    protocol_bridge_typescript(generated_portal_bridge_names(), None)
}

fn protocol_bridge_typescript(
    names: ProtocolBridgeNames,
    event_id_types: Option<String>,
) -> String {
    let mut sections = vec![
        runtime_typescript(&names),
        transport_typescript(&names),
        field_typescript(&names),
        runtime_event_contract_typescript(&names),
        network_status_ref_typescript(&names),
        network_status_contract_typescript(&names),
        tracking_retention_settings_write_typescript(&names),
        delimiter_typescript(&names),
        literal_typescript(
            names.command_const,
            names.command_type,
            &command_descriptors(),
        ),
        literal_typescript(names.event_const, names.event_type, &event_descriptors()),
        activity_surface_contract_typescript(&names),
        lan_value_typescript(&names),
    ];
    if let Some(event_id_types) = event_id_types {
        sections.push(event_id_types);
    }
    sections.join(" ")
}

fn parent_bridge_names() -> ProtocolBridgeNames {
    ProtocolBridgeNames {
        runtime_const: "ParentAgentProtocolRuntime",
        payload_value_type: "ParentAgentProtocolPayloadValue",
        payload_type: "ParentAgentProtocolPayload",
        field_const: "ParentAgentProtocolField",
        field_type: "ParentAgentProtocolFieldName",
        delimiter_const: "ParentAgentProtocolDelimiter",
        delimiter_type: "ParentAgentProtocolDelimiter",
        peer_role_const: "ParentAgentPeerRole",
        peer_role_type: "ParentAgentPeerRole",
        route_const: "ParentAgentRoute",
        route_type: "ParentAgentRoute",
        peer_type: "ParentAgentPeer",
        target_type: "ParentAgentMessageTarget",
        peer_defaults_const: "ParentAgentPeerDefaults",
        target_defaults_const: "ParentAgentTargetDefaults",
        command_envelope_type: "ParentAgentCommandEnvelope",
        command_decoder_fn: "decodeParentAgentCommandEnvelope",
        event_envelope_type: "ParentAgentEventEnvelope",
        event_decoder_fn: "decodeParentAgentEventEnvelope",
        log_level_const: "ParentAgentProtocolLogLevel",
        log_level_type: "ParentAgentProtocolLogLevel",
        message_id_decoder_fn: "decodeParentAgentMessageId",
        timestamp_decoder_fn: "decodeParentAgentTimestamp",
        serialized_message_decoder_fn: "decodeParentSerializedAgentMessage",
        log_text_guard_fn: "isParentAgentProtocolLogText",
        command_const: "ParentAgentCommand",
        command_type: "ParentAgentCommandName",
        event_const: "ParentAgentEvent",
        event_type: "ParentAgentEventName",
        lan_household_action_kind_const: "ParentAgentLanHouseholdActionKind",
        lan_household_action_kind_type: "ParentAgentLanHouseholdActionKind",
        lan_intent_kind_const: "ParentAgentLanIntentKind",
        lan_intent_kind_type: "ParentAgentLanIntentKind",
        lan_parent_authority_const: "ParentAgentLanParentAuthority",
        lan_parent_authority_type: "ParentAgentLanParentAuthority",
        lan_household_device_kind_values_const: "ParentAgentLanHouseholdDeviceKindValues",
        lan_household_device_kind_type: "ParentAgentLanHouseholdDeviceKind",
        lan_household_action_device_kind_field_const:
            "ParentAgentLanHouseholdActionDeviceKindField",
        network_remote_delivery_status_refs_const: "ParentAgentNetworkRemoteDeliveryStatusRefs",
        network_live_capture_status_refs_const: "ParentAgentNetworkLiveCaptureStatusRefs",
        network_linux_nftables_lab_status_refs_const:
            "ParentAgentNetworkLinuxNftablesLabStatusRefs",
        network_windows_firewall_lab_status_refs_const:
            "ParentAgentNetworkWindowsFirewallLabStatusRefs",
        network_windows_wfp_gate_status_refs_const: "ParentAgentNetworkWindowsWfpGateStatusRefs",
        network_android_vpn_service_gate_status_refs_const:
            "ParentAgentNetworkAndroidVpnServiceGateStatusRefs",
        network_apple_network_extension_gate_status_refs_const:
            "ParentAgentNetworkAppleNetworkExtensionGateStatusRefs",
        tracking_retention_settings_write_defaults_const:
            "ParentAgentTrackingRetentionSettingsWriteDefaults",
        tracking_delete_after_alert_resolution_state_const:
            "ParentAgentTrackingDeleteAfterAlertResolutionState",
        tracking_parent_export_state_const: "ParentAgentTrackingParentExportState",
        tracking_remote_sync_state_const: "ParentAgentTrackingRemoteSyncState",
        tracking_remote_ai_state_const: "ParentAgentTrackingRemoteAiState",
        tracking_durable_settings_persistence_state_const:
            "ParentAgentTrackingDurableSettingsPersistenceState",
        tracking_config_ack_state_const: "ParentAgentTrackingConfigAckState",
        tracking_execution_claim_state_const: "ParentAgentTrackingExecutionClaimState",
        tracking_config_update_response_state_const: "ParentAgentTrackingConfigUpdateResponseState",
        tracking_effective_state_const: "ParentAgentTrackingEffectiveState",
        tracking_retention_settings_write_result_type:
            "ParentAgentTrackingRetentionSettingsWriteResult",
        tracking_retention_settings_write_result_decoder_fn:
            "decodeParentAgentTrackingRetentionSettingsWriteResult",
        tracking_retention_settings_write_result_schema_const:
            "ParentAgentTrackingRetentionSettingsWriteResultSchema",
    }
}

fn generated_portal_bridge_names() -> ProtocolBridgeNames {
    ProtocolBridgeNames {
        runtime_const: "GeneratedPortalAgentProtocolRuntime",
        payload_value_type: "GeneratedPortalAgentProtocolPayloadValue",
        payload_type: "GeneratedPortalAgentProtocolPayload",
        field_const: "GeneratedPortalAgentProtocolField",
        field_type: "GeneratedPortalAgentProtocolFieldName",
        delimiter_const: "GeneratedPortalAgentProtocolDelimiter",
        delimiter_type: "GeneratedPortalAgentProtocolDelimiter",
        peer_role_const: "GeneratedPortalAgentPeerRole",
        peer_role_type: "GeneratedPortalAgentPeerRole",
        route_const: "GeneratedPortalAgentRoute",
        route_type: "GeneratedPortalAgentRoute",
        peer_type: "GeneratedPortalAgentPeer",
        target_type: "GeneratedPortalAgentMessageTarget",
        peer_defaults_const: "GeneratedPortalAgentPeerDefaults",
        target_defaults_const: "GeneratedPortalAgentTargetDefaults",
        command_envelope_type: "GeneratedPortalAgentCommandEnvelope",
        command_decoder_fn: "decodeGeneratedPortalAgentCommandEnvelope",
        event_envelope_type: "GeneratedPortalAgentEventEnvelope",
        event_decoder_fn: "decodeGeneratedPortalAgentEventEnvelope",
        log_level_const: "GeneratedPortalAgentProtocolLogLevel",
        log_level_type: "GeneratedPortalAgentProtocolLogLevel",
        message_id_decoder_fn: "decodeGeneratedPortalAgentMessageId",
        timestamp_decoder_fn: "decodeGeneratedPortalAgentTimestamp",
        serialized_message_decoder_fn: "decodeGeneratedPortalSerializedAgentMessage",
        log_text_guard_fn: "isGeneratedPortalAgentProtocolLogText",
        command_const: "GeneratedPortalAgentCommand",
        command_type: "GeneratedPortalAgentCommandName",
        event_const: "GeneratedPortalAgentEvent",
        event_type: "GeneratedPortalAgentEventName",
        lan_household_action_kind_const: "GeneratedPortalAgentLanHouseholdActionKind",
        lan_household_action_kind_type: "GeneratedPortalAgentLanHouseholdActionKind",
        lan_intent_kind_const: "GeneratedPortalAgentLanIntentKind",
        lan_intent_kind_type: "GeneratedPortalAgentLanIntentKind",
        lan_parent_authority_const: "GeneratedPortalAgentLanParentAuthority",
        lan_parent_authority_type: "GeneratedPortalAgentLanParentAuthority",
        lan_household_device_kind_values_const: "GeneratedPortalAgentLanHouseholdDeviceKindValues",
        lan_household_device_kind_type: "GeneratedPortalAgentLanHouseholdDeviceKind",
        lan_household_action_device_kind_field_const:
            "GeneratedPortalAgentLanHouseholdActionDeviceKindField",
        network_remote_delivery_status_refs_const:
            "GeneratedPortalAgentNetworkRemoteDeliveryStatusRefs",
        network_live_capture_status_refs_const: "GeneratedPortalAgentNetworkLiveCaptureStatusRefs",
        network_linux_nftables_lab_status_refs_const:
            "GeneratedPortalAgentNetworkLinuxNftablesLabStatusRefs",
        network_windows_firewall_lab_status_refs_const:
            "GeneratedPortalAgentNetworkWindowsFirewallLabStatusRefs",
        network_windows_wfp_gate_status_refs_const:
            "GeneratedPortalAgentNetworkWindowsWfpGateStatusRefs",
        network_android_vpn_service_gate_status_refs_const:
            "GeneratedPortalAgentNetworkAndroidVpnServiceGateStatusRefs",
        network_apple_network_extension_gate_status_refs_const:
            "GeneratedPortalAgentNetworkAppleNetworkExtensionGateStatusRefs",
        tracking_retention_settings_write_defaults_const:
            "GeneratedPortalAgentTrackingRetentionSettingsWriteDefaults",
        tracking_delete_after_alert_resolution_state_const:
            "GeneratedPortalAgentTrackingDeleteAfterAlertResolutionState",
        tracking_parent_export_state_const: "GeneratedPortalAgentTrackingParentExportState",
        tracking_remote_sync_state_const: "GeneratedPortalAgentTrackingRemoteSyncState",
        tracking_remote_ai_state_const: "GeneratedPortalAgentTrackingRemoteAiState",
        tracking_durable_settings_persistence_state_const:
            "GeneratedPortalAgentTrackingDurableSettingsPersistenceState",
        tracking_config_ack_state_const: "GeneratedPortalAgentTrackingConfigAckState",
        tracking_execution_claim_state_const: "GeneratedPortalAgentTrackingExecutionClaimState",
        tracking_config_update_response_state_const:
            "GeneratedPortalAgentTrackingConfigUpdateResponseState",
        tracking_effective_state_const: "GeneratedPortalAgentTrackingEffectiveState",
        tracking_retention_settings_write_result_type:
            "GeneratedPortalAgentTrackingRetentionSettingsWriteResult",
        tracking_retention_settings_write_result_decoder_fn:
            "decodeGeneratedPortalAgentTrackingRetentionSettingsWriteResult",
        tracking_retention_settings_write_result_schema_const:
            "GeneratedPortalAgentTrackingRetentionSettingsWriteResultSchema",
    }
}

fn runtime_typescript(names: &ProtocolBridgeNames) -> String {
    format!(
        "export const {} = {{ SchemaVersion: {}, MessageIdPrefix: {} }} as const; export type {} = string | number | boolean | null; export type {} = Readonly<Record<string, {}>>;",
        names.runtime_const,
        AGENT_PROTOCOL_SCHEMA_VERSION,
        json_literal(&PARENT_AGENT_MESSAGE_ID_PREFIX),
        names.payload_value_type,
        names.payload_type,
        names.payload_value_type
    )
}

fn delimiter_typescript(names: &ProtocolBridgeNames) -> String {
    format!(
        "export const {} = {{ List: {}, EventIdSuffix: {} }} as const; export type {} = (typeof {})[keyof typeof {}];",
        names.delimiter_const,
        json_literal(&delimiter::LIST),
        json_literal(&delimiter::HYPHEN),
        names.delimiter_type,
        names.delimiter_const,
        names.delimiter_const
    )
}

fn transport_typescript(names: &ProtocolBridgeNames) -> String {
    [
        literal_typescript(
            names.peer_role_const,
            names.peer_role_type,
            &peer_role_descriptors(),
        ),
        literal_typescript(names.route_const, names.route_type, &route_descriptors()),
        peer_target_typescript(names),
        const_object_typescript(names.peer_defaults_const, &peer_default_descriptors()),
        const_object_typescript(names.target_defaults_const, &target_default_descriptors()),
        command_envelope_typescript(names),
        log_level_typescript(names),
        event_envelope_typescript(names),
        primitive_decoders_typescript(names),
    ]
    .join(" ")
}

fn field_typescript(names: &ProtocolBridgeNames) -> String {
    format!(
        "{} export type {} = (typeof {})[keyof typeof {}];",
        const_object_typescript(names.field_const, &field_descriptors()),
        names.field_type,
        names.field_const,
        names.field_const
    )
}

fn runtime_event_contract_typescript(names: &ProtocolBridgeNames) -> String {
    let prefix = bridge_prefix(names);
    let browser_event_type_const = format!("{prefix}BrowserRuntimeEventType");
    let browser_event_type_type = browser_event_type_const.clone();
    let browser_phase_const = format!("{prefix}BrowserRuntimePhase");
    let browser_phase_type = browser_phase_const.clone();
    let browser_capability_status_const = format!("{prefix}BrowserRuntimeCapabilityStatus");
    let browser_capability_status_type = browser_capability_status_const.clone();
    let browser_custody_label_const = format!("{prefix}BrowserRuntimeCustodyLabel");
    let browser_custody_label_type = browser_custody_label_const.clone();
    let browser_query_visibility_const = format!("{prefix}BrowserRuntimeQueryVisibility");
    let browser_query_visibility_type = browser_query_visibility_const.clone();
    let network_event_type_const = format!("{prefix}NetworkRuntimeEventType");
    let network_event_type_type = network_event_type_const.clone();
    let network_evidence_grade_const = format!("{prefix}NetworkEvidenceGrade");
    let network_evidence_grade_type = network_evidence_grade_const.clone();
    let network_domain_attribution_kind_const = format!("{prefix}NetworkDomainAttributionKind");
    let network_domain_attribution_kind_type = network_domain_attribution_kind_const.clone();
    let network_activity_kind_const = format!("{prefix}NetworkRuntimeActivityKind");
    let network_activity_kind_type = network_activity_kind_const.clone();
    let network_ai_advisory_state_const = format!("{prefix}NetworkAiAdvisoryState");
    let network_ai_advisory_state_type = network_ai_advisory_state_const.clone();
    let network_policy_decision_action_const = format!("{prefix}NetworkPolicyDecisionAction");
    let network_policy_decision_action_type = network_policy_decision_action_const.clone();
    let network_enforcement_mode_const = format!("{prefix}NetworkEnforcementMode");
    let network_enforcement_mode_type = network_enforcement_mode_const.clone();
    let network_enforcement_result_status_const = format!("{prefix}NetworkEnforcementResultStatus");
    let network_enforcement_result_status_type = network_enforcement_result_status_const.clone();
    let network_audit_outcome_const = format!("{prefix}NetworkAuditOutcome");
    let network_audit_outcome_type = network_audit_outcome_const.clone();
    let network_portal_update_kind_const = format!("{prefix}NetworkPortalUpdateKind");
    let network_portal_update_kind_type = network_portal_update_kind_const.clone();

    [
        literal_typescript(
            &browser_event_type_const,
            &browser_event_type_type,
            &browser_runtime_event_type_descriptors(),
        ),
        literal_typescript(
            &browser_phase_const,
            &browser_phase_type,
            &browser_runtime_phase_descriptors(),
        ),
        literal_typescript(
            &browser_capability_status_const,
            &browser_capability_status_type,
            &browser_capability_status_descriptors(),
        ),
        literal_typescript(
            &browser_custody_label_const,
            &browser_custody_label_type,
            &browser_custody_label_descriptors(),
        ),
        literal_typescript(
            &browser_query_visibility_const,
            &browser_query_visibility_type,
            &browser_query_visibility_descriptors(),
        ),
        browser_runtime_contract_decoders_typescript(
            names,
            prefix,
            &browser_event_type_const,
            &browser_event_type_type,
            &browser_phase_const,
            &browser_phase_type,
            &browser_capability_status_const,
            &browser_capability_status_type,
            &browser_custody_label_const,
            &browser_custody_label_type,
            &browser_query_visibility_const,
            &browser_query_visibility_type,
        ),
        literal_typescript(
            &network_event_type_const,
            &network_event_type_type,
            &network_runtime_event_type_descriptors(),
        ),
        literal_typescript(
            &network_evidence_grade_const,
            &network_evidence_grade_type,
            &network_evidence_grade_descriptors(),
        ),
        literal_typescript(
            &network_domain_attribution_kind_const,
            &network_domain_attribution_kind_type,
            &network_domain_attribution_kind_descriptors(),
        ),
        literal_typescript(
            &network_activity_kind_const,
            &network_activity_kind_type,
            &network_activity_kind_descriptors(),
        ),
        literal_typescript(
            &network_ai_advisory_state_const,
            &network_ai_advisory_state_type,
            &network_ai_advisory_state_descriptors(),
        ),
        literal_typescript(
            &network_policy_decision_action_const,
            &network_policy_decision_action_type,
            &network_policy_decision_action_descriptors(),
        ),
        literal_typescript(
            &network_enforcement_mode_const,
            &network_enforcement_mode_type,
            &network_enforcement_mode_descriptors(),
        ),
        literal_typescript(
            &network_enforcement_result_status_const,
            &network_enforcement_result_status_type,
            &network_enforcement_result_status_descriptors(),
        ),
        literal_typescript(
            &network_audit_outcome_const,
            &network_audit_outcome_type,
            &network_audit_outcome_descriptors(),
        ),
        literal_typescript(
            &network_portal_update_kind_const,
            &network_portal_update_kind_type,
            &network_portal_update_kind_descriptors(),
        ),
        network_runtime_contract_decoders_typescript(
            names,
            prefix,
            &network_event_type_const,
            &network_event_type_type,
            &network_evidence_grade_const,
            &network_evidence_grade_type,
            &network_domain_attribution_kind_const,
            &network_domain_attribution_kind_type,
            &network_activity_kind_const,
            &network_activity_kind_type,
            &network_ai_advisory_state_const,
            &network_ai_advisory_state_type,
            &network_policy_decision_action_const,
            &network_policy_decision_action_type,
            &network_enforcement_mode_const,
            &network_enforcement_mode_type,
            &network_enforcement_result_status_const,
            &network_enforcement_result_status_type,
            &network_audit_outcome_const,
            &network_audit_outcome_type,
            &network_portal_update_kind_const,
            &network_portal_update_kind_type,
        ),
    ]
    .join(" ")
}

fn bridge_prefix(names: &ProtocolBridgeNames) -> &str {
    names
        .runtime_const
        .strip_suffix("ProtocolRuntime")
        .unwrap_or(names.runtime_const)
}

fn browser_runtime_contract_decoders_typescript(
    names: &ProtocolBridgeNames,
    prefix: &str,
    browser_event_type_const: &str,
    browser_event_type_type: &str,
    browser_phase_const: &str,
    browser_phase_type: &str,
    browser_capability_status_const: &str,
    browser_capability_status_type: &str,
    browser_custody_label_const: &str,
    browser_custody_label_type: &str,
    browser_query_visibility_const: &str,
    browser_query_visibility_type: &str,
) -> String {
    let payload_type = format!("{prefix}BrowserRuntimeEventPayload");
    let entry_type = format!("{prefix}BrowserRuntimeEventChainEntry");
    let stream_type = format!("{prefix}BrowserRuntimeEventChainStream");
    let action_intent_candidate_type = format!("{prefix}BrowserRuntimeActionIntentCandidate");
    let payload_decoder_fn = format!("decode{prefix}BrowserRuntimeEventPayload");
    let entry_decoder_fn = format!("decode{prefix}BrowserRuntimeEventChainEntry");
    let stream_decoder_fn = format!("decode{prefix}BrowserRuntimeEventChainStream");
    let payload_schema_const = format!("{prefix}BrowserRuntimeEventPayloadSchema");
    let entry_schema_const = format!("{prefix}BrowserRuntimeEventChainEntrySchema");
    let stream_schema_const = format!("{prefix}BrowserRuntimeEventChainStreamSchema");
    let phase_event_type_const = format!("{prefix}BrowserRuntimePhaseEventType");
    let helper_prefix = format!("__{prefix}BrowserRuntime");
    replace_tokens(
        r#"
export type __BROWSER_PAYLOAD_TYPE__ = { readonly phase: __BROWSER_PHASE_TYPE__; readonly sourceRef: string; readonly evidenceRef: string; readonly capabilityStatus: __BROWSER_CAPABILITY_STATUS_TYPE__; readonly custodyLabel: __BROWSER_CUSTODY_LABEL_TYPE__; readonly queryVisibility: __BROWSER_QUERY_VISIBILITY_TYPE__; readonly degradedReason: string | null; readonly journalRef: string | null; readonly aiRequestRef: string | null; readonly aiAnalysisRef: string | null; readonly policyEvaluationRef: string | null; readonly policyDecisionRef: string | null; readonly policyPreviewId: string | null; readonly assistantActionIntentId: string | null; readonly interventionCommandRef: string | null; readonly interventionResultRef: string | null; readonly auditEntryRef: string | null; readonly readModelRef: string | null; readonly previousPhaseRef: string | null; readonly exactUrlClaimed: boolean; readonly aiAuthority: false; readonly policyAuthority: boolean; readonly dryRun: boolean; readonly adapterDispatchClaimed: boolean; readonly interventionCommandAllowed: boolean; readonly observedAt: string; };
export type __BROWSER_ENTRY_TYPE__ = { readonly eventType: __BROWSER_EVENT_TYPE_TYPE__; readonly eventRef: string; readonly payload: __BROWSER_PAYLOAD_TYPE__; };
export type __BROWSER_STREAM_TYPE__ = { readonly observedRows: number; readonly streamedEvents: number; readonly failedRows: number; readonly exactUrlRows: number; readonly manualRequiredRows: number; readonly interventionCommandEvents: number; readonly readModelProjectionEvents: number; readonly actionIntentCandidates: number; readonly actionIntentHandoffCandidates: number; readonly actionIntentHandoffOutboxRefs: readonly string[]; readonly actionIntentHandoffRefs: readonly string[]; readonly actionIntentChildAcceptedRows: number; readonly actionIntentChildCommandRefs: readonly string[]; readonly actionIntentChildAcceptedEventRefs: readonly string[]; readonly actionIntentParentReadModelRefs: readonly string[]; readonly actionIntentDispatchAttempts: 0; readonly actionIntentAdapterExecutions: 0; readonly actionIntentChildInterventionExecutions: 0; readonly actionIntentEnforcementExecutions: 0; readonly socialProviderReceiptBoundaryRows: number; readonly socialProviderDispatchRequiredRows: number; readonly socialProviderManualReceiptRequiredRows: number; readonly socialProviderAttemptRefs: readonly string[]; readonly socialProviderReceiptProofRefs: readonly string[]; readonly socialProviderDurableRows: number; readonly socialProviderDurableResultRefs: readonly string[]; readonly socialProviderDurableStoreRefs: readonly string[]; readonly socialProviderReadModelRefs: readonly string[]; readonly socialProviderSupportStatusRefs: readonly string[]; readonly entries: readonly __BROWSER_ENTRY_TYPE__[]; };
export type __BROWSER_ACTION_INTENT_CANDIDATE_TYPE__ = { readonly eventRef: string; readonly policyPreviewId: string; readonly assistantActionIntentId: string; readonly sourceRef: string; readonly evidenceRef: string; readonly observedAt: string; };
const __BROWSER_PHASE_EVENT_TYPE_CONST__ = { [__BROWSER_PHASE_CONST__.EvidenceObserved]: __BROWSER_EVENT_TYPE_CONST__.EvidenceObserved, [__BROWSER_PHASE_CONST__.EvidenceJournaled]: __BROWSER_EVENT_TYPE_CONST__.EvidenceJournaled, [__BROWSER_PHASE_CONST__.AiAnalysisRequested]: __BROWSER_EVENT_TYPE_CONST__.AiAnalysisRequested, [__BROWSER_PHASE_CONST__.AiAnalysisCompleted]: __BROWSER_EVENT_TYPE_CONST__.AiAnalysisCompleted, [__BROWSER_PHASE_CONST__.PolicyEvaluationRequested]: __BROWSER_EVENT_TYPE_CONST__.PolicyEvaluationRequested, [__BROWSER_PHASE_CONST__.PolicyDecisionCompleted]: __BROWSER_EVENT_TYPE_CONST__.PolicyDecisionCompleted, [__BROWSER_PHASE_CONST__.InterventionCommandIssued]: __BROWSER_EVENT_TYPE_CONST__.InterventionCommandIssued, [__BROWSER_PHASE_CONST__.InterventionResultObserved]: __BROWSER_EVENT_TYPE_CONST__.InterventionResultObserved, [__BROWSER_PHASE_CONST__.AuditEntryCommitted]: __BROWSER_EVENT_TYPE_CONST__.AuditEntryCommitted, [__BROWSER_PHASE_CONST__.ReadModelProjected]: __BROWSER_EVENT_TYPE_CONST__.ReadModelProjected } as const;
function __HELPER_PREFIX__IsRecord(value: unknown): value is Readonly<Record<string, unknown>> { return typeof value === 'object' && value !== null && !Array.isArray(value); }
function __HELPER_PREFIX__ReadString(record: Readonly<Record<string, unknown>>, field: string): string { const value = record[field]; if (typeof value !== 'string' || value.length === 0) { throw new TypeError(`${field} must be a non-empty browser runtime string`); } return value; }
function __HELPER_PREFIX__ReadNullableString(record: Readonly<Record<string, unknown>>, field: string): string | null { const value = record[field]; if (value === null) { return null; } if (typeof value !== 'string' || value.length === 0) { throw new TypeError(`${field} must be a non-empty browser runtime string or null`); } return value; }
function __HELPER_PREFIX__ReadNumber(record: Readonly<Record<string, unknown>>, field: string): number { const value = record[field]; if (typeof value !== 'number' || !Number.isFinite(value)) { throw new TypeError(`${field} must be a finite browser runtime number`); } return value; }
function __HELPER_PREFIX__ReadBoolean(record: Readonly<Record<string, unknown>>, field: string): boolean { const value = record[field]; if (typeof value !== 'boolean') { throw new TypeError(`${field} must be a browser runtime boolean`); } return value; }
function __HELPER_PREFIX__ReadRequiredBoolean<T extends boolean>(record: Readonly<Record<string, unknown>>, field: string, expected: T): T { const value = __HELPER_PREFIX__ReadBoolean(record, field); if (value !== expected) { throw new TypeError(`${field} must be ${expected}`); } return expected; }
function __HELPER_PREFIX__ReadLiteral<T extends string>(record: Readonly<Record<string, unknown>>, field: string, allowed: readonly T[]): T { const value = __HELPER_PREFIX__ReadString(record, field); if (!allowed.includes(value as T)) { throw new TypeError(`${field} is not a Rust-owned browser runtime literal`); } return value as T; }
function __HELPER_PREFIX__ReadStringArray(record: Readonly<Record<string, unknown>>, field: string): readonly string[] { const value = record[field]; if (!Array.isArray(value) || value.some((entry) => typeof entry !== 'string' || entry.length === 0)) { throw new TypeError(`${field} must be a browser runtime string array`); } return value as readonly string[]; }
function __HELPER_PREFIX__PayloadIsHonest(payload: __BROWSER_PAYLOAD_TYPE__): boolean { if (!__HELPER_PREFIX__ContextSupportsExactUrl(payload) && payload.exactUrlClaimed) { return false; } if (!__HELPER_PREFIX__UnavailableContextHasReason(payload)) { return false; } if (!payload.exactUrlClaimed && payload.interventionCommandAllowed) { return false; } if (!__HELPER_PREFIX__DryRunHasNoDispatch(payload)) { return false; } if (payload.adapterDispatchClaimed && !payload.interventionCommandAllowed) { return false; } if (!payload.interventionCommandAllowed) { return payload.interventionCommandRef === null && payload.interventionResultRef === null; } return payload.interventionCommandRef !== null && payload.adapterDispatchClaimed; }
function __HELPER_PREFIX__ContextSupportsExactUrl(payload: __BROWSER_PAYLOAD_TYPE__): boolean { const capabilityAllowsExactUrl = payload.capabilityStatus === __BROWSER_CAPABILITY_STATUS_CONST__.Available || payload.capabilityStatus === __BROWSER_CAPABILITY_STATUS_CONST__.TabListOnly; const queryAllowsExactUrl = payload.queryVisibility === __BROWSER_QUERY_VISIBILITY_CONST__.LiveLocal || payload.queryVisibility === __BROWSER_QUERY_VISIBILITY_CONST__.LiveLan; return capabilityAllowsExactUrl && queryAllowsExactUrl && payload.custodyLabel !== __BROWSER_CUSTODY_LABEL_CONST__.Unavailable; }
function __HELPER_PREFIX__UnavailableContextHasReason(payload: __BROWSER_PAYLOAD_TYPE__): boolean { if (payload.queryVisibility !== __BROWSER_QUERY_VISIBILITY_CONST__.Unavailable && payload.capabilityStatus !== __BROWSER_CAPABILITY_STATUS_CONST__.BridgeMissing && payload.capabilityStatus !== __BROWSER_CAPABILITY_STATUS_CONST__.Stale && payload.capabilityStatus !== __BROWSER_CAPABILITY_STATUS_CONST__.AdapterError) { return true; } return payload.degradedReason !== null; }
function __HELPER_PREFIX__DryRunHasNoDispatch(payload: __BROWSER_PAYLOAD_TYPE__): boolean { if (!payload.dryRun) { return true; } return !payload.adapterDispatchClaimed && !payload.interventionCommandAllowed && payload.interventionCommandRef === null && payload.interventionResultRef === null; }
function __HELPER_PREFIX__ActionIntentCandidatesFromEntries(entries: readonly __BROWSER_ENTRY_TYPE__[]): __BROWSER_ACTION_INTENT_CANDIDATE_TYPE__[] { return entries.flatMap((entry) => { const payload = entry.payload; if (payload.phase !== __BROWSER_PHASE_CONST__.PolicyDecisionCompleted || !payload.dryRun || !payload.policyAuthority || payload.policyPreviewId === null || payload.assistantActionIntentId === null) { return []; } return [{ eventRef: entry.eventRef, policyPreviewId: payload.policyPreviewId, assistantActionIntentId: payload.assistantActionIntentId, sourceRef: payload.sourceRef, evidenceRef: payload.evidenceRef, observedAt: payload.observedAt }]; }); }
function __HELPER_PREFIX__ActionIntentChildStatusIsHonest(stream: __BROWSER_STREAM_TYPE__): boolean { return stream.actionIntentChildCommandRefs.length === stream.actionIntentChildAcceptedRows && stream.actionIntentChildAcceptedEventRefs.length === stream.actionIntentChildAcceptedRows && stream.actionIntentParentReadModelRefs.length === stream.actionIntentChildAcceptedRows; }
function __HELPER_PREFIX__SocialProviderReceiptRefsAreEmpty(stream: __BROWSER_STREAM_TYPE__): boolean { return stream.socialProviderAttemptRefs.length === 0 && stream.socialProviderReceiptProofRefs.length === 0 && stream.socialProviderDurableRows === 0 && stream.socialProviderDurableResultRefs.length === 0 && stream.socialProviderDurableStoreRefs.length === 0 && stream.socialProviderReadModelRefs.length === 0 && stream.socialProviderSupportStatusRefs.length === 0; }
function __HELPER_PREFIX__SocialProviderReceiptStateIsHonest(stream: __BROWSER_STREAM_TYPE__): boolean { if (stream.socialProviderReceiptBoundaryRows !== stream.socialProviderDispatchRequiredRows + stream.socialProviderManualReceiptRequiredRows) { return false; } if (stream.socialProviderDispatchRequiredRows === 0) { return __HELPER_PREFIX__SocialProviderReceiptRefsAreEmpty(stream); } return stream.socialProviderAttemptRefs.length === stream.socialProviderDispatchRequiredRows && stream.socialProviderReceiptProofRefs.length === stream.socialProviderDispatchRequiredRows && stream.socialProviderDurableRows === stream.socialProviderDispatchRequiredRows && stream.socialProviderDurableResultRefs.length === stream.socialProviderDurableRows && stream.socialProviderDurableStoreRefs.length === stream.socialProviderDurableRows && stream.socialProviderReadModelRefs.length === stream.socialProviderDurableRows && stream.socialProviderSupportStatusRefs.length === stream.socialProviderDurableRows; }
function __HELPER_PREFIX__StreamIsHonest(stream: __BROWSER_STREAM_TYPE__): boolean { return stream.streamedEvents === stream.entries.length && stream.actionIntentCandidates >= __HELPER_PREFIX__ActionIntentCandidatesFromEntries(stream.entries).length && stream.actionIntentHandoffCandidates >= stream.actionIntentHandoffOutboxRefs.length && stream.actionIntentHandoffCandidates >= stream.actionIntentHandoffRefs.length && stream.actionIntentHandoffOutboxRefs.length === stream.actionIntentHandoffRefs.length && __HELPER_PREFIX__ActionIntentChildStatusIsHonest(stream) && __HELPER_PREFIX__SocialProviderReceiptStateIsHonest(stream); }
export function __BROWSER_PAYLOAD_DECODER_FN__(value: unknown): __BROWSER_PAYLOAD_TYPE__ { if (!__HELPER_PREFIX__IsRecord(value)) { throw new TypeError('browser runtime payload must be an object'); } const payload: __BROWSER_PAYLOAD_TYPE__ = { phase: __HELPER_PREFIX__ReadLiteral(value, 'phase', Object.values(__BROWSER_PHASE_CONST__)), sourceRef: __HELPER_PREFIX__ReadString(value, 'sourceRef'), evidenceRef: __HELPER_PREFIX__ReadString(value, 'evidenceRef'), capabilityStatus: __HELPER_PREFIX__ReadLiteral(value, 'capabilityStatus', Object.values(__BROWSER_CAPABILITY_STATUS_CONST__)), custodyLabel: __HELPER_PREFIX__ReadLiteral(value, 'custodyLabel', Object.values(__BROWSER_CUSTODY_LABEL_CONST__)), queryVisibility: __HELPER_PREFIX__ReadLiteral(value, 'queryVisibility', Object.values(__BROWSER_QUERY_VISIBILITY_CONST__)), degradedReason: __HELPER_PREFIX__ReadNullableString(value, 'degradedReason'), journalRef: __HELPER_PREFIX__ReadNullableString(value, 'journalRef'), aiRequestRef: __HELPER_PREFIX__ReadNullableString(value, 'aiRequestRef'), aiAnalysisRef: __HELPER_PREFIX__ReadNullableString(value, 'aiAnalysisRef'), policyEvaluationRef: __HELPER_PREFIX__ReadNullableString(value, 'policyEvaluationRef'), policyDecisionRef: __HELPER_PREFIX__ReadNullableString(value, 'policyDecisionRef'), policyPreviewId: __HELPER_PREFIX__ReadNullableString(value, 'policyPreviewId'), assistantActionIntentId: __HELPER_PREFIX__ReadNullableString(value, 'assistantActionIntentId'), interventionCommandRef: __HELPER_PREFIX__ReadNullableString(value, 'interventionCommandRef'), interventionResultRef: __HELPER_PREFIX__ReadNullableString(value, 'interventionResultRef'), auditEntryRef: __HELPER_PREFIX__ReadNullableString(value, 'auditEntryRef'), readModelRef: __HELPER_PREFIX__ReadNullableString(value, 'readModelRef'), previousPhaseRef: __HELPER_PREFIX__ReadNullableString(value, 'previousPhaseRef'), exactUrlClaimed: __HELPER_PREFIX__ReadBoolean(value, 'exactUrlClaimed'), aiAuthority: __HELPER_PREFIX__ReadRequiredBoolean(value, 'aiAuthority', false), policyAuthority: __HELPER_PREFIX__ReadBoolean(value, 'policyAuthority'), dryRun: __HELPER_PREFIX__ReadBoolean(value, 'dryRun'), adapterDispatchClaimed: __HELPER_PREFIX__ReadBoolean(value, 'adapterDispatchClaimed'), interventionCommandAllowed: __HELPER_PREFIX__ReadBoolean(value, 'interventionCommandAllowed'), observedAt: __HELPER_PREFIX__ReadString(value, 'observedAt') }; if (!__HELPER_PREFIX__PayloadIsHonest(payload)) { throw new TypeError('browser runtime payload violates Rust-owned claim boundaries'); } return payload; }
export function __BROWSER_ENTRY_DECODER_FN__(value: unknown): __BROWSER_ENTRY_TYPE__ { if (!__HELPER_PREFIX__IsRecord(value)) { throw new TypeError('browser runtime entry must be an object'); } const entry: __BROWSER_ENTRY_TYPE__ = { eventType: __HELPER_PREFIX__ReadLiteral(value, 'eventType', Object.values(__BROWSER_EVENT_TYPE_CONST__)), eventRef: __HELPER_PREFIX__ReadString(value, 'eventRef'), payload: __BROWSER_PAYLOAD_DECODER_FN__(value['payload']) }; if (__BROWSER_PHASE_EVENT_TYPE_CONST__[entry.payload.phase] !== entry.eventType) { throw new TypeError('browser runtime event type must match payload phase'); } return entry; }
export function __BROWSER_STREAM_DECODER_FN__(value: unknown): __BROWSER_STREAM_TYPE__ { if (!__HELPER_PREFIX__IsRecord(value)) { throw new TypeError('browser runtime stream must be an object'); } const entriesValue = value['entries']; if (!Array.isArray(entriesValue)) { throw new TypeError('entries must be a browser runtime array'); } const stream: __BROWSER_STREAM_TYPE__ = { observedRows: __HELPER_PREFIX__ReadNumber(value, 'observedRows'), streamedEvents: __HELPER_PREFIX__ReadNumber(value, 'streamedEvents'), failedRows: __HELPER_PREFIX__ReadNumber(value, 'failedRows'), exactUrlRows: __HELPER_PREFIX__ReadNumber(value, 'exactUrlRows'), manualRequiredRows: __HELPER_PREFIX__ReadNumber(value, 'manualRequiredRows'), interventionCommandEvents: __HELPER_PREFIX__ReadNumber(value, 'interventionCommandEvents'), readModelProjectionEvents: __HELPER_PREFIX__ReadNumber(value, 'readModelProjectionEvents'), actionIntentCandidates: __HELPER_PREFIX__ReadNumber(value, 'actionIntentCandidates'), actionIntentHandoffCandidates: __HELPER_PREFIX__ReadNumber(value, 'actionIntentHandoffCandidates'), actionIntentHandoffOutboxRefs: __HELPER_PREFIX__ReadStringArray(value, 'actionIntentHandoffOutboxRefs'), actionIntentHandoffRefs: __HELPER_PREFIX__ReadStringArray(value, 'actionIntentHandoffRefs'), actionIntentChildAcceptedRows: __HELPER_PREFIX__ReadNumber(value, 'actionIntentChildAcceptedRows'), actionIntentChildCommandRefs: __HELPER_PREFIX__ReadStringArray(value, 'actionIntentChildCommandRefs'), actionIntentChildAcceptedEventRefs: __HELPER_PREFIX__ReadStringArray(value, 'actionIntentChildAcceptedEventRefs'), actionIntentParentReadModelRefs: __HELPER_PREFIX__ReadStringArray(value, 'actionIntentParentReadModelRefs'), actionIntentDispatchAttempts: __HELPER_PREFIX__ReadRequiredNumber(value, 'actionIntentDispatchAttempts', 0), actionIntentAdapterExecutions: __HELPER_PREFIX__ReadRequiredNumber(value, 'actionIntentAdapterExecutions', 0), actionIntentChildInterventionExecutions: __HELPER_PREFIX__ReadRequiredNumber(value, 'actionIntentChildInterventionExecutions', 0), actionIntentEnforcementExecutions: __HELPER_PREFIX__ReadRequiredNumber(value, 'actionIntentEnforcementExecutions', 0), socialProviderReceiptBoundaryRows: __HELPER_PREFIX__ReadNumber(value, 'socialProviderReceiptBoundaryRows'), socialProviderDispatchRequiredRows: __HELPER_PREFIX__ReadNumber(value, 'socialProviderDispatchRequiredRows'), socialProviderManualReceiptRequiredRows: __HELPER_PREFIX__ReadNumber(value, 'socialProviderManualReceiptRequiredRows'), socialProviderAttemptRefs: __HELPER_PREFIX__ReadStringArray(value, 'socialProviderAttemptRefs'), socialProviderReceiptProofRefs: __HELPER_PREFIX__ReadStringArray(value, 'socialProviderReceiptProofRefs'), socialProviderDurableRows: __HELPER_PREFIX__ReadNumber(value, 'socialProviderDurableRows'), socialProviderDurableResultRefs: __HELPER_PREFIX__ReadStringArray(value, 'socialProviderDurableResultRefs'), socialProviderDurableStoreRefs: __HELPER_PREFIX__ReadStringArray(value, 'socialProviderDurableStoreRefs'), socialProviderReadModelRefs: __HELPER_PREFIX__ReadStringArray(value, 'socialProviderReadModelRefs'), socialProviderSupportStatusRefs: __HELPER_PREFIX__ReadStringArray(value, 'socialProviderSupportStatusRefs'), entries: entriesValue.map((entry) => __BROWSER_ENTRY_DECODER_FN__(entry)) }; if (!__HELPER_PREFIX__StreamIsHonest(stream)) { throw new TypeError('browser runtime stream violates Rust-owned claim boundaries'); } return stream; }
function __HELPER_PREFIX__ReadRequiredNumber<T extends number>(record: Readonly<Record<string, unknown>>, field: string, expected: T): T { const value = __HELPER_PREFIX__ReadNumber(record, field); if (value !== expected) { throw new TypeError(`${field} must be ${expected}`); } return expected; }
export const __BROWSER_PAYLOAD_SCHEMA_CONST__ = { safeParse(value: unknown): { readonly success: true; readonly data: __BROWSER_PAYLOAD_TYPE__ } | { readonly success: false } { try { return { success: true, data: __BROWSER_PAYLOAD_DECODER_FN__(value) }; } catch { return { success: false }; } } } as const;
export const __BROWSER_ENTRY_SCHEMA_CONST__ = { safeParse(value: unknown): { readonly success: true; readonly data: __BROWSER_ENTRY_TYPE__ } | { readonly success: false } { try { return { success: true, data: __BROWSER_ENTRY_DECODER_FN__(value) }; } catch { return { success: false }; } } } as const;
export const __BROWSER_STREAM_SCHEMA_CONST__ = { safeParse(value: unknown): { readonly success: true; readonly data: __BROWSER_STREAM_TYPE__ } | { readonly success: false } { try { return { success: true, data: __BROWSER_STREAM_DECODER_FN__(value) }; } catch { return { success: false }; } } } as const;
"#
        .to_string(),
        &[
            ("__RUNTIME_CONST__", names.runtime_const),
            ("__BROWSER_EVENT_TYPE_CONST__", browser_event_type_const),
            ("__BROWSER_EVENT_TYPE_TYPE__", browser_event_type_type),
            ("__BROWSER_PHASE_CONST__", browser_phase_const),
            ("__BROWSER_PHASE_TYPE__", browser_phase_type),
            (
                "__BROWSER_CAPABILITY_STATUS_CONST__",
                browser_capability_status_const,
            ),
            (
                "__BROWSER_CAPABILITY_STATUS_TYPE__",
                browser_capability_status_type,
            ),
            ("__BROWSER_CUSTODY_LABEL_CONST__", browser_custody_label_const),
            ("__BROWSER_CUSTODY_LABEL_TYPE__", browser_custody_label_type),
            (
                "__BROWSER_QUERY_VISIBILITY_CONST__",
                browser_query_visibility_const,
            ),
            (
                "__BROWSER_QUERY_VISIBILITY_TYPE__",
                browser_query_visibility_type,
            ),
            ("__BROWSER_PAYLOAD_TYPE__", &payload_type),
            ("__BROWSER_ENTRY_TYPE__", &entry_type),
            ("__BROWSER_STREAM_TYPE__", &stream_type),
            (
                "__BROWSER_ACTION_INTENT_CANDIDATE_TYPE__",
                &action_intent_candidate_type,
            ),
            ("__BROWSER_PAYLOAD_DECODER_FN__", &payload_decoder_fn),
            ("__BROWSER_ENTRY_DECODER_FN__", &entry_decoder_fn),
            ("__BROWSER_STREAM_DECODER_FN__", &stream_decoder_fn),
            ("__BROWSER_PAYLOAD_SCHEMA_CONST__", &payload_schema_const),
            ("__BROWSER_ENTRY_SCHEMA_CONST__", &entry_schema_const),
            ("__BROWSER_STREAM_SCHEMA_CONST__", &stream_schema_const),
            ("__BROWSER_PHASE_EVENT_TYPE_CONST__", &phase_event_type_const),
            ("__HELPER_PREFIX__", &helper_prefix),
        ],
    )
}

fn network_runtime_contract_decoders_typescript(
    names: &ProtocolBridgeNames,
    prefix: &str,
    network_event_type_const: &str,
    network_event_type_type: &str,
    network_evidence_grade_const: &str,
    network_evidence_grade_type: &str,
    network_domain_attribution_kind_const: &str,
    network_domain_attribution_kind_type: &str,
    network_activity_kind_const: &str,
    network_activity_kind_type: &str,
    network_ai_advisory_state_const: &str,
    network_ai_advisory_state_type: &str,
    network_policy_decision_action_const: &str,
    network_policy_decision_action_type: &str,
    network_enforcement_mode_const: &str,
    network_enforcement_mode_type: &str,
    network_enforcement_result_status_const: &str,
    network_enforcement_result_status_type: &str,
    network_audit_outcome_const: &str,
    network_audit_outcome_type: &str,
    network_portal_update_kind_const: &str,
    network_portal_update_kind_type: &str,
) -> String {
    let claim_boundary_type = format!("{prefix}NetworkClaimBoundary");
    let flow_observed_type = format!("{prefix}NetworkFlowObservedEvent");
    let domain_observed_type = format!("{prefix}NetworkDomainObservedEvent");
    let activity_classified_type = format!("{prefix}NetworkActivityClassifiedEvent");
    let ai_analysis_requested_type = format!("{prefix}NetworkAiAnalysisRequestedEvent");
    let ai_analysis_completed_type = format!("{prefix}NetworkAiAnalysisCompletedEvent");
    let policy_evaluation_requested_type = format!("{prefix}NetworkPolicyEvaluationRequestedEvent");
    let policy_decision_completed_type = format!("{prefix}NetworkPolicyDecisionCompletedEvent");
    let enforcement_command_issued_type = format!("{prefix}NetworkEnforcementCommandIssuedEvent");
    let enforcement_result_observed_type = format!("{prefix}NetworkEnforcementResultObservedEvent");
    let audit_entry_committed_type = format!("{prefix}NetworkAuditEntryCommittedEvent");
    let portal_read_model_updated_type = format!("{prefix}NetworkPortalReadModelUpdatedEvent");
    let runtime_event_payload_type = format!("{prefix}NetworkRuntimeEventPayload");
    let payload_decoder_fn = format!("decode{prefix}NetworkRuntimeEventPayload");
    let event_type_schema_const = format!("{prefix}NetworkRuntimeEventTypeSchema");
    let helper_prefix = format!("__{prefix}NetworkRuntime");
    replace_tokens(
        r#"
export type __NETWORK_CLAIM_BOUNDARY_TYPE__ = { readonly exactUrlAvailable: boolean; readonly decryptedHttpsPayloadAvailable: boolean; readonly messageContentAvailable: boolean; readonly searchQueryAvailable: boolean; readonly adapterActionExecuted: boolean; };
export type __NETWORK_FLOW_OBSERVED_TYPE__ = { readonly schemaVersion: number; readonly flowEventRef: string; readonly observedAt: string; readonly deviceRef: string; readonly flowEvidenceRef: string; readonly custody: string; readonly evidenceGrade: __NETWORK_EVIDENCE_GRADE_TYPE__; readonly claimBoundary: __NETWORK_CLAIM_BOUNDARY_TYPE__; };
export type __NETWORK_DOMAIN_OBSERVED_TYPE__ = { readonly schemaVersion: number; readonly domainEventRef: string; readonly previousEventRef: string; readonly flowEvidenceRef: string; readonly domainEvidenceRef: string; readonly attribution: __NETWORK_DOMAIN_ATTRIBUTION_KIND_TYPE__; readonly evidenceGrade: __NETWORK_EVIDENCE_GRADE_TYPE__; readonly uncertaintyCodes: readonly string[]; readonly claimBoundary: __NETWORK_CLAIM_BOUNDARY_TYPE__; };
export type __NETWORK_ACTIVITY_CLASSIFIED_TYPE__ = { readonly schemaVersion: number; readonly classificationEventRef: string; readonly previousEventRef: string; readonly evidenceRefs: readonly string[]; readonly activityKind: __NETWORK_ACTIVITY_KIND_TYPE__; readonly confidence: number; readonly evidenceGrade: __NETWORK_EVIDENCE_GRADE_TYPE__; readonly uncertaintyCodes: readonly string[]; };
export type __NETWORK_AI_ANALYSIS_REQUESTED_TYPE__ = { readonly schemaVersion: number; readonly aiRequestRef: string; readonly previousEventRef: string; readonly evidenceRefs: readonly string[]; readonly promptTemplateRef: string; readonly custody: string; readonly rawPacketPayloadIncluded: false; };
export type __NETWORK_AI_ANALYSIS_COMPLETED_TYPE__ = { readonly schemaVersion: number; readonly aiAnalysisRef: string; readonly aiRequestRef: string; readonly previousEventRef: string; readonly advisoryState: __NETWORK_AI_ADVISORY_STATE_TYPE__; readonly evidenceRefs: readonly string[]; readonly unsupportedClaims: readonly string[]; };
export type __NETWORK_POLICY_EVALUATION_REQUESTED_TYPE__ = { readonly schemaVersion: number; readonly policyEvaluationRef: string; readonly previousEventRef: string; readonly evidenceRefs: readonly string[]; readonly aiAnalysisRef: string | null; readonly parentRuleRefs: readonly string[]; readonly dryRun: boolean; };
export type __NETWORK_POLICY_DECISION_COMPLETED_TYPE__ = { readonly schemaVersion: number; readonly policyDecisionRef: string; readonly policyEvaluationRef: string; readonly previousEventRef: string; readonly decisionAction: __NETWORK_POLICY_DECISION_ACTION_TYPE__; readonly evidenceRefs: readonly string[]; readonly parentRuleRefs: readonly string[]; readonly adapterCapabilityRequired: boolean; };
export type __NETWORK_ENFORCEMENT_COMMAND_ISSUED_TYPE__ = { readonly schemaVersion: number; readonly enforcementCommandRef: string; readonly previousEventRef: string; readonly policyDecisionRef: string; readonly adapterCapabilityRef: string; readonly enforcementMode: __NETWORK_ENFORCEMENT_MODE_TYPE__; readonly evidenceRefs: readonly string[]; readonly rollbackRef: string | null; };
export type __NETWORK_ENFORCEMENT_RESULT_OBSERVED_TYPE__ = { readonly schemaVersion: number; readonly enforcementResultRef: string; readonly enforcementCommandRef: string; readonly previousEventRef: string; readonly resultStatus: __NETWORK_ENFORCEMENT_RESULT_STATUS_TYPE__; readonly adapterActionExecuted: false; readonly rollbackRef: string | null; readonly unavailableReasonCode: string | null; };
export type __NETWORK_AUDIT_ENTRY_COMMITTED_TYPE__ = { readonly schemaVersion: number; readonly auditEntryRef: string; readonly previousEventRef: string; readonly policyDecisionRef: string; readonly enforcementCommandRef: string | null; readonly enforcementResultRef: string | null; readonly evidenceRefs: readonly string[]; readonly auditOutcome: __NETWORK_AUDIT_OUTCOME_TYPE__; };
export type __NETWORK_PORTAL_READ_MODEL_UPDATED_TYPE__ = { readonly schemaVersion: number; readonly readModelRef: string; readonly previousEventRef: string; readonly auditEntryRef: string; readonly updateKind: __NETWORK_PORTAL_UPDATE_KIND_TYPE__; readonly visibleManualRequired: boolean; readonly visibleUnavailable: boolean; };
export type __NETWORK_RUNTIME_EVENT_PAYLOAD_TYPE__ = __NETWORK_FLOW_OBSERVED_TYPE__ | __NETWORK_DOMAIN_OBSERVED_TYPE__ | __NETWORK_ACTIVITY_CLASSIFIED_TYPE__ | __NETWORK_AI_ANALYSIS_REQUESTED_TYPE__ | __NETWORK_AI_ANALYSIS_COMPLETED_TYPE__ | __NETWORK_POLICY_EVALUATION_REQUESTED_TYPE__ | __NETWORK_POLICY_DECISION_COMPLETED_TYPE__ | __NETWORK_ENFORCEMENT_COMMAND_ISSUED_TYPE__ | __NETWORK_ENFORCEMENT_RESULT_OBSERVED_TYPE__ | __NETWORK_AUDIT_ENTRY_COMMITTED_TYPE__ | __NETWORK_PORTAL_READ_MODEL_UPDATED_TYPE__;
function __HELPER_PREFIX__IsRecord(value: unknown): value is Readonly<Record<string, unknown>> { return typeof value === 'object' && value !== null && !Array.isArray(value); }
function __HELPER_PREFIX__ReadRecord(value: unknown, label: string): Readonly<Record<string, unknown>> { if (!__HELPER_PREFIX__IsRecord(value)) { throw new TypeError(`${label} must be a network runtime object`); } return value; }
function __HELPER_PREFIX__ReadString(record: Readonly<Record<string, unknown>>, field: string): string { const value = record[field]; if (typeof value !== 'string' || value.length === 0) { throw new TypeError(`${field} must be a non-empty network runtime string`); } return value; }
function __HELPER_PREFIX__ReadNullableString(record: Readonly<Record<string, unknown>>, field: string): string | null { const value = record[field]; if (value === null) { return null; } if (typeof value !== 'string' || value.length === 0) { throw new TypeError(`${field} must be a non-empty network runtime string or null`); } return value; }
function __HELPER_PREFIX__ReadBoolean(record: Readonly<Record<string, unknown>>, field: string): boolean { const value = record[field]; if (typeof value !== 'boolean') { throw new TypeError(`${field} must be a network runtime boolean`); } return value; }
function __HELPER_PREFIX__ReadRequiredBoolean<T extends boolean>(record: Readonly<Record<string, unknown>>, field: string, expected: T): T { const value = __HELPER_PREFIX__ReadBoolean(record, field); if (value !== expected) { throw new TypeError(`${field} must be ${expected}`); } return expected; }
function __HELPER_PREFIX__ReadSchemaVersion(record: Readonly<Record<string, unknown>>): number { const value = record['schemaVersion']; if (value !== __RUNTIME_CONST__.SchemaVersion) { throw new TypeError('schemaVersion is not the Rust-owned agent protocol schema version'); } return __RUNTIME_CONST__.SchemaVersion; }
function __HELPER_PREFIX__ReadLiteral<T extends string>(record: Readonly<Record<string, unknown>>, field: string, allowed: readonly T[]): T { const value = __HELPER_PREFIX__ReadString(record, field); if (!allowed.includes(value as T)) { throw new TypeError(`${field} is not a Rust-owned network runtime literal`); } return value as T; }
function __HELPER_PREFIX__ReadStringArray(record: Readonly<Record<string, unknown>>, field: string): readonly string[] { const value = record[field]; if (!Array.isArray(value) || value.some((entry) => typeof entry !== 'string' || entry.length === 0)) { throw new TypeError(`${field} must be a network runtime string array`); } return value as readonly string[]; }
function __HELPER_PREFIX__ReadNonEmptyStringArray(record: Readonly<Record<string, unknown>>, field: string): readonly string[] { const value = __HELPER_PREFIX__ReadStringArray(record, field); if (value.length === 0) { throw new TypeError(`${field} must be a non-empty network runtime string array`); } return value; }
function __HELPER_PREFIX__ReadConfidence(record: Readonly<Record<string, unknown>>, field: string): number { const value = record[field]; if (typeof value !== 'number' || !Number.isFinite(value) || value < 0 || value > 1) { throw new TypeError(`${field} must be a network runtime confidence from 0 to 1`); } return value; }
function __HELPER_PREFIX__ReadClaimBoundary(value: unknown): __NETWORK_CLAIM_BOUNDARY_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'claimBoundary'); const boundary = { exactUrlAvailable: __HELPER_PREFIX__ReadBoolean(record, 'exactUrlAvailable'), decryptedHttpsPayloadAvailable: __HELPER_PREFIX__ReadBoolean(record, 'decryptedHttpsPayloadAvailable'), messageContentAvailable: __HELPER_PREFIX__ReadBoolean(record, 'messageContentAvailable'), searchQueryAvailable: __HELPER_PREFIX__ReadBoolean(record, 'searchQueryAvailable'), adapterActionExecuted: __HELPER_PREFIX__ReadBoolean(record, 'adapterActionExecuted') }; if (boundary.exactUrlAvailable || boundary.decryptedHttpsPayloadAvailable || boundary.messageContentAvailable || boundary.searchQueryAvailable || boundary.adapterActionExecuted) { throw new TypeError('network runtime claim boundary cannot claim unsupported content or adapter action'); } return boundary; }
function __HELPER_PREFIX__DecodeFlowObserved(value: unknown): __NETWORK_FLOW_OBSERVED_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'network flow observed payload'); return { schemaVersion: __HELPER_PREFIX__ReadSchemaVersion(record), flowEventRef: __HELPER_PREFIX__ReadString(record, 'flowEventRef'), observedAt: __HELPER_PREFIX__ReadString(record, 'observedAt'), deviceRef: __HELPER_PREFIX__ReadString(record, 'deviceRef'), flowEvidenceRef: __HELPER_PREFIX__ReadString(record, 'flowEvidenceRef'), custody: __HELPER_PREFIX__ReadString(record, 'custody'), evidenceGrade: __HELPER_PREFIX__ReadLiteral(record, 'evidenceGrade', Object.values(__NETWORK_EVIDENCE_GRADE_CONST__)), claimBoundary: __HELPER_PREFIX__ReadClaimBoundary(record['claimBoundary']) }; }
function __HELPER_PREFIX__DecodeDomainObserved(value: unknown): __NETWORK_DOMAIN_OBSERVED_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'network domain observed payload'); return { schemaVersion: __HELPER_PREFIX__ReadSchemaVersion(record), domainEventRef: __HELPER_PREFIX__ReadString(record, 'domainEventRef'), previousEventRef: __HELPER_PREFIX__ReadString(record, 'previousEventRef'), flowEvidenceRef: __HELPER_PREFIX__ReadString(record, 'flowEvidenceRef'), domainEvidenceRef: __HELPER_PREFIX__ReadString(record, 'domainEvidenceRef'), attribution: __HELPER_PREFIX__ReadLiteral(record, 'attribution', Object.values(__NETWORK_DOMAIN_ATTRIBUTION_KIND_CONST__)), evidenceGrade: __HELPER_PREFIX__ReadLiteral(record, 'evidenceGrade', Object.values(__NETWORK_EVIDENCE_GRADE_CONST__)), uncertaintyCodes: __HELPER_PREFIX__ReadStringArray(record, 'uncertaintyCodes'), claimBoundary: __HELPER_PREFIX__ReadClaimBoundary(record['claimBoundary']) }; }
function __HELPER_PREFIX__DecodeActivityClassified(value: unknown): __NETWORK_ACTIVITY_CLASSIFIED_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'network activity classified payload'); return { schemaVersion: __HELPER_PREFIX__ReadSchemaVersion(record), classificationEventRef: __HELPER_PREFIX__ReadString(record, 'classificationEventRef'), previousEventRef: __HELPER_PREFIX__ReadString(record, 'previousEventRef'), evidenceRefs: __HELPER_PREFIX__ReadNonEmptyStringArray(record, 'evidenceRefs'), activityKind: __HELPER_PREFIX__ReadLiteral(record, 'activityKind', Object.values(__NETWORK_ACTIVITY_KIND_CONST__)), confidence: __HELPER_PREFIX__ReadConfidence(record, 'confidence'), evidenceGrade: __HELPER_PREFIX__ReadLiteral(record, 'evidenceGrade', Object.values(__NETWORK_EVIDENCE_GRADE_CONST__)), uncertaintyCodes: __HELPER_PREFIX__ReadStringArray(record, 'uncertaintyCodes') }; }
function __HELPER_PREFIX__DecodeAiAnalysisRequested(value: unknown): __NETWORK_AI_ANALYSIS_REQUESTED_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'network AI analysis requested payload'); return { schemaVersion: __HELPER_PREFIX__ReadSchemaVersion(record), aiRequestRef: __HELPER_PREFIX__ReadString(record, 'aiRequestRef'), previousEventRef: __HELPER_PREFIX__ReadString(record, 'previousEventRef'), evidenceRefs: __HELPER_PREFIX__ReadNonEmptyStringArray(record, 'evidenceRefs'), promptTemplateRef: __HELPER_PREFIX__ReadString(record, 'promptTemplateRef'), custody: __HELPER_PREFIX__ReadString(record, 'custody'), rawPacketPayloadIncluded: __HELPER_PREFIX__ReadRequiredBoolean(record, 'rawPacketPayloadIncluded', false) }; }
function __HELPER_PREFIX__DecodeAiAnalysisCompleted(value: unknown): __NETWORK_AI_ANALYSIS_COMPLETED_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'network AI analysis completed payload'); return { schemaVersion: __HELPER_PREFIX__ReadSchemaVersion(record), aiAnalysisRef: __HELPER_PREFIX__ReadString(record, 'aiAnalysisRef'), aiRequestRef: __HELPER_PREFIX__ReadString(record, 'aiRequestRef'), previousEventRef: __HELPER_PREFIX__ReadString(record, 'previousEventRef'), advisoryState: __HELPER_PREFIX__ReadLiteral(record, 'advisoryState', Object.values(__NETWORK_AI_ADVISORY_STATE_CONST__)), evidenceRefs: __HELPER_PREFIX__ReadNonEmptyStringArray(record, 'evidenceRefs'), unsupportedClaims: __HELPER_PREFIX__ReadStringArray(record, 'unsupportedClaims') }; }
function __HELPER_PREFIX__DecodePolicyEvaluationRequested(value: unknown): __NETWORK_POLICY_EVALUATION_REQUESTED_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'network policy evaluation requested payload'); return { schemaVersion: __HELPER_PREFIX__ReadSchemaVersion(record), policyEvaluationRef: __HELPER_PREFIX__ReadString(record, 'policyEvaluationRef'), previousEventRef: __HELPER_PREFIX__ReadString(record, 'previousEventRef'), evidenceRefs: __HELPER_PREFIX__ReadNonEmptyStringArray(record, 'evidenceRefs'), aiAnalysisRef: __HELPER_PREFIX__ReadNullableString(record, 'aiAnalysisRef'), parentRuleRefs: __HELPER_PREFIX__ReadNonEmptyStringArray(record, 'parentRuleRefs'), dryRun: __HELPER_PREFIX__ReadBoolean(record, 'dryRun') }; }
function __HELPER_PREFIX__DecodePolicyDecisionCompleted(value: unknown): __NETWORK_POLICY_DECISION_COMPLETED_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'network policy decision completed payload'); return { schemaVersion: __HELPER_PREFIX__ReadSchemaVersion(record), policyDecisionRef: __HELPER_PREFIX__ReadString(record, 'policyDecisionRef'), policyEvaluationRef: __HELPER_PREFIX__ReadString(record, 'policyEvaluationRef'), previousEventRef: __HELPER_PREFIX__ReadString(record, 'previousEventRef'), decisionAction: __HELPER_PREFIX__ReadLiteral(record, 'decisionAction', Object.values(__NETWORK_POLICY_DECISION_ACTION_CONST__)), evidenceRefs: __HELPER_PREFIX__ReadNonEmptyStringArray(record, 'evidenceRefs'), parentRuleRefs: __HELPER_PREFIX__ReadNonEmptyStringArray(record, 'parentRuleRefs'), adapterCapabilityRequired: __HELPER_PREFIX__ReadBoolean(record, 'adapterCapabilityRequired') }; }
function __HELPER_PREFIX__DecodeEnforcementCommandIssued(value: unknown): __NETWORK_ENFORCEMENT_COMMAND_ISSUED_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'network enforcement command issued payload'); return { schemaVersion: __HELPER_PREFIX__ReadSchemaVersion(record), enforcementCommandRef: __HELPER_PREFIX__ReadString(record, 'enforcementCommandRef'), previousEventRef: __HELPER_PREFIX__ReadString(record, 'previousEventRef'), policyDecisionRef: __HELPER_PREFIX__ReadString(record, 'policyDecisionRef'), adapterCapabilityRef: __HELPER_PREFIX__ReadString(record, 'adapterCapabilityRef'), enforcementMode: __HELPER_PREFIX__ReadLiteral(record, 'enforcementMode', Object.values(__NETWORK_ENFORCEMENT_MODE_CONST__)), evidenceRefs: __HELPER_PREFIX__ReadNonEmptyStringArray(record, 'evidenceRefs'), rollbackRef: __HELPER_PREFIX__ReadNullableString(record, 'rollbackRef') }; }
function __HELPER_PREFIX__DecodeEnforcementResultObserved(value: unknown): __NETWORK_ENFORCEMENT_RESULT_OBSERVED_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'network enforcement result observed payload'); return { schemaVersion: __HELPER_PREFIX__ReadSchemaVersion(record), enforcementResultRef: __HELPER_PREFIX__ReadString(record, 'enforcementResultRef'), enforcementCommandRef: __HELPER_PREFIX__ReadString(record, 'enforcementCommandRef'), previousEventRef: __HELPER_PREFIX__ReadString(record, 'previousEventRef'), resultStatus: __HELPER_PREFIX__ReadLiteral(record, 'resultStatus', Object.values(__NETWORK_ENFORCEMENT_RESULT_STATUS_CONST__)), adapterActionExecuted: __HELPER_PREFIX__ReadRequiredBoolean(record, 'adapterActionExecuted', false), rollbackRef: __HELPER_PREFIX__ReadNullableString(record, 'rollbackRef'), unavailableReasonCode: __HELPER_PREFIX__ReadNullableString(record, 'unavailableReasonCode') }; }
function __HELPER_PREFIX__DecodeAuditEntryCommitted(value: unknown): __NETWORK_AUDIT_ENTRY_COMMITTED_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'network audit entry committed payload'); return { schemaVersion: __HELPER_PREFIX__ReadSchemaVersion(record), auditEntryRef: __HELPER_PREFIX__ReadString(record, 'auditEntryRef'), previousEventRef: __HELPER_PREFIX__ReadString(record, 'previousEventRef'), policyDecisionRef: __HELPER_PREFIX__ReadString(record, 'policyDecisionRef'), enforcementCommandRef: __HELPER_PREFIX__ReadNullableString(record, 'enforcementCommandRef'), enforcementResultRef: __HELPER_PREFIX__ReadNullableString(record, 'enforcementResultRef'), evidenceRefs: __HELPER_PREFIX__ReadNonEmptyStringArray(record, 'evidenceRefs'), auditOutcome: __HELPER_PREFIX__ReadLiteral(record, 'auditOutcome', Object.values(__NETWORK_AUDIT_OUTCOME_CONST__)) }; }
function __HELPER_PREFIX__DecodePortalReadModelUpdated(value: unknown): __NETWORK_PORTAL_READ_MODEL_UPDATED_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'network portal read model updated payload'); return { schemaVersion: __HELPER_PREFIX__ReadSchemaVersion(record), readModelRef: __HELPER_PREFIX__ReadString(record, 'readModelRef'), previousEventRef: __HELPER_PREFIX__ReadString(record, 'previousEventRef'), auditEntryRef: __HELPER_PREFIX__ReadString(record, 'auditEntryRef'), updateKind: __HELPER_PREFIX__ReadLiteral(record, 'updateKind', Object.values(__NETWORK_PORTAL_UPDATE_KIND_CONST__)), visibleManualRequired: __HELPER_PREFIX__ReadBoolean(record, 'visibleManualRequired'), visibleUnavailable: __HELPER_PREFIX__ReadBoolean(record, 'visibleUnavailable') }; }
export function __NETWORK_PAYLOAD_DECODER_FN__(eventType: __NETWORK_EVENT_TYPE_TYPE__, value: unknown): __NETWORK_RUNTIME_EVENT_PAYLOAD_TYPE__ { switch (eventType) { case __NETWORK_EVENT_TYPE_CONST__.NetworkFlowObserved: return __HELPER_PREFIX__DecodeFlowObserved(value); case __NETWORK_EVENT_TYPE_CONST__.NetworkDomainObserved: return __HELPER_PREFIX__DecodeDomainObserved(value); case __NETWORK_EVENT_TYPE_CONST__.NetworkActivityClassified: return __HELPER_PREFIX__DecodeActivityClassified(value); case __NETWORK_EVENT_TYPE_CONST__.AiAnalysisRequested: return __HELPER_PREFIX__DecodeAiAnalysisRequested(value); case __NETWORK_EVENT_TYPE_CONST__.AiAnalysisCompleted: return __HELPER_PREFIX__DecodeAiAnalysisCompleted(value); case __NETWORK_EVENT_TYPE_CONST__.PolicyEvaluationRequested: return __HELPER_PREFIX__DecodePolicyEvaluationRequested(value); case __NETWORK_EVENT_TYPE_CONST__.PolicyDecisionCompleted: return __HELPER_PREFIX__DecodePolicyDecisionCompleted(value); case __NETWORK_EVENT_TYPE_CONST__.EnforcementCommandIssued: return __HELPER_PREFIX__DecodeEnforcementCommandIssued(value); case __NETWORK_EVENT_TYPE_CONST__.EnforcementResultObserved: return __HELPER_PREFIX__DecodeEnforcementResultObserved(value); case __NETWORK_EVENT_TYPE_CONST__.AuditEntryCommitted: return __HELPER_PREFIX__DecodeAuditEntryCommitted(value); case __NETWORK_EVENT_TYPE_CONST__.PortalReadModelUpdated: return __HELPER_PREFIX__DecodePortalReadModelUpdated(value); } }
export const __NETWORK_EVENT_TYPE_SCHEMA_CONST__ = { safeParse(value: unknown): { readonly success: true; readonly data: __NETWORK_EVENT_TYPE_TYPE__ } | { readonly success: false } { if (typeof value === 'string' && (Object.values(__NETWORK_EVENT_TYPE_CONST__) as readonly string[]).includes(value)) { return { success: true, data: value as __NETWORK_EVENT_TYPE_TYPE__ }; } return { success: false }; } } as const;
"#
        .to_string(),
        &[
            ("__RUNTIME_CONST__", names.runtime_const),
            ("__NETWORK_EVENT_TYPE_CONST__", network_event_type_const),
            ("__NETWORK_EVENT_TYPE_TYPE__", network_event_type_type),
            ("__NETWORK_EVIDENCE_GRADE_CONST__", network_evidence_grade_const),
            ("__NETWORK_EVIDENCE_GRADE_TYPE__", network_evidence_grade_type),
            (
                "__NETWORK_DOMAIN_ATTRIBUTION_KIND_CONST__",
                network_domain_attribution_kind_const,
            ),
            (
                "__NETWORK_DOMAIN_ATTRIBUTION_KIND_TYPE__",
                network_domain_attribution_kind_type,
            ),
            ("__NETWORK_ACTIVITY_KIND_CONST__", network_activity_kind_const),
            ("__NETWORK_ACTIVITY_KIND_TYPE__", network_activity_kind_type),
            (
                "__NETWORK_AI_ADVISORY_STATE_CONST__",
                network_ai_advisory_state_const,
            ),
            (
                "__NETWORK_AI_ADVISORY_STATE_TYPE__",
                network_ai_advisory_state_type,
            ),
            (
                "__NETWORK_POLICY_DECISION_ACTION_CONST__",
                network_policy_decision_action_const,
            ),
            (
                "__NETWORK_POLICY_DECISION_ACTION_TYPE__",
                network_policy_decision_action_type,
            ),
            (
                "__NETWORK_ENFORCEMENT_MODE_CONST__",
                network_enforcement_mode_const,
            ),
            (
                "__NETWORK_ENFORCEMENT_MODE_TYPE__",
                network_enforcement_mode_type,
            ),
            (
                "__NETWORK_ENFORCEMENT_RESULT_STATUS_CONST__",
                network_enforcement_result_status_const,
            ),
            (
                "__NETWORK_ENFORCEMENT_RESULT_STATUS_TYPE__",
                network_enforcement_result_status_type,
            ),
            ("__NETWORK_AUDIT_OUTCOME_CONST__", network_audit_outcome_const),
            ("__NETWORK_AUDIT_OUTCOME_TYPE__", network_audit_outcome_type),
            (
                "__NETWORK_PORTAL_UPDATE_KIND_CONST__",
                network_portal_update_kind_const,
            ),
            (
                "__NETWORK_PORTAL_UPDATE_KIND_TYPE__",
                network_portal_update_kind_type,
            ),
            ("__NETWORK_CLAIM_BOUNDARY_TYPE__", &claim_boundary_type),
            ("__NETWORK_FLOW_OBSERVED_TYPE__", &flow_observed_type),
            ("__NETWORK_DOMAIN_OBSERVED_TYPE__", &domain_observed_type),
            (
                "__NETWORK_ACTIVITY_CLASSIFIED_TYPE__",
                &activity_classified_type,
            ),
            (
                "__NETWORK_AI_ANALYSIS_REQUESTED_TYPE__",
                &ai_analysis_requested_type,
            ),
            (
                "__NETWORK_AI_ANALYSIS_COMPLETED_TYPE__",
                &ai_analysis_completed_type,
            ),
            (
                "__NETWORK_POLICY_EVALUATION_REQUESTED_TYPE__",
                &policy_evaluation_requested_type,
            ),
            (
                "__NETWORK_POLICY_DECISION_COMPLETED_TYPE__",
                &policy_decision_completed_type,
            ),
            (
                "__NETWORK_ENFORCEMENT_COMMAND_ISSUED_TYPE__",
                &enforcement_command_issued_type,
            ),
            (
                "__NETWORK_ENFORCEMENT_RESULT_OBSERVED_TYPE__",
                &enforcement_result_observed_type,
            ),
            (
                "__NETWORK_AUDIT_ENTRY_COMMITTED_TYPE__",
                &audit_entry_committed_type,
            ),
            (
                "__NETWORK_PORTAL_READ_MODEL_UPDATED_TYPE__",
                &portal_read_model_updated_type,
            ),
            (
                "__NETWORK_RUNTIME_EVENT_PAYLOAD_TYPE__",
                &runtime_event_payload_type,
            ),
            ("__NETWORK_PAYLOAD_DECODER_FN__", &payload_decoder_fn),
            (
                "__NETWORK_EVENT_TYPE_SCHEMA_CONST__",
                &event_type_schema_const,
            ),
            ("__HELPER_PREFIX__", &helper_prefix),
        ],
    )
}

fn replace_tokens(mut template: String, tokens: &[(&str, &str)]) -> String {
    for (token, value) in tokens {
        template = template.replace(token, value);
    }
    template
}

fn browser_runtime_event_type_descriptors() -> Vec<ProtocolLiteralDescriptor<&'static str>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "EvidenceObserved",
            value: BrowserRuntimePhase::EvidenceObserved.event_type(),
        },
        ProtocolLiteralDescriptor {
            key: "EvidenceJournaled",
            value: BrowserRuntimePhase::EvidenceJournaled.event_type(),
        },
        ProtocolLiteralDescriptor {
            key: "AiAnalysisRequested",
            value: BrowserRuntimePhase::AiAnalysisRequested.event_type(),
        },
        ProtocolLiteralDescriptor {
            key: "AiAnalysisCompleted",
            value: BrowserRuntimePhase::AiAnalysisCompleted.event_type(),
        },
        ProtocolLiteralDescriptor {
            key: "PolicyEvaluationRequested",
            value: BrowserRuntimePhase::PolicyEvaluationRequested.event_type(),
        },
        ProtocolLiteralDescriptor {
            key: "PolicyDecisionCompleted",
            value: BrowserRuntimePhase::PolicyDecisionCompleted.event_type(),
        },
        ProtocolLiteralDescriptor {
            key: "InterventionCommandIssued",
            value: BrowserRuntimePhase::InterventionCommandIssued.event_type(),
        },
        ProtocolLiteralDescriptor {
            key: "InterventionResultObserved",
            value: BrowserRuntimePhase::InterventionResultObserved.event_type(),
        },
        ProtocolLiteralDescriptor {
            key: "AuditEntryCommitted",
            value: BrowserRuntimePhase::AuditEntryCommitted.event_type(),
        },
        ProtocolLiteralDescriptor {
            key: "ReadModelProjected",
            value: BrowserRuntimePhase::ReadModelProjected.event_type(),
        },
    ]
}

fn browser_runtime_phase_descriptors() -> Vec<ProtocolLiteralDescriptor<BrowserRuntimePhase>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "EvidenceObserved",
            value: BrowserRuntimePhase::EvidenceObserved,
        },
        ProtocolLiteralDescriptor {
            key: "EvidenceJournaled",
            value: BrowserRuntimePhase::EvidenceJournaled,
        },
        ProtocolLiteralDescriptor {
            key: "AiAnalysisRequested",
            value: BrowserRuntimePhase::AiAnalysisRequested,
        },
        ProtocolLiteralDescriptor {
            key: "AiAnalysisCompleted",
            value: BrowserRuntimePhase::AiAnalysisCompleted,
        },
        ProtocolLiteralDescriptor {
            key: "PolicyEvaluationRequested",
            value: BrowserRuntimePhase::PolicyEvaluationRequested,
        },
        ProtocolLiteralDescriptor {
            key: "PolicyDecisionCompleted",
            value: BrowserRuntimePhase::PolicyDecisionCompleted,
        },
        ProtocolLiteralDescriptor {
            key: "InterventionCommandIssued",
            value: BrowserRuntimePhase::InterventionCommandIssued,
        },
        ProtocolLiteralDescriptor {
            key: "InterventionResultObserved",
            value: BrowserRuntimePhase::InterventionResultObserved,
        },
        ProtocolLiteralDescriptor {
            key: "AuditEntryCommitted",
            value: BrowserRuntimePhase::AuditEntryCommitted,
        },
        ProtocolLiteralDescriptor {
            key: "ReadModelProjected",
            value: BrowserRuntimePhase::ReadModelProjected,
        },
    ]
}

fn browser_capability_status_descriptors() -> Vec<ProtocolLiteralDescriptor<BrowserCapabilityStatus>>
{
    vec![
        ProtocolLiteralDescriptor {
            key: "Available",
            value: BrowserCapabilityStatus::Available,
        },
        ProtocolLiteralDescriptor {
            key: "TabListOnly",
            value: BrowserCapabilityStatus::TabListOnly,
        },
        ProtocolLiteralDescriptor {
            key: "UnsupportedBrowser",
            value: BrowserCapabilityStatus::UnsupportedBrowser,
        },
        ProtocolLiteralDescriptor {
            key: "UnmanagedBrowser",
            value: BrowserCapabilityStatus::UnmanagedBrowser,
        },
        ProtocolLiteralDescriptor {
            key: "ManagedProfileMissing",
            value: BrowserCapabilityStatus::ManagedProfileMissing,
        },
        ProtocolLiteralDescriptor {
            key: "BridgeMissing",
            value: BrowserCapabilityStatus::BridgeMissing,
        },
        ProtocolLiteralDescriptor {
            key: "PermissionLimited",
            value: BrowserCapabilityStatus::PermissionLimited,
        },
        ProtocolLiteralDescriptor {
            key: "Stale",
            value: BrowserCapabilityStatus::Stale,
        },
        ProtocolLiteralDescriptor {
            key: "AdapterError",
            value: BrowserCapabilityStatus::AdapterError,
        },
        ProtocolLiteralDescriptor {
            key: "DisabledByParent",
            value: BrowserCapabilityStatus::DisabledByParent,
        },
    ]
}

fn browser_custody_label_descriptors() -> Vec<ProtocolLiteralDescriptor<BrowserCustodyLabel>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "ChildDeviceLocal",
            value: BrowserCustodyLabel::ChildDeviceLocal,
        },
        ProtocolLiteralDescriptor {
            key: "LocalNetworkChildAgent",
            value: BrowserCustodyLabel::LocalNetworkChildAgent,
        },
        ProtocolLiteralDescriptor {
            key: "ParentCache",
            value: BrowserCustodyLabel::ParentCache,
        },
        ProtocolLiteralDescriptor {
            key: "ParentOwnedExport",
            value: BrowserCustodyLabel::ParentOwnedExport,
        },
        ProtocolLiteralDescriptor {
            key: "Unavailable",
            value: BrowserCustodyLabel::Unavailable,
        },
    ]
}

fn browser_query_visibility_descriptors(
) -> Vec<ProtocolLiteralDescriptor<BrowserQueryVisibilityLabel>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "LiveLocal",
            value: BrowserQueryVisibilityLabel::LiveLocal,
        },
        ProtocolLiteralDescriptor {
            key: "LiveLan",
            value: BrowserQueryVisibilityLabel::LiveLan,
        },
        ProtocolLiteralDescriptor {
            key: "ParentCache",
            value: BrowserQueryVisibilityLabel::ParentCache,
        },
        ProtocolLiteralDescriptor {
            key: "ParentOwnedExport",
            value: BrowserQueryVisibilityLabel::ParentOwnedExport,
        },
        ProtocolLiteralDescriptor {
            key: "Unavailable",
            value: BrowserQueryVisibilityLabel::Unavailable,
        },
    ]
}

fn network_runtime_event_type_descriptors() -> Vec<ProtocolLiteralDescriptor<&'static str>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "NetworkFlowObserved",
            value: NetworkRuntimePhase::FlowObserved.event_type(),
        },
        ProtocolLiteralDescriptor {
            key: "NetworkDomainObserved",
            value: NetworkRuntimePhase::DomainObserved.event_type(),
        },
        ProtocolLiteralDescriptor {
            key: "NetworkActivityClassified",
            value: NetworkRuntimePhase::ActivityClassified.event_type(),
        },
        ProtocolLiteralDescriptor {
            key: "AiAnalysisRequested",
            value: NetworkRuntimePhase::AiAnalysisRequested.event_type(),
        },
        ProtocolLiteralDescriptor {
            key: "AiAnalysisCompleted",
            value: NetworkRuntimePhase::AiAnalysisCompleted.event_type(),
        },
        ProtocolLiteralDescriptor {
            key: "PolicyEvaluationRequested",
            value: NetworkRuntimePhase::PolicyEvaluationRequested.event_type(),
        },
        ProtocolLiteralDescriptor {
            key: "PolicyDecisionCompleted",
            value: NetworkRuntimePhase::PolicyDecisionCompleted.event_type(),
        },
        ProtocolLiteralDescriptor {
            key: "EnforcementCommandIssued",
            value: NetworkRuntimePhase::EnforcementCommandIssued.event_type(),
        },
        ProtocolLiteralDescriptor {
            key: "EnforcementResultObserved",
            value: NetworkRuntimePhase::EnforcementResultObserved.event_type(),
        },
        ProtocolLiteralDescriptor {
            key: "AuditEntryCommitted",
            value: NetworkRuntimePhase::AuditEntryCommitted.event_type(),
        },
        ProtocolLiteralDescriptor {
            key: "PortalReadModelUpdated",
            value: NetworkRuntimePhase::PortalReadModelUpdated.event_type(),
        },
    ]
}

fn network_evidence_grade_descriptors() -> Vec<ProtocolLiteralDescriptor<NetworkEvidenceGrade>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "A",
            value: NetworkEvidenceGrade::A,
        },
        ProtocolLiteralDescriptor {
            key: "B",
            value: NetworkEvidenceGrade::B,
        },
        ProtocolLiteralDescriptor {
            key: "C",
            value: NetworkEvidenceGrade::C,
        },
        ProtocolLiteralDescriptor {
            key: "D",
            value: NetworkEvidenceGrade::D,
        },
    ]
}

fn network_domain_attribution_kind_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkDomainAttributionKind>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "DnsAnswer",
            value: NetworkDomainAttributionKind::DnsAnswer,
        },
        ProtocolLiteralDescriptor {
            key: "SniVisible",
            value: NetworkDomainAttributionKind::SniVisible,
        },
        ProtocolLiteralDescriptor {
            key: "HttpHost",
            value: NetworkDomainAttributionKind::HttpHost,
        },
        ProtocolLiteralDescriptor {
            key: "ReverseLookup",
            value: NetworkDomainAttributionKind::ReverseLookup,
        },
        ProtocolLiteralDescriptor {
            key: "IpOnly",
            value: NetworkDomainAttributionKind::IpOnly,
        },
        ProtocolLiteralDescriptor {
            key: "Unavailable",
            value: NetworkDomainAttributionKind::Unavailable,
        },
    ]
}

fn network_activity_kind_descriptors() -> Vec<ProtocolLiteralDescriptor<NetworkActivityKind>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "SocialCandidate",
            value: NetworkActivityKind::SocialCandidate,
        },
        ProtocolLiteralDescriptor {
            key: "VideoCandidate",
            value: NetworkActivityKind::VideoCandidate,
        },
        ProtocolLiteralDescriptor {
            key: "GameCandidate",
            value: NetworkActivityKind::GameCandidate,
        },
        ProtocolLiteralDescriptor {
            key: "VpnProxyTunnelCandidate",
            value: NetworkActivityKind::VpnProxyTunnelCandidate,
        },
        ProtocolLiteralDescriptor {
            key: "Unknown",
            value: NetworkActivityKind::Unknown,
        },
    ]
}

fn network_ai_advisory_state_descriptors() -> Vec<ProtocolLiteralDescriptor<NetworkAiAdvisoryState>>
{
    vec![
        ProtocolLiteralDescriptor {
            key: "Requested",
            value: NetworkAiAdvisoryState::Requested,
        },
        ProtocolLiteralDescriptor {
            key: "Completed",
            value: NetworkAiAdvisoryState::Completed,
        },
        ProtocolLiteralDescriptor {
            key: "ManualReviewRequired",
            value: NetworkAiAdvisoryState::ManualReviewRequired,
        },
        ProtocolLiteralDescriptor {
            key: "ProviderUnavailable",
            value: NetworkAiAdvisoryState::ProviderUnavailable,
        },
    ]
}

fn network_policy_decision_action_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkPolicyDecisionAction>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "Observe",
            value: NetworkPolicyDecisionAction::Observe,
        },
        ProtocolLiteralDescriptor {
            key: "Warn",
            value: NetworkPolicyDecisionAction::Warn,
        },
        ProtocolLiteralDescriptor {
            key: "AskParent",
            value: NetworkPolicyDecisionAction::AskParent,
        },
        ProtocolLiteralDescriptor {
            key: "Limit",
            value: NetworkPolicyDecisionAction::Limit,
        },
        ProtocolLiteralDescriptor {
            key: "Block",
            value: NetworkPolicyDecisionAction::Block,
        },
        ProtocolLiteralDescriptor {
            key: "ManualReview",
            value: NetworkPolicyDecisionAction::ManualReview,
        },
        ProtocolLiteralDescriptor {
            key: "Unknown",
            value: NetworkPolicyDecisionAction::Unknown,
        },
    ]
}

fn network_enforcement_mode_descriptors() -> Vec<ProtocolLiteralDescriptor<NetworkEnforcementMode>>
{
    vec![
        ProtocolLiteralDescriptor {
            key: "DryRun",
            value: NetworkEnforcementMode::DryRun,
        },
        ProtocolLiteralDescriptor {
            key: "ManualRequired",
            value: NetworkEnforcementMode::ManualRequired,
        },
        ProtocolLiteralDescriptor {
            key: "Unavailable",
            value: NetworkEnforcementMode::Unavailable,
        },
    ]
}

fn network_enforcement_result_status_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkEnforcementResultStatus>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "DryRun",
            value: NetworkEnforcementResultStatus::DryRun,
        },
        ProtocolLiteralDescriptor {
            key: "ManualRequired",
            value: NetworkEnforcementResultStatus::ManualRequired,
        },
        ProtocolLiteralDescriptor {
            key: "Unavailable",
            value: NetworkEnforcementResultStatus::Unavailable,
        },
        ProtocolLiteralDescriptor {
            key: "Rejected",
            value: NetworkEnforcementResultStatus::Rejected,
        },
    ]
}

fn network_audit_outcome_descriptors() -> Vec<ProtocolLiteralDescriptor<NetworkAuditOutcome>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "Committed",
            value: NetworkAuditOutcome::Committed,
        },
        ProtocolLiteralDescriptor {
            key: "Failed",
            value: NetworkAuditOutcome::Failed,
        },
    ]
}

fn network_portal_update_kind_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkPortalUpdateKind>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "NetworkReadModel",
            value: NetworkPortalUpdateKind::NetworkReadModel,
        },
        ProtocolLiteralDescriptor {
            key: "CapabilityState",
            value: NetworkPortalUpdateKind::CapabilityState,
        },
        ProtocolLiteralDescriptor {
            key: "ManualRequiredState",
            value: NetworkPortalUpdateKind::ManualRequiredState,
        },
    ]
}

fn network_status_ref_typescript(names: &ProtocolBridgeNames) -> String {
    [
        const_object_typescript(
            names.network_remote_delivery_status_refs_const,
            &network_remote_delivery_status_ref_descriptors(),
        ),
        const_object_typescript(
            names.network_live_capture_status_refs_const,
            &network_live_capture_status_ref_descriptors(),
        ),
        const_object_typescript(
            names.network_linux_nftables_lab_status_refs_const,
            &network_linux_nftables_lab_status_ref_descriptors(),
        ),
        const_object_typescript(
            names.network_windows_firewall_lab_status_refs_const,
            &network_windows_firewall_lab_status_ref_descriptors(),
        ),
        const_object_typescript(
            names.network_windows_wfp_gate_status_refs_const,
            &network_windows_wfp_gate_status_ref_descriptors(),
        ),
        const_object_typescript(
            names.network_android_vpn_service_gate_status_refs_const,
            &network_android_vpn_service_gate_status_ref_descriptors(),
        ),
        const_object_typescript(
            names.network_apple_network_extension_gate_status_refs_const,
            &network_apple_network_extension_gate_status_ref_descriptors(),
        ),
    ]
    .join(" ")
}

fn network_status_contract_typescript(names: &ProtocolBridgeNames) -> String {
    let prefix = bridge_prefix(names);
    let remote_status_state_const = format!("{prefix}NetworkRemoteDeliveryStatusState");
    let remote_transport_dispatch_state_const =
        format!("{prefix}NetworkRemoteDeliveryTransportDispatchState");
    let remote_provider_child_readiness_state_const =
        format!("{prefix}NetworkRemoteDeliveryProviderChildReadinessState");
    let remote_cross_process_custody_readiness_state_const =
        format!("{prefix}NetworkRemoteDeliveryCrossProcessCustodyReadinessState");
    let remote_external_cross_process_transport_state_const =
        format!("{prefix}NetworkRemoteDeliveryExternalCrossProcessTransportState");
    let live_capture_platform_const = format!("{prefix}NetworkLiveCapturePlatform");
    let live_capture_proof_state_const = format!("{prefix}NetworkLiveCaptureProofState");
    let live_capture_storage_state_const = format!("{prefix}NetworkRawCaptureStorageState");
    let live_capture_execution_state_const = format!("{prefix}NetworkLiveCaptureExecutionState");
    let linux_nftables_state_const = format!("{prefix}NetworkLinuxNftablesLabState");
    let linux_nftables_command_kind_const = format!("{prefix}NetworkLinuxNftablesLabCommandKind");
    let windows_firewall_state_const = format!("{prefix}NetworkWindowsFirewallLabState");
    let windows_firewall_command_kind_const =
        format!("{prefix}NetworkWindowsFirewallLabCommandKind");
    let windows_wfp_gate_state_const = format!("{prefix}NetworkWindowsWfpGateState");
    let windows_wfp_capability_state_const = format!("{prefix}NetworkWindowsWfpCapabilityState");
    let android_vpn_gate_state_const = format!("{prefix}NetworkAndroidVpnServiceGateState");
    let android_vpn_capability_state_const =
        format!("{prefix}NetworkAndroidVpnServiceCapabilityState");
    let android_vpn_required_artifact_const =
        format!("{prefix}NetworkAndroidVpnServiceRequiredArtifact");
    let android_vpn_boundary_reason_const =
        format!("{prefix}NetworkAndroidVpnServiceBoundaryReason");
    let apple_network_extension_platform_const =
        format!("{prefix}NetworkAppleNetworkExtensionPlatform");
    let apple_network_extension_capability_state_const =
        format!("{prefix}NetworkAppleNetworkExtensionCapabilityState");
    let apple_network_extension_gate_state_const =
        format!("{prefix}NetworkAppleNetworkExtensionGateState");
    let apple_network_extension_required_artifact_const =
        format!("{prefix}NetworkAppleNetworkExtensionRequiredArtifact");
    let apple_network_extension_boundary_reason_const =
        format!("{prefix}NetworkAppleNetworkExtensionBoundaryReason");
    [
        literal_typescript(
            &remote_status_state_const,
            &remote_status_state_const,
            &network_remote_delivery_status_state_descriptors(),
        ),
        literal_typescript(
            &remote_transport_dispatch_state_const,
            &remote_transport_dispatch_state_const,
            &network_remote_delivery_transport_dispatch_state_descriptors(),
        ),
        literal_typescript(
            &remote_provider_child_readiness_state_const,
            &remote_provider_child_readiness_state_const,
            &network_remote_delivery_provider_child_readiness_state_descriptors(),
        ),
        literal_typescript(
            &remote_cross_process_custody_readiness_state_const,
            &remote_cross_process_custody_readiness_state_const,
            &network_remote_delivery_cross_process_custody_readiness_state_descriptors(),
        ),
        literal_typescript(
            &remote_external_cross_process_transport_state_const,
            &remote_external_cross_process_transport_state_const,
            &network_remote_delivery_external_cross_process_transport_state_descriptors(),
        ),
        literal_typescript(
            &live_capture_platform_const,
            &live_capture_platform_const,
            &network_live_capture_platform_descriptors(),
        ),
        literal_typescript(
            &live_capture_proof_state_const,
            &live_capture_proof_state_const,
            &network_live_capture_proof_state_descriptors(),
        ),
        literal_typescript(
            &live_capture_storage_state_const,
            &live_capture_storage_state_const,
            &network_live_capture_storage_state_descriptors(),
        ),
        literal_typescript(
            &live_capture_execution_state_const,
            &live_capture_execution_state_const,
            &network_live_capture_execution_state_descriptors(),
        ),
        literal_typescript(
            &linux_nftables_state_const,
            &linux_nftables_state_const,
            &network_linux_nftables_status_state_descriptors(),
        ),
        literal_typescript(
            &linux_nftables_command_kind_const,
            &linux_nftables_command_kind_const,
            &network_linux_nftables_command_kind_descriptors(),
        ),
        literal_typescript(
            &windows_firewall_state_const,
            &windows_firewall_state_const,
            &network_windows_firewall_status_state_descriptors(),
        ),
        literal_typescript(
            &windows_firewall_command_kind_const,
            &windows_firewall_command_kind_const,
            &network_windows_firewall_command_kind_descriptors(),
        ),
        literal_typescript(
            &windows_wfp_gate_state_const,
            &windows_wfp_gate_state_const,
            &network_windows_wfp_gate_state_descriptors(),
        ),
        literal_typescript(
            &windows_wfp_capability_state_const,
            &windows_wfp_capability_state_const,
            &network_windows_wfp_capability_state_descriptors(),
        ),
        literal_typescript(
            &android_vpn_gate_state_const,
            &android_vpn_gate_state_const,
            &network_android_vpn_gate_state_descriptors(),
        ),
        literal_typescript(
            &android_vpn_capability_state_const,
            &android_vpn_capability_state_const,
            &network_android_vpn_capability_state_descriptors(),
        ),
        literal_typescript(
            &android_vpn_required_artifact_const,
            &android_vpn_required_artifact_const,
            &network_android_vpn_required_artifact_descriptors(),
        ),
        literal_typescript(
            &android_vpn_boundary_reason_const,
            &android_vpn_boundary_reason_const,
            &network_android_vpn_boundary_reason_descriptors(),
        ),
        literal_typescript(
            &apple_network_extension_platform_const,
            &apple_network_extension_platform_const,
            &network_apple_network_extension_platform_descriptors(),
        ),
        literal_typescript(
            &apple_network_extension_capability_state_const,
            &apple_network_extension_capability_state_const,
            &network_apple_network_extension_capability_state_descriptors(),
        ),
        literal_typescript(
            &apple_network_extension_gate_state_const,
            &apple_network_extension_gate_state_const,
            &network_apple_network_extension_gate_state_descriptors(),
        ),
        literal_typescript(
            &apple_network_extension_required_artifact_const,
            &apple_network_extension_required_artifact_const,
            &network_apple_network_extension_required_artifact_descriptors(),
        ),
        literal_typescript(
            &apple_network_extension_boundary_reason_const,
            &apple_network_extension_boundary_reason_const,
            &network_apple_network_extension_boundary_reason_descriptors(),
        ),
        network_status_contract_decoders_typescript(
            prefix,
            &remote_status_state_const,
            &remote_transport_dispatch_state_const,
            &remote_provider_child_readiness_state_const,
            &remote_cross_process_custody_readiness_state_const,
            &remote_external_cross_process_transport_state_const,
            &live_capture_platform_const,
            &live_capture_proof_state_const,
            &live_capture_storage_state_const,
            &live_capture_execution_state_const,
            &linux_nftables_state_const,
            &linux_nftables_command_kind_const,
            &windows_firewall_state_const,
            &windows_firewall_command_kind_const,
            &windows_wfp_gate_state_const,
            &windows_wfp_capability_state_const,
            &android_vpn_gate_state_const,
            &android_vpn_capability_state_const,
            &android_vpn_required_artifact_const,
            &android_vpn_boundary_reason_const,
            &apple_network_extension_platform_const,
            &apple_network_extension_capability_state_const,
            &apple_network_extension_gate_state_const,
            &apple_network_extension_required_artifact_const,
            &apple_network_extension_boundary_reason_const,
        ),
    ]
    .join(" ")
}

fn network_status_contract_decoders_typescript(
    prefix: &str,
    remote_status_state_const: &str,
    remote_transport_dispatch_state_const: &str,
    remote_provider_child_readiness_state_const: &str,
    remote_cross_process_custody_readiness_state_const: &str,
    remote_external_cross_process_transport_state_const: &str,
    live_capture_platform_const: &str,
    live_capture_proof_state_const: &str,
    live_capture_storage_state_const: &str,
    live_capture_execution_state_const: &str,
    linux_nftables_state_const: &str,
    linux_nftables_command_kind_const: &str,
    windows_firewall_state_const: &str,
    windows_firewall_command_kind_const: &str,
    windows_wfp_gate_state_const: &str,
    windows_wfp_capability_state_const: &str,
    android_vpn_gate_state_const: &str,
    android_vpn_capability_state_const: &str,
    android_vpn_required_artifact_const: &str,
    android_vpn_boundary_reason_const: &str,
    apple_network_extension_platform_const: &str,
    apple_network_extension_capability_state_const: &str,
    apple_network_extension_gate_state_const: &str,
    apple_network_extension_required_artifact_const: &str,
    apple_network_extension_boundary_reason_const: &str,
) -> String {
    let remote_status_type = format!("{prefix}NetworkRemoteDeliveryStatus");
    let remote_status_schema_const = format!("{prefix}NetworkRemoteDeliveryStatusSchema");
    let live_capture_row_type = format!("{prefix}NetworkLiveCaptureStatusRow");
    let live_capture_status_type = format!("{prefix}NetworkLiveCaptureStatus");
    let live_capture_row_schema_const = format!("{prefix}NetworkLiveCaptureStatusRowSchema");
    let live_capture_status_schema_const = format!("{prefix}NetworkLiveCaptureStatusSchema");
    let linux_nftables_command_row_type = format!("{prefix}NetworkLinuxNftablesLabCommandRow");
    let linux_nftables_status_type = format!("{prefix}NetworkLinuxNftablesLabStatus");
    let linux_nftables_status_schema_const = format!("{prefix}NetworkLinuxNftablesLabStatusSchema");
    let windows_firewall_command_row_type = format!("{prefix}NetworkWindowsFirewallLabCommandRow");
    let windows_firewall_status_type = format!("{prefix}NetworkWindowsFirewallLabStatus");
    let windows_firewall_status_schema_const =
        format!("{prefix}NetworkWindowsFirewallLabStatusSchema");
    let windows_wfp_status_type = format!("{prefix}NetworkWindowsWfpGateStatus");
    let windows_wfp_status_schema_const = format!("{prefix}NetworkWindowsWfpGateStatusSchema");
    let android_vpn_status_type = format!("{prefix}NetworkAndroidVpnServiceGateStatus");
    let android_vpn_status_schema_const =
        format!("{prefix}NetworkAndroidVpnServiceGateStatusSchema");
    let apple_network_extension_status_type =
        format!("{prefix}NetworkAppleNetworkExtensionGateStatus");
    let apple_network_extension_status_schema_const =
        format!("{prefix}NetworkAppleNetworkExtensionGateStatusSchema");
    let helper_prefix = format!("__{prefix}NetworkStatus");
    replace_tokens(
        r#"
export type __REMOTE_STATUS_TYPE__ = Readonly<Record<string, unknown>>;
export type __LIVE_CAPTURE_ROW_TYPE__ = Readonly<Record<string, unknown>>;
export type __LIVE_CAPTURE_STATUS_TYPE__ = Readonly<Record<string, unknown>> & { readonly rows: readonly __LIVE_CAPTURE_ROW_TYPE__[] };
export type __LINUX_NFTABLES_COMMAND_ROW_TYPE__ = Readonly<Record<string, unknown>> & { readonly kind: __LINUX_NFTABLES_COMMAND_KIND_CONST__ };
export type __LINUX_NFTABLES_STATUS_TYPE__ = Readonly<Record<string, unknown>> & { readonly commandEvidence: readonly __LINUX_NFTABLES_COMMAND_ROW_TYPE__[] };
export type __WINDOWS_FIREWALL_COMMAND_ROW_TYPE__ = Readonly<Record<string, unknown>> & { readonly kind: __WINDOWS_FIREWALL_COMMAND_KIND_CONST__ };
export type __WINDOWS_FIREWALL_STATUS_TYPE__ = Readonly<Record<string, unknown>> & { readonly commandEvidence: readonly __WINDOWS_FIREWALL_COMMAND_ROW_TYPE__[] };
export type __WINDOWS_WFP_STATUS_TYPE__ = Readonly<Record<string, unknown>> & { readonly boundaryReasons: readonly string[]; readonly missingRequiredArtifacts: readonly string[]; readonly wfpLabProofReady: boolean; readonly enforcementCommandPublished: false };
export type __ANDROID_VPN_STATUS_TYPE__ = Readonly<Record<string, unknown>> & { readonly boundaryReasons: readonly __ANDROID_VPN_BOUNDARY_REASON_CONST__[]; readonly missingRequiredArtifacts: readonly __ANDROID_VPN_REQUIRED_ARTIFACT_CONST__[]; readonly gateState: __ANDROID_VPN_GATE_STATE_CONST__; readonly physicalDeviceProofReady: boolean; readonly enforcementCommandPublished: false };
export type __APPLE_NETWORK_EXTENSION_STATUS_TYPE__ = Readonly<Record<string, unknown>> & { readonly boundaryReasons: readonly __APPLE_NETWORK_EXTENSION_BOUNDARY_REASON_CONST__[]; readonly missingRequiredArtifacts: readonly __APPLE_NETWORK_EXTENSION_REQUIRED_ARTIFACT_CONST__[]; readonly platform: __APPLE_NETWORK_EXTENSION_PLATFORM_CONST__; readonly gateState: __APPLE_NETWORK_EXTENSION_GATE_STATE_CONST__; readonly appleEntitlementProofReady: boolean; readonly enforcementCommandPublished: false };
const __HELPER_PREFIX__RemoteStringFields = ['statusRef','custodyProofRef','publisherAuthRef','subscriberAuthRef','encryptionRef','retentionPolicyRef','replayPlanRef','deletionPlanRef','offsetPolicyRef','dedupePolicyRef','transportConfigRef','relayIdentityRef','relayPolicyRef','eventChainJournalRef','receiptLedgerRef','localReceiptAckRef','durableEnvelopeRef','durableStoreRef','durableReplayRef','durableDeleteExportRef','durableSupportStatusRef','outboxRef','outboxHandoffRef','outboxReplayRef','outboxSupportStatusRef','transportDispatchStateRef','blockedDispatchRef','futureTransportSeamRef','fixtureTransportRef','fixtureDispatchAttemptRef','fixtureAckRef','deleteExportPropagationRef','remoteDeleteReadinessRef','remoteExportReadinessRef','providerRouteRef','childDeviceRouteRef','providerDeliveryReadinessRef','childDeviceDeliveryReadinessRef','crossProcessCustodyStatusRef','crossProcessReplayReadinessRef','remoteRetentionReadinessRef','remoteDeleteCustodyReadinessRef','remoteExportCustodyReadinessRef','crossProcessReplayRef','crossProcessReplayStoreRef','crossProcessReplayCursorRef','externalCrossProcessTransportRef','externalCrossProcessTransportEnvelopeRef','externalCrossProcessTransportAckRef'] as const;
const __HELPER_PREFIX__RemoteCountFields = ['brokerMissingArtifactCount','familyHubMissingArtifactCount','acceptedEventTypeCount','droppedEventDeadLetterCount','durableEnvelopeMissingArtifactCount','outboxCandidateCount','sourceOutboxCandidateCount','preparedNotDispatchedCount','blockedDispatchRecordCount','fixtureSourceOutboxCandidateCount','fixtureDispatchAttemptCount','fixtureRemoteAckCount','deleteExportReadinessRecordCount','remoteDeleteReadyCount','remoteExportReadyCount','providerDeliveryReadinessRecordCount','childDeviceDeliveryReadinessRecordCount','crossProcessReplayReadinessRecordCount','remoteRetentionReadinessRecordCount','remoteDeleteCustodyReadinessRecordCount','remoteExportCustodyReadinessRecordCount','crossProcessReplayRecordCount','crossProcessReplayStoreWriteCount','crossProcessReplayCursorNextSequence','externalCrossProcessTransportRecordCount','externalCrossProcessTransportEnvelopeCount','externalCrossProcessTransportAckCount'] as const;
const __HELPER_PREFIX__RemoteZeroFields = ['providerDeliveryArtifactCount','childDeviceDeliveryArtifactCount','crossProcessReplayArtifactCount','remoteRetentionArtifactCount','remoteDeleteCustodyArtifactCount','remoteExportCustodyArtifactCount','dispatchReadyCandidateCount','dispatchAttemptCount','remoteAckCount','sequenceGapCount','eventIdMismatchCount','eventTypeMismatchCount','correlationMismatchCount','enforcementCommandEventCount','adapterActionExecutedCount','rawPcapAvailableCount','exactUrlAvailableCount','decryptedPayloadAvailableCount','pageContentAvailableCount','videoContentAvailableCount','privateMessageContentAvailableCount','searchQueryAvailableCount'] as const;
const __HELPER_PREFIX__RemoteBooleanFields = ['localIdempotencyQueueProved','queuedDuplicateRejected','completedDuplicateRejected','durableEnvelopeReady','blockedDispatchRecordsMatchOutboxCandidates','fixtureRecordsMatchOutboxCandidates','deleteExportRecordsMatchFixtureAcks','providerDeliveryRecordsMatchFixtureAcks','childDeviceDeliveryRecordsMatchFixtureAcks','crossProcessCustodyRecordsMatchProviderChildReadiness','crossProcessReplayRecordsMatchDurableEnvelopes','crossProcessReplayRecordsMatchCustodyReadiness','externalCrossProcessTransportRecordsMatchReplayRecords','externalCrossProcessTransportAckRecordsMatchEnvelopes','duplicateDurableEnvelopeRejected','outboxCandidatesMatchDurableEnvelopes','outboxCandidatesMatchReceipts'] as const;
const __HELPER_PREFIX__RemoteFalseFields = ['brokerDeliveryImplemented','familyHubDeliveryImplemented','remoteDeliveryAckImplemented','providerDeliveryImplemented','childDeviceDeliveryImplemented','remoteDeleteExportPropagationImplemented','productReadyRemoteDelivery','policyAuthority','sideEffectAuthority','hostFilteringClaimed'] as const;
const __HELPER_PREFIX__RemoteTrueFields = ['crossProcessReplayImplemented','externalCrossProcessTransportImplemented'] as const;
const __HELPER_PREFIX__LiveStatusStringFields = ['statusRef','row13StatusRef','executionStatusRef','rawStorageStatusRef'] as const;
const __HELPER_PREFIX__LiveStatusCountFields = ['platformRowCount','proofReadyCount','manualRequiredCount','unavailableCount','degradedCount','requiredArtifactCount','missingArtifactCount','storageCustodyReadyCount','storageManualRequiredCount','storageUnavailableCount','storageDegradedCount','storageMissingArtifactCount','boundedExecutedCount','executionManualRequiredCount','executionUnavailableCount','executionDegradedCount','executionMissingArtifactCount','metadataSnapshotExecutedCount','capturedPacketCount','captureReadyCount','rawArtifactStorageAuthorizedCount','driverInvokedCount','liveCaptureExecutedCount'] as const;
const __HELPER_PREFIX__LiveStatusZeroFields = ['rawArtifactCreatedCount','remoteUploadEnabledCount','rawPcapWithoutCustodyAvailableCount','exactUrlAvailableCount','decryptedPayloadAvailableCount','pageContentAvailableCount','privateMessageAvailableCount','searchQueryAvailableCount','policyAuthorityCount','adapterAuthorityCount','enforcementCommandEventCount','netstatMetadataSubstitutionCount','hostFilteringClaimCount'] as const;
const __HELPER_PREFIX__LiveRowStringFields = ['captureProofRef','storageProofRef'] as const;
const __HELPER_PREFIX__LiveRowNullableStringFields = ['interfaceRef','driverProofRef','permissionProofRef','boundedCaptureRef','cleanStopRef','quotaRotationRef','retentionDeleteExportRef','custodyRef','privateTrafficExclusionRef','rawArtifactManifestRef','storageLocationRef','encryptionAtRestRef','storageQuotaRotationRef','retentionPolicyRef','storageDeleteExportRef','custodyChainRef','storagePrivateTrafficExclusionRef','executionRef','driverInvocationRef','interfaceObservationRef','executionPermissionRef','boundedWindowRef','executionCleanStopRef','executionCustodyRef','executionRetentionDeleteExportRef','metadataOnlySanitizationRef','executionPrivateTrafficExclusionRef'] as const;
const __HELPER_PREFIX__LiveRowCountFields = ['executionMissingArtifactCount','capturedPacketCount','missingArtifactCount','storageMissingArtifactCount'] as const;
const __HELPER_PREFIX__LiveRowBooleanFields = ['metadataSnapshotExecuted','captureReady','rawArtifactStorageAuthorized','driverInvoked','liveCaptureExecuted'] as const;
const __HELPER_PREFIX__LiveRowFalseFields = ['rawArtifactCreated','remoteUploadEnabled','rawPcapWithoutCustodyAvailable','exactUrlAvailable','decryptedPayloadAvailable','pageContentAvailable','privateMessageAvailable','searchQueryAvailable','policyAuthority','adapterAuthority','netstatMetadataSubstitutedForLiveCapture','hostFilteringClaimed'] as const;
const __HELPER_PREFIX__LiveRowZeroFields = ['enforcementCommandsPublished'] as const;
const __HELPER_PREFIX__LinuxNftablesStringFields = ['statusRef','labRef','linuxAdapterGateRef','policyDecisionRef','parentRuleRef','distroRef','kernelRef','tableName','chainName','targetRemoteAddress'] as const;
const __HELPER_PREFIX__LinuxNftablesBooleanFields = ['wslHostObserved','rootPermissionObserved','nftToolObserved','tableCreateObserved','chainCreateObserved','ruleAddObserved','verifyPresentObserved','rollbackObserved','verifyRemovedObserved','labPacketFilterRuleExecuted','rollbackVerified'] as const;
const __HELPER_PREFIX__LinuxNftablesFalseFields = ['productionEnforcementClaimed','persistentRuleClaimed','genericLinuxSupportClaimed','serviceManagerInstallClaimed','exactUrlAvailable','decryptedPayloadAvailable','pageContentAvailable','policyEngineExecutionClaimed','enforcementCommandPublished'] as const;
const __HELPER_PREFIX__LinuxNftablesObservedFlags = [{ field: 'tableCreateObserved', kind: __LINUX_NFTABLES_COMMAND_KIND_CONST__.CreateTable },{ field: 'chainCreateObserved', kind: __LINUX_NFTABLES_COMMAND_KIND_CONST__.CreateChain },{ field: 'ruleAddObserved', kind: __LINUX_NFTABLES_COMMAND_KIND_CONST__.AddRule },{ field: 'verifyPresentObserved', kind: __LINUX_NFTABLES_COMMAND_KIND_CONST__.VerifyRulePresent },{ field: 'rollbackObserved', kind: __LINUX_NFTABLES_COMMAND_KIND_CONST__.DeleteTable },{ field: 'verifyRemovedObserved', kind: __LINUX_NFTABLES_COMMAND_KIND_CONST__.VerifyTableRemoved }] as const;
const __HELPER_PREFIX__LinuxNftablesExpectedOutcomes = [{ kind: __LINUX_NFTABLES_COMMAND_KIND_CONST__.CreateTable, table: true, chain: false, rule: false },{ kind: __LINUX_NFTABLES_COMMAND_KIND_CONST__.CreateChain, table: true, chain: true, rule: false },{ kind: __LINUX_NFTABLES_COMMAND_KIND_CONST__.AddRule, table: true, chain: true, rule: true },{ kind: __LINUX_NFTABLES_COMMAND_KIND_CONST__.VerifyRulePresent, table: true, chain: true, rule: true },{ kind: __LINUX_NFTABLES_COMMAND_KIND_CONST__.DeleteTable, table: false, chain: false, rule: false },{ kind: __LINUX_NFTABLES_COMMAND_KIND_CONST__.VerifyTableRemoved, table: false, chain: false, rule: false }] as const;
const __HELPER_PREFIX__WindowsFirewallStringFields = ['statusRef','labRef','firewallAdapterPlanRef','policyDecisionRef','parentRuleRef','windowsOsScopeRef','targetRef','firewallRuleRef','ruleName','targetRemoteAddress'] as const;
const __HELPER_PREFIX__WindowsFirewallBooleanFields = ['windowsHostObserved','administratorPermissionObserved','applyCommandObserved','verifyPresentObserved','rollbackCommandObserved','verifyRemovedObserved','labFirewallMutationExecuted','rollbackVerified','adapterApplyAuthorized'] as const;
const __HELPER_PREFIX__WindowsFirewallFalseFields = ['productionEnforcementClaimed','persistentRuleClaimed','exactUrlAvailable','decryptedPayloadAvailable','pageContentAvailable','hostFirewallMutationClaimed','netshCommandInvoked','powershellCommandInvoked','policyEngineExecutionClaimed','enforcementCommandPublished'] as const;
const __HELPER_PREFIX__WindowsFirewallObservedFlags = [{ field: 'applyCommandObserved', kind: __WINDOWS_FIREWALL_COMMAND_KIND_CONST__.ApplyRule },{ field: 'verifyPresentObserved', kind: __WINDOWS_FIREWALL_COMMAND_KIND_CONST__.VerifyRulePresent },{ field: 'rollbackCommandObserved', kind: __WINDOWS_FIREWALL_COMMAND_KIND_CONST__.RollbackRule },{ field: 'verifyRemovedObserved', kind: __WINDOWS_FIREWALL_COMMAND_KIND_CONST__.VerifyRuleRemoved }] as const;
const __HELPER_PREFIX__WindowsFirewallExpectedOutcomes = [{ kind: __WINDOWS_FIREWALL_COMMAND_KIND_CONST__.ApplyRule, rulePresentAfterCommand: true },{ kind: __WINDOWS_FIREWALL_COMMAND_KIND_CONST__.VerifyRulePresent, rulePresentAfterCommand: true },{ kind: __WINDOWS_FIREWALL_COMMAND_KIND_CONST__.RollbackRule, rulePresentAfterCommand: false },{ kind: __WINDOWS_FIREWALL_COMMAND_KIND_CONST__.VerifyRuleRemoved, rulePresentAfterCommand: false }] as const;
const __HELPER_PREFIX__WindowsWfpStringFields = ['statusRef','wfpGateRef','policyDecisionRef','parentRuleRef','targetRef','wfpProviderRef','wfpLayerRef'] as const;
const __HELPER_PREFIX__WindowsWfpNullableStringFields = ['localAiResultRef','administratorPermissionProofRef','driverSigningProofRef','driverPackageProofRef','providerRegistrationPlanRef','layerCapabilityMatrixRef','rollbackPlanRef','labResultArtifactRef','auditEventRef'] as const;
const __HELPER_PREFIX__WindowsWfpStringArrayFields = ['evidenceRefs','boundaryReasons','missingRequiredArtifacts'] as const;
const __HELPER_PREFIX__WindowsWfpFalseFields = ['adapterApplyAuthorized','enforcementCommandPublished','liveDriverInstallClaimed','calloutRegistrationClaimed','packetBlockClaimed','kernelPayloadInspectionClaimed','commandInvocationClaimed','exactUrlAvailable','decryptedPayloadAvailable','pageContentAvailable'] as const;
const __HELPER_PREFIX__AndroidVpnStringFields = ['statusRef','androidVpnServiceGateRef','policyDecisionRef','parentRuleRef','packageRef','vpnServiceRef'] as const;
const __HELPER_PREFIX__AndroidVpnNullableStringFields = ['localAiResultRef','vpnServiceDeclarationRef','userConsentProofRef','physicalDeviceProofRef','packageIdentityProofRef','virtualInterfaceProofRef','trafficObservationProofRef','rollbackPlanRef','auditEventRef','deviceOwnerProofRef'] as const;
const __HELPER_PREFIX__AndroidVpnBooleanFields = ['deviceOwnerRequired','physicalDeviceProofReady','deviceOwnerAuthorityProved'] as const;
const __HELPER_PREFIX__AndroidVpnFalseFields = ['adapterApplyAuthorized','enforcementCommandPublished','emulatorOnlyProductSupportClaimed','liveVpnTunnelClaimed','packetBlockClaimed','appPackageCorrelationClaimed','exactUrlAvailable','decryptedPayloadAvailable','pageContentAvailable'] as const;
const __HELPER_PREFIX__AppleNetworkExtensionStringFields = ['statusRef','appleNetworkExtensionGateRef','policyDecisionRef','parentRuleRef','bundleRef','networkExtensionRef'] as const;
const __HELPER_PREFIX__AppleNetworkExtensionNullableStringFields = ['localAiResultRef','developerTeamProofRef','entitlementApprovalProofRef','provisioningProfileProofRef','signingProofRef','deviceOrTestFlightProofRef','networkExtensionDeclarationRef','extensionConfigurationProofRef','rollbackPlanRef','auditEventRef','supervisionOrMdmProofRef'] as const;
const __HELPER_PREFIX__AppleNetworkExtensionBooleanFields = ['supervisionRequired','appleEntitlementProofReady','supervisionAuthorityProved'] as const;
const __HELPER_PREFIX__AppleNetworkExtensionFalseFields = ['adapterApplyAuthorized','enforcementCommandPublished','simulatorOnlyProductSupportClaimed','liveNetworkExtensionClaimed','packetBlockClaimed','appLevelControlClaimed','exactUrlAvailable','decryptedPayloadAvailable','pageContentAvailable'] as const;
function __HELPER_PREFIX__IsRecord(value: unknown): value is Readonly<Record<string, unknown>> { return typeof value === 'object' && value !== null && !Array.isArray(value); }
function __HELPER_PREFIX__ReadRecord(value: unknown, label: string): Readonly<Record<string, unknown>> { if (!__HELPER_PREFIX__IsRecord(value)) { throw new TypeError(`${label} must be a network status object`); } return value; }
function __HELPER_PREFIX__ReadString(record: Readonly<Record<string, unknown>>, field: string): string { const value = record[field]; if (typeof value !== 'string' || value.length === 0) { throw new TypeError(`${field} must be a non-empty network status string`); } return value; }
function __HELPER_PREFIX__ReadNullableString(record: Readonly<Record<string, unknown>>, field: string): string | null { const value = record[field]; if (value === null) { return null; } if (typeof value !== 'string' || value.length === 0) { throw new TypeError(`${field} must be a non-empty network status string or null`); } return value; }
function __HELPER_PREFIX__ReadCount(record: Readonly<Record<string, unknown>>, field: string): number { const value = record[field]; if (typeof value !== 'number' || !Number.isInteger(value) || value < 0) { throw new TypeError(`${field} must be a non-negative integer`); } return value; }
function __HELPER_PREFIX__ReadInteger(record: Readonly<Record<string, unknown>>, field: string): number { const value = record[field]; if (typeof value !== 'number' || !Number.isInteger(value)) { throw new TypeError(`${field} must be an integer`); } return value; }
function __HELPER_PREFIX__ReadRequiredCount(record: Readonly<Record<string, unknown>>, field: string, expected: number): number { const value = __HELPER_PREFIX__ReadCount(record, field); if (value !== expected) { throw new TypeError(`${field} must be ${expected}`); } return expected; }
function __HELPER_PREFIX__ReadBoolean(record: Readonly<Record<string, unknown>>, field: string): boolean { const value = record[field]; if (typeof value !== 'boolean') { throw new TypeError(`${field} must be a network status boolean`); } return value; }
function __HELPER_PREFIX__ReadRequiredBoolean(record: Readonly<Record<string, unknown>>, field: string, expected: boolean): boolean { const value = __HELPER_PREFIX__ReadBoolean(record, field); if (value !== expected) { throw new TypeError(`${field} must be ${expected}`); } return expected; }
function __HELPER_PREFIX__ReadLiteral<T extends string>(record: Readonly<Record<string, unknown>>, field: string, allowed: readonly T[]): T { const value = __HELPER_PREFIX__ReadString(record, field); if (!allowed.includes(value as T)) { throw new TypeError(`${field} is not a Rust-owned network status literal`); } return value as T; }
function __HELPER_PREFIX__ReadStringArray(record: Readonly<Record<string, unknown>>, field: string): readonly string[] { const values = record[field]; if (!Array.isArray(values)) { throw new TypeError(`${field} must be a network status string array`); } values.forEach((value) => { if (typeof value !== 'string' || value.length === 0) { throw new TypeError(`${field} entries must be non-empty network status strings`); } }); return values; }
function __HELPER_PREFIX__ReadLiteralArray<T extends string>(record: Readonly<Record<string, unknown>>, field: string, allowed: readonly T[]): readonly T[] { return __HELPER_PREFIX__ReadStringArray(record, field).map((value) => { if (!allowed.includes(value as T)) { throw new TypeError(`${field} entries must be Rust-owned network status literals`); } return value as T; }); }
function __HELPER_PREFIX__ReadRecordArray(record: Readonly<Record<string, unknown>>, field: string, label: string): readonly Readonly<Record<string, unknown>>[] { const values = record[field]; if (!Array.isArray(values)) { throw new TypeError(`${field} must be a ${label} array`); } return values.map((value) => __HELPER_PREFIX__ReadRecord(value, label)); }
function __HELPER_PREFIX__RequireCountMatches(record: Readonly<Record<string, unknown>>, field: string, expected: number): void { const value = __HELPER_PREFIX__ReadCount(record, field); if (value !== expected) { throw new TypeError(`${field} must match command evidence length`); } }
function __HELPER_PREFIX__RequireUniqueRowsByKind<T extends Readonly<Record<string, unknown>> & { readonly kind: string }>(rows: readonly T[], label: string): ReadonlyMap<string, T> { const byKind = new Map(rows.map((row) => [row.kind, row] as const)); if (byKind.size !== rows.length) { throw new TypeError(`${label} command evidence must use unique command kinds`); } return byKind; }
function __HELPER_PREFIX__RequireObservedFlags(record: Readonly<Record<string, unknown>>, byKind: ReadonlyMap<string, Readonly<Record<string, unknown>>>, flags: readonly { readonly field: string; readonly kind: string }[], label: string): void { flags.forEach(({ field, kind }) => { if (__HELPER_PREFIX__ReadBoolean(record, field) !== byKind.has(kind)) { throw new TypeError(`${label} observed flags must match command evidence`); } }); }
function __HELPER_PREFIX__RequireLinuxNftablesOutcomes(byKind: ReadonlyMap<string, __LINUX_NFTABLES_COMMAND_ROW_TYPE__>): void { __HELPER_PREFIX__LinuxNftablesExpectedOutcomes.forEach(({ kind, table, chain, rule }) => { const row = byKind.get(kind); if (row === undefined || row['tablePresentAfterCommand'] !== table || row['chainPresentAfterCommand'] !== chain || row['rulePresentAfterCommand'] !== rule) { throw new TypeError('Linux nftables command evidence must match bounded apply and rollback outcomes'); } }); }
function __HELPER_PREFIX__RequireWindowsFirewallOutcomes(byKind: ReadonlyMap<string, __WINDOWS_FIREWALL_COMMAND_ROW_TYPE__>): void { __HELPER_PREFIX__WindowsFirewallExpectedOutcomes.forEach(({ kind, rulePresentAfterCommand }) => { const row = byKind.get(kind); if (row === undefined || row['rulePresentAfterCommand'] !== rulePresentAfterCommand) { throw new TypeError('Windows firewall command evidence must match bounded apply and rollback outcomes'); } }); }
function __HELPER_PREFIX__GateProofReadyIsValid(capabilityReady: boolean, proofReady: boolean, boundaryReasons: readonly string[], missingRequiredArtifacts: readonly string[]): boolean { return capabilityReady && proofReady && boundaryReasons.length === 0 && missingRequiredArtifacts.length === 0; }
function __HELPER_PREFIX__GateManualRequiredIsValid(capabilityManualRequired: boolean, proofReady: boolean, boundaryReasons: readonly string[], missingRequiredArtifacts: readonly string[]): boolean { return capabilityManualRequired || boundaryReasons.length > 0 || missingRequiredArtifacts.length > 0 || !proofReady; }
function __HELPER_PREFIX__RequireGateConsistency(label: string, proofReadyGate: boolean, proofReadyValid: boolean, manualRequiredGate: boolean, manualRequiredValid: boolean): void { if (proofReadyGate && !proofReadyValid) { throw new TypeError(`${label} proof-ready status must preserve bounded proof invariants`); } if (manualRequiredGate && !manualRequiredValid) { throw new TypeError(`${label} manual-required status must preserve bounded blockers`); } }
export function decode__REMOTE_STATUS_TYPE__(value: unknown): __REMOTE_STATUS_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'network remote delivery status'); for (const field of __HELPER_PREFIX__RemoteStringFields) { __HELPER_PREFIX__ReadString(record, field); } for (const field of __HELPER_PREFIX__RemoteCountFields) { __HELPER_PREFIX__ReadCount(record, field); } for (const field of __HELPER_PREFIX__RemoteZeroFields) { __HELPER_PREFIX__ReadRequiredCount(record, field, 0); } for (const field of __HELPER_PREFIX__RemoteBooleanFields) { __HELPER_PREFIX__ReadBoolean(record, field); } for (const field of __HELPER_PREFIX__RemoteFalseFields) { __HELPER_PREFIX__ReadRequiredBoolean(record, field, false); } for (const field of __HELPER_PREFIX__RemoteTrueFields) { __HELPER_PREFIX__ReadRequiredBoolean(record, field, true); } __HELPER_PREFIX__ReadLiteral(record, 'brokerStatus', Object.values(__REMOTE_STATUS_STATE_CONST__)); __HELPER_PREFIX__ReadLiteral(record, 'familyHubStatus', Object.values(__REMOTE_STATUS_STATE_CONST__)); __HELPER_PREFIX__ReadLiteral(record, 'transportDispatchState', Object.values(__REMOTE_TRANSPORT_DISPATCH_STATE_CONST__)); __HELPER_PREFIX__ReadLiteral(record, 'providerDeliveryReadinessState', Object.values(__REMOTE_PROVIDER_CHILD_READINESS_STATE_CONST__)); __HELPER_PREFIX__ReadLiteral(record, 'childDeviceDeliveryReadinessState', Object.values(__REMOTE_PROVIDER_CHILD_READINESS_STATE_CONST__)); __HELPER_PREFIX__ReadLiteral(record, 'crossProcessCustodyReadinessState', Object.values(__REMOTE_CROSS_PROCESS_CUSTODY_READINESS_STATE_CONST__)); __HELPER_PREFIX__ReadLiteral(record, 'externalCrossProcessTransportState', Object.values(__REMOTE_EXTERNAL_CROSS_PROCESS_TRANSPORT_STATE_CONST__)); return record as __REMOTE_STATUS_TYPE__; }
export function decode__LIVE_CAPTURE_ROW_TYPE__(value: unknown): __LIVE_CAPTURE_ROW_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'network live capture status row'); for (const field of __HELPER_PREFIX__LiveRowStringFields) { __HELPER_PREFIX__ReadString(record, field); } for (const field of __HELPER_PREFIX__LiveRowNullableStringFields) { __HELPER_PREFIX__ReadNullableString(record, field); } for (const field of __HELPER_PREFIX__LiveRowCountFields) { __HELPER_PREFIX__ReadCount(record, field); } for (const field of __HELPER_PREFIX__LiveRowBooleanFields) { __HELPER_PREFIX__ReadBoolean(record, field); } for (const field of __HELPER_PREFIX__LiveRowFalseFields) { __HELPER_PREFIX__ReadRequiredBoolean(record, field, false); } for (const field of __HELPER_PREFIX__LiveRowZeroFields) { __HELPER_PREFIX__ReadRequiredCount(record, field, 0); } __HELPER_PREFIX__ReadLiteral(record, 'platform', Object.values(__LIVE_CAPTURE_PLATFORM_CONST__)); __HELPER_PREFIX__ReadLiteral(record, 'proofState', Object.values(__LIVE_CAPTURE_PROOF_STATE_CONST__)); __HELPER_PREFIX__ReadLiteral(record, 'storageState', Object.values(__LIVE_CAPTURE_STORAGE_STATE_CONST__)); __HELPER_PREFIX__ReadLiteral(record, 'executionState', Object.values(__LIVE_CAPTURE_EXECUTION_STATE_CONST__)); return record as __LIVE_CAPTURE_ROW_TYPE__; }
export function decode__LIVE_CAPTURE_STATUS_TYPE__(value: unknown): __LIVE_CAPTURE_STATUS_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'network live capture status'); for (const field of __HELPER_PREFIX__LiveStatusStringFields) { __HELPER_PREFIX__ReadString(record, field); } for (const field of __HELPER_PREFIX__LiveStatusCountFields) { __HELPER_PREFIX__ReadCount(record, field); } for (const field of __HELPER_PREFIX__LiveStatusZeroFields) { __HELPER_PREFIX__ReadRequiredCount(record, field, 0); } const rows = record['rows']; if (!Array.isArray(rows)) { throw new TypeError('rows must be a network live capture status row array'); } rows.forEach((row) => decode__LIVE_CAPTURE_ROW_TYPE__(row)); return record as __LIVE_CAPTURE_STATUS_TYPE__; }
export function decode__LINUX_NFTABLES_COMMAND_ROW_TYPE__(value: unknown): __LINUX_NFTABLES_COMMAND_ROW_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'network Linux nftables lab command row'); const kind = __HELPER_PREFIX__ReadLiteral(record, 'kind', Object.values(__LINUX_NFTABLES_COMMAND_KIND_CONST__)); __HELPER_PREFIX__ReadString(record, 'commandRef'); __HELPER_PREFIX__ReadInteger(record, 'exitStatus'); __HELPER_PREFIX__ReadString(record, 'outputSha256'); __HELPER_PREFIX__ReadBoolean(record, 'tablePresentAfterCommand'); __HELPER_PREFIX__ReadBoolean(record, 'chainPresentAfterCommand'); __HELPER_PREFIX__ReadBoolean(record, 'rulePresentAfterCommand'); return { ...record, kind } as __LINUX_NFTABLES_COMMAND_ROW_TYPE__; }
export function decode__LINUX_NFTABLES_STATUS_TYPE__(value: unknown): __LINUX_NFTABLES_STATUS_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'network Linux nftables lab status'); for (const field of __HELPER_PREFIX__LinuxNftablesStringFields) { __HELPER_PREFIX__ReadString(record, field); } __HELPER_PREFIX__ReadStringArray(record, 'evidenceRefs'); for (const field of __HELPER_PREFIX__LinuxNftablesBooleanFields) { __HELPER_PREFIX__ReadBoolean(record, field); } for (const field of __HELPER_PREFIX__LinuxNftablesFalseFields) { __HELPER_PREFIX__ReadRequiredBoolean(record, field, false); } const state = __HELPER_PREFIX__ReadLiteral(record, 'state', Object.values(__LINUX_NFTABLES_STATE_CONST__)); const commandEvidence = __HELPER_PREFIX__ReadRecordArray(record, 'commandEvidence', 'network Linux nftables lab command row').map((row) => decode__LINUX_NFTABLES_COMMAND_ROW_TYPE__(row)); __HELPER_PREFIX__RequireCountMatches(record, 'commandCount', commandEvidence.length); __HELPER_PREFIX__RequireCountMatches(record, 'requiredCommandCount', commandEvidence.length); if (state === __LINUX_NFTABLES_STATE_CONST__.ExecutedAndRolledBack) { const byKind = __HELPER_PREFIX__RequireUniqueRowsByKind(commandEvidence, 'Linux nftables lab'); __HELPER_PREFIX__RequireObservedFlags(record, byKind, __HELPER_PREFIX__LinuxNftablesObservedFlags, 'Linux nftables lab'); __HELPER_PREFIX__RequireLinuxNftablesOutcomes(byKind); } return { ...record, commandEvidence } as __LINUX_NFTABLES_STATUS_TYPE__; }
export function decode__WINDOWS_FIREWALL_COMMAND_ROW_TYPE__(value: unknown): __WINDOWS_FIREWALL_COMMAND_ROW_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'network Windows firewall lab command row'); const kind = __HELPER_PREFIX__ReadLiteral(record, 'kind', Object.values(__WINDOWS_FIREWALL_COMMAND_KIND_CONST__)); __HELPER_PREFIX__ReadString(record, 'commandRef'); __HELPER_PREFIX__ReadInteger(record, 'exitStatus'); __HELPER_PREFIX__ReadString(record, 'outputSha256'); __HELPER_PREFIX__ReadBoolean(record, 'rulePresentAfterCommand'); return { ...record, kind } as __WINDOWS_FIREWALL_COMMAND_ROW_TYPE__; }
export function decode__WINDOWS_FIREWALL_STATUS_TYPE__(value: unknown): __WINDOWS_FIREWALL_STATUS_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'network Windows firewall lab status'); for (const field of __HELPER_PREFIX__WindowsFirewallStringFields) { __HELPER_PREFIX__ReadString(record, field); } __HELPER_PREFIX__ReadStringArray(record, 'evidenceRefs'); for (const field of __HELPER_PREFIX__WindowsFirewallBooleanFields) { __HELPER_PREFIX__ReadBoolean(record, field); } for (const field of __HELPER_PREFIX__WindowsFirewallFalseFields) { __HELPER_PREFIX__ReadRequiredBoolean(record, field, false); } const state = __HELPER_PREFIX__ReadLiteral(record, 'state', Object.values(__WINDOWS_FIREWALL_STATE_CONST__)); const commandEvidence = __HELPER_PREFIX__ReadRecordArray(record, 'commandEvidence', 'network Windows firewall lab command row').map((row) => decode__WINDOWS_FIREWALL_COMMAND_ROW_TYPE__(row)); __HELPER_PREFIX__RequireCountMatches(record, 'commandCount', commandEvidence.length); __HELPER_PREFIX__RequireCountMatches(record, 'requiredCommandCount', commandEvidence.length); if (state === __WINDOWS_FIREWALL_STATE_CONST__.ExecutedAndRolledBack) { const byKind = __HELPER_PREFIX__RequireUniqueRowsByKind(commandEvidence, 'Windows firewall lab'); __HELPER_PREFIX__RequireObservedFlags(record, byKind, __HELPER_PREFIX__WindowsFirewallObservedFlags, 'Windows firewall lab'); __HELPER_PREFIX__RequireWindowsFirewallOutcomes(byKind); } return { ...record, commandEvidence } as __WINDOWS_FIREWALL_STATUS_TYPE__; }
export function decode__WINDOWS_WFP_STATUS_TYPE__(value: unknown): __WINDOWS_WFP_STATUS_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'network Windows WFP gate status'); for (const field of __HELPER_PREFIX__WindowsWfpStringFields) { __HELPER_PREFIX__ReadString(record, field); } for (const field of __HELPER_PREFIX__WindowsWfpNullableStringFields) { __HELPER_PREFIX__ReadNullableString(record, field); } for (const field of __HELPER_PREFIX__WindowsWfpStringArrayFields) { __HELPER_PREFIX__ReadStringArray(record, field); } const capabilityState = __HELPER_PREFIX__ReadLiteral(record, 'capabilityState', Object.values(__WINDOWS_WFP_CAPABILITY_STATE_CONST__)); const gateState = __HELPER_PREFIX__ReadLiteral(record, 'gateState', Object.values(__WINDOWS_WFP_GATE_STATE_CONST__)); const boundaryReasons = __HELPER_PREFIX__ReadStringArray(record, 'boundaryReasons'); const missingRequiredArtifacts = __HELPER_PREFIX__ReadStringArray(record, 'missingRequiredArtifacts'); const wfpLabProofReady = __HELPER_PREFIX__ReadBoolean(record, 'wfpLabProofReady'); for (const field of __HELPER_PREFIX__WindowsWfpFalseFields) { __HELPER_PREFIX__ReadRequiredBoolean(record, field, false); } __HELPER_PREFIX__RequireGateConsistency('Windows WFP', gateState === __WINDOWS_WFP_GATE_STATE_CONST__.LabProofReady, __HELPER_PREFIX__GateProofReadyIsValid(capabilityState === __WINDOWS_WFP_CAPABILITY_STATE_CONST__.LabReady, wfpLabProofReady, boundaryReasons, missingRequiredArtifacts), gateState === __WINDOWS_WFP_GATE_STATE_CONST__.ManualRequired, __HELPER_PREFIX__GateManualRequiredIsValid(capabilityState === __WINDOWS_WFP_CAPABILITY_STATE_CONST__.ManualRequired, wfpLabProofReady, boundaryReasons, missingRequiredArtifacts)); return { ...record, boundaryReasons, missingRequiredArtifacts, wfpLabProofReady, enforcementCommandPublished: false } as __WINDOWS_WFP_STATUS_TYPE__; }
export function decode__ANDROID_VPN_STATUS_TYPE__(value: unknown): __ANDROID_VPN_STATUS_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'network Android VpnService gate status'); for (const field of __HELPER_PREFIX__AndroidVpnStringFields) { __HELPER_PREFIX__ReadString(record, field); } for (const field of __HELPER_PREFIX__AndroidVpnNullableStringFields) { __HELPER_PREFIX__ReadNullableString(record, field); } __HELPER_PREFIX__ReadStringArray(record, 'evidenceRefs'); const capabilityState = __HELPER_PREFIX__ReadLiteral(record, 'capabilityState', Object.values(__ANDROID_VPN_CAPABILITY_STATE_CONST__)); const gateState = __HELPER_PREFIX__ReadLiteral(record, 'gateState', Object.values(__ANDROID_VPN_GATE_STATE_CONST__)); const boundaryReasons = __HELPER_PREFIX__ReadLiteralArray(record, 'boundaryReasons', Object.values(__ANDROID_VPN_BOUNDARY_REASON_CONST__)); const missingRequiredArtifacts = __HELPER_PREFIX__ReadLiteralArray(record, 'missingRequiredArtifacts', Object.values(__ANDROID_VPN_REQUIRED_ARTIFACT_CONST__)); for (const field of __HELPER_PREFIX__AndroidVpnBooleanFields) { __HELPER_PREFIX__ReadBoolean(record, field); } const physicalDeviceProofReady = __HELPER_PREFIX__ReadBoolean(record, 'physicalDeviceProofReady'); for (const field of __HELPER_PREFIX__AndroidVpnFalseFields) { __HELPER_PREFIX__ReadRequiredBoolean(record, field, false); } __HELPER_PREFIX__RequireGateConsistency('Android VpnService', gateState === __ANDROID_VPN_GATE_STATE_CONST__.PhysicalDeviceProofReady, __HELPER_PREFIX__GateProofReadyIsValid(capabilityState === __ANDROID_VPN_CAPABILITY_STATE_CONST__.PhysicalDeviceReady, physicalDeviceProofReady, boundaryReasons, missingRequiredArtifacts), gateState === __ANDROID_VPN_GATE_STATE_CONST__.ManualRequired, __HELPER_PREFIX__GateManualRequiredIsValid(capabilityState === __ANDROID_VPN_CAPABILITY_STATE_CONST__.ManualRequired, physicalDeviceProofReady, boundaryReasons, missingRequiredArtifacts)); return { ...record, boundaryReasons, missingRequiredArtifacts, gateState, physicalDeviceProofReady, enforcementCommandPublished: false } as __ANDROID_VPN_STATUS_TYPE__; }
export function decode__APPLE_NETWORK_EXTENSION_STATUS_TYPE__(value: unknown): __APPLE_NETWORK_EXTENSION_STATUS_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'network Apple Network Extension gate status'); for (const field of __HELPER_PREFIX__AppleNetworkExtensionStringFields) { __HELPER_PREFIX__ReadString(record, field); } for (const field of __HELPER_PREFIX__AppleNetworkExtensionNullableStringFields) { __HELPER_PREFIX__ReadNullableString(record, field); } __HELPER_PREFIX__ReadStringArray(record, 'evidenceRefs'); const platform = __HELPER_PREFIX__ReadLiteral(record, 'platform', Object.values(__APPLE_NETWORK_EXTENSION_PLATFORM_CONST__)); const capabilityState = __HELPER_PREFIX__ReadLiteral(record, 'capabilityState', Object.values(__APPLE_NETWORK_EXTENSION_CAPABILITY_STATE_CONST__)); const gateState = __HELPER_PREFIX__ReadLiteral(record, 'gateState', Object.values(__APPLE_NETWORK_EXTENSION_GATE_STATE_CONST__)); const boundaryReasons = __HELPER_PREFIX__ReadLiteralArray(record, 'boundaryReasons', Object.values(__APPLE_NETWORK_EXTENSION_BOUNDARY_REASON_CONST__)); const missingRequiredArtifacts = __HELPER_PREFIX__ReadLiteralArray(record, 'missingRequiredArtifacts', Object.values(__APPLE_NETWORK_EXTENSION_REQUIRED_ARTIFACT_CONST__)); for (const field of __HELPER_PREFIX__AppleNetworkExtensionBooleanFields) { __HELPER_PREFIX__ReadBoolean(record, field); } const appleEntitlementProofReady = __HELPER_PREFIX__ReadBoolean(record, 'appleEntitlementProofReady'); for (const field of __HELPER_PREFIX__AppleNetworkExtensionFalseFields) { __HELPER_PREFIX__ReadRequiredBoolean(record, field, false); } __HELPER_PREFIX__RequireGateConsistency('Apple Network Extension', gateState === __APPLE_NETWORK_EXTENSION_GATE_STATE_CONST__.AppleEntitlementProofReady, __HELPER_PREFIX__GateProofReadyIsValid(capabilityState === __APPLE_NETWORK_EXTENSION_CAPABILITY_STATE_CONST__.AppleDeviceReady, appleEntitlementProofReady, boundaryReasons, missingRequiredArtifacts), gateState === __APPLE_NETWORK_EXTENSION_GATE_STATE_CONST__.ManualRequired, __HELPER_PREFIX__GateManualRequiredIsValid(capabilityState === __APPLE_NETWORK_EXTENSION_CAPABILITY_STATE_CONST__.ManualRequired, appleEntitlementProofReady, boundaryReasons, missingRequiredArtifacts)); return { ...record, boundaryReasons, missingRequiredArtifacts, platform, gateState, appleEntitlementProofReady, enforcementCommandPublished: false } as __APPLE_NETWORK_EXTENSION_STATUS_TYPE__; }
export const __REMOTE_STATUS_SCHEMA_CONST__ = { safeParse(value: unknown): { readonly success: true; readonly data: __REMOTE_STATUS_TYPE__ } | { readonly success: false } { try { return { success: true, data: decode__REMOTE_STATUS_TYPE__(value) }; } catch { return { success: false }; } } } as const;
export const __LIVE_CAPTURE_ROW_SCHEMA_CONST__ = { safeParse(value: unknown): { readonly success: true; readonly data: __LIVE_CAPTURE_ROW_TYPE__ } | { readonly success: false } { try { return { success: true, data: decode__LIVE_CAPTURE_ROW_TYPE__(value) }; } catch { return { success: false }; } } } as const;
export const __LIVE_CAPTURE_STATUS_SCHEMA_CONST__ = { safeParse(value: unknown): { readonly success: true; readonly data: __LIVE_CAPTURE_STATUS_TYPE__ } | { readonly success: false } { try { return { success: true, data: decode__LIVE_CAPTURE_STATUS_TYPE__(value) }; } catch { return { success: false }; } } } as const;
export const __LINUX_NFTABLES_STATUS_SCHEMA_CONST__ = { safeParse(value: unknown): { readonly success: true; readonly data: __LINUX_NFTABLES_STATUS_TYPE__ } | { readonly success: false } { try { return { success: true, data: decode__LINUX_NFTABLES_STATUS_TYPE__(value) }; } catch { return { success: false }; } } } as const;
export const __WINDOWS_FIREWALL_STATUS_SCHEMA_CONST__ = { safeParse(value: unknown): { readonly success: true; readonly data: __WINDOWS_FIREWALL_STATUS_TYPE__ } | { readonly success: false } { try { return { success: true, data: decode__WINDOWS_FIREWALL_STATUS_TYPE__(value) }; } catch { return { success: false }; } } } as const;
export const __WINDOWS_WFP_STATUS_SCHEMA_CONST__ = { safeParse(value: unknown): { readonly success: true; readonly data: __WINDOWS_WFP_STATUS_TYPE__ } | { readonly success: false } { try { return { success: true, data: decode__WINDOWS_WFP_STATUS_TYPE__(value) }; } catch { return { success: false }; } } } as const;
export const __ANDROID_VPN_STATUS_SCHEMA_CONST__ = { safeParse(value: unknown): { readonly success: true; readonly data: __ANDROID_VPN_STATUS_TYPE__ } | { readonly success: false } { try { return { success: true, data: decode__ANDROID_VPN_STATUS_TYPE__(value) }; } catch { return { success: false }; } } } as const;
export const __APPLE_NETWORK_EXTENSION_STATUS_SCHEMA_CONST__ = { safeParse(value: unknown): { readonly success: true; readonly data: __APPLE_NETWORK_EXTENSION_STATUS_TYPE__ } | { readonly success: false } { try { return { success: true, data: decode__APPLE_NETWORK_EXTENSION_STATUS_TYPE__(value) }; } catch { return { success: false }; } } } as const;
"#
        .to_string(),
        &[
            ("__REMOTE_STATUS_STATE_CONST__", remote_status_state_const),
            (
                "__REMOTE_TRANSPORT_DISPATCH_STATE_CONST__",
                remote_transport_dispatch_state_const,
            ),
            (
                "__REMOTE_PROVIDER_CHILD_READINESS_STATE_CONST__",
                remote_provider_child_readiness_state_const,
            ),
            (
                "__REMOTE_CROSS_PROCESS_CUSTODY_READINESS_STATE_CONST__",
                remote_cross_process_custody_readiness_state_const,
            ),
            (
                "__REMOTE_EXTERNAL_CROSS_PROCESS_TRANSPORT_STATE_CONST__",
                remote_external_cross_process_transport_state_const,
            ),
            ("__LIVE_CAPTURE_PLATFORM_CONST__", live_capture_platform_const),
            ("__LIVE_CAPTURE_PROOF_STATE_CONST__", live_capture_proof_state_const),
            (
                "__LIVE_CAPTURE_STORAGE_STATE_CONST__",
                live_capture_storage_state_const,
            ),
            (
                "__LIVE_CAPTURE_EXECUTION_STATE_CONST__",
                live_capture_execution_state_const,
            ),
            ("__LINUX_NFTABLES_STATE_CONST__", linux_nftables_state_const),
            (
                "__LINUX_NFTABLES_COMMAND_KIND_CONST__",
                linux_nftables_command_kind_const,
            ),
            ("__WINDOWS_FIREWALL_STATE_CONST__", windows_firewall_state_const),
            (
                "__WINDOWS_FIREWALL_COMMAND_KIND_CONST__",
                windows_firewall_command_kind_const,
            ),
            ("__WINDOWS_WFP_GATE_STATE_CONST__", windows_wfp_gate_state_const),
            (
                "__WINDOWS_WFP_CAPABILITY_STATE_CONST__",
                windows_wfp_capability_state_const,
            ),
            ("__ANDROID_VPN_GATE_STATE_CONST__", android_vpn_gate_state_const),
            (
                "__ANDROID_VPN_CAPABILITY_STATE_CONST__",
                android_vpn_capability_state_const,
            ),
            (
                "__ANDROID_VPN_REQUIRED_ARTIFACT_CONST__",
                android_vpn_required_artifact_const,
            ),
            (
                "__ANDROID_VPN_BOUNDARY_REASON_CONST__",
                android_vpn_boundary_reason_const,
            ),
            (
                "__APPLE_NETWORK_EXTENSION_PLATFORM_CONST__",
                apple_network_extension_platform_const,
            ),
            (
                "__APPLE_NETWORK_EXTENSION_CAPABILITY_STATE_CONST__",
                apple_network_extension_capability_state_const,
            ),
            (
                "__APPLE_NETWORK_EXTENSION_GATE_STATE_CONST__",
                apple_network_extension_gate_state_const,
            ),
            (
                "__APPLE_NETWORK_EXTENSION_REQUIRED_ARTIFACT_CONST__",
                apple_network_extension_required_artifact_const,
            ),
            (
                "__APPLE_NETWORK_EXTENSION_BOUNDARY_REASON_CONST__",
                apple_network_extension_boundary_reason_const,
            ),
            ("__REMOTE_STATUS_TYPE__", &remote_status_type),
            ("__REMOTE_STATUS_SCHEMA_CONST__", &remote_status_schema_const),
            ("__LIVE_CAPTURE_ROW_TYPE__", &live_capture_row_type),
            ("__LIVE_CAPTURE_STATUS_TYPE__", &live_capture_status_type),
            (
                "__LIVE_CAPTURE_ROW_SCHEMA_CONST__",
                &live_capture_row_schema_const,
            ),
            (
                "__LIVE_CAPTURE_STATUS_SCHEMA_CONST__",
                &live_capture_status_schema_const,
            ),
            (
                "__LINUX_NFTABLES_COMMAND_ROW_TYPE__",
                &linux_nftables_command_row_type,
            ),
            (
                "__LINUX_NFTABLES_STATUS_TYPE__",
                &linux_nftables_status_type,
            ),
            (
                "__LINUX_NFTABLES_STATUS_SCHEMA_CONST__",
                &linux_nftables_status_schema_const,
            ),
            (
                "__WINDOWS_FIREWALL_COMMAND_ROW_TYPE__",
                &windows_firewall_command_row_type,
            ),
            (
                "__WINDOWS_FIREWALL_STATUS_TYPE__",
                &windows_firewall_status_type,
            ),
            (
                "__WINDOWS_FIREWALL_STATUS_SCHEMA_CONST__",
                &windows_firewall_status_schema_const,
            ),
            ("__WINDOWS_WFP_STATUS_TYPE__", &windows_wfp_status_type),
            (
                "__WINDOWS_WFP_STATUS_SCHEMA_CONST__",
                &windows_wfp_status_schema_const,
            ),
            ("__ANDROID_VPN_STATUS_TYPE__", &android_vpn_status_type),
            (
                "__ANDROID_VPN_STATUS_SCHEMA_CONST__",
                &android_vpn_status_schema_const,
            ),
            (
                "__APPLE_NETWORK_EXTENSION_STATUS_TYPE__",
                &apple_network_extension_status_type,
            ),
            (
                "__APPLE_NETWORK_EXTENSION_STATUS_SCHEMA_CONST__",
                &apple_network_extension_status_schema_const,
            ),
            ("__HELPER_PREFIX__", &helper_prefix),
        ],
    )
}

fn network_remote_delivery_status_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkRemoteDeliveryStatusState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "FixtureRequirementsRecordedButNotImplemented",
            value: NetworkRemoteDeliveryStatusState::FixtureRequirementsRecordedButNotImplemented,
        },
        ProtocolLiteralDescriptor {
            key: "ManualRequired",
            value: NetworkRemoteDeliveryStatusState::ManualRequired,
        },
    ]
}

fn network_remote_delivery_transport_dispatch_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkRemoteDeliveryTransportDispatchState>> {
    vec![ProtocolLiteralDescriptor {
        key: "ManualRequiredBlocked",
        value: NetworkRemoteDeliveryTransportDispatchState::ManualRequiredBlocked,
    }]
}

fn network_remote_delivery_provider_child_readiness_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkRemoteDeliveryProviderChildReadinessState>> {
    vec![ProtocolLiteralDescriptor {
        key: "ManualRequiredUnavailable",
        value: NetworkRemoteDeliveryProviderChildReadinessState::ManualRequiredUnavailable,
    }]
}

fn network_remote_delivery_cross_process_custody_readiness_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkRemoteDeliveryCrossProcessCustodyReadinessState>> {
    vec![ProtocolLiteralDescriptor {
        key: "ManualRequiredUnavailable",
        value: NetworkRemoteDeliveryCrossProcessCustodyReadinessState::ManualRequiredUnavailable,
    }]
}

fn network_remote_delivery_external_cross_process_transport_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkRemoteDeliveryExternalCrossProcessTransportState>> {
    vec![ProtocolLiteralDescriptor {
        key: "DeterministicEnvelopeAckRecorded",
        value: NetworkRemoteDeliveryExternalCrossProcessTransportState::DeterministicEnvelopeAckRecorded,
    }]
}

fn network_live_capture_platform_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkLiveCaptureStatusPlatform>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "WindowsNpcap",
            value: NetworkLiveCaptureStatusPlatform::WindowsNpcap,
        },
        ProtocolLiteralDescriptor {
            key: "LinuxLibpcap",
            value: NetworkLiveCaptureStatusPlatform::LinuxLibpcap,
        },
        ProtocolLiteralDescriptor {
            key: "MacosBpfLibpcap",
            value: NetworkLiveCaptureStatusPlatform::MacosBpfLibpcap,
        },
    ]
}

fn network_live_capture_proof_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkLiveCaptureProofStatusState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "ProofReady",
            value: NetworkLiveCaptureProofStatusState::ProofReady,
        },
        ProtocolLiteralDescriptor {
            key: "ManualRequired",
            value: NetworkLiveCaptureProofStatusState::ManualRequired,
        },
        ProtocolLiteralDescriptor {
            key: "Unavailable",
            value: NetworkLiveCaptureProofStatusState::Unavailable,
        },
        ProtocolLiteralDescriptor {
            key: "Degraded",
            value: NetworkLiveCaptureProofStatusState::Degraded,
        },
    ]
}

fn network_live_capture_storage_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkRawCaptureStorageStatusState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "CustodyReady",
            value: NetworkRawCaptureStorageStatusState::CustodyReady,
        },
        ProtocolLiteralDescriptor {
            key: "ManualRequired",
            value: NetworkRawCaptureStorageStatusState::ManualRequired,
        },
        ProtocolLiteralDescriptor {
            key: "Unavailable",
            value: NetworkRawCaptureStorageStatusState::Unavailable,
        },
        ProtocolLiteralDescriptor {
            key: "Degraded",
            value: NetworkRawCaptureStorageStatusState::Degraded,
        },
    ]
}

fn network_live_capture_execution_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkLiveCaptureExecutionStatusState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "ManualRequired",
            value: NetworkLiveCaptureExecutionStatusState::ManualRequired,
        },
        ProtocolLiteralDescriptor {
            key: "BoundedExecuted",
            value: NetworkLiveCaptureExecutionStatusState::BoundedExecuted,
        },
        ProtocolLiteralDescriptor {
            key: "Unavailable",
            value: NetworkLiveCaptureExecutionStatusState::Unavailable,
        },
        ProtocolLiteralDescriptor {
            key: "Degraded",
            value: NetworkLiveCaptureExecutionStatusState::Degraded,
        },
    ]
}

fn network_linux_nftables_status_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkLinuxNftablesLabStatusState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "ManualRequired",
            value: NetworkLinuxNftablesLabStatusState::ManualRequired,
        },
        ProtocolLiteralDescriptor {
            key: "ExecutedAndRolledBack",
            value: NetworkLinuxNftablesLabStatusState::ExecutedAndRolledBack,
        },
        ProtocolLiteralDescriptor {
            key: "Unavailable",
            value: NetworkLinuxNftablesLabStatusState::Unavailable,
        },
    ]
}

fn network_linux_nftables_command_kind_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkLinuxNftablesLabCommandStatusKind>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "CreateTable",
            value: NetworkLinuxNftablesLabCommandStatusKind::CreateTable,
        },
        ProtocolLiteralDescriptor {
            key: "CreateChain",
            value: NetworkLinuxNftablesLabCommandStatusKind::CreateChain,
        },
        ProtocolLiteralDescriptor {
            key: "AddRule",
            value: NetworkLinuxNftablesLabCommandStatusKind::AddRule,
        },
        ProtocolLiteralDescriptor {
            key: "VerifyRulePresent",
            value: NetworkLinuxNftablesLabCommandStatusKind::VerifyRulePresent,
        },
        ProtocolLiteralDescriptor {
            key: "DeleteTable",
            value: NetworkLinuxNftablesLabCommandStatusKind::DeleteTable,
        },
        ProtocolLiteralDescriptor {
            key: "VerifyTableRemoved",
            value: NetworkLinuxNftablesLabCommandStatusKind::VerifyTableRemoved,
        },
    ]
}

fn network_windows_firewall_status_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkWindowsFirewallLabStatusState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "ManualRequired",
            value: NetworkWindowsFirewallLabStatusState::ManualRequired,
        },
        ProtocolLiteralDescriptor {
            key: "ExecutedAndRolledBack",
            value: NetworkWindowsFirewallLabStatusState::ExecutedAndRolledBack,
        },
        ProtocolLiteralDescriptor {
            key: "Unavailable",
            value: NetworkWindowsFirewallLabStatusState::Unavailable,
        },
    ]
}

fn network_windows_firewall_command_kind_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkWindowsFirewallLabCommandStatusKind>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "ApplyRule",
            value: NetworkWindowsFirewallLabCommandStatusKind::ApplyRule,
        },
        ProtocolLiteralDescriptor {
            key: "VerifyRulePresent",
            value: NetworkWindowsFirewallLabCommandStatusKind::VerifyRulePresent,
        },
        ProtocolLiteralDescriptor {
            key: "RollbackRule",
            value: NetworkWindowsFirewallLabCommandStatusKind::RollbackRule,
        },
        ProtocolLiteralDescriptor {
            key: "VerifyRuleRemoved",
            value: NetworkWindowsFirewallLabCommandStatusKind::VerifyRuleRemoved,
        },
    ]
}

fn network_windows_wfp_gate_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkWindowsWfpGateStatusState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "ManualRequired",
            value: NetworkWindowsWfpGateStatusState::ManualRequired,
        },
        ProtocolLiteralDescriptor {
            key: "ResearchOnly",
            value: NetworkWindowsWfpGateStatusState::ResearchOnly,
        },
        ProtocolLiteralDescriptor {
            key: "Unavailable",
            value: NetworkWindowsWfpGateStatusState::Unavailable,
        },
        ProtocolLiteralDescriptor {
            key: "LabProofReady",
            value: NetworkWindowsWfpGateStatusState::LabProofReady,
        },
    ]
}

fn network_windows_wfp_capability_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkWindowsWfpGateCapabilityStatusState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "ManualRequired",
            value: NetworkWindowsWfpGateCapabilityStatusState::ManualRequired,
        },
        ProtocolLiteralDescriptor {
            key: "LabReady",
            value: NetworkWindowsWfpGateCapabilityStatusState::LabReady,
        },
        ProtocolLiteralDescriptor {
            key: "Unavailable",
            value: NetworkWindowsWfpGateCapabilityStatusState::Unavailable,
        },
    ]
}

fn network_android_vpn_gate_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkAndroidVpnServiceGateStatusState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "ManualRequired",
            value: NetworkAndroidVpnServiceGateStatusState::ManualRequired,
        },
        ProtocolLiteralDescriptor {
            key: "ResearchOnly",
            value: NetworkAndroidVpnServiceGateStatusState::ResearchOnly,
        },
        ProtocolLiteralDescriptor {
            key: "Unavailable",
            value: NetworkAndroidVpnServiceGateStatusState::Unavailable,
        },
        ProtocolLiteralDescriptor {
            key: "PhysicalDeviceProofReady",
            value: NetworkAndroidVpnServiceGateStatusState::PhysicalDeviceProofReady,
        },
    ]
}

fn network_android_vpn_capability_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkAndroidVpnServiceGateCapabilityStatusState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "PhysicalDeviceReady",
            value: NetworkAndroidVpnServiceGateCapabilityStatusState::PhysicalDeviceReady,
        },
        ProtocolLiteralDescriptor {
            key: "ManualRequired",
            value: NetworkAndroidVpnServiceGateCapabilityStatusState::ManualRequired,
        },
        ProtocolLiteralDescriptor {
            key: "Unavailable",
            value: NetworkAndroidVpnServiceGateCapabilityStatusState::Unavailable,
        },
    ]
}

fn network_android_vpn_required_artifact_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkAndroidVpnServiceGateRequiredArtifact>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "VpnServiceDeclaration",
            value: NetworkAndroidVpnServiceGateRequiredArtifact::VpnServiceDeclaration,
        },
        ProtocolLiteralDescriptor {
            key: "UserConsentProof",
            value: NetworkAndroidVpnServiceGateRequiredArtifact::UserConsentProof,
        },
        ProtocolLiteralDescriptor {
            key: "PhysicalDeviceProof",
            value: NetworkAndroidVpnServiceGateRequiredArtifact::PhysicalDeviceProof,
        },
        ProtocolLiteralDescriptor {
            key: "PackageIdentityProof",
            value: NetworkAndroidVpnServiceGateRequiredArtifact::PackageIdentityProof,
        },
        ProtocolLiteralDescriptor {
            key: "VirtualInterfaceProof",
            value: NetworkAndroidVpnServiceGateRequiredArtifact::VirtualInterfaceProof,
        },
        ProtocolLiteralDescriptor {
            key: "TrafficObservationProof",
            value: NetworkAndroidVpnServiceGateRequiredArtifact::TrafficObservationProof,
        },
        ProtocolLiteralDescriptor {
            key: "RollbackPlan",
            value: NetworkAndroidVpnServiceGateRequiredArtifact::RollbackPlan,
        },
        ProtocolLiteralDescriptor {
            key: "AuditEvent",
            value: NetworkAndroidVpnServiceGateRequiredArtifact::AuditEvent,
        },
        ProtocolLiteralDescriptor {
            key: "DeviceOwnerProof",
            value: NetworkAndroidVpnServiceGateRequiredArtifact::DeviceOwnerProof,
        },
    ]
}

fn network_android_vpn_boundary_reason_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkAndroidVpnServiceGateBoundaryReason>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "ResearchOnlyRequested",
            value: NetworkAndroidVpnServiceGateBoundaryReason::ResearchOnlyRequested,
        },
        ProtocolLiteralDescriptor {
            key: "CapabilityManualRequired",
            value: NetworkAndroidVpnServiceGateBoundaryReason::CapabilityManualRequired,
        },
        ProtocolLiteralDescriptor {
            key: "CapabilityUnavailable",
            value: NetworkAndroidVpnServiceGateBoundaryReason::CapabilityUnavailable,
        },
        ProtocolLiteralDescriptor {
            key: "EvidenceGradeBelowProofThreshold",
            value: NetworkAndroidVpnServiceGateBoundaryReason::EvidenceGradeBelowProofThreshold,
        },
        ProtocolLiteralDescriptor {
            key: "PolicyNotVpnServiceApproved",
            value: NetworkAndroidVpnServiceGateBoundaryReason::PolicyNotVpnServiceApproved,
        },
        ProtocolLiteralDescriptor {
            key: "MissingRequiredArtifact",
            value: NetworkAndroidVpnServiceGateBoundaryReason::MissingRequiredArtifact,
        },
    ]
}

fn network_apple_network_extension_platform_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkAppleNetworkExtensionPlatformStatus>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "MacOs",
            value: NetworkAppleNetworkExtensionPlatformStatus::MacOs,
        },
        ProtocolLiteralDescriptor {
            key: "Ios",
            value: NetworkAppleNetworkExtensionPlatformStatus::Ios,
        },
    ]
}

fn network_apple_network_extension_capability_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkAppleNetworkExtensionGateCapabilityStatusState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "AppleDeviceReady",
            value: NetworkAppleNetworkExtensionGateCapabilityStatusState::AppleDeviceReady,
        },
        ProtocolLiteralDescriptor {
            key: "ManualRequired",
            value: NetworkAppleNetworkExtensionGateCapabilityStatusState::ManualRequired,
        },
        ProtocolLiteralDescriptor {
            key: "Unavailable",
            value: NetworkAppleNetworkExtensionGateCapabilityStatusState::Unavailable,
        },
    ]
}

fn network_apple_network_extension_gate_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkAppleNetworkExtensionGateStatusState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "ResearchOnly",
            value: NetworkAppleNetworkExtensionGateStatusState::ResearchOnly,
        },
        ProtocolLiteralDescriptor {
            key: "ManualRequired",
            value: NetworkAppleNetworkExtensionGateStatusState::ManualRequired,
        },
        ProtocolLiteralDescriptor {
            key: "Unavailable",
            value: NetworkAppleNetworkExtensionGateStatusState::Unavailable,
        },
        ProtocolLiteralDescriptor {
            key: "AppleEntitlementProofReady",
            value: NetworkAppleNetworkExtensionGateStatusState::AppleEntitlementProofReady,
        },
    ]
}

fn network_apple_network_extension_required_artifact_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkAppleNetworkExtensionGateRequiredArtifact>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "DeveloperTeamProof",
            value: NetworkAppleNetworkExtensionGateRequiredArtifact::DeveloperTeamProof,
        },
        ProtocolLiteralDescriptor {
            key: "EntitlementApprovalProof",
            value: NetworkAppleNetworkExtensionGateRequiredArtifact::EntitlementApprovalProof,
        },
        ProtocolLiteralDescriptor {
            key: "ProvisioningProfileProof",
            value: NetworkAppleNetworkExtensionGateRequiredArtifact::ProvisioningProfileProof,
        },
        ProtocolLiteralDescriptor {
            key: "SigningProof",
            value: NetworkAppleNetworkExtensionGateRequiredArtifact::SigningProof,
        },
        ProtocolLiteralDescriptor {
            key: "DeviceOrTestflightProof",
            value: NetworkAppleNetworkExtensionGateRequiredArtifact::DeviceOrTestflightProof,
        },
        ProtocolLiteralDescriptor {
            key: "NetworkExtensionDeclaration",
            value: NetworkAppleNetworkExtensionGateRequiredArtifact::NetworkExtensionDeclaration,
        },
        ProtocolLiteralDescriptor {
            key: "ExtensionConfigurationProof",
            value: NetworkAppleNetworkExtensionGateRequiredArtifact::ExtensionConfigurationProof,
        },
        ProtocolLiteralDescriptor {
            key: "RollbackPlan",
            value: NetworkAppleNetworkExtensionGateRequiredArtifact::RollbackPlan,
        },
        ProtocolLiteralDescriptor {
            key: "AuditEvent",
            value: NetworkAppleNetworkExtensionGateRequiredArtifact::AuditEvent,
        },
        ProtocolLiteralDescriptor {
            key: "SupervisionOrMdmProof",
            value: NetworkAppleNetworkExtensionGateRequiredArtifact::SupervisionOrMdmProof,
        },
    ]
}

fn network_apple_network_extension_boundary_reason_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkAppleNetworkExtensionGateBoundaryReason>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "ResearchOnlyRequested",
            value: NetworkAppleNetworkExtensionGateBoundaryReason::ResearchOnlyRequested,
        },
        ProtocolLiteralDescriptor {
            key: "CapabilityManualRequired",
            value: NetworkAppleNetworkExtensionGateBoundaryReason::CapabilityManualRequired,
        },
        ProtocolLiteralDescriptor {
            key: "CapabilityUnavailable",
            value: NetworkAppleNetworkExtensionGateBoundaryReason::CapabilityUnavailable,
        },
        ProtocolLiteralDescriptor {
            key: "EvidenceGradeBelowProofThreshold",
            value: NetworkAppleNetworkExtensionGateBoundaryReason::EvidenceGradeBelowProofThreshold,
        },
        ProtocolLiteralDescriptor {
            key: "PolicyNotNetworkExtensionApproved",
            value:
                NetworkAppleNetworkExtensionGateBoundaryReason::PolicyNotNetworkExtensionApproved,
        },
        ProtocolLiteralDescriptor {
            key: "MissingRequiredArtifact",
            value: NetworkAppleNetworkExtensionGateBoundaryReason::MissingRequiredArtifact,
        },
    ]
}

include!("parent_agent_protocol_bridge_ts_part2.rs");

include!("parent_agent_protocol_bridge_ts_part3.rs");
