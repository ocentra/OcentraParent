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

fn parent_agent_protocol_bridge_ts_network_status_01_template() -> String {
    [
        include_str!("parent_agent_protocol_bridge_ts_network_descriptors_parent_agent_protocol_bridge_ts_network_status_01_01.template.txt"),
        include_str!("parent_agent_protocol_bridge_ts_network_descriptors_parent_agent_protocol_bridge_ts_network_status_01_02.template.txt"),
        include_str!("parent_agent_protocol_bridge_ts_network_descriptors_parent_agent_protocol_bridge_ts_network_status_01_03.template.txt"),
        include_str!("parent_agent_protocol_bridge_ts_network_descriptors_parent_agent_protocol_bridge_ts_network_status_01_04_helpers.template.txt"),
        include_str!("parent_agent_protocol_bridge_ts_network_descriptors_parent_agent_protocol_bridge_ts_network_status_01_04_remote-linux.template.txt"),
        include_str!("parent_agent_protocol_bridge_ts_network_descriptors_parent_agent_protocol_bridge_ts_network_status_01_04_platform.template.txt"),
        include_str!("parent_agent_protocol_bridge_ts_network_descriptors_parent_agent_protocol_bridge_ts_network_status_01_05.template.txt"),
    ]
    .concat()
}
