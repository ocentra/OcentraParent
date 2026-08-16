struct NetworkStatusContractNames {
    remote_status_state_const: String,
    remote_transport_dispatch_state_const: String,
    remote_provider_child_readiness_state_const: String,
    remote_cross_process_custody_readiness_state_const: String,
    remote_external_cross_process_transport_state_const: String,
    live_capture_platform_const: String,
    live_capture_proof_state_const: String,
    live_capture_storage_state_const: String,
    live_capture_execution_state_const: String,
    linux_nftables_state_const: String,
    linux_nftables_command_kind_const: String,
    windows_firewall_state_const: String,
    windows_firewall_command_kind_const: String,
    windows_wfp_gate_state_const: String,
    windows_wfp_capability_state_const: String,
    android_vpn_gate_state_const: String,
    android_vpn_capability_state_const: String,
    android_vpn_required_artifact_const: String,
    android_vpn_boundary_reason_const: String,
    apple_network_extension_platform_const: String,
    apple_network_extension_capability_state_const: String,
    apple_network_extension_gate_state_const: String,
    apple_network_extension_required_artifact_const: String,
    apple_network_extension_boundary_reason_const: String,
}

impl NetworkStatusContractNames {
    fn new(prefix: &str) -> Self {
        Self {
            remote_status_state_const: format!("{prefix}NetworkRemoteDeliveryStatusState"),
            remote_transport_dispatch_state_const: format!(
                "{prefix}NetworkRemoteDeliveryTransportDispatchState"
            ),
            remote_provider_child_readiness_state_const: format!(
                "{prefix}NetworkRemoteDeliveryProviderChildReadinessState"
            ),
            remote_cross_process_custody_readiness_state_const: format!(
                "{prefix}NetworkRemoteDeliveryCrossProcessCustodyReadinessState"
            ),
            remote_external_cross_process_transport_state_const: format!(
                "{prefix}NetworkRemoteDeliveryExternalCrossProcessTransportState"
            ),
            live_capture_platform_const: format!("{prefix}NetworkLiveCapturePlatform"),
            live_capture_proof_state_const: format!("{prefix}NetworkLiveCaptureProofState"),
            live_capture_storage_state_const: format!("{prefix}NetworkRawCaptureStorageState"),
            live_capture_execution_state_const: format!("{prefix}NetworkLiveCaptureExecutionState"),
            linux_nftables_state_const: format!("{prefix}NetworkLinuxNftablesLabState"),
            linux_nftables_command_kind_const: format!(
                "{prefix}NetworkLinuxNftablesLabCommandKind"
            ),
            windows_firewall_state_const: format!("{prefix}NetworkWindowsFirewallLabState"),
            windows_firewall_command_kind_const: format!(
                "{prefix}NetworkWindowsFirewallLabCommandKind"
            ),
            windows_wfp_gate_state_const: format!("{prefix}NetworkWindowsWfpGateState"),
            windows_wfp_capability_state_const: format!("{prefix}NetworkWindowsWfpCapabilityState"),
            android_vpn_gate_state_const: format!("{prefix}NetworkAndroidVpnServiceGateState"),
            android_vpn_capability_state_const: format!(
                "{prefix}NetworkAndroidVpnServiceCapabilityState"
            ),
            android_vpn_required_artifact_const: format!(
                "{prefix}NetworkAndroidVpnServiceRequiredArtifact"
            ),
            android_vpn_boundary_reason_const: format!(
                "{prefix}NetworkAndroidVpnServiceBoundaryReason"
            ),
            apple_network_extension_platform_const: format!(
                "{prefix}NetworkAppleNetworkExtensionPlatform"
            ),
            apple_network_extension_capability_state_const: format!(
                "{prefix}NetworkAppleNetworkExtensionCapabilityState"
            ),
            apple_network_extension_gate_state_const: format!(
                "{prefix}NetworkAppleNetworkExtensionGateState"
            ),
            apple_network_extension_required_artifact_const: format!(
                "{prefix}NetworkAppleNetworkExtensionRequiredArtifact"
            ),
            apple_network_extension_boundary_reason_const: format!(
                "{prefix}NetworkAppleNetworkExtensionBoundaryReason"
            ),
        }
    }
}

fn network_status_contract_typescript(names: &ProtocolBridgeNames) -> String {
    let prefix = bridge_prefix(names);
    let contract_names = NetworkStatusContractNames::new(prefix);
    let mut sections = network_status_literal_sections(&contract_names);
    sections.push(network_status_contract_decoders_typescript(
        prefix,
        &contract_names,
    ));
    sections.join(" ")
}

fn network_status_literal_sections(contract_names: &NetworkStatusContractNames) -> Vec<String> {
    let mut sections = network_status_remote_literal_sections(contract_names);
    sections.extend(network_status_platform_literal_sections(contract_names));
    sections
}

fn network_status_remote_literal_sections(
    contract_names: &NetworkStatusContractNames,
) -> Vec<String> {
    vec![
        literal_typescript(
            &contract_names.remote_status_state_const,
            &contract_names.remote_status_state_const,
            &network_remote_delivery_status_state_descriptors(),
        ),
        literal_typescript(
            &contract_names.remote_transport_dispatch_state_const,
            &contract_names.remote_transport_dispatch_state_const,
            &network_remote_delivery_transport_dispatch_state_descriptors(),
        ),
        literal_typescript(
            &contract_names.remote_provider_child_readiness_state_const,
            &contract_names.remote_provider_child_readiness_state_const,
            &network_remote_delivery_provider_child_readiness_state_descriptors(),
        ),
        literal_typescript(
            &contract_names.remote_cross_process_custody_readiness_state_const,
            &contract_names.remote_cross_process_custody_readiness_state_const,
            &network_remote_delivery_cross_process_custody_readiness_state_descriptors(),
        ),
        literal_typescript(
            &contract_names.remote_external_cross_process_transport_state_const,
            &contract_names.remote_external_cross_process_transport_state_const,
            &network_remote_delivery_external_cross_process_transport_state_descriptors(),
        ),
        literal_typescript(
            &contract_names.live_capture_platform_const,
            &contract_names.live_capture_platform_const,
            &network_live_capture_platform_descriptors(),
        ),
        literal_typescript(
            &contract_names.live_capture_proof_state_const,
            &contract_names.live_capture_proof_state_const,
            &network_live_capture_proof_state_descriptors(),
        ),
        literal_typescript(
            &contract_names.live_capture_storage_state_const,
            &contract_names.live_capture_storage_state_const,
            &network_live_capture_storage_state_descriptors(),
        ),
        literal_typescript(
            &contract_names.live_capture_execution_state_const,
            &contract_names.live_capture_execution_state_const,
            &network_live_capture_execution_state_descriptors(),
        ),
    ]
}

fn network_status_platform_literal_sections(
    contract_names: &NetworkStatusContractNames,
) -> Vec<String> {
    let mut sections = network_status_lab_literal_sections(contract_names);
    sections.extend(network_status_gate_literal_sections(contract_names));
    sections
}

fn network_status_lab_literal_sections(contract_names: &NetworkStatusContractNames) -> Vec<String> {
    vec![
        literal_typescript(
            &contract_names.linux_nftables_state_const,
            &contract_names.linux_nftables_state_const,
            &network_linux_nftables_status_state_descriptors(),
        ),
        literal_typescript(
            &contract_names.linux_nftables_command_kind_const,
            &contract_names.linux_nftables_command_kind_const,
            &network_linux_nftables_command_kind_descriptors(),
        ),
        literal_typescript(
            &contract_names.windows_firewall_state_const,
            &contract_names.windows_firewall_state_const,
            &network_windows_firewall_status_state_descriptors(),
        ),
        literal_typescript(
            &contract_names.windows_firewall_command_kind_const,
            &contract_names.windows_firewall_command_kind_const,
            &network_windows_firewall_command_kind_descriptors(),
        ),
        literal_typescript(
            &contract_names.windows_wfp_gate_state_const,
            &contract_names.windows_wfp_gate_state_const,
            &network_windows_wfp_gate_state_descriptors(),
        ),
        literal_typescript(
            &contract_names.windows_wfp_capability_state_const,
            &contract_names.windows_wfp_capability_state_const,
            &network_windows_wfp_capability_state_descriptors(),
        ),
    ]
}

fn network_status_gate_literal_sections(
    contract_names: &NetworkStatusContractNames,
) -> Vec<String> {
    vec![
        literal_typescript(
            &contract_names.android_vpn_gate_state_const,
            &contract_names.android_vpn_gate_state_const,
            &network_android_vpn_gate_state_descriptors(),
        ),
        literal_typescript(
            &contract_names.android_vpn_capability_state_const,
            &contract_names.android_vpn_capability_state_const,
            &network_android_vpn_capability_state_descriptors(),
        ),
        literal_typescript(
            &contract_names.android_vpn_required_artifact_const,
            &contract_names.android_vpn_required_artifact_const,
            &network_android_vpn_required_artifact_descriptors(),
        ),
        literal_typescript(
            &contract_names.android_vpn_boundary_reason_const,
            &contract_names.android_vpn_boundary_reason_const,
            &network_android_vpn_boundary_reason_descriptors(),
        ),
        literal_typescript(
            &contract_names.apple_network_extension_platform_const,
            &contract_names.apple_network_extension_platform_const,
            &network_apple_network_extension_platform_descriptors(),
        ),
        literal_typescript(
            &contract_names.apple_network_extension_capability_state_const,
            &contract_names.apple_network_extension_capability_state_const,
            &network_apple_network_extension_capability_state_descriptors(),
        ),
        literal_typescript(
            &contract_names.apple_network_extension_gate_state_const,
            &contract_names.apple_network_extension_gate_state_const,
            &network_apple_network_extension_gate_state_descriptors(),
        ),
        literal_typescript(
            &contract_names.apple_network_extension_required_artifact_const,
            &contract_names.apple_network_extension_required_artifact_const,
            &network_apple_network_extension_required_artifact_descriptors(),
        ),
        literal_typescript(
            &contract_names.apple_network_extension_boundary_reason_const,
            &contract_names.apple_network_extension_boundary_reason_const,
            &network_apple_network_extension_boundary_reason_descriptors(),
        ),
    ]
}

struct NetworkStatusDecoderNames {
    remote_status_type: String,
    remote_status_schema_const: String,
    live_capture_row_type: String,
    live_capture_status_type: String,
    live_capture_row_schema_const: String,
    live_capture_status_schema_const: String,
    linux_nftables_command_row_type: String,
    linux_nftables_status_type: String,
    linux_nftables_status_schema_const: String,
    windows_firewall_command_row_type: String,
    windows_firewall_status_type: String,
    windows_firewall_status_schema_const: String,
    windows_wfp_status_type: String,
    windows_wfp_status_schema_const: String,
    android_vpn_status_type: String,
    android_vpn_status_schema_const: String,
    apple_network_extension_status_type: String,
    apple_network_extension_status_schema_const: String,
    helper_prefix: String,
}

impl NetworkStatusDecoderNames {
    fn new(prefix: &str) -> Self {
        Self {
            remote_status_type: format!("{prefix}NetworkRemoteDeliveryStatus"),
            remote_status_schema_const: format!("{prefix}NetworkRemoteDeliveryStatusSchema"),
            live_capture_row_type: format!("{prefix}NetworkLiveCaptureStatusRow"),
            live_capture_status_type: format!("{prefix}NetworkLiveCaptureStatus"),
            live_capture_row_schema_const: format!("{prefix}NetworkLiveCaptureStatusRowSchema"),
            live_capture_status_schema_const: format!("{prefix}NetworkLiveCaptureStatusSchema"),
            linux_nftables_command_row_type: format!("{prefix}NetworkLinuxNftablesLabCommandRow"),
            linux_nftables_status_type: format!("{prefix}NetworkLinuxNftablesLabStatus"),
            linux_nftables_status_schema_const: format!(
                "{prefix}NetworkLinuxNftablesLabStatusSchema"
            ),
            windows_firewall_command_row_type: format!(
                "{prefix}NetworkWindowsFirewallLabCommandRow"
            ),
            windows_firewall_status_type: format!("{prefix}NetworkWindowsFirewallLabStatus"),
            windows_firewall_status_schema_const: format!(
                "{prefix}NetworkWindowsFirewallLabStatusSchema"
            ),
            windows_wfp_status_type: format!("{prefix}NetworkWindowsWfpGateStatus"),
            windows_wfp_status_schema_const: format!("{prefix}NetworkWindowsWfpGateStatusSchema"),
            android_vpn_status_type: format!("{prefix}NetworkAndroidVpnServiceGateStatus"),
            android_vpn_status_schema_const: format!(
                "{prefix}NetworkAndroidVpnServiceGateStatusSchema"
            ),
            apple_network_extension_status_type: format!(
                "{prefix}NetworkAppleNetworkExtensionGateStatus"
            ),
            apple_network_extension_status_schema_const: format!(
                "{prefix}NetworkAppleNetworkExtensionGateStatusSchema"
            ),
            helper_prefix: format!("__{prefix}NetworkStatus"),
        }
    }
}

fn network_status_contract_decoders_typescript(
    prefix: &str,
    contract_names: &NetworkStatusContractNames,
) -> String {
    let decoder_names = NetworkStatusDecoderNames::new(prefix);
    let tokens = network_status_decoder_tokens(contract_names, &decoder_names);
    replace_tokens(
        parent_agent_protocol_bridge_ts_network_status_01_template(),
        &tokens,
    )
}

fn network_status_decoder_tokens<'a>(
    contract_names: &'a NetworkStatusContractNames,
    decoder_names: &'a NetworkStatusDecoderNames,
) -> Vec<(&'static str, &'a str)> {
    let mut tokens = network_status_decoder_const_tokens(contract_names);
    tokens.extend(network_status_decoder_type_tokens(decoder_names));
    tokens
}

fn network_status_decoder_const_tokens(
    contract_names: &NetworkStatusContractNames,
) -> Vec<(&'static str, &str)> {
    let mut tokens = network_status_decoder_remote_tokens(contract_names);
    tokens.extend(network_status_decoder_platform_tokens(contract_names));
    tokens
}

fn network_status_decoder_remote_tokens(
    contract_names: &NetworkStatusContractNames,
) -> Vec<(&'static str, &str)> {
    vec![
        (
            "__REMOTE_STATUS_STATE_CONST__",
            &contract_names.remote_status_state_const,
        ),
        (
            "__REMOTE_TRANSPORT_DISPATCH_STATE_CONST__",
            &contract_names.remote_transport_dispatch_state_const,
        ),
        (
            "__REMOTE_PROVIDER_CHILD_READINESS_STATE_CONST__",
            &contract_names.remote_provider_child_readiness_state_const,
        ),
        (
            "__REMOTE_CROSS_PROCESS_CUSTODY_READINESS_STATE_CONST__",
            &contract_names.remote_cross_process_custody_readiness_state_const,
        ),
        (
            "__REMOTE_EXTERNAL_CROSS_PROCESS_TRANSPORT_STATE_CONST__",
            &contract_names.remote_external_cross_process_transport_state_const,
        ),
        (
            "__LIVE_CAPTURE_PLATFORM_CONST__",
            &contract_names.live_capture_platform_const,
        ),
        (
            "__LIVE_CAPTURE_PROOF_STATE_CONST__",
            &contract_names.live_capture_proof_state_const,
        ),
        (
            "__LIVE_CAPTURE_STORAGE_STATE_CONST__",
            &contract_names.live_capture_storage_state_const,
        ),
        (
            "__LIVE_CAPTURE_EXECUTION_STATE_CONST__",
            &contract_names.live_capture_execution_state_const,
        ),
    ]
}

fn network_status_decoder_platform_tokens(
    contract_names: &NetworkStatusContractNames,
) -> Vec<(&'static str, &str)> {
    vec![
        (
            "__LINUX_NFTABLES_STATE_CONST__",
            &contract_names.linux_nftables_state_const,
        ),
        (
            "__LINUX_NFTABLES_COMMAND_KIND_CONST__",
            &contract_names.linux_nftables_command_kind_const,
        ),
        (
            "__WINDOWS_FIREWALL_STATE_CONST__",
            &contract_names.windows_firewall_state_const,
        ),
        (
            "__WINDOWS_FIREWALL_COMMAND_KIND_CONST__",
            &contract_names.windows_firewall_command_kind_const,
        ),
        (
            "__WINDOWS_WFP_GATE_STATE_CONST__",
            &contract_names.windows_wfp_gate_state_const,
        ),
        (
            "__WINDOWS_WFP_CAPABILITY_STATE_CONST__",
            &contract_names.windows_wfp_capability_state_const,
        ),
        (
            "__ANDROID_VPN_GATE_STATE_CONST__",
            &contract_names.android_vpn_gate_state_const,
        ),
        (
            "__ANDROID_VPN_CAPABILITY_STATE_CONST__",
            &contract_names.android_vpn_capability_state_const,
        ),
        (
            "__ANDROID_VPN_REQUIRED_ARTIFACT_CONST__",
            &contract_names.android_vpn_required_artifact_const,
        ),
        (
            "__ANDROID_VPN_BOUNDARY_REASON_CONST__",
            &contract_names.android_vpn_boundary_reason_const,
        ),
        (
            "__APPLE_NETWORK_EXTENSION_PLATFORM_CONST__",
            &contract_names.apple_network_extension_platform_const,
        ),
        (
            "__APPLE_NETWORK_EXTENSION_CAPABILITY_STATE_CONST__",
            &contract_names.apple_network_extension_capability_state_const,
        ),
        (
            "__APPLE_NETWORK_EXTENSION_GATE_STATE_CONST__",
            &contract_names.apple_network_extension_gate_state_const,
        ),
        (
            "__APPLE_NETWORK_EXTENSION_REQUIRED_ARTIFACT_CONST__",
            &contract_names.apple_network_extension_required_artifact_const,
        ),
        (
            "__APPLE_NETWORK_EXTENSION_BOUNDARY_REASON_CONST__",
            &contract_names.apple_network_extension_boundary_reason_const,
        ),
    ]
}

fn network_status_decoder_type_tokens(
    decoder_names: &NetworkStatusDecoderNames,
) -> Vec<(&'static str, &str)> {
    vec![
        ("__REMOTE_STATUS_TYPE__", &decoder_names.remote_status_type),
        (
            "__REMOTE_STATUS_SCHEMA_CONST__",
            &decoder_names.remote_status_schema_const,
        ),
        (
            "__LIVE_CAPTURE_ROW_TYPE__",
            &decoder_names.live_capture_row_type,
        ),
        (
            "__LIVE_CAPTURE_STATUS_TYPE__",
            &decoder_names.live_capture_status_type,
        ),
        (
            "__LIVE_CAPTURE_ROW_SCHEMA_CONST__",
            &decoder_names.live_capture_row_schema_const,
        ),
        (
            "__LIVE_CAPTURE_STATUS_SCHEMA_CONST__",
            &decoder_names.live_capture_status_schema_const,
        ),
        (
            "__LINUX_NFTABLES_COMMAND_ROW_TYPE__",
            &decoder_names.linux_nftables_command_row_type,
        ),
        (
            "__LINUX_NFTABLES_STATUS_TYPE__",
            &decoder_names.linux_nftables_status_type,
        ),
        (
            "__LINUX_NFTABLES_STATUS_SCHEMA_CONST__",
            &decoder_names.linux_nftables_status_schema_const,
        ),
        (
            "__WINDOWS_FIREWALL_COMMAND_ROW_TYPE__",
            &decoder_names.windows_firewall_command_row_type,
        ),
        (
            "__WINDOWS_FIREWALL_STATUS_TYPE__",
            &decoder_names.windows_firewall_status_type,
        ),
        (
            "__WINDOWS_FIREWALL_STATUS_SCHEMA_CONST__",
            &decoder_names.windows_firewall_status_schema_const,
        ),
        (
            "__WINDOWS_WFP_STATUS_TYPE__",
            &decoder_names.windows_wfp_status_type,
        ),
        (
            "__WINDOWS_WFP_STATUS_SCHEMA_CONST__",
            &decoder_names.windows_wfp_status_schema_const,
        ),
        (
            "__ANDROID_VPN_STATUS_TYPE__",
            &decoder_names.android_vpn_status_type,
        ),
        (
            "__ANDROID_VPN_STATUS_SCHEMA_CONST__",
            &decoder_names.android_vpn_status_schema_const,
        ),
        (
            "__APPLE_NETWORK_EXTENSION_STATUS_TYPE__",
            &decoder_names.apple_network_extension_status_type,
        ),
        (
            "__APPLE_NETWORK_EXTENSION_STATUS_SCHEMA_CONST__",
            &decoder_names.apple_network_extension_status_schema_const,
        ),
        ("__HELPER_PREFIX__", &decoder_names.helper_prefix),
    ]
}
