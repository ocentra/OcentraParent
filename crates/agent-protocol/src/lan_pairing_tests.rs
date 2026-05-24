use super::{
    constants, policy_constants, LanChildAgentResponse, LanPairingAuditEvent,
    LanPairingAuditEventType, LanPairingAuthenticationState, LanPairingChallenge,
    LanPairingDeviceReachability, LanPairingDeviceRef, LanPairingDiscoveryDevice,
    LanPairingDiscoveryRuntimeStatus, LanPairingHttpEndpointSupport, LanPairingIntentKind,
    LanPairingManualProofGap, LanPairingNetworkMode, LanPairingPersistenceMode, LanPairingProof,
    LanPairingProofMode, LanPairingProofPreview, LanPairingRejectionReason,
    LanPairingResponseState, LanPairingRouteRequirement, LanPairingRouteSelectionRequest,
    LanPairingRoutingDecision, LanPairingRuntimeSupportSurface, LanPairingTransport,
    LanPairingUnsupportedHttpEndpoint, LanTrustedDeviceRegistryEntry,
    LanTrustedDeviceRegistrySnapshot, ParentEvidenceReference, ParentEvidenceReferenceKind,
};

#[test]
fn lan_pairing_contracts_serialize_to_typescript_shapes() {
    let proof = LanPairingProof {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        pairing_id: constants::lan_pairing::PAIRING_ID.to_string(),
        challenge_id: constants::lan_pairing::CHALLENGE_ID.to_string(),
        child_device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
        parent_device_id: constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        origin: constants::lan_pairing::ALLOWED_ORIGIN.to_string(),
        proof_digest: constants::lan_pairing::PROOF_DIGEST.to_string(),
        issued_at: constants::lan_pairing::ISSUED_AT.to_string(),
        expires_at: constants::lan_pairing::EXPIRES_AT.to_string(),
    };
    let entry = LanTrustedDeviceRegistryEntry {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        pairing_id: proof.pairing_id.clone(),
        child_device: child_device(),
        parent_device: parent_device(),
        route_id: proof.route_id.clone(),
        origin: proof.origin.clone(),
        proof_digest: proof.proof_digest.clone(),
        trust_state: super::LanPairingTrustState::Paired,
        trusted_at: constants::lan_pairing::ISSUED_AT.to_string(),
        expires_at: constants::lan_pairing::EXPIRES_AT.to_string(),
        revoked_at: None,
    };
    let response = LanChildAgentResponse {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        intent_id: constants::lan_pairing::INTENT_ID.to_string(),
        target_child_device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        state: LanPairingResponseState::Accepted,
        rejection_reason: None,
        audit_event_id: constants::lan_pairing::AUDIT_EVENT_ID.to_string(),
        responded_at: constants::lan_pairing::OBSERVED_AT.to_string(),
    };

    let proof_json = serde_json::to_value(proof).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let entry_json = serde_json::to_value(entry).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let response_json =
        serde_json::to_value(response).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        proof_json["proofDigest"],
        constants::lan_pairing::PROOF_DIGEST
    );
    assert_eq!(entry_json["trustState"], "paired");
    assert_eq!(response_json["state"], "accepted");
}

#[test]
fn lan_pairing_audit_event_records_rejection_reason_without_raw_secret() {
    let event = LanPairingAuditEvent {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        audit_event_id: constants::lan_pairing::AUDIT_EVENT_ID.to_string(),
        event_type: LanPairingAuditEventType::ControlRejected,
        pairing_id: Some(constants::lan_pairing::PAIRING_ID.to_string()),
        intent_id: Some(constants::lan_pairing::INTENT_ID.to_string()),
        child_device_id: Some(constants::lan_pairing::CHILD_DEVICE_ID.to_string()),
        parent_device_id: Some(constants::lan_pairing::PARENT_DEVICE_ID.to_string()),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        origin: Some(constants::lan_pairing::WRONG_ORIGIN.to_string()),
        rejection_reason: Some(LanPairingRejectionReason::WrongOrigin),
        observed_at: constants::lan_pairing::OBSERVED_AT.to_string(),
        evidence_references: vec![evidence()],
    };

    let event_json = serde_json::to_value(event).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let event_text =
        serde_json::to_string(&event_json).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(event_json["eventType"], "control-rejected");
    assert_eq!(event_json["rejectionReason"], "wrong-origin");
    assert_eq!(
        event_json["evidenceReferences"][0]["evidenceReferenceId"],
        constants::lan_pairing::EVIDENCE_REFERENCE_ID
    );
    assert!(!event_text.contains("rawToken"));
    assert!(!event_text.contains("rawEvidence"));
}

#[test]
fn lan_pairing_discovery_challenge_and_proof_preview_make_unsupported_status_explicit() {
    let discovery = LanPairingDiscoveryDevice {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        discovered_at: constants::lan_pairing::OBSERVED_AT.to_string(),
        child_device: child_device(),
        agent_peer_id: constants::lan_pairing::PARENT_PEER_ID.to_string(),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        network_mode: LanPairingNetworkMode::LocalNetwork,
        reachability: LanPairingDeviceReachability::Stale,
        address_ref: constants::lan_pairing::ADDRESS_REF_UNPROVEN.to_string(),
        discovery_status: LanPairingDiscoveryRuntimeStatus::PlannedUnsupported,
    };
    let challenge = LanPairingChallenge {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        challenge_id: constants::lan_pairing::CHALLENGE_ID.to_string(),
        child_device: child_device(),
        parent_device: parent_device(),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        origin: constants::lan_pairing::ALLOWED_ORIGIN.to_string(),
        issued_at: constants::lan_pairing::ISSUED_AT.to_string(),
        expires_at: constants::lan_pairing::EXPIRES_AT.to_string(),
        challenge_status: LanPairingDiscoveryRuntimeStatus::PlannedUnsupported,
    };
    let preview = LanPairingProofPreview {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        challenge_id: constants::lan_pairing::CHALLENGE_ID.to_string(),
        child_device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
        parent_device_id: constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        origin: constants::lan_pairing::ALLOWED_ORIGIN.to_string(),
        proof_digest: constants::lan_pairing::PROOF_DIGEST.to_string(),
        issued_at: constants::lan_pairing::ISSUED_AT.to_string(),
        expires_at: constants::lan_pairing::EXPIRES_AT.to_string(),
        proof_preview_status: LanPairingDiscoveryRuntimeStatus::PlannedUnsupported,
    };

    let discovery_json =
        serde_json::to_value(discovery).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let challenge_json =
        serde_json::to_value(challenge).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let preview_json =
        serde_json::to_value(preview).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let preview_text =
        serde_json::to_string(&preview_json).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        discovery_json["discoveryStatus"],
        constants::lan_pairing::SUPPORT_PLANNED_UNSUPPORTED
    );
    assert_eq!(
        discovery_json["addressRef"],
        constants::lan_pairing::ADDRESS_REF_UNPROVEN
    );
    assert_eq!(
        challenge_json["challengeStatus"],
        constants::lan_pairing::SUPPORT_PLANNED_UNSUPPORTED
    );
    assert_eq!(
        preview_json["proofPreviewStatus"],
        constants::lan_pairing::SUPPORT_PLANNED_UNSUPPORTED
    );
    assert_eq!(
        preview_json["proofDigest"],
        constants::lan_pairing::PROOF_DIGEST
    );
    assert!(!preview_text.contains("rawToken"));
}

#[test]
fn lan_pairing_read_model_values_keep_local_network_state_explicit() {
    let selected = super::LanSelectedRouteTarget {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        selected_child_device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        pairing_id: Some(constants::lan_pairing::PAIRING_ID.to_string()),
        network_mode: LanPairingNetworkMode::LocalNetwork,
        reachability: LanPairingDeviceReachability::Stale,
        stale_at: Some(constants::lan_pairing::OBSERVED_AT.to_string()),
    };
    let intent = super::LanParentIntentEnvelope {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        intent_id: constants::lan_pairing::INTENT_ID.to_string(),
        intent_kind: LanPairingIntentKind::RuleQuery,
        target_child_device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        pairing_id: constants::lan_pairing::PAIRING_ID.to_string(),
        proof_digest: constants::lan_pairing::PROOF_DIGEST.to_string(),
        origin: constants::lan_pairing::ALLOWED_ORIGIN.to_string(),
        issued_at: constants::lan_pairing::ISSUED_AT.to_string(),
        expires_at: constants::lan_pairing::EXPIRES_AT.to_string(),
        evidence_references: vec![evidence()],
    };

    let selected_json =
        serde_json::to_value(selected).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let intent_json = serde_json::to_value(intent).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(selected_json["networkMode"], "local-network");
    assert_eq!(selected_json["reachability"], "stale");
    assert_eq!(intent_json["intentKind"], "rule-query");
    assert_eq!(
        intent_json["evidenceReferences"][0]["kind"],
        "activity-event"
    );
}

#[test]
fn lan_pairing_parent_intent_and_child_response_cover_rule_query_approval_spine() {
    let rule_query = parent_intent(
        constants::lan_pairing::RULE_QUERY_INTENT_ID,
        LanPairingIntentKind::RuleQuery,
    );
    let rule_update = parent_intent(
        constants::lan_pairing::RULE_UPDATE_INTENT_ID,
        LanPairingIntentKind::RuleUpdate,
    );
    let approval_decision = parent_intent(
        constants::lan_pairing::APPROVAL_DECISION_INTENT_ID,
        LanPairingIntentKind::ApprovalDecision,
    );
    let rejected = LanChildAgentResponse {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        intent_id: constants::lan_pairing::RULE_QUERY_INTENT_ID.to_string(),
        target_child_device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        state: LanPairingResponseState::Rejected,
        rejection_reason: Some(LanPairingRejectionReason::WrongOrigin),
        audit_event_id: constants::lan_pairing::AUDIT_EVENT_ID.to_string(),
        responded_at: constants::lan_pairing::OBSERVED_AT.to_string(),
    };

    let rule_query_json =
        serde_json::to_value(rule_query).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let rule_update_json =
        serde_json::to_value(rule_update).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let approval_decision_json =
        serde_json::to_value(approval_decision).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let rejected_json =
        serde_json::to_value(rejected).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let rejected_text =
        serde_json::to_string(&rejected_json).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        rule_query_json["intentKind"],
        constants::value::LAN_INTENT_RULE_QUERY
    );
    assert_eq!(
        rule_update_json["intentKind"],
        constants::value::LAN_INTENT_RULE_UPDATE
    );
    assert_eq!(
        approval_decision_json["intentKind"],
        constants::value::LAN_INTENT_APPROVAL_DECISION
    );
    assert_eq!(
        rejected_json["state"],
        constants::value::LAN_CONTROL_REJECTED
    );
    assert_eq!(
        rejected_json["rejectionReason"],
        constants::value::LAN_REASON_WRONG_ORIGIN
    );
    assert!(!rejected_text.contains("rawEvidence"));
}

#[test]
fn lan_pairing_runtime_support_surface_serializes_supported_and_planned_api_claims() {
    let support = LanPairingRuntimeSupportSurface {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        transport: LanPairingTransport::Websocket,
        supported_websocket_commands: constants::lan_pairing::SUPPORTED_WEBSOCKET_COMMANDS
            .iter()
            .map(|command| command.to_string())
            .collect(),
        unsupported_http_endpoints: planned_http_endpoints(),
        pairing_state: super::LanPairingTrustState::Unpaired,
        trusted_device_count: 0,
        discovery_status: LanPairingDiscoveryRuntimeStatus::PlannedUnsupported,
        challenge_status: LanPairingDiscoveryRuntimeStatus::PlannedUnsupported,
        proof_preview_status: LanPairingDiscoveryRuntimeStatus::PlannedUnsupported,
        persistence_mode: LanPairingPersistenceMode::InMemoryFailClosed,
        proof_mode: LanPairingProofMode::DirectProofSubmit,
        route_requirements: vec![
            LanPairingRouteRequirement::PairedDevice,
            LanPairingRouteRequirement::AllowedOrigin,
            LanPairingRouteRequirement::TargetDeviceMatch,
            LanPairingRouteRequirement::RouteIdMatch,
            LanPairingRouteRequirement::UnexpiredIntent,
            LanPairingRouteRequirement::NonReplayedIntent,
            LanPairingRouteRequirement::UnrevokedPairing,
            LanPairingRouteRequirement::SelectedDeviceReachable,
        ],
        manual_proof_gaps: vec![
            LanPairingManualProofGap::ManualLanBindProof,
            LanPairingManualProofGap::ManualFirewallProof,
            LanPairingManualProofGap::ManualPhysicalDeviceProof,
        ],
    };

    let support_json =
        serde_json::to_value(support).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(support_json["transport"], "websocket");
    assert_eq!(
        support_json["supportedWebSocketCommands"][0],
        constants::lan_pairing::COMMAND_PROOF_SUBMIT
    );
    assert_eq!(
        support_json["supportedWebSocketCommands"][1],
        constants::lan_pairing::COMMAND_ROUTE_SELECT
    );
    assert_eq!(
        support_json["supportedWebSocketCommands"][2],
        constants::lan_pairing::COMMAND_ROUTE_REVOKE
    );
    assert_eq!(
        support_json["unsupportedHttpEndpoints"][0]["support"],
        constants::lan_pairing::SUPPORT_PLANNED_UNSUPPORTED
    );
    assert_eq!(
        support_json["discoveryStatus"],
        constants::lan_pairing::SUPPORT_PLANNED_UNSUPPORTED
    );
    assert_eq!(
        support_json["challengeStatus"],
        constants::lan_pairing::SUPPORT_PLANNED_UNSUPPORTED
    );
    assert_eq!(
        support_json["proofPreviewStatus"],
        constants::lan_pairing::SUPPORT_PLANNED_UNSUPPORTED
    );
    assert_eq!(
        support_json["persistenceMode"],
        constants::value::LAN_PERSISTENCE_IN_MEMORY_FAIL_CLOSED
    );
}

#[test]
fn lan_pairing_registry_snapshot_and_route_decision_make_selection_explicit() {
    let selected = super::LanSelectedRouteTarget {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        selected_child_device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        pairing_id: Some(constants::lan_pairing::PAIRING_ID.to_string()),
        network_mode: LanPairingNetworkMode::LocalNetwork,
        reachability: LanPairingDeviceReachability::Online,
        stale_at: None,
    };
    let snapshot = LanTrustedDeviceRegistrySnapshot {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        entries: vec![trusted_entry(
            constants::lan_pairing::PAIRING_ID,
            child_device(),
        )],
        selected_target: Some(selected),
        authentication_state: LanPairingAuthenticationState::Paired,
        trusted_device_count: 1,
        updated_at: constants::lan_pairing::OBSERVED_AT.to_string(),
    };
    let selection = LanPairingRouteSelectionRequest {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        pairing_id: constants::lan_pairing::PAIRING_ID.to_string(),
        target_child_device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        origin: constants::lan_pairing::ALLOWED_ORIGIN.to_string(),
        issued_at: constants::lan_pairing::ISSUED_AT.to_string(),
        expires_at: constants::lan_pairing::EXPIRES_AT.to_string(),
    };
    let rejected = LanPairingRoutingDecision {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        intent_id: Some(constants::lan_pairing::INTENT_ID.to_string()),
        target_child_device_id: constants::lan_pairing::SECOND_CHILD_DEVICE_ID.to_string(),
        route_id: constants::lan_pairing::ROUTE_ID_SECOND_LOCAL_NETWORK.to_string(),
        pairing_id: Some(constants::lan_pairing::SECOND_PAIRING_ID.to_string()),
        authentication_state: LanPairingAuthenticationState::Paired,
        state: LanPairingResponseState::Rejected,
        rejection_reason: Some(LanPairingRejectionReason::UnselectedDevice),
        audit_event_id: constants::lan_pairing::AUDIT_EVENT_ID.to_string(),
        decided_at: constants::lan_pairing::OBSERVED_AT.to_string(),
    };

    let snapshot_json =
        serde_json::to_value(snapshot).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let selection_json =
        serde_json::to_value(selection).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let rejected_json =
        serde_json::to_value(rejected).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(snapshot_json["authenticationState"], "paired");
    assert_eq!(
        snapshot_json["selectedTarget"]["selectedChildDeviceId"],
        constants::lan_pairing::CHILD_DEVICE_ID
    );
    assert_eq!(
        selection_json["targetChildDeviceId"],
        constants::lan_pairing::CHILD_DEVICE_ID
    );
    assert_eq!(rejected_json["rejectionReason"], "unselected-device");
}

fn parent_intent(
    intent_id: &str,
    intent_kind: LanPairingIntentKind,
) -> super::LanParentIntentEnvelope {
    super::LanParentIntentEnvelope {
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
        evidence_references: vec![evidence()],
    }
}

fn evidence() -> ParentEvidenceReference {
    ParentEvidenceReference {
        evidence_reference_id: constants::lan_pairing::EVIDENCE_REFERENCE_ID.to_string(),
        kind: ParentEvidenceReferenceKind::ActivityEvent,
        observed_at: constants::lan_pairing::OBSERVED_AT.to_string(),
    }
}

fn planned_http_endpoints() -> Vec<LanPairingUnsupportedHttpEndpoint> {
    vec![
        planned_http_endpoint(
            constants::lan_pairing::PLANNED_HTTP_ENDPOINT_DISCOVERY_ID,
            constants::lan_pairing::PLANNED_HTTP_ENDPOINT_DISCOVERY_PATH,
        ),
        planned_http_endpoint(
            constants::lan_pairing::PLANNED_HTTP_ENDPOINT_CHALLENGE_ID,
            constants::lan_pairing::PLANNED_HTTP_ENDPOINT_CHALLENGE_PATH,
        ),
        planned_http_endpoint(
            constants::lan_pairing::PLANNED_HTTP_ENDPOINT_PROOF_ID,
            constants::lan_pairing::PLANNED_HTTP_ENDPOINT_PROOF_PATH,
        ),
        planned_http_endpoint(
            constants::lan_pairing::PLANNED_HTTP_ENDPOINT_CONTROL_ID,
            constants::lan_pairing::PLANNED_HTTP_ENDPOINT_CONTROL_PATH,
        ),
        planned_http_endpoint(
            constants::lan_pairing::PLANNED_HTTP_ENDPOINT_REGISTRY_ID,
            constants::lan_pairing::PLANNED_HTTP_ENDPOINT_REGISTRY_PATH,
        ),
    ]
}

fn planned_http_endpoint(endpoint_id: &str, path: &str) -> LanPairingUnsupportedHttpEndpoint {
    LanPairingUnsupportedHttpEndpoint {
        endpoint_id: endpoint_id.to_string(),
        path: path.to_string(),
        support: LanPairingHttpEndpointSupport::PlannedUnsupported,
    }
}

fn trusted_entry(
    pairing_id: &str,
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
        trust_state: super::LanPairingTrustState::Paired,
        trusted_at: constants::lan_pairing::ISSUED_AT.to_string(),
        expires_at: constants::lan_pairing::EXPIRES_AT.to_string(),
        revoked_at: None,
    }
}

fn child_device() -> LanPairingDeviceRef {
    LanPairingDeviceRef {
        device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
        child_profile_id: Some(policy_constants::TEST_CHILD_PROFILE_ID.to_string()),
        label: policy_constants::TEST_PARENT_DEVICE_LABEL.to_string(),
        platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
    }
}

fn parent_device() -> LanPairingDeviceRef {
    LanPairingDeviceRef {
        device_id: constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        child_profile_id: None,
        label: policy_constants::TEST_PARENT_DEVICE_LABEL.to_string(),
        platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
    }
}
