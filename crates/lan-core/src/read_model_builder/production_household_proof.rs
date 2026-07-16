use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingProductionDiscoveryState;
use ocentra_parent_agent_protocol::lan_pairing::LanTrustedDeviceRegistryEntry;
use ocentra_parent_agent_protocol::lan_pairing::V09ProductionDiscoveryHouseholdProofState;
use ocentra_parent_agent_protocol::lan_pairing::V09ProductionDiscoveryHouseholdRuntimeOwner;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::production_household_proof::LanProductionHouseholdProofCapability;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::production_household_proof::LanProductionHouseholdProofStatus;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::production_household_proof::LanProductionHouseholdProofSummary;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceScanSummary;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanHouseholdDeviceActionKind;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanHouseholdDeviceDecision;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanSelectedDeviceReadiness;

pub(super) fn production_household_proof_summary(
    generated_at: &str,
    physical_household_lan_state: LanPairingProductionDiscoveryState,
    scan_summary: &LanBrowserAddDeviceScanSummary,
    trusted_device_registry: &[LanTrustedDeviceRegistryEntry],
    household_device_decisions: &[LanHouseholdDeviceDecision],
    selected_device_readiness: &LanSelectedDeviceReadiness,
) -> LanProductionHouseholdProofSummary {
    let status_rows = production_household_proof_rows(
        physical_household_lan_state,
        scan_summary,
        trusted_device_registry,
        household_device_decisions,
        selected_device_readiness,
    );

    let manual_proof_required = status_rows
        .iter()
        .filter(|row| row.proof_state == V09ProductionDiscoveryHouseholdProofState::ManualRequired)
        .map(|row| row.capability.clone())
        .collect();
    let not_implemented = status_rows
        .iter()
        .filter(|row| row.proof_state == V09ProductionDiscoveryHouseholdProofState::NotImplemented)
        .map(|row| row.capability.clone())
        .collect();

    LanProductionHouseholdProofSummary {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        generated_at: generated_at.to_string(),
        status_rows,
        manual_proof_required,
        not_implemented,
        claims_proved: vec![
            constants::lan_pairing::PRODUCTION_PROOF_CLAIM_PASSIVE_NEIGHBOR.to_string(),
            constants::lan_pairing::PRODUCTION_PROOF_CLAIM_REGISTRY_ROUTE.to_string(),
        ],
        claims_not_proved: vec![
            constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_PHYSICAL.to_string(),
            constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_SIGNED.to_string(),
            constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_CLOUD.to_string(),
            constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_ANDROID.to_string(),
            constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_IOS.to_string(),
            constants::lan_pairing::PRODUCTION_PROOF_NON_CLAIM_STORE.to_string(),
        ],
    }
}

fn production_household_proof_rows(
    physical_household_lan_state: LanPairingProductionDiscoveryState,
    scan_summary: &LanBrowserAddDeviceScanSummary,
    trusted_device_registry: &[LanTrustedDeviceRegistryEntry],
    household_device_decisions: &[LanHouseholdDeviceDecision],
    selected_device_readiness: &LanSelectedDeviceReadiness,
) -> Vec<LanProductionHouseholdProofStatus> {
    [
        production_discovery_status_rows(
            physical_household_lan_state,
            scan_summary,
            trusted_device_registry,
        ),
        production_decision_status_rows(household_device_decisions, trusted_device_registry),
        production_route_status_rows(selected_device_readiness),
        production_manual_platform_status_rows(),
    ]
    .concat()
}

fn production_discovery_status_rows(
    physical_household_lan_state: LanPairingProductionDiscoveryState,
    scan_summary: &LanBrowserAddDeviceScanSummary,
    trusted_device_registry: &[LanTrustedDeviceRegistryEntry],
) -> Vec<LanProductionHouseholdProofStatus> {
    vec![
        manual_status(
            LanProductionHouseholdProofCapability::SignedLanHello,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_SIGNED_HELLO,
            constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_SIGNED_HELLO,
        ),
        manual_status(
            LanProductionHouseholdProofCapability::SignedLanHeartbeat,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_SIGNED_HEARTBEAT,
            constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_SIGNED_HEARTBEAT,
        ),
        ci_status(
            LanProductionHouseholdProofCapability::PassiveNeighborDiscovery,
            physical_household_lan_state,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_PASSIVE_NEIGHBOR,
        ),
        ci_status(
            LanProductionHouseholdProofCapability::RouterNeighborDiscovery,
            router_neighbor_state(scan_summary),
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_ROUTER_NEIGHBOR,
        ),
        manual_status(
            LanProductionHouseholdProofCapability::MdnsNameDiscovery,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_MDNS,
            constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_MDNS,
        ),
        manual_status(
            LanProductionHouseholdProofCapability::SsdpNameDiscovery,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_SSDP,
            constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_SSDP,
        ),
        manual_status(
            LanProductionHouseholdProofCapability::RouterDhcpNameDiscovery,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_ROUTER_DHCP,
            constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_ROUTER_DHCP,
        ),
        ci_status(
            LanProductionHouseholdProofCapability::TrustedRegistry,
            registry_state(trusted_device_registry),
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_TRUSTED_REGISTRY,
        ),
    ]
}

fn production_decision_status_rows(
    household_device_decisions: &[LanHouseholdDeviceDecision],
    trusted_device_registry: &[LanTrustedDeviceRegistryEntry],
) -> Vec<LanProductionHouseholdProofStatus> {
    vec![
        ci_status(
            LanProductionHouseholdProofCapability::ParentAssignment,
            decision_state(
                household_device_decisions,
                &LanHouseholdDeviceActionKind::Assign,
            ),
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_PARENT_ASSIGNMENT,
        ),
        ci_status(
            LanProductionHouseholdProofCapability::ParentRename,
            decision_state(
                household_device_decisions,
                &LanHouseholdDeviceActionKind::Rename,
            ),
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_PARENT_RENAME,
        ),
        ci_status(
            LanProductionHouseholdProofCapability::ParentIgnore,
            decision_state(
                household_device_decisions,
                &LanHouseholdDeviceActionKind::Ignore,
            ),
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_PARENT_IGNORE,
        ),
        ci_status(
            LanProductionHouseholdProofCapability::ParentRevocation,
            revocation_state(trusted_device_registry, household_device_decisions),
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_PARENT_REVOCATION,
        ),
    ]
}

fn production_route_status_rows(
    selected_device_readiness: &LanSelectedDeviceReadiness,
) -> Vec<LanProductionHouseholdProofStatus> {
    vec![
        ci_status(
            LanProductionHouseholdProofCapability::RouteCustody,
            route_custody_state(selected_device_readiness),
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_ROUTE_CUSTODY,
        ),
        ci_status(
            LanProductionHouseholdProofCapability::StaleSelectedDevice,
            stale_selected_state(selected_device_readiness),
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_STALE_SELECTED,
        ),
        ci_status(
            LanProductionHouseholdProofCapability::OfflineSelectedDevice,
            offline_selected_state(selected_device_readiness),
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_OFFLINE_SELECTED,
        ),
        not_implemented_status(
            LanProductionHouseholdProofCapability::RelayRoute,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_RELAY_ROUTE,
        ),
        not_implemented_status(
            LanProductionHouseholdProofCapability::CacheRoute,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_CACHE_ROUTE,
        ),
    ]
}

fn production_manual_platform_status_rows() -> Vec<LanProductionHouseholdProofStatus> {
    vec![
        manual_status(
            LanProductionHouseholdProofCapability::SecondPhysicalChildAgent,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_SECOND_PHYSICAL_AGENT,
            constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_SECOND_PHYSICAL_AGENT,
        ),
        manual_status(
            LanProductionHouseholdProofCapability::AndroidChildAgentParity,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_ANDROID_PARITY,
            constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_ANDROID_PARITY,
        ),
        manual_status(
            LanProductionHouseholdProofCapability::IosChildAgentParity,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_IOS_PARITY,
            constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_IOS_PARITY,
        ),
        manual_status(
            LanProductionHouseholdProofCapability::StoreSigning,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_STORE_SIGNING,
            constants::lan_pairing::PRODUCTION_PROOF_ARTIFACT_STORE_SIGNING,
        ),
    ]
}

fn ci_status(
    capability: LanProductionHouseholdProofCapability,
    discovery_state: LanPairingProductionDiscoveryState,
    evidence_label: &str,
) -> LanProductionHouseholdProofStatus {
    production_status(
        capability,
        discovery_state,
        V09ProductionDiscoveryHouseholdProofState::CiMechanicalProof,
        V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel,
        evidence_label,
        None,
    )
}

fn manual_status(
    capability: LanProductionHouseholdProofCapability,
    evidence_label: &str,
    required_artifact_summary: &str,
) -> LanProductionHouseholdProofStatus {
    production_status(
        capability,
        LanPairingProductionDiscoveryState::ManualRequired,
        V09ProductionDiscoveryHouseholdProofState::ManualRequired,
        V09ProductionDiscoveryHouseholdRuntimeOwner::ManualProof,
        evidence_label,
        Some(required_artifact_summary.to_string()),
    )
}

fn not_implemented_status(
    capability: LanProductionHouseholdProofCapability,
    evidence_label: &str,
) -> LanProductionHouseholdProofStatus {
    production_status(
        capability,
        LanPairingProductionDiscoveryState::Unavailable,
        V09ProductionDiscoveryHouseholdProofState::NotImplemented,
        V09ProductionDiscoveryHouseholdRuntimeOwner::ManualProof,
        evidence_label,
        None,
    )
}

fn production_status(
    capability: LanProductionHouseholdProofCapability,
    discovery_state: LanPairingProductionDiscoveryState,
    proof_state: V09ProductionDiscoveryHouseholdProofState,
    runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner,
    evidence_label: &str,
    required_artifact_summary: Option<String>,
) -> LanProductionHouseholdProofStatus {
    LanProductionHouseholdProofStatus {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        capability,
        discovery_state,
        proof_state,
        runtime_owner,
        evidence_label: evidence_label.to_string(),
        required_artifact_summary,
    }
}

fn router_neighbor_state(
    scan_summary: &LanBrowserAddDeviceScanSummary,
) -> LanPairingProductionDiscoveryState {
    if scan_summary.infrastructure_device_count > 0 {
        LanPairingProductionDiscoveryState::Discovered
    } else {
        LanPairingProductionDiscoveryState::ManualRequired
    }
}

fn registry_state(
    trusted_device_registry: &[LanTrustedDeviceRegistryEntry],
) -> LanPairingProductionDiscoveryState {
    if trusted_device_registry.is_empty() {
        LanPairingProductionDiscoveryState::Pending
    } else {
        LanPairingProductionDiscoveryState::Paired
    }
}

fn decision_state(
    household_device_decisions: &[LanHouseholdDeviceDecision],
    action_kind: &LanHouseholdDeviceActionKind,
) -> LanPairingProductionDiscoveryState {
    if household_device_decisions
        .iter()
        .any(|decision| decision.action_kind == *action_kind && decision.revoked_at.is_none())
    {
        LanPairingProductionDiscoveryState::Discovered
    } else {
        LanPairingProductionDiscoveryState::ManualRequired
    }
}

fn revocation_state(
    trusted_device_registry: &[LanTrustedDeviceRegistryEntry],
    household_device_decisions: &[LanHouseholdDeviceDecision],
) -> LanPairingProductionDiscoveryState {
    if household_device_decisions.iter().any(|decision| {
        decision.action_kind == LanHouseholdDeviceActionKind::Revoke
            && decision.revoked_at.is_none()
    }) {
        return LanPairingProductionDiscoveryState::Revoked;
    }
    if trusted_device_registry.iter().any(|entry| {
        entry.revoked_at.is_some()
            || entry.trust_state
                == ocentra_parent_agent_protocol::lan_pairing::LanPairingTrustState::Revoked
    }) {
        LanPairingProductionDiscoveryState::Revoked
    } else {
        LanPairingProductionDiscoveryState::ManualRequired
    }
}

fn route_custody_state(
    selected_device_readiness: &LanSelectedDeviceReadiness,
) -> LanPairingProductionDiscoveryState {
    if selected_device_readiness.ready_for_control {
        LanPairingProductionDiscoveryState::Paired
    } else if selected_device_readiness.route_id.is_some() {
        LanPairingProductionDiscoveryState::Pending
    } else {
        LanPairingProductionDiscoveryState::ManualRequired
    }
}

fn stale_selected_state(
    selected_device_readiness: &LanSelectedDeviceReadiness,
) -> LanPairingProductionDiscoveryState {
    if selected_device_readiness.reachability == LanPairingDeviceReachability::Stale {
        LanPairingProductionDiscoveryState::Stale
    } else {
        LanPairingProductionDiscoveryState::ManualRequired
    }
}

fn offline_selected_state(
    selected_device_readiness: &LanSelectedDeviceReadiness,
) -> LanPairingProductionDiscoveryState {
    if selected_device_readiness.reachability == LanPairingDeviceReachability::Offline {
        LanPairingProductionDiscoveryState::Offline
    } else {
        LanPairingProductionDiscoveryState::ManualRequired
    }
}
