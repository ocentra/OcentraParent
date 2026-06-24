use std::fs::remove_file;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingAuthenticationState, LanPairingDeviceReachability, LanPairingRejectionReason,
    LanPairingTrustState,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanCanonicalHouseholdDeviceClassification,
    LanCanonicalHouseholdDeviceConfidence, LanCanonicalHouseholdDeviceSource,
    LanCanonicalHouseholdNetworkIdentity, LanCanonicalHouseholdRouteState,
    LanCanonicalHouseholdSurface, LanDiscoveryEvidenceConfidence, LanDiscoveryEvidenceKind,
    LanDiscoveryEvidenceRecord,
};

use crate::trusted_device_registry_test_fixtures::{
    agent_event_option, agent_event_result, child_device, household_decision, intent,
    intent_for_pairing, intent_with_route, parent_device, proof, proof_for, second_child_device,
    selected_registry, temp_registry_path,
};
use crate::TrustedDeviceRegistry;

#[test]
fn trusted_device_registry_accepts_pairing_and_validates_intent() {
    let mut registry = TrustedDeviceRegistry::empty();
    let active_proof = proof(constants::lan_pairing::EXPIRES_AT);
    registry.accept_pairing_proof(
        &active_proof,
        child_device(),
        parent_device(),
        constants::lan_pairing::ISSUED_AT,
    );
    agent_event_result(registry.select_pairing(
        constants::lan_pairing::PAIRING_ID,
        constants::lan_pairing::CHILD_DEVICE_ID,
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK,
        constants::lan_pairing::EXPIRES_AT,
    ));
    agent_event_result(registry.select_pairing(
        constants::lan_pairing::PAIRING_ID,
        constants::lan_pairing::CHILD_DEVICE_ID,
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK,
        constants::lan_pairing::EXPIRES_AT,
    ));

    let result = registry.validate_intent(
        &intent(
            constants::lan_pairing::INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ),
        Some(constants::lan_pairing::ALLOWED_ORIGIN),
        constants::lan_pairing::OBSERVED_AT,
    );

    assert_eq!(result, Ok(()));
}

#[test]
fn trusted_device_registry_rejects_anonymous_wrong_origin_wrong_device_and_revoked() {
    let mut registry = TrustedDeviceRegistry::empty();
    let proof = proof(constants::lan_pairing::EXPIRES_AT);
    registry.accept_pairing_proof(
        &proof,
        child_device(),
        parent_device(),
        constants::lan_pairing::ISSUED_AT,
    );

    assert_eq!(
        registry.validate_intent(
            &intent(
                constants::lan_pairing::INTENT_ID,
                constants::lan_pairing::CHILD_DEVICE_ID,
                constants::lan_pairing::PROOF_DIGEST,
                constants::lan_pairing::EXPIRES_AT,
            ),
            None,
            constants::lan_pairing::OBSERVED_AT,
        ),
        Err(LanPairingRejectionReason::WrongOrigin)
    );
    assert_eq!(
        registry.validate_intent(
            &intent(
                constants::lan_pairing::INTENT_ID,
                constants::peer::LOCAL_DEV_AGENT,
                constants::lan_pairing::PROOF_DIGEST,
                constants::lan_pairing::EXPIRES_AT,
            ),
            Some(constants::lan_pairing::ALLOWED_ORIGIN),
            constants::lan_pairing::OBSERVED_AT,
        ),
        Err(LanPairingRejectionReason::WrongDevice)
    );
    assert_eq!(
        registry.validate_intent(
            &intent_with_route(
                constants::lan_pairing::INTENT_ID,
                constants::lan_pairing::CHILD_DEVICE_ID,
                constants::lan_pairing::ROUTE_ID_UNSUPPORTED,
                constants::lan_pairing::PROOF_DIGEST,
                constants::lan_pairing::EXPIRES_AT,
            ),
            Some(constants::lan_pairing::ALLOWED_ORIGIN),
            constants::lan_pairing::OBSERVED_AT,
        ),
        Err(LanPairingRejectionReason::UnsupportedRoute)
    );
    assert!(registry.revoke_pairing(
        constants::lan_pairing::PAIRING_ID,
        constants::lan_pairing::OBSERVED_AT
    ));
    assert_eq!(
        registry.validate_intent(
            &intent(
                constants::lan_pairing::INTENT_ID,
                constants::lan_pairing::CHILD_DEVICE_ID,
                constants::lan_pairing::PROOF_DIGEST,
                constants::lan_pairing::EXPIRES_AT,
            ),
            Some(constants::lan_pairing::ALLOWED_ORIGIN),
            constants::lan_pairing::OBSERVED_AT,
        ),
        Err(LanPairingRejectionReason::Revoked)
    );
}

#[test]
fn trusted_device_registry_rejects_stale_and_replayed_intents() {
    let mut registry = selected_registry(constants::lan_pairing::EXPIRES_AT);
    assert_eq!(
        registry.validate_intent(
            &intent(
                constants::lan_pairing::INTENT_ID,
                constants::lan_pairing::CHILD_DEVICE_ID,
                constants::lan_pairing::PROOF_DIGEST,
                constants::lan_pairing::EXPIRED_AT,
            ),
            Some(constants::lan_pairing::ALLOWED_ORIGIN),
            constants::lan_pairing::OBSERVED_AT,
        ),
        Err(LanPairingRejectionReason::Stale)
    );
    assert_eq!(
        registry.validate_intent(
            &intent(
                constants::lan_pairing::REPLAYED_INTENT_ID,
                constants::lan_pairing::CHILD_DEVICE_ID,
                constants::lan_pairing::PROOF_DIGEST,
                constants::lan_pairing::EXPIRES_AT,
            ),
            Some(constants::lan_pairing::ALLOWED_ORIGIN),
            constants::lan_pairing::OBSERVED_AT,
        ),
        Ok(())
    );
    assert_eq!(
        registry.validate_intent(
            &intent(
                constants::lan_pairing::REPLAYED_INTENT_ID,
                constants::lan_pairing::CHILD_DEVICE_ID,
                constants::lan_pairing::PROOF_DIGEST,
                constants::lan_pairing::EXPIRES_AT,
            ),
            Some(constants::lan_pairing::ALLOWED_ORIGIN),
            constants::lan_pairing::OBSERVED_AT,
        ),
        Err(LanPairingRejectionReason::Replayed)
    );
}

#[test]
fn trusted_device_registry_rejects_expired_pairings() {
    let mut expired_registry = selected_registry(constants::lan_pairing::EXPIRED_AT);
    assert_eq!(
        expired_registry.validate_intent(
            &intent(
                constants::lan_pairing::INTENT_ID,
                constants::lan_pairing::CHILD_DEVICE_ID,
                constants::lan_pairing::PROOF_DIGEST,
                constants::lan_pairing::EXPIRES_AT,
            ),
            Some(constants::lan_pairing::ALLOWED_ORIGIN),
            constants::lan_pairing::OBSERVED_AT,
        ),
        Err(LanPairingRejectionReason::Expired)
    );
}

#[test]
fn trusted_device_registry_requires_selected_device_for_multi_device_control() {
    let mut registry = TrustedDeviceRegistry::empty();
    registry.accept_pairing_proof(
        &proof(constants::lan_pairing::EXPIRES_AT),
        child_device(),
        parent_device(),
        constants::lan_pairing::ISSUED_AT,
    );
    registry.accept_pairing_proof(
        &proof_for(
            constants::lan_pairing::SECOND_PAIRING_ID,
            constants::lan_pairing::SECOND_CHALLENGE_ID,
            constants::lan_pairing::SECOND_CHILD_DEVICE_ID,
            constants::lan_pairing::ROUTE_ID_SECOND_LOCAL_NETWORK,
            constants::lan_pairing::SECOND_PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ),
        second_child_device(),
        parent_device(),
        constants::lan_pairing::ISSUED_AT,
    );

    assert_eq!(
        registry.validate_intent(
            &intent(
                constants::lan_pairing::INTENT_ID,
                constants::lan_pairing::CHILD_DEVICE_ID,
                constants::lan_pairing::PROOF_DIGEST,
                constants::lan_pairing::EXPIRES_AT,
            ),
            Some(constants::lan_pairing::ALLOWED_ORIGIN),
            constants::lan_pairing::OBSERVED_AT,
        ),
        Err(LanPairingRejectionReason::UnselectedDevice)
    );

    let selected = agent_event_result(registry.select_pairing(
        constants::lan_pairing::PAIRING_ID,
        constants::lan_pairing::CHILD_DEVICE_ID,
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK,
        constants::lan_pairing::EXPIRES_AT,
    ));

    assert_eq!(
        selected.selected_child_device_id,
        constants::lan_pairing::CHILD_DEVICE_ID
    );
    assert_eq!(
        registry.validate_intent(
            &intent(
                constants::lan_pairing::INTENT_ID,
                constants::lan_pairing::CHILD_DEVICE_ID,
                constants::lan_pairing::PROOF_DIGEST,
                constants::lan_pairing::EXPIRES_AT,
            ),
            Some(constants::lan_pairing::ALLOWED_ORIGIN),
            constants::lan_pairing::OBSERVED_AT,
        ),
        Ok(())
    );
    assert_eq!(
        registry.validate_intent(
            &intent_for_pairing(
                constants::lan_pairing::SECOND_INTENT_ID,
                constants::lan_pairing::SECOND_PAIRING_ID,
                constants::lan_pairing::SECOND_CHILD_DEVICE_ID,
                constants::lan_pairing::ROUTE_ID_SECOND_LOCAL_NETWORK,
                constants::lan_pairing::SECOND_PROOF_DIGEST,
                constants::lan_pairing::EXPIRES_AT,
            ),
            Some(constants::lan_pairing::ALLOWED_ORIGIN),
            constants::lan_pairing::OBSERVED_AT,
        ),
        Err(LanPairingRejectionReason::UnselectedDevice)
    );
}

#[test]
fn trusted_device_registry_reports_revoked_stale_and_offline_selected_state() {
    let mut stale_registry = selected_registry(constants::lan_pairing::EXPIRES_AT);
    assert!(stale_registry.mark_selected_stale(constants::lan_pairing::EXPIRED_AT));
    let stale_target =
        agent_event_option(stale_registry.selected_target_at(constants::lan_pairing::OBSERVED_AT));
    let mut offline_registry = selected_registry(constants::lan_pairing::EXPIRES_AT);
    assert!(offline_registry.mark_selected_offline(constants::lan_pairing::OBSERVED_AT));
    let offline_target = agent_event_option(
        offline_registry.selected_target_at(constants::lan_pairing::OBSERVED_AT),
    );
    let mut revoked_registry = selected_registry(constants::lan_pairing::EXPIRES_AT);

    assert_eq!(
        stale_target.reachability,
        LanPairingDeviceReachability::Stale
    );
    assert_eq!(stale_target.trust_state, LanPairingTrustState::Paired);
    assert_eq!(stale_target.offline_at, None);
    assert_eq!(
        stale_registry.validate_intent(
            &intent(
                constants::lan_pairing::INTENT_ID,
                constants::lan_pairing::CHILD_DEVICE_ID,
                constants::lan_pairing::PROOF_DIGEST,
                constants::lan_pairing::EXPIRES_AT,
            ),
            Some(constants::lan_pairing::ALLOWED_ORIGIN),
            constants::lan_pairing::OBSERVED_AT,
        ),
        Err(LanPairingRejectionReason::Stale)
    );
    assert_eq!(
        offline_target.reachability,
        LanPairingDeviceReachability::Offline
    );
    assert_eq!(offline_target.trust_state, LanPairingTrustState::Paired);
    assert_eq!(
        offline_target.offline_at.as_deref(),
        Some(constants::lan_pairing::OBSERVED_AT)
    );
    assert_eq!(
        offline_registry.validate_intent(
            &intent(
                constants::lan_pairing::INTENT_ID,
                constants::lan_pairing::CHILD_DEVICE_ID,
                constants::lan_pairing::PROOF_DIGEST,
                constants::lan_pairing::EXPIRES_AT,
            ),
            Some(constants::lan_pairing::ALLOWED_ORIGIN),
            constants::lan_pairing::OBSERVED_AT,
        ),
        Err(LanPairingRejectionReason::Offline)
    );
    assert!(revoked_registry.revoke_pairing(
        constants::lan_pairing::PAIRING_ID,
        constants::lan_pairing::OBSERVED_AT
    ));
    assert_eq!(revoked_registry.selected_target(), None);
    assert_eq!(revoked_registry.trusted_device_count(), 0);
    assert_eq!(
        revoked_registry.revoked_device_ids(),
        vec![constants::lan_pairing::CHILD_DEVICE_ID.to_string()]
    );
}

#[test]
fn trusted_device_registry_can_survive_restart_or_fail_closed_when_missing() {
    let path = temp_registry_path();
    let _ = remove_file(&path);
    let mut registry = TrustedDeviceRegistry::empty();
    registry.accept_pairing_proof(
        &proof(constants::lan_pairing::EXPIRES_AT),
        child_device(),
        parent_device(),
        constants::lan_pairing::ISSUED_AT,
    );
    agent_event_result(registry.save_json(&path));

    let loaded = TrustedDeviceRegistry::load_json(&path);
    let missing_path = temp_registry_path();
    let _ = remove_file(&path);
    let _ = remove_file(&missing_path);
    let missing = TrustedDeviceRegistry::load_json(&missing_path);

    assert_eq!(loaded.entries().len(), 1);
    assert_eq!(loaded.selected_target(), None);
    assert_eq!(
        loaded.authentication_state(),
        LanPairingAuthenticationState::Unpaired
    );
    assert_eq!(missing.entries().len(), 0);
}

#[test]
fn trusted_device_registry_persists_selected_route_for_restart_recovery() {
    let path = temp_registry_path();
    let _ = remove_file(&path);
    let registry = selected_registry(constants::lan_pairing::EXPIRES_AT);
    agent_event_result(registry.save_json(&path));

    let mut loaded = TrustedDeviceRegistry::load_json(&path);
    let _ = remove_file(&path);
    let selected =
        agent_event_option(loaded.selected_target_at(constants::lan_pairing::OBSERVED_AT));

    assert_eq!(
        loaded.authentication_state(),
        LanPairingAuthenticationState::Paired
    );
    assert_eq!(
        selected.selected_child_device_id,
        constants::lan_pairing::CHILD_DEVICE_ID
    );
    assert_eq!(
        selected.route_id,
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK
    );
    assert_eq!(selected.trust_state, LanPairingTrustState::Paired);
    assert_eq!(
        loaded.validate_intent(
            &intent(
                constants::lan_pairing::INTENT_ID,
                constants::lan_pairing::CHILD_DEVICE_ID,
                constants::lan_pairing::PROOF_DIGEST,
                constants::lan_pairing::EXPIRES_AT,
            ),
            Some(constants::lan_pairing::ALLOWED_ORIGIN),
            constants::lan_pairing::OBSERVED_AT,
        ),
        Ok(())
    );
}

#[test]
fn trusted_device_registry_persists_household_device_decisions_for_restart_recovery() {
    let path = temp_registry_path();
    let _ = remove_file(&path);
    let mut registry = TrustedDeviceRegistry::empty();
    registry.apply_household_device_decision(household_decision());
    agent_event_result(registry.save_json(&path));

    let loaded = TrustedDeviceRegistry::load_json(&path);
    let _ = remove_file(&path);

    assert_eq!(loaded.household_device_decisions().len(), 1);
    assert_eq!(
        loaded.household_device_decisions()[0]
            .display_name
            .as_deref(),
        Some(constants::lan_pairing::HOUSEHOLD_RENAMED_DEVICE_LABEL)
    );
    assert_eq!(
        loaded.household_device_decisions()[0]
            .device_kind
            .as_deref(),
        Some(constants::lan_pairing::HOUSEHOLD_DEVICE_KIND_DESKTOP)
    );
}

#[test]
fn trusted_device_registry_persists_known_household_devices_for_restart_recovery() {
    let path = temp_registry_path();
    let _ = remove_file(&path);
    let mut registry = TrustedDeviceRegistry::empty();
    assert!(
        registry.merge_known_household_devices(vec![known_household_device(
            constants::lan_pairing::OBSERVED_AT,
            constants::lan_pairing::OBSERVED_AT,
        )])
    );
    agent_event_result(registry.save_json(&path));

    let loaded = TrustedDeviceRegistry::load_json(&path);
    let _ = remove_file(&path);

    assert_eq!(loaded.known_household_devices().len(), 1);
    assert_eq!(
        loaded.known_household_devices()[0]
            .network_identity
            .mac_address
            .as_deref(),
        Some(constants::lan_pairing::TEST_LAN_MAC)
    );
    assert_eq!(
        loaded.known_household_devices()[0]
            .network_identity
            .evidence_records[0]
            .first_seen_at,
        constants::lan_pairing::OBSERVED_AT
    );
}

#[test]
fn known_household_device_merge_preserves_first_seen_and_updates_last_seen() {
    let mut registry = TrustedDeviceRegistry::empty();
    assert!(
        registry.merge_known_household_devices(vec![known_household_device(
            "2026-06-01T00:00:00Z",
            "2026-06-01T00:00:00Z",
        )])
    );
    assert!(
        registry.merge_known_household_devices(vec![known_household_device(
            "2026-06-02T00:00:00Z",
            "2026-06-03T00:00:00Z",
        )])
    );

    let device = &registry.known_household_devices()[0];
    let evidence = &device.network_identity.evidence_records[0];
    assert_eq!(evidence.first_seen_at, "2026-06-01T00:00:00Z");
    assert_eq!(evidence.last_seen_at, "2026-06-03T00:00:00Z");
}

fn known_household_device(first_seen_at: &str, last_seen_at: &str) -> LanCanonicalHouseholdDevice {
    LanCanonicalHouseholdDevice {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        canonical_device_id: "lan-physical-mac-001122334455".to_string(),
        display_name: "Family Tablet".to_string(),
        classification: LanCanonicalHouseholdDeviceClassification::UnknownLanDevice,
        role_badges: Vec::new(),
        enrollable: false,
        discovery_state: ocentra_parent_agent_protocol::lan_pairing::LanPairingProductionDiscoveryState::Discovered,
        trust_state: LanPairingTrustState::Unpaired,
        route_id: None,
        route_state: LanCanonicalHouseholdRouteState::ManualRequired,
        network_mode: ocentra_parent_agent_protocol::lan_pairing::LanPairingNetworkMode::LocalNetwork,
        source_labels: vec![LanCanonicalHouseholdDeviceSource::NetworkNeighbor],
        network_identity: LanCanonicalHouseholdNetworkIdentity {
            hostname: Some("family-tablet".to_string()),
            ip_addresses: vec![constants::lan_pairing::TEST_LAN_IP.to_string()],
            mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
            mac_vendor: Some("Example Vendor".to_string()),
            network_interfaces: vec![constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()],
            reachability: LanPairingDeviceReachability::Online,
            confidence: LanCanonicalHouseholdDeviceConfidence::NetworkNeighbor,
            stale_at: None,
            offline_at: None,
            evidence_records: vec![LanDiscoveryEvidenceRecord {
                schema_version: constants::lan_pairing::SCHEMA_VERSION,
                evidence_id: "evidence-1".to_string(),
                source: ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource::WindowsNeighborTable,
                evidence_kind: LanDiscoveryEvidenceKind::MacAddress,
                device_id: "lan-physical-mac-001122334455".to_string(),
                value: constants::lan_pairing::TEST_LAN_MAC.to_string(),
                normalized_value: constants::lan_pairing::TEST_LAN_MAC.to_string(),
                first_seen_at: first_seen_at.to_string(),
                last_seen_at: last_seen_at.to_string(),
                expires_at: None,
                confidence: LanDiscoveryEvidenceConfidence::Confirmed,
                merge_key: "mac:001122334455".to_string(),
                note: None,
            }],
        },
        child_agent_inventory: None,
        policy_target_surfaces: vec![
            LanCanonicalHouseholdSurface::Devices,
            LanCanonicalHouseholdSurface::Network,
        ],
    }
}
