use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingProductionDiscoveryState;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingResponseState;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingTrustState;
use ocentra_parent_agent_protocol::lan_pairing::LanTrustedDeviceRegistryEntry;
use ocentra_parent_agent_protocol::lan_pairing::V09ProductionDiscoveryHouseholdProofState;
use ocentra_parent_agent_protocol::lan_pairing::V09ProductionDiscoveryHouseholdRuntimeOwner;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::signed_discovery_relay_spine::LanSignedDiscoveryRelayCustodyLabel;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::signed_discovery_relay_spine::LanSignedDiscoveryRelayRouteSafetyCheck;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::signed_discovery_relay_spine::LanSignedDiscoveryRelayRouteSafetyRow;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanHouseholdDeviceActionKind;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanHouseholdDeviceDecision;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanSelectedDeviceReadiness;

pub(super) fn route_safety_rows(
    trusted_device_registry: &[LanTrustedDeviceRegistryEntry],
    household_device_decisions: &[LanHouseholdDeviceDecision],
    selected_device_readiness: &LanSelectedDeviceReadiness,
) -> Vec<LanSignedDiscoveryRelayRouteSafetyRow> {
    let route_id = selected_device_readiness
        .route_id
        .clone()
        .unwrap_or_else(|| constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string());
    let mut rows = base_route_safety_rows(
        trusted_device_registry,
        selected_device_readiness,
        route_id.clone(),
    );
    rows.extend(parent_decision_rows(
        trusted_device_registry,
        household_device_decisions,
        route_id,
    ));
    rows
}

fn base_route_safety_rows(
    trusted_device_registry: &[LanTrustedDeviceRegistryEntry],
    selected_device_readiness: &LanSelectedDeviceReadiness,
    route_id: String,
) -> Vec<LanSignedDiscoveryRelayRouteSafetyRow> {
    vec![
        accepted_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::TrustedRegistryRestartRecovery,
            Some(route_id.clone()),
            registry_state(trusted_device_registry),
            constants::lan_pairing::ROUTE_REQUIREMENT_ROUTE_RECOVERY_PERSISTED,
        ),
        accepted_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::SelectedRouteCustody,
            Some(route_id.clone()),
            route_custody_state(selected_device_readiness),
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_ROUTE_CUSTODY,
        ),
        rejected_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::StaleSelectedDeviceRejected,
            Some(route_id.clone()),
            stale_selected_state(selected_device_readiness),
            LanPairingRejectionReason::Stale,
            constants::value::LAN_REASON_STALE,
        ),
        rejected_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::OfflineSelectedDeviceRejected,
            Some(route_id.clone()),
            offline_selected_state(selected_device_readiness),
            LanPairingRejectionReason::Offline,
            constants::value::LAN_REASON_OFFLINE,
        ),
        rejected_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::WrongRouteRejected,
            Some(route_id.clone()),
            LanPairingProductionDiscoveryState::Rejected,
            LanPairingRejectionReason::WrongDevice,
            constants::value::LAN_REASON_WRONG_DEVICE,
        ),
        rejected_route_safety_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::RevokedRouteRejected,
            Some(route_id),
            LanPairingProductionDiscoveryState::Revoked,
            LanPairingRejectionReason::Revoked,
            constants::value::LAN_REASON_REVOKED,
        ),
    ]
}

fn parent_decision_rows(
    trusted_device_registry: &[LanTrustedDeviceRegistryEntry],
    household_device_decisions: &[LanHouseholdDeviceDecision],
    route_id: String,
) -> Vec<LanSignedDiscoveryRelayRouteSafetyRow> {
    vec![
        parent_decision_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::ParentAssignDecisionAudited,
            Some(route_id.clone()),
            decision_state(
                household_device_decisions,
                &LanHouseholdDeviceActionKind::Assign,
            ),
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_PARENT_ASSIGNMENT,
        ),
        parent_decision_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::ParentRenameDecisionAudited,
            Some(route_id.clone()),
            decision_state(
                household_device_decisions,
                &LanHouseholdDeviceActionKind::Rename,
            ),
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_PARENT_RENAME,
        ),
        parent_decision_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::ParentIgnoreDecisionAudited,
            Some(route_id.clone()),
            decision_state(
                household_device_decisions,
                &LanHouseholdDeviceActionKind::Ignore,
            ),
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_PARENT_IGNORE,
        ),
        parent_decision_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::ParentRestoreDecisionAudited,
            Some(route_id.clone()),
            decision_state(
                household_device_decisions,
                &LanHouseholdDeviceActionKind::Restore,
            ),
            constants::lan_pairing::HOUSEHOLD_ACTION_RESTORE,
        ),
        parent_decision_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::ParentTrustDecisionAudited,
            Some(route_id.clone()),
            decision_state(
                household_device_decisions,
                &LanHouseholdDeviceActionKind::Trust,
            ),
            constants::lan_pairing::HOUSEHOLD_ACTION_TRUST,
        ),
        parent_decision_row(
            LanSignedDiscoveryRelayRouteSafetyCheck::ParentRevokeDecisionAudited,
            Some(route_id),
            revocation_state(trusted_device_registry, household_device_decisions),
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_PARENT_REVOCATION,
        ),
    ]
}

fn accepted_route_safety_row(
    check: LanSignedDiscoveryRelayRouteSafetyCheck,
    route_id: Option<String>,
    discovery_state: LanPairingProductionDiscoveryState,
    evidence_label: &str,
) -> LanSignedDiscoveryRelayRouteSafetyRow {
    route_safety_row(
        check,
        route_id,
        discovery_state,
        LanPairingResponseState::Accepted,
        None,
        evidence_label,
    )
}

fn rejected_route_safety_row(
    check: LanSignedDiscoveryRelayRouteSafetyCheck,
    route_id: Option<String>,
    discovery_state: LanPairingProductionDiscoveryState,
    rejection_reason: LanPairingRejectionReason,
    evidence_label: &str,
) -> LanSignedDiscoveryRelayRouteSafetyRow {
    route_safety_row(
        check,
        route_id,
        discovery_state,
        LanPairingResponseState::Rejected,
        Some(rejection_reason),
        evidence_label,
    )
}

fn parent_decision_row(
    check: LanSignedDiscoveryRelayRouteSafetyCheck,
    route_id: Option<String>,
    discovery_state: LanPairingProductionDiscoveryState,
    evidence_label: &str,
) -> LanSignedDiscoveryRelayRouteSafetyRow {
    accepted_route_safety_row(check, route_id, discovery_state, evidence_label)
}

fn route_safety_row(
    check: LanSignedDiscoveryRelayRouteSafetyCheck,
    route_id: Option<String>,
    discovery_state: LanPairingProductionDiscoveryState,
    response_state: LanPairingResponseState,
    rejection_reason: Option<LanPairingRejectionReason>,
    evidence_label: &str,
) -> LanSignedDiscoveryRelayRouteSafetyRow {
    LanSignedDiscoveryRelayRouteSafetyRow {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        check,
        route_id,
        discovery_state,
        response_state,
        rejection_reason,
        proof_state: V09ProductionDiscoveryHouseholdProofState::CiMechanicalProof,
        runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel,
        custody_label: LanSignedDiscoveryRelayCustodyLabel::ParentLocalService,
        evidence_label: evidence_label.to_string(),
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
        entry.revoked_at.is_some() || entry.trust_state == LanPairingTrustState::Revoked
    }) {
        LanPairingProductionDiscoveryState::Revoked
    } else {
        LanPairingProductionDiscoveryState::ManualRequired
    }
}
