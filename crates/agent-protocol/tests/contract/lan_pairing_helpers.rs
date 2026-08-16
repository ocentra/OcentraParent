use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingDeviceRef, LanPairingProductionDiscoveryState,
    LanPairingRejectionReason, LanPairingTrustState, LanParentIntentEnvelope,
    LanSignedChildAgentClaim, LanSignedChildAgentEnvelope, LanSignedChildAgentMessageKind,
    LanTrustedDeviceRegistryEntry, V09ProductionDiscoveryHouseholdCheck,
    V09ProductionDiscoveryHouseholdManualChecklistItem,
    V09ProductionDiscoveryHouseholdManualProofGate, V09ProductionDiscoveryHouseholdProofBoundary,
    V09ProductionDiscoveryHouseholdProofReadModel, V09ProductionDiscoveryHouseholdProofState,
    V09ProductionDiscoveryHouseholdReadinessDecision,
    V09ProductionDiscoveryHouseholdRouteRecoveryState, V09ProductionDiscoveryHouseholdRuntimeOwner,
    V09ProductionDiscoveryHouseholdSourceState, V09ProductionDiscoveryHouseholdStateEvidence,
};
use ocentra_parent_agent_protocol::lan_pairing_authority::LanPairingParentAuthority;
use ocentra_parent_agent_protocol::lan_pairing_support::{
    LanPairingHttpEndpointSupport, LanPairingUnsupportedHttpEndpoint,
};
use ocentra_parent_agent_protocol::ParentEvidenceReference;
use ocentra_parent_agent_protocol::ParentEvidenceReferenceKind;

pub fn child_device() -> LanPairingDeviceRef {
    LanPairingDeviceRef {
        device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
        child_profile_id: Some("child-profile-1".to_string()),
        label: "Child Device".to_string(),
        platform: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
        install_id: Some("child-install-1".to_string()),
        ip_address: None,
        mac_address: None,
        hostname: None,
        network_interface: None,
        agent_status: None,
        hardware_profile: None,
    }
}

pub fn parent_device() -> LanPairingDeviceRef {
    LanPairingDeviceRef {
        device_id: constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        child_profile_id: None,
        label: "Parent Device".to_string(),
        platform: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
        install_id: Some("parent-install-1".to_string()),
        ip_address: None,
        mac_address: None,
        hostname: None,
        network_interface: None,
        agent_status: None,
        hardware_profile: None,
    }
}

pub fn signed_child_agent_envelope(
    message_kind: LanSignedChildAgentMessageKind,
    install_id: impl std::fmt::Display,
    nonce: impl std::fmt::Display,
    sequence: u64,
) -> LanSignedChildAgentEnvelope {
    LanSignedChildAgentEnvelope {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        claim: LanSignedChildAgentClaim {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            message_kind,
            child_device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
            parent_device_id: constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
            install_id: install_id.to_string(),
            family_hash: "sha256:family-1".to_string(),
            child_profile_hash: Some("sha256:child-profile-1".to_string()),
            platform: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
            hostname: constants::lan_pairing::TEST_HOSTNAME.to_string(),
            agent_version: "1.2.3".to_string(),
            local_ips: vec![constants::lan_pairing::TEST_LAN_IP.to_string()],
            mac_addresses: vec![constants::lan_pairing::TEST_LAN_MAC.to_string()],
            capabilities: vec![
                constants::lan_pairing::CHILD_AGENT_CAPABILITY_PAIRING_ROUTE.to_string(),
                "future-safe-local-capability".to_string(),
            ],
            route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
            nonce: nonce.to_string(),
            sequence,
            issued_at: constants::lan_pairing::ISSUED_AT.to_string(),
            expires_at: constants::lan_pairing::EXPIRES_AT.to_string(),
        },
        public_key_base64: "public-key-base64".to_string(),
        public_key_id: "public-key-id".to_string(),
        signature_base64: "signature-base64".to_string(),
        signature_algorithm: constants::lan_pairing::SIGNED_CHILD_AGENT_SIGNATURE_ALGORITHM_ED25519
            .to_string(),
    }
}

pub fn trusted_entry(
    pairing_id: impl std::fmt::Display,
    child_device: LanPairingDeviceRef,
) -> LanTrustedDeviceRegistryEntry {
    LanTrustedDeviceRegistryEntry {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        pairing_id: pairing_id.to_string(),
        child_device,
        parent_device: parent_device(),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        origin: constants::lan_pairing::ALLOWED_ORIGIN.to_string(),
        proof_digest: constants::lan_pairing::PROOF_DIGEST.to_string(),
        trust_state: ocentra_parent_agent_protocol::lan_pairing::LanPairingTrustState::Paired,
        trusted_at: constants::lan_pairing::ISSUED_AT.to_string(),
        expires_at: constants::lan_pairing::EXPIRES_AT.to_string(),
        revoked_at: None,
    }
}

pub fn evidence() -> ParentEvidenceReference {
    ParentEvidenceReference {
        evidence_reference_id: constants::lan_pairing::EVIDENCE_REFERENCE_ID.to_string(),
        kind: ParentEvidenceReferenceKind::ActivityEvent,
        observed_at: constants::lan_pairing::OBSERVED_AT.to_string(),
    }
}

pub fn parent_intent(
    intent_id: impl std::fmt::Display,
    intent_kind: ocentra_parent_agent_protocol::lan_pairing::LanPairingIntentKind,
) -> LanParentIntentEnvelope {
    LanParentIntentEnvelope {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        intent_id: intent_id.to_string(),
        intent_kind,
        target_child_device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        pairing_id: constants::lan_pairing::PAIRING_ID.to_string(),
        proof_digest: constants::lan_pairing::PROOF_DIGEST.to_string(),
        origin: constants::lan_pairing::ALLOWED_ORIGIN.to_string(),
        issued_at: constants::lan_pairing::ISSUED_AT.to_string(),
        expires_at: constants::lan_pairing::EXPIRES_AT.to_string(),
        controller_lease_id: constants::lan_pairing::CONTROLLER_LEASE_ID.to_string(),
        controller_device_id: constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        parent_actor_id: constants::lan_pairing::PARENT_ACTOR_ID.to_string(),
        parent_authority: LanPairingParentAuthority::ActiveController,
        controller_lease_issued_at: constants::lan_pairing::ISSUED_AT.to_string(),
        controller_lease_expires_at: constants::lan_pairing::CONTROLLER_LEASE_EXPIRES_AT
            .to_string(),
        evidence_references: vec![evidence()],
    }
}

pub fn planned_http_endpoints() -> Vec<LanPairingUnsupportedHttpEndpoint> {
    vec![LanPairingUnsupportedHttpEndpoint {
        endpoint_id: "lan-planned-http-endpoint".to_string(),
        path: "/lan/planned".to_string(),
        support: LanPairingHttpEndpointSupport::PlannedUnsupported,
    }]
}

pub fn result_error_or_unreachable<T>(
    result: serde_json::Result<T>,
    context: impl std::fmt::Display,
) -> serde_json::Error {
    result.err().unwrap_or_else(|| {
        serde_json::Error::io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            context.to_string(),
        ))
    })
}

#[derive(Clone, Copy)]
pub enum JsonSurfaceMarker {
    RawEvidence,
    RawToken,
    ActivitySqlite,
}

pub fn json_surface_contains_marker(value: &serde_json::Value, marker: JsonSurfaceMarker) -> bool {
    value.to_string().contains(match marker {
        JsonSurfaceMarker::RawEvidence => constants::lan_pairing::RAW_MARKER_RAW_EVIDENCE,
        JsonSurfaceMarker::RawToken => constants::lan_pairing::RAW_MARKER_RAW_TOKEN,
        JsonSurfaceMarker::ActivitySqlite => constants::lan_pairing::RAW_MARKER_ACTIVITY_SQLITE,
    })
}

pub fn read_model() -> V09ProductionDiscoveryHouseholdProofReadModel {
    V09ProductionDiscoveryHouseholdProofReadModel {
        schema_version: constants::lan_pairing::SCHEMA_VERSION_TEXT.to_string().into(),
        checked_at: constants::lan_pairing::OBSERVED_AT.to_string(),
        proof_boundary:
            V09ProductionDiscoveryHouseholdProofBoundary::LocalRealServiceNotPhysicalHouseholdLan,
        product_readiness_decision:
            V09ProductionDiscoveryHouseholdReadinessDecision::NotReadyForProductReadyHouseholdLanClaim,
        production_discovery_states: production_states(),
        route_checks: route_checks(),
        restart_recovery: restart_recovery(),
        source_device_states: source_device_states(),
        manual_household_proof_checklist: manual_checklist(),
        claims_proved: vec![constants::lan_pairing::ROUTE_REQUIREMENT_ROUTE_RECOVERY_PERSISTED
            .to_string()],
        claims_not_proved: vec![constants::lan_pairing::MANUAL_PROOF_GAP_PHYSICAL_DEVICE
            .to_string()],
    }
}

pub fn production_states() -> Vec<V09ProductionDiscoveryHouseholdStateEvidence> {
    vec![
        household_evidence(HouseholdEvidenceCase {
            check: V09ProductionDiscoveryHouseholdCheck::ProductionDiscoveryStates,
            source_state: V09ProductionDiscoveryHouseholdSourceState::Discovered,
            discovery_state: LanPairingProductionDiscoveryState::Discovered,
            trust_state: LanPairingTrustState::Unpaired,
            reachability: LanPairingDeviceReachability::Online,
            rejection_reason: None,
            route_recovery_state:
                V09ProductionDiscoveryHouseholdRouteRecoveryState::FailClosedUnpaired,
            runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::AgentProtocol,
        }),
        household_evidence(HouseholdEvidenceCase {
            check: V09ProductionDiscoveryHouseholdCheck::ProductionDiscoveryStates,
            source_state: V09ProductionDiscoveryHouseholdSourceState::Pending,
            discovery_state: LanPairingProductionDiscoveryState::Pending,
            trust_state: LanPairingTrustState::Pairing,
            reachability: LanPairingDeviceReachability::Online,
            rejection_reason: None,
            route_recovery_state:
                V09ProductionDiscoveryHouseholdRouteRecoveryState::FailClosedUnpaired,
            runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::AgentProtocol,
        }),
        household_evidence(HouseholdEvidenceCase {
            check: V09ProductionDiscoveryHouseholdCheck::ProductionDiscoveryStates,
            source_state: V09ProductionDiscoveryHouseholdSourceState::Paired,
            discovery_state: LanPairingProductionDiscoveryState::Paired,
            trust_state: LanPairingTrustState::Paired,
            reachability: LanPairingDeviceReachability::Online,
            rejection_reason: None,
            route_recovery_state:
                V09ProductionDiscoveryHouseholdRouteRecoveryState::SelectedRoutePersisted,
            runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel,
        }),
    ]
}

pub fn route_checks() -> Vec<V09ProductionDiscoveryHouseholdStateEvidence> {
    vec![
        route_check(
            V09ProductionDiscoveryHouseholdCheck::PairedRouteAccepted,
            None,
        ),
        route_check(
            V09ProductionDiscoveryHouseholdCheck::FailedUnpairedRejected,
            Some(LanPairingRejectionReason::Anonymous),
        ),
        route_check(
            V09ProductionDiscoveryHouseholdCheck::WrongOriginRejected,
            Some(LanPairingRejectionReason::WrongOrigin),
        ),
        route_check(
            V09ProductionDiscoveryHouseholdCheck::WrongDeviceRejected,
            Some(LanPairingRejectionReason::WrongDevice),
        ),
    ]
}

pub fn restart_recovery() -> Vec<V09ProductionDiscoveryHouseholdStateEvidence> {
    vec![
        household_evidence(HouseholdEvidenceCase {
            check: V09ProductionDiscoveryHouseholdCheck::RestartSelectedRouteRecovered,
            source_state: V09ProductionDiscoveryHouseholdSourceState::RestartRecovered,
            discovery_state: LanPairingProductionDiscoveryState::Paired,
            trust_state: LanPairingTrustState::Paired,
            reachability: LanPairingDeviceReachability::Online,
            rejection_reason: None,
            route_recovery_state:
                V09ProductionDiscoveryHouseholdRouteRecoveryState::RegistryRestoredAfterRestart,
            runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel,
        }),
        household_evidence(HouseholdEvidenceCase {
            check: V09ProductionDiscoveryHouseholdCheck::RestartRegistryStateRecovered,
            source_state: V09ProductionDiscoveryHouseholdSourceState::RestartRecovered,
            discovery_state: LanPairingProductionDiscoveryState::Paired,
            trust_state: LanPairingTrustState::Paired,
            reachability: LanPairingDeviceReachability::Online,
            rejection_reason: None,
            route_recovery_state:
                V09ProductionDiscoveryHouseholdRouteRecoveryState::SelectedRoutePersisted,
            runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel,
        }),
    ]
}

pub fn source_device_states() -> Vec<V09ProductionDiscoveryHouseholdStateEvidence> {
    vec![
        source_state(
            V09ProductionDiscoveryHouseholdCheck::StaleSourceRejected,
            V09ProductionDiscoveryHouseholdSourceState::Stale,
            LanPairingProductionDiscoveryState::Stale,
            LanPairingDeviceReachability::Stale,
            LanPairingRejectionReason::Stale,
        ),
        source_state(
            V09ProductionDiscoveryHouseholdCheck::OfflineDeviceRejected,
            V09ProductionDiscoveryHouseholdSourceState::Offline,
            LanPairingProductionDiscoveryState::Offline,
            LanPairingDeviceReachability::Offline,
            LanPairingRejectionReason::Offline,
        ),
        source_state(
            V09ProductionDiscoveryHouseholdCheck::RevokedPairingRejected,
            V09ProductionDiscoveryHouseholdSourceState::Revoked,
            LanPairingProductionDiscoveryState::Revoked,
            LanPairingDeviceReachability::Online,
            LanPairingRejectionReason::Revoked,
        ),
        manual_source_state(),
    ]
}

pub fn route_check(
    check: V09ProductionDiscoveryHouseholdCheck,
    rejection_reason: Option<LanPairingRejectionReason>,
) -> V09ProductionDiscoveryHouseholdStateEvidence {
    let source_state = match check {
        V09ProductionDiscoveryHouseholdCheck::PairedRouteAccepted => {
            V09ProductionDiscoveryHouseholdSourceState::Paired
        }
        V09ProductionDiscoveryHouseholdCheck::FailedUnpairedRejected => {
            V09ProductionDiscoveryHouseholdSourceState::FailedUnpaired
        }
        V09ProductionDiscoveryHouseholdCheck::WrongOriginRejected => {
            V09ProductionDiscoveryHouseholdSourceState::WrongOrigin
        }
        V09ProductionDiscoveryHouseholdCheck::WrongDeviceRejected => {
            V09ProductionDiscoveryHouseholdSourceState::WrongDevice
        }
        _ => V09ProductionDiscoveryHouseholdSourceState::Unavailable,
    };
    let trust_state = if rejection_reason.is_none() {
        LanPairingTrustState::Paired
    } else {
        LanPairingTrustState::Unpaired
    };
    household_evidence(HouseholdEvidenceCase {
        check,
        source_state,
        discovery_state: LanPairingProductionDiscoveryState::Unavailable,
        trust_state,
        reachability: LanPairingDeviceReachability::Online,
        rejection_reason,
        route_recovery_state: V09ProductionDiscoveryHouseholdRouteRecoveryState::FailClosedUnpaired,
        runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::ProofHarness,
    })
}

pub fn source_state(
    check: V09ProductionDiscoveryHouseholdCheck,
    source_state: V09ProductionDiscoveryHouseholdSourceState,
    discovery_state: LanPairingProductionDiscoveryState,
    reachability: LanPairingDeviceReachability,
    rejection_reason: LanPairingRejectionReason,
) -> V09ProductionDiscoveryHouseholdStateEvidence {
    household_evidence(HouseholdEvidenceCase {
        check,
        source_state,
        discovery_state,
        trust_state: LanPairingTrustState::Paired,
        reachability,
        rejection_reason: Some(rejection_reason),
        route_recovery_state: V09ProductionDiscoveryHouseholdRouteRecoveryState::FailClosedUnpaired,
        runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel,
    })
}

pub fn manual_source_state() -> V09ProductionDiscoveryHouseholdStateEvidence {
    V09ProductionDiscoveryHouseholdStateEvidence {
        proof_state: V09ProductionDiscoveryHouseholdProofState::ManualRequired,
        runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::ManualProof,
        ..household_evidence(HouseholdEvidenceCase {
            check: V09ProductionDiscoveryHouseholdCheck::ManualPhysicalHouseholdChecklist,
            source_state: V09ProductionDiscoveryHouseholdSourceState::ManualRequired,
            discovery_state: LanPairingProductionDiscoveryState::Unavailable,
            trust_state: LanPairingTrustState::Unpaired,
            reachability: LanPairingDeviceReachability::Offline,
            rejection_reason: Some(LanPairingRejectionReason::LocalNetworkDisabled),
            route_recovery_state:
                V09ProductionDiscoveryHouseholdRouteRecoveryState::ManualRequiredPhysicalRouteRecovery,
            runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::ManualProof,
        })
    }
}

pub struct HouseholdEvidenceCase {
    pub check: V09ProductionDiscoveryHouseholdCheck,
    pub source_state: V09ProductionDiscoveryHouseholdSourceState,
    pub discovery_state: LanPairingProductionDiscoveryState,
    pub trust_state: LanPairingTrustState,
    pub reachability: LanPairingDeviceReachability,
    pub rejection_reason: Option<LanPairingRejectionReason>,
    pub route_recovery_state: V09ProductionDiscoveryHouseholdRouteRecoveryState,
    pub runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner,
}

pub fn household_evidence(
    case: HouseholdEvidenceCase,
) -> V09ProductionDiscoveryHouseholdStateEvidence {
    V09ProductionDiscoveryHouseholdStateEvidence {
        schema_version: constants::lan_pairing::SCHEMA_VERSION_TEXT
            .to_string()
            .into(),
        check: case.check,
        source_state: case.source_state,
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        discovery_state: case.discovery_state,
        trust_state: case.trust_state,
        reachability: case.reachability,
        rejection_reason: case.rejection_reason,
        route_recovery_state: case.route_recovery_state,
        proof_state: V09ProductionDiscoveryHouseholdProofState::CiMechanicalProof,
        runtime_owner: case.runtime_owner,
        evidence_label: constants::lan_pairing::ROUTE_REQUIREMENT_DISCOVERY_STATE_EXPLICIT
            .to_string(),
    }
}

pub fn manual_checklist() -> Vec<V09ProductionDiscoveryHouseholdManualChecklistItem> {
    vec![
        household_gate(V09ProductionDiscoveryHouseholdManualProofGate::TwoPhysicalHosts),
        household_gate(V09ProductionDiscoveryHouseholdManualProofGate::HouseholdRouterReachability),
        household_gate(
            V09ProductionDiscoveryHouseholdManualProofGate::OsFirewallOrLocalNetworkPermission,
        ),
        household_gate(
            V09ProductionDiscoveryHouseholdManualProofGate::AllowedOriginOnPhysicalController,
        ),
        household_gate(
            V09ProductionDiscoveryHouseholdManualProofGate::PhysicalRouteSelectionAndTakeover,
        ),
        household_gate(
            V09ProductionDiscoveryHouseholdManualProofGate::PhysicalRevocationAndRejection,
        ),
        household_gate(
            V09ProductionDiscoveryHouseholdManualProofGate::PhysicalStaleOfflineSelectedDevice,
        ),
        household_gate(V09ProductionDiscoveryHouseholdManualProofGate::RealMobileControllerPackage),
        household_gate(V09ProductionDiscoveryHouseholdManualProofGate::RealMobileObserverPackage),
        household_gate(V09ProductionDiscoveryHouseholdManualProofGate::RealLanAiProviderHost),
        household_gate(V09ProductionDiscoveryHouseholdManualProofGate::CloudRelaySeparateProof),
    ]
}

pub fn household_gate(
    gate: V09ProductionDiscoveryHouseholdManualProofGate,
) -> V09ProductionDiscoveryHouseholdManualChecklistItem {
    V09ProductionDiscoveryHouseholdManualChecklistItem {
        schema_version: constants::lan_pairing::SCHEMA_VERSION_TEXT
            .to_string()
            .into(),
        gate,
        state: V09ProductionDiscoveryHouseholdProofState::ManualRequired,
        required_artifact_summary: constants::lan_pairing::MANUAL_PROOF_GAP_PHYSICAL_DEVICE
            .to_string(),
        runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::ManualProof,
    }
}
