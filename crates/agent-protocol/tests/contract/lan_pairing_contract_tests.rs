use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::{ExpectErrValue, ExpectValue};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::signed_household_mesh_ingress::transport::{
    lan_signed_household_mesh_transport_signing_bytes, LanHouseholdMeshChildDeviceId,
    LanHouseholdMeshFamilyHash, LanHouseholdMeshIdempotencyKey,
    LanHouseholdMeshIngressSchemaVersionDto, LanHouseholdMeshInstallId,
    LanHouseholdMeshLocalEventRef, LanHouseholdMeshMessageId, LanHouseholdMeshNonce,
    LanHouseholdMeshPairingId, LanHouseholdMeshParentDeviceId, LanHouseholdMeshPayloadSha256,
    LanHouseholdMeshRegistryProofDigest, LanHouseholdMeshRouteId, LanHouseholdMeshSequenceDto,
    LanHouseholdMeshTargetDeviceId, LanHouseholdMeshTimestamp, LanSignedHouseholdMeshMessageType,
    LanSignedHouseholdMeshTransportClaimDto, LanSignedHouseholdMeshTransportEnvelope,
    LAN_SIGNED_HOUSEHOLD_MESH_INGRESS_SCHEMA_VERSION,
};
use ocentra_parent_agent_protocol::lan_pairing::{
    DeviceRoleRuntimeReadModel, LanAiProviderRoutingState, LanChildAgentResponse,
    LanChildMdnsAdvertisement, LanChildMdnsAdvertisementInput, LanMdnsAdvertisementLifecycleState,
    LanMdnsAdvertisementSupportState, LanMdnsTxtRecord, LanPairingAuditEvent,
    LanPairingAuditEventType, LanPairingAuthenticationState, LanPairingChallenge,
    LanPairingChallengeRequest, LanPairingDeviceReachability, LanPairingDiscoveryDevice,
    LanPairingDiscoveryRuntimeStatus, LanPairingIntentKind, LanPairingNetworkMode,
    LanPairingProductionDiscoveryState, LanPairingProof, LanPairingProofPreview,
    LanPairingRejectionReason, LanPairingResponseState, LanPairingRouteSelectionRequest,
    LanPairingRoutingDecision, LanPairingTrustState, LanParentIntentEnvelope,
    LanParentMdnsAdvertisement, LanSelectedRouteTarget, LanSignedChildAgentEnvelope,
    LanSignedChildAgentMessageKind, LanTrustedDeviceRegistryEntry,
    LanTrustedDeviceRegistrySnapshot,
};
use ocentra_parent_agent_protocol::lan_pairing_authority::LanPairingParentAuthority;
use ocentra_parent_agent_protocol::lan_pairing_support::{
    LanPairingManualProofGap, LanPairingPersistenceMode, LanPairingProofMode,
    LanPairingRestartBehavior, LanPairingRouteRequirement, LanPairingRuntimeSupportSurface,
    LanPairingTransport,
};

use super::lan_pairing_helpers::*;

fn to_json<T: serde::Serialize>(value: T) -> serde_json::Value {
    serde_json::to_value(value).expect_value("lan pairing contract serializes")
}

const TRANSPORT_PAYLOAD_SHA256: &str =
    "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

fn signed_household_mesh_transport_claim() -> LanSignedHouseholdMeshTransportClaimDto {
    LanSignedHouseholdMeshTransportClaimDto {
        schema_version: LanHouseholdMeshIngressSchemaVersionDto::current(),
        message_kind: LanSignedChildAgentMessageKind::Hello,
        message_id: LanHouseholdMeshMessageId::try_new("household-mesh-message-1")
            .expect_value("transport message id constructs"),
        idempotency_key: LanHouseholdMeshIdempotencyKey::try_new("household-mesh-idempotency-1")
            .expect_value("transport idempotency key constructs"),
        local_event_ref: LanHouseholdMeshLocalEventRef::try_new(
            constants::household_mesh::LOCAL_EVENT_DEVICE_DISCOVERY,
        )
        .expect_value("transport local event reference constructs"),
        lan_message_type: LanSignedHouseholdMeshMessageType::try_new(
            constants::household_mesh::LAN_MESSAGE_DEVICE_DISCOVERY,
        )
        .expect_value("transport LAN message type constructs"),
        canonical_payload_sha256: LanHouseholdMeshPayloadSha256::try_new(TRANSPORT_PAYLOAD_SHA256)
            .expect_value("transport payload digest constructs"),
        family_hash: LanHouseholdMeshFamilyHash::try_new("family-hash-1")
            .expect_value("transport family hash constructs"),
        parent_device_id: LanHouseholdMeshParentDeviceId::try_new("parent-device-1")
            .expect_value("transport parent device id constructs"),
        child_device_id: LanHouseholdMeshChildDeviceId::try_new("child-device-1")
            .expect_value("transport child device id constructs"),
        target_device_id: LanHouseholdMeshTargetDeviceId::try_new("child-device-1")
            .expect_value("transport target device id constructs"),
        install_id: LanHouseholdMeshInstallId::try_new("child-install-1")
            .expect_value("transport install id constructs"),
        route_id: LanHouseholdMeshRouteId::try_new("local-network")
            .expect_value("transport route id constructs"),
        pairing_id: LanHouseholdMeshPairingId::try_new("pairing-1")
            .expect_value("transport pairing id constructs"),
        registry_proof_digest: LanHouseholdMeshRegistryProofDigest::try_new("registry-proof-1")
            .expect_value("transport registry proof digest constructs"),
        nonce: LanHouseholdMeshNonce::try_new("nonce-1").expect_value("transport nonce constructs"),
        sequence: LanHouseholdMeshSequenceDto::try_new(7)
            .expect_value("transport sequence constructs"),
        issued_at: LanHouseholdMeshTimestamp::try_new("2026-08-28T22:00:00Z")
            .expect_value("transport issued-at timestamp constructs"),
        expires_at: LanHouseholdMeshTimestamp::try_new("2026-08-28T23:00:00Z")
            .expect_value("transport expires-at timestamp constructs"),
    }
}

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
        trust_state: LanPairingTrustState::Paired,
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

    let proof_json = to_json(proof);
    let entry_json = to_json(entry);
    let response_json = to_json(response);

    assert_eq!(
        proof_json["proofDigest"],
        constants::lan_pairing::PROOF_DIGEST
    );
    assert_eq!(entry_json["trustState"], "paired");
    assert_eq!(response_json["state"], "accepted");
}

#[test]
fn signed_child_agent_envelopes_keep_hello_and_heartbeat_fields_explicit() {
    let hello = signed_child_agent_envelope(
        LanSignedChildAgentMessageKind::Hello,
        "child-install-1",
        "nonce-1",
        1,
    );
    let heartbeat = signed_child_agent_envelope(
        LanSignedChildAgentMessageKind::Heartbeat,
        "child-install-2",
        "nonce-2",
        2,
    );

    let hello_json = to_json(hello);
    let heartbeat_json = to_json(heartbeat);

    assert_eq!(hello_json["claim"]["messageKind"], "hello");
    assert_eq!(heartbeat_json["claim"]["messageKind"], "heartbeat");
    assert_eq!(
        hello_json["claim"]["installId"],
        serde_json::json!("child-install-1")
    );
    assert_eq!(
        heartbeat_json["claim"]["installId"],
        serde_json::json!("child-install-2")
    );
    assert_eq!(
        hello_json["claim"]["familyHash"],
        serde_json::json!("sha256:family-1")
    );
    assert_eq!(
        hello_json["claim"]["childProfileHash"],
        serde_json::json!("sha256:child-profile-1")
    );
    assert_eq!(
        hello_json["claim"]["platform"],
        serde_json::json!(constants::lan_pairing::PLATFORM_WINDOWS)
    );
    assert_eq!(
        hello_json["claim"]["hostname"],
        serde_json::json!(constants::lan_pairing::TEST_HOSTNAME)
    );
    assert_eq!(
        hello_json["claim"]["agentVersion"],
        serde_json::json!("1.2.3")
    );
    assert_eq!(
        hello_json["claim"]["localIps"],
        serde_json::json!([constants::lan_pairing::TEST_LAN_IP])
    );
    assert_eq!(
        hello_json["claim"]["macAddresses"],
        serde_json::json!([constants::lan_pairing::TEST_LAN_MAC])
    );
    assert_eq!(
        hello_json["claim"]["capabilities"],
        serde_json::json!([
            constants::lan_pairing::CHILD_AGENT_CAPABILITY_PAIRING_ROUTE,
            "future-safe-local-capability"
        ])
    );
    assert_eq!(hello_json["claim"]["nonce"], serde_json::json!("nonce-1"));
    assert_eq!(
        heartbeat_json["claim"]["nonce"],
        serde_json::json!("nonce-2")
    );
    assert_eq!(
        hello_json["claim"]["childDeviceId"],
        constants::lan_pairing::CHILD_DEVICE_ID
    );
    assert_eq!(
        hello_json["claim"]["parentDeviceId"],
        constants::lan_pairing::PARENT_DEVICE_ID
    );
    assert_eq!(
        hello_json["claim"]["routeId"],
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK
    );
    assert_eq!(
        hello_json["claim"]["issuedAt"],
        constants::lan_pairing::ISSUED_AT
    );
    assert_eq!(
        hello_json["claim"]["expiresAt"],
        constants::lan_pairing::EXPIRES_AT
    );
    assert_eq!(hello_json["claim"]["sequence"], 1);
    assert_eq!(heartbeat_json["claim"]["sequence"], 2);
    assert_eq!(
        heartbeat_json["claim"]["issuedAt"],
        constants::lan_pairing::ISSUED_AT
    );
    assert_eq!(
        heartbeat_json["signatureAlgorithm"],
        constants::lan_pairing::SIGNED_CHILD_AGENT_SIGNATURE_ALGORITHM_ED25519
    );
}

#[test]
fn signed_household_mesh_transport_claim_round_trips_versioned_wire_shape() {
    let claim = signed_household_mesh_transport_claim();
    let claim_json = to_json(claim.clone());

    assert_eq!(
        claim_json["schemaVersion"],
        serde_json::json!(LAN_SIGNED_HOUSEHOLD_MESH_INGRESS_SCHEMA_VERSION)
    );
    assert_eq!(claim_json["messageKind"], serde_json::json!("hello"));
    assert_eq!(
        claim_json["lanMessageType"],
        serde_json::json!(constants::household_mesh::LAN_MESSAGE_DEVICE_DISCOVERY)
    );
    assert_eq!(
        claim_json["localEventRef"],
        serde_json::json!(constants::household_mesh::LOCAL_EVENT_DEVICE_DISCOVERY)
    );
    assert_eq!(claim_json["sequence"], serde_json::json!(7));

    let decoded = serde_json::from_value::<LanSignedHouseholdMeshTransportClaimDto>(claim_json)
        .expect_value("signed household mesh transport claim round trips");
    assert_eq!(decoded, claim);

    let mut heartbeat_json = to_json(claim);
    heartbeat_json["messageKind"] = serde_json::json!("heartbeat");
    let heartbeat =
        serde_json::from_value::<LanSignedHouseholdMeshTransportClaimDto>(heartbeat_json)
            .expect_value("signed household mesh heartbeat claim round trips");
    assert_eq!(
        heartbeat.message_kind,
        LanSignedChildAgentMessageKind::Heartbeat
    );
}

#[test]
fn signed_household_mesh_transport_scalars_reject_invalid_values() {
    assert_eq!(
        LanHouseholdMeshMessageId::try_new(""),
        Err(EventingError::EmptyValue {
            field: "LanHouseholdMeshMessageId"
        })
    );
    assert_eq!(
        LanHouseholdMeshMessageId::try_new("message with whitespace"),
        Err(EventingError::InvalidValue {
            field: "LanHouseholdMeshMessageId",
            value: "[redacted]".to_string(),
        })
    );
    assert_eq!(
        LanHouseholdMeshPayloadSha256::try_new("f".repeat(63)),
        Err(EventingError::InvalidValue {
            field: "LanHouseholdMeshPayloadSha256",
            value: "[redacted]".to_string(),
        })
    );
    assert_eq!(
        LanHouseholdMeshPayloadSha256::try_new("g".repeat(64)),
        Err(EventingError::InvalidValue {
            field: "LanHouseholdMeshPayloadSha256",
            value: "[redacted]".to_string(),
        })
    );
    assert_eq!(
        LanSignedHouseholdMeshMessageType::try_new("future-lan-message"),
        Err(EventingError::InvalidValue {
            field: "LanSignedHouseholdMeshMessageType",
            value: "[redacted]".to_string(),
        })
    );

    let message_type = LanSignedHouseholdMeshMessageType::try_new(
        constants::household_mesh::LAN_MESSAGE_DEVICE_DISCOVERY,
    )
    .expect_value("registered household mesh message type constructs");
    assert_eq!(
        message_type.local_event_ref(),
        Some(constants::household_mesh::LOCAL_EVENT_DEVICE_DISCOVERY)
    );
}

#[test]
fn signed_household_mesh_transport_schema_and_sequence_reject_drift() {
    let schema_version = LanHouseholdMeshIngressSchemaVersionDto::current();
    assert_eq!(schema_version.value(), 1);
    assert_eq!(
        LanHouseholdMeshIngressSchemaVersionDto::try_new(2),
        Err(EventingError::InvalidValue {
            field: "LanHouseholdMeshIngressSchemaVersionDto",
            value: "[redacted]".to_string(),
        })
    );
    let schema_error =
        serde_json::from_value::<LanHouseholdMeshIngressSchemaVersionDto>(serde_json::json!(2))
            .expect_err_value("future signed household-mesh schema must be rejected");
    assert!(schema_error.to_string().contains("invalid eventing value"));

    let sequence = LanHouseholdMeshSequenceDto::try_new(7)
        .expect_value("nonzero household mesh sequence constructs");
    assert_eq!(sequence.value(), 7);
    assert_eq!(
        LanHouseholdMeshSequenceDto::try_new(0),
        Err(EventingError::InvalidValue {
            field: "LanHouseholdMeshSequenceDto",
            value: "[redacted]".to_string(),
        })
    );
    let sequence_error =
        serde_json::from_value::<LanHouseholdMeshSequenceDto>(serde_json::json!(0))
            .expect_err_value("zero signed household-mesh sequence must be rejected");
    assert!(sequence_error
        .to_string()
        .contains("invalid eventing value"));
}

#[test]
fn signed_household_mesh_transport_claim_rejects_unknown_and_future_fields() {
    let claim_json = to_json(signed_household_mesh_transport_claim());

    let mut unknown_field = claim_json.clone();
    unknown_field["futureField"] = serde_json::json!(true);
    let unknown_error =
        serde_json::from_value::<LanSignedHouseholdMeshTransportClaimDto>(unknown_field)
            .expect_err_value("unknown signed household-mesh claim field must be rejected");
    assert!(unknown_error.to_string().contains("unknown field"));

    let mut future_schema = claim_json.clone();
    future_schema["schemaVersion"] = serde_json::json!(2);
    let schema_error =
        serde_json::from_value::<LanSignedHouseholdMeshTransportClaimDto>(future_schema)
            .expect_err_value("future signed household-mesh claim schema must be rejected");
    assert!(schema_error.to_string().contains("invalid eventing value"));

    let mut future_message_kind = claim_json;
    future_message_kind["messageKind"] = serde_json::json!("future-message-kind");
    let kind_error =
        serde_json::from_value::<LanSignedHouseholdMeshTransportClaimDto>(future_message_kind)
            .expect_err_value("future signed household-mesh message kind must be rejected");
    assert!(kind_error.to_string().contains("unknown variant"));
}

#[test]
fn signed_household_mesh_transport_envelope_rejects_unknown_and_missing_fields() {
    let claim = to_json(signed_household_mesh_transport_claim());
    let unknown_field = serde_json::json!({
        "schemaVersion": 1,
        "claim": claim,
        "futureField": true,
    });
    let error = serde_json::from_value::<LanSignedHouseholdMeshTransportEnvelope>(unknown_field)
        .expect_err_value("unknown signed transport envelope fields must be rejected");
    assert!(error.to_string().contains("unknown field"));

    let missing_fields = serde_json::json!({
        "schemaVersion": 1,
        "claim": to_json(signed_household_mesh_transport_claim()),
    });
    let missing_error =
        serde_json::from_value::<LanSignedHouseholdMeshTransportEnvelope>(missing_fields)
            .expect_err_value("missing signed transport envelope fields must be rejected");
    assert!(missing_error.to_string().contains("missing field"));
}

#[test]
fn signed_household_mesh_transport_signing_bytes_are_domain_separated_and_stable() {
    let claim = signed_household_mesh_transport_claim();
    let first = lan_signed_household_mesh_transport_signing_bytes(&claim)
        .expect_value("signed household mesh transport signing bytes construct");
    let second = lan_signed_household_mesh_transport_signing_bytes(&claim)
        .expect_value("signed household mesh transport signing bytes remain stable");
    let claim_bytes = serde_json::to_vec(&claim)
        .expect_value("signed household mesh transport claim serializes for signing");

    assert_eq!(first, second);
    assert!(first.starts_with(b"ocentra.lan.household-mesh.transport-claim.v1\0"));
    assert!(first.ends_with(&claim_bytes));
}

#[test]
fn signed_child_agent_envelope_rejects_missing_claim_fields() {
    let error = result_error_or_unreachable(
        serde_json::from_value::<LanSignedChildAgentEnvelope>(serde_json::json!({
            "schemaVersion": constants::lan_pairing::SCHEMA_VERSION,
            "claim": {
                "schemaVersion": constants::lan_pairing::SCHEMA_VERSION,
                "messageKind": "hello"
            },
            "publicKeyBase64": "public-key-base64",
            "publicKeyId": "public-key-id",
            "signatureBase64": "signature-base64",
            "signatureAlgorithm": constants::lan_pairing::SIGNED_CHILD_AGENT_SIGNATURE_ALGORITHM_ED25519,
        })),
        "missing claim fields must fail closed",
    );

    assert_eq!(error.classify(), serde_json::error::Category::Data);
}

#[test]
fn signed_child_agent_envelope_rejects_missing_signature_fields() {
    let mut envelope_json = to_json(signed_child_agent_envelope(
        LanSignedChildAgentMessageKind::Hello,
        "child-install-3",
        "nonce-3",
        3,
    ));
    let envelope_object = envelope_json
        .as_object_mut()
        .expect_value("envelope is an object");
    envelope_object.remove("signatureBase64");

    let error = result_error_or_unreachable(
        serde_json::from_value::<LanSignedChildAgentEnvelope>(envelope_json),
        "missing signature fields must fail closed",
    );

    assert_eq!(error.classify(), serde_json::error::Category::Data);
}

#[test]
fn lan_pairing_contracts_reject_wrong_schema_versions() {
    let mut proof_json = to_json(LanPairingProof {
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
    });
    proof_json["schemaVersion"] = serde_json::json!(constants::lan_pairing::SCHEMA_VERSION + 1);

    let proof_error = result_error_or_unreachable(
        serde_json::from_value::<LanPairingProof>(proof_json),
        "future LAN schema version must fail closed",
    );
    assert!(proof_error
        .to_string()
        .contains("unsupported LAN schema version"));

    let mut envelope_json = to_json(signed_child_agent_envelope(
        LanSignedChildAgentMessageKind::Hello,
        "child-install-version",
        "nonce-version",
        11,
    ));
    envelope_json["claim"]["schemaVersion"] =
        serde_json::json!(constants::lan_pairing::SCHEMA_VERSION + 1);

    let envelope_error = result_error_or_unreachable(
        serde_json::from_value::<LanSignedChildAgentEnvelope>(envelope_json),
        "nested LAN schema version must fail closed",
    );
    assert!(envelope_error
        .to_string()
        .contains("unsupported LAN schema version"));
}

#[test]
fn lan_pairing_string_schema_read_models_reject_wrong_schema_version() {
    let error = result_error_or_unreachable(
        serde_json::from_value::<DeviceRoleRuntimeReadModel>(serde_json::json!({
            "schemaVersion": "v1.0",
            "physicalDeviceId": "physical-device-1",
            "surface": "child-desktop",
            "platform": constants::lan_pairing::PLATFORM_WINDOWS,
            "roles": [
                {
                    "role": "child-agent",
                    "state": "implemented"
                }
            ],
            "primaryRole": "child-agent",
            "controllerLeaseId": null,
            "parentAuthority": null,
            "selectedRouteId": null,
            "routeState": "local-network",
            "lanAiProviderState": "available",
            "localAiRuntimeClaim": "none",
            "updatedAt": constants::lan_pairing::OBSERVED_AT
        })),
        "string LAN schema version must fail closed",
    );

    assert!(error.to_string().contains("unsupported LAN schema version"));
}

#[test]
fn signed_child_agent_envelope_rejects_unknown_message_kind() {
    let mut envelope_json = serde_json::to_value(signed_child_agent_envelope(
        LanSignedChildAgentMessageKind::Hello,
        "child-install-unknown-kind",
        "nonce-unknown-kind",
        17,
    ))
    .expect_value("signed envelope serializes");
    envelope_json["claim"]["messageKind"] = serde_json::json!("future-lan-message-kind");

    let error = result_error_or_unreachable(
        serde_json::from_value::<LanSignedChildAgentEnvelope>(envelope_json),
        "unknown LAN message kind must be rejected",
    );

    assert!(error.is_data());
}

#[test]
fn mdns_advertisements_use_opaque_metadata_and_hint_only_txt_records() {
    let parent = LanParentMdnsAdvertisement::new(
        "sha256:parent-family-1",
        constants::lan_pairing::SCHEMA_VERSION_TEXT,
        "sha256:family-1",
        LanPairingTrustState::Paired,
        LanMdnsAdvertisementLifecycleState::Start,
        LanMdnsAdvertisementSupportState::Supported,
    )
    .expect_value("parent advertisement constructs");
    let child = LanChildMdnsAdvertisement::new(LanChildMdnsAdvertisementInput {
        advertisement_id: "sha256:child-family-1".to_string(),
        opaque_device_id: "sha256:child-device-1".to_string(),
        protocol_version: constants::lan_pairing::SCHEMA_VERSION_TEXT.to_string(),
        family_hash: "sha256:family-1".to_string(),
        platform: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
        agent_version: "1.2.3".to_string(),
        pairing_state: LanPairingTrustState::Unpaired,
        lifecycle_state: LanMdnsAdvertisementLifecycleState::Degraded,
        support_state: LanMdnsAdvertisementSupportState::Degraded,
    })
    .expect_value("child advertisement constructs");

    let parent_json =
        serde_json::to_value(parent).expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let child_json =
        serde_json::to_value(child).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        parent_json["serviceType"],
        constants::lan_pairing::MDNS_PARENT_SERVICE_TYPE
    );
    assert_eq!(
        child_json["serviceType"],
        constants::lan_pairing::MDNS_CHILD_SERVICE_TYPE
    );
    assert_eq!(
        parent_json["confirmationState"],
        constants::lan_pairing::MDNS_TXT_VALUE_HINT_ONLY
    );
    assert_eq!(
        child_json["confirmationState"],
        constants::lan_pairing::MDNS_TXT_VALUE_HINT_ONLY
    );
    assert_eq!(
        parent_json["txtRecords"][0]["key"],
        constants::lan_pairing::MDNS_TXT_KEY_SCHEMA_VERSION
    );
    assert_eq!(
        parent_json["txtRecords"][4]["value"],
        serde_json::json!(constants::lan_pairing::MDNS_TXT_VALUE_START)
    );
    assert_eq!(
        parent_json["txtRecords"][5]["value"],
        serde_json::json!(constants::lan_pairing::MDNS_TXT_VALUE_SUPPORTED)
    );
    assert_eq!(
        parent_json["txtRecords"][6]["value"],
        serde_json::json!(constants::lan_pairing::MDNS_TXT_VALUE_HINT_ONLY)
    );
    assert_eq!(
        child_json["txtRecords"][2]["key"],
        constants::lan_pairing::MDNS_TXT_KEY_OPAQUE_DEVICE_ID
    );
    assert_eq!(
        child_json["txtRecords"][7]["value"],
        serde_json::json!(constants::lan_pairing::MDNS_TXT_VALUE_DEGRADED)
    );
    assert_eq!(
        child_json["txtRecords"][8]["value"],
        serde_json::json!(constants::lan_pairing::MDNS_TXT_VALUE_DEGRADED)
    );
    assert_eq!(
        child_json["txtRecords"][9]["value"],
        serde_json::json!(constants::lan_pairing::MDNS_TXT_VALUE_HINT_ONLY)
    );
    assert_lan_discovery_surface_has_no_sensitive_markers(&parent_json);
    assert_lan_discovery_surface_has_no_sensitive_markers(&child_json);
}

#[test]
fn mdns_lifecycle_and_support_states_map_to_contract_values() {
    assert_eq!(
        LanMdnsAdvertisementLifecycleState::Start.as_str(),
        constants::lan_pairing::MDNS_TXT_VALUE_START.into()
    );
    assert_eq!(
        LanMdnsAdvertisementLifecycleState::Update.as_str(),
        constants::lan_pairing::MDNS_TXT_VALUE_UPDATE.into()
    );
    assert_eq!(
        LanMdnsAdvertisementLifecycleState::Stop.as_str(),
        constants::lan_pairing::MDNS_TXT_VALUE_STOP.into()
    );
    assert_eq!(
        LanMdnsAdvertisementLifecycleState::Degraded.as_str(),
        constants::lan_pairing::MDNS_TXT_VALUE_DEGRADED.into()
    );
    assert_eq!(
        LanMdnsAdvertisementSupportState::UnsupportedPlatform.as_str(),
        constants::lan_pairing::MDNS_TXT_VALUE_UNSUPPORTED_PLATFORM.into()
    );
}

#[test]
fn mdns_advertisement_constructors_reject_missing_or_unsanitized_values() {
    assert!(LanParentMdnsAdvertisement::new(
        "",
        constants::lan_pairing::SCHEMA_VERSION_TEXT,
        "sha256:family-1",
        LanPairingTrustState::Unpaired,
        LanMdnsAdvertisementLifecycleState::Start,
        LanMdnsAdvertisementSupportState::Supported,
    )
    .is_err());
    assert!(
        LanChildMdnsAdvertisement::new(LanChildMdnsAdvertisementInput {
            advertisement_id: "sha256:child-family-1".to_string(),
            opaque_device_id: "child display name".to_string(),
            protocol_version: constants::lan_pairing::SCHEMA_VERSION_TEXT.to_string(),
            family_hash: "sha256:family-1".to_string(),
            platform: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
            agent_version: "1.2.3".to_string(),
            pairing_state: LanPairingTrustState::Unpaired,
            lifecycle_state: LanMdnsAdvertisementLifecycleState::Start,
            support_state: LanMdnsAdvertisementSupportState::Supported,
        })
        .is_err()
    );
    let error =
        LanMdnsTxtRecord::new("bad key", "value").expect_err_value("invalid TXT key must fail");
    assert!(error.to_string().contains("bad key"));
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
        controller_lease_id: Some(constants::lan_pairing::CONTROLLER_LEASE_ID.to_string()),
        controller_device_id: Some(constants::lan_pairing::PARENT_DEVICE_ID.to_string()),
        parent_actor_id: Some(constants::lan_pairing::PARENT_ACTOR_ID.to_string()),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        origin: Some(constants::lan_pairing::WRONG_ORIGIN.to_string()),
        rejection_reason: Some(LanPairingRejectionReason::WrongOrigin),
        observed_at: constants::lan_pairing::OBSERVED_AT.to_string(),
        evidence_references: vec![evidence()],
    };

    let event_json = to_json(event);
    assert_eq!(event_json["eventType"], "control-rejected");
    assert_eq!(event_json["rejectionReason"], "wrong-origin");
    assert_eq!(
        event_json["evidenceReferences"][0]["evidenceReferenceId"],
        constants::lan_pairing::EVIDENCE_REFERENCE_ID
    );
    assert_lan_pairing_audit_event_has_no_sensitive_markers(&event_json);
}

#[test]
fn lan_pairing_discovery_challenge_and_proof_preview_make_websocket_ceremony_explicit() {
    let discovery = LanPairingDiscoveryDevice {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        discovered_at: constants::lan_pairing::OBSERVED_AT.to_string(),
        child_device: child_device(),
        agent_peer_id: constants::lan_pairing::PARENT_PEER_ID.to_string(),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        network_mode: LanPairingNetworkMode::LocalNetwork,
        reachability: LanPairingDeviceReachability::Stale,
        address_ref: constants::lan_pairing::ADDRESS_REF_UNPROVEN.to_string(),
        discovery_status: LanPairingDiscoveryRuntimeStatus::WebsocketDirect,
        discovery_state: LanPairingProductionDiscoveryState::Stale,
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
        challenge_status: LanPairingDiscoveryRuntimeStatus::WebsocketDirect,
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
        proof_preview_status: LanPairingDiscoveryRuntimeStatus::WebsocketDirect,
    };

    let discovery_json = to_json(discovery);
    let challenge_request_json = challenge_request_json();
    let challenge_json = to_json(challenge);
    let preview_json = to_json(preview);

    assert_eq!(
        discovery_json["discoveryStatus"],
        constants::lan_pairing::SUPPORT_WEBSOCKET_DIRECT
    );
    assert_eq!(
        challenge_request_json["parentDeviceId"],
        constants::lan_pairing::PARENT_DEVICE_ID
    );
    assert_eq!(
        discovery_json["addressRef"],
        constants::lan_pairing::ADDRESS_REF_UNPROVEN
    );
    assert_eq!(
        challenge_json["challengeStatus"],
        constants::lan_pairing::SUPPORT_WEBSOCKET_DIRECT
    );
    assert_eq!(
        preview_json["proofPreviewStatus"],
        constants::lan_pairing::SUPPORT_WEBSOCKET_DIRECT
    );
    assert_eq!(
        preview_json["proofDigest"],
        constants::lan_pairing::PROOF_DIGEST
    );
    assert_lan_discovery_surface_has_no_sensitive_markers(&discovery_json);
    assert_lan_discovery_surface_has_no_sensitive_markers(&challenge_json);
    assert_lan_discovery_surface_has_no_sensitive_markers(&preview_json);
}

fn challenge_request_json() -> serde_json::Value {
    to_json(LanPairingChallengeRequest {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        child_device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
        parent_device_id: constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        origin: constants::lan_pairing::ALLOWED_ORIGIN.to_string(),
        issued_at: constants::lan_pairing::ISSUED_AT.to_string(),
        expires_at: constants::lan_pairing::EXPIRES_AT.to_string(),
    })
}

fn assert_lan_discovery_surface_has_no_sensitive_markers(value: &serde_json::Value) {
    fn value_has_sensitive_marker(value: &serde_json::Value) -> bool {
        const MARKERS: [&str; 11] = [
            "activity.sqlite",
            "activity.ndjson",
            "activityDigest",
            "controlAuthority",
            "decryptedEvidence",
            "evidenceReferences",
            "journalPath",
            "rawEvidence",
            "rawProofSecret",
            "rawToken",
            "sqlitePath",
        ];

        match value {
            serde_json::Value::String(text) => MARKERS
                .iter()
                .any(|marker| text.match_indices(marker).next().is_some()),
            serde_json::Value::Array(items) => items.iter().any(value_has_sensitive_marker),
            serde_json::Value::Object(map) => map.iter().any(|(key, item)| {
                MARKERS
                    .iter()
                    .any(|marker| key.match_indices(marker).next().is_some())
                    || value_has_sensitive_marker(item)
            }),
            _ => false,
        }
    }

    assert!(!value_has_sensitive_marker(value));
}

fn assert_lan_pairing_audit_event_has_no_sensitive_markers(value: &serde_json::Value) {
    fn value_has_sensitive_marker(value: &serde_json::Value) -> bool {
        const MARKERS: [&str; 2] = ["rawToken", "rawEvidence"];

        match value {
            serde_json::Value::String(text) => MARKERS
                .iter()
                .any(|marker| text.match_indices(marker).next().is_some()),
            serde_json::Value::Array(items) => items.iter().any(value_has_sensitive_marker),
            serde_json::Value::Object(map) => map.iter().any(|(key, item)| {
                MARKERS
                    .iter()
                    .any(|marker| key.match_indices(marker).next().is_some())
                    || value_has_sensitive_marker(item)
            }),
            _ => false,
        }
    }

    assert!(!value_has_sensitive_marker(value));
}

#[test]
fn lan_pairing_read_model_values_keep_local_network_state_explicit() {
    let selected = LanSelectedRouteTarget {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        selected_child_device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        pairing_id: Some(constants::lan_pairing::PAIRING_ID.to_string()),
        trust_state: LanPairingTrustState::Paired,
        network_mode: LanPairingNetworkMode::LocalNetwork,
        reachability: LanPairingDeviceReachability::Stale,
        stale_at: Some(constants::lan_pairing::OBSERVED_AT.to_string()),
        offline_at: None,
    };
    let intent = LanParentIntentEnvelope {
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
        controller_lease_id: constants::lan_pairing::CONTROLLER_LEASE_ID.to_string(),
        controller_device_id: constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        parent_actor_id: constants::lan_pairing::PARENT_ACTOR_ID.to_string(),
        parent_authority: LanPairingParentAuthority::ActiveController,
        controller_lease_issued_at: constants::lan_pairing::ISSUED_AT.to_string(),
        controller_lease_expires_at: constants::lan_pairing::CONTROLLER_LEASE_EXPIRES_AT
            .to_string(),
        evidence_references: vec![evidence()],
    };

    let selected_json = to_json(selected);
    let intent_json = to_json(intent);

    assert_eq!(selected_json["networkMode"], "local-network");
    assert_eq!(selected_json["reachability"], "stale");
    assert_eq!(selected_json["trustState"], "paired");
    assert_eq!(selected_json["offlineAt"], serde_json::Value::Null);
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

    let rule_query_json = to_json(rule_query);
    let rule_update_json = to_json(rule_update);
    let approval_decision_json = to_json(approval_decision);
    let rejected_json = to_json(rejected);
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
    assert_lan_pairing_audit_event_has_no_sensitive_markers(&rejected_json);
}

#[test]
fn lan_pairing_runtime_support_surface_serializes_supported_and_planned_api_claims() {
    let support = websocket_runtime_support_surface();

    let support_json =
        serde_json::to_value(support).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

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
    assert!(support_json["supportedWebSocketCommands"]
        .as_array()
        .expect_value("supported websocket commands serializes as an array")
        .iter()
        .any(|command| command == constants::lan_pairing::COMMAND_RUNTIME_EVENT_CHAIN_STREAM_GET));
    assert_eq!(
        support_json["unsupportedHttpEndpoints"][0]["support"],
        constants::lan_pairing::SUPPORT_PLANNED_UNSUPPORTED
    );
    assert_eq!(
        support_json["discoveryStatus"],
        constants::lan_pairing::SUPPORT_WEBSOCKET_DIRECT
    );
    assert_eq!(
        support_json["discoveryState"],
        constants::value::LAN_DISCOVERY_STATE_DISCOVERED
    );
    assert_eq!(
        support_json["challengeStatus"],
        constants::lan_pairing::SUPPORT_WEBSOCKET_DIRECT
    );
    assert_eq!(
        support_json["proofPreviewStatus"],
        constants::lan_pairing::SUPPORT_WEBSOCKET_DIRECT
    );
    assert_eq!(
        support_json["persistenceMode"],
        constants::value::LAN_PERSISTENCE_IN_MEMORY_FAIL_CLOSED
    );
    assert_eq!(
        support_json["restartBehavior"],
        constants::value::LAN_RESTART_FAIL_CLOSED_UNPAIRED
    );
}

fn websocket_runtime_support_surface() -> LanPairingRuntimeSupportSurface {
    LanPairingRuntimeSupportSurface {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        transport: LanPairingTransport::Websocket,
        supported_websocket_commands: constants::lan_pairing::SUPPORTED_WEBSOCKET_COMMANDS
            .iter()
            .map(|command| command.to_string())
            .collect(),
        unsupported_http_endpoints: planned_http_endpoints(),
        pairing_state: LanPairingTrustState::Unpaired,
        trusted_device_count: 0,
        discovery_status: LanPairingDiscoveryRuntimeStatus::WebsocketDirect,
        discovery_state: LanPairingProductionDiscoveryState::Discovered,
        challenge_status: LanPairingDiscoveryRuntimeStatus::WebsocketDirect,
        proof_preview_status: LanPairingDiscoveryRuntimeStatus::WebsocketDirect,
        lan_ai_provider_status: LanPairingDiscoveryRuntimeStatus::WebsocketDirect,
        lan_ai_provider_routing_state: LanAiProviderRoutingState::Unavailable,
        lan_ai_provider_custody_label:
            constants::value::LAN_PROVIDER_CUSTODY_LOCAL_NETWORK_AI_PROVIDER.to_string(),
        lan_ai_job_status: LanPairingDiscoveryRuntimeStatus::WebsocketDirect,
        persistence_mode: LanPairingPersistenceMode::InMemoryFailClosed,
        restart_behavior: LanPairingRestartBehavior::FailClosedUnpaired,
        proof_mode: LanPairingProofMode::DirectProofSubmit,
        route_requirements: vec![
            LanPairingRouteRequirement::PairedDevice,
            LanPairingRouteRequirement::AllowedOrigin,
            LanPairingRouteRequirement::TargetDeviceMatch,
            LanPairingRouteRequirement::RouteIdMatch,
            LanPairingRouteRequirement::UnexpiredIntent,
            LanPairingRouteRequirement::NonReplayedIntent,
            LanPairingRouteRequirement::UnrevokedPairing,
            LanPairingRouteRequirement::ActiveControllerLease,
            LanPairingRouteRequirement::SelectedDeviceReachable,
            LanPairingRouteRequirement::ParentWriteAuthority,
            LanPairingRouteRequirement::LanAiJobAuthorized,
            LanPairingRouteRequirement::DiscoveryStateExplicit,
            LanPairingRouteRequirement::RouteRecoveryPersisted,
        ],
        manual_proof_gaps: vec![
            LanPairingManualProofGap::ManualLanBindProof,
            LanPairingManualProofGap::ManualFirewallProof,
            LanPairingManualProofGap::ManualPhysicalDeviceProof,
        ],
    }
}

#[test]
fn lan_pairing_runtime_support_surface_serializes_local_registry_persistence() {
    let persistence_json = serde_json::to_value(LanPairingPersistenceMode::LocalJsonRegistry)
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let restart_json =
        serde_json::to_value(LanPairingRestartBehavior::RestoreTrustedRegistryUnselected)
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        persistence_json,
        constants::value::LAN_PERSISTENCE_LOCAL_JSON_REGISTRY
    );
    assert_eq!(
        restart_json,
        constants::value::LAN_RESTART_RESTORE_TRUSTED_REGISTRY_UNSELECTED
    );
}

#[test]
fn lan_pairing_registry_snapshot_and_route_decision_make_selection_explicit() {
    let selected = LanSelectedRouteTarget {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        selected_child_device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        pairing_id: Some(constants::lan_pairing::PAIRING_ID.to_string()),
        trust_state: LanPairingTrustState::Paired,
        network_mode: LanPairingNetworkMode::LocalNetwork,
        reachability: LanPairingDeviceReachability::Online,
        stale_at: None,
        offline_at: None,
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
        serde_json::to_value(snapshot).expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let selection_json =
        serde_json::to_value(selection).expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let rejected_json =
        serde_json::to_value(rejected).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(snapshot_json["authenticationState"], "paired");
    assert_eq!(
        snapshot_json["selectedTarget"]["selectedChildDeviceId"],
        constants::lan_pairing::CHILD_DEVICE_ID
    );
    assert_eq!(snapshot_json["selectedTarget"]["trustState"], "paired");
    assert_eq!(
        snapshot_json["selectedTarget"]["offlineAt"],
        serde_json::Value::Null
    );
    assert_eq!(
        selection_json["targetChildDeviceId"],
        constants::lan_pairing::CHILD_DEVICE_ID
    );
    assert_eq!(rejected_json["rejectionReason"], "unselected-device");
}
