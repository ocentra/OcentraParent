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
    lan_discovery_event_kind_const: &'static str,
    lan_discovery_event_kind_type: &'static str,
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
        &parent_bridge_names(),
        Some(parent_route_event_id_typescript()),
    )
}

pub(crate) fn parent_agent_protocol_domain_contracts_typescript() -> String {
    protocol_bridge_typescript(
        &parent_bridge_names(),
        Some(standalone_parent_route_event_id_typescript()),
    )
}

pub(crate) fn generated_portal_agent_protocol_bridge_typescript() -> String {
    protocol_bridge_typescript(&generated_portal_bridge_names(), None)
}

fn protocol_bridge_typescript(
    names: &ProtocolBridgeNames,
    event_id_types: Option<String>,
) -> String {
    let mut sections = vec![
        runtime_typescript(names),
        transport_typescript(names),
        field_typescript(names),
        runtime_event_contract_typescript(names),
        network_status_ref_typescript(names),
        network_status_contract_typescript(names),
        tracking_retention_settings_write_typescript(names),
        delimiter_typescript(names),
        literal_typescript(
            names.command_const,
            names.command_type,
            &command_descriptors(),
        ),
        literal_typescript(names.event_const, names.event_type, &event_descriptors()),
        activity_surface_contract_typescript(names),
        lan_value_typescript(names),
    ];
    if let Some(event_id_types) = event_id_types {
        sections.push(event_id_types);
    }
    sections
        .join(" ")
        .replace("\n\n export const", "\n export const")
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
        lan_discovery_event_kind_const: "ParentAgentLanDiscoveryEventKind",
        lan_discovery_event_kind_type: "ParentAgentLanDiscoveryEventKind",
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
        lan_discovery_event_kind_const: "GeneratedPortalAgentLanDiscoveryEventKind",
        lan_discovery_event_kind_type: "GeneratedPortalAgentLanDiscoveryEventKind",
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

include!("parent_agent_protocol_bridge_ts_runtime.rs");
include!("parent_agent_protocol_bridge_ts_runtime_contracts.rs");
include!("parent_agent_protocol_bridge_ts_browser_descriptors.rs");
include!("parent_agent_protocol_bridge_ts_network_descriptors.rs");
include!("parent_agent_protocol_bridge_ts_network_status_contracts.rs");
include!("parent_agent_protocol_bridge_ts_remote_delivery.rs");
include!("parent_agent_protocol_bridge_ts_platform_descriptors.rs");
include!("parent_agent_protocol_bridge_ts_part2.rs");
include!("parent_agent_protocol_bridge_ts_part3.rs");
