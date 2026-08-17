use ocentra_eventing::error::EventingError;
use serde::{Deserialize, Serialize};

use crate::{
    constants::household_mesh,
    household_mesh::HouseholdMeshTransportEnvelope,
    lan_pairing::{LanSignedChildAgentEnvelope, LanSignedChildAgentMessageKind},
};

mod validation;

use validation::{
    deserialize_schema_version, deserialize_sequence, validate_transport_message_type,
    validate_transport_sha256, validate_transport_text,
};

// BOUNDARY-INVARIANT: every externally decoded identifier, digest, timestamp,
// algorithm, and sequence is a validated protocol-owned value before LAN
// cryptographic verification can inspect the packet.
pub const LAN_SIGNED_HOUSEHOLD_MESH_INGRESS_SCHEMA_VERSION: u16 = 1;
const LAN_SIGNED_HOUSEHOLD_MESH_TRANSPORT_SIGNATURE_DOMAIN: &[u8] =
    b"ocentra.lan.household-mesh.transport-claim.v1\0";
const MAX_SIGNED_TRANSPORT_TEXT_BYTES: usize = 4_096;

macro_rules! bounded_transport_text_value {
    ($name:ident, $validator:ident) => {
        /// Validated protocol-owned scalar used by signed household-mesh ingress.
        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            /// Validates and creates this protocol-owned scalar.
            pub fn try_new(value: impl Into<String>) -> Result<Self, EventingError> {
                $validator(value, stringify!($name)).map(Self)
            }

            /// Returns the validated scalar text.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = EventingError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(self.as_str())
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .debug_tuple(stringify!($name))
                    .field(&"[redacted]")
                    .finish()
            }
        }
    };
}

bounded_transport_text_value!(LanHouseholdMeshMessageId, validate_transport_text);
bounded_transport_text_value!(LanHouseholdMeshIdempotencyKey, validate_transport_text);
bounded_transport_text_value!(LanHouseholdMeshLocalEventRef, validate_transport_text);
bounded_transport_text_value!(LanHouseholdMeshPayloadSha256, validate_transport_sha256);
bounded_transport_text_value!(LanHouseholdMeshFamilyHash, validate_transport_text);
bounded_transport_text_value!(LanHouseholdMeshParentDeviceId, validate_transport_text);
bounded_transport_text_value!(LanHouseholdMeshChildDeviceId, validate_transport_text);
bounded_transport_text_value!(LanHouseholdMeshTargetDeviceId, validate_transport_text);
bounded_transport_text_value!(LanHouseholdMeshInstallId, validate_transport_text);
bounded_transport_text_value!(LanHouseholdMeshRouteId, validate_transport_text);
bounded_transport_text_value!(LanHouseholdMeshPairingId, validate_transport_text);
bounded_transport_text_value!(LanHouseholdMeshRegistryProofDigest, validate_transport_text);
bounded_transport_text_value!(LanHouseholdMeshNonce, validate_transport_text);
bounded_transport_text_value!(LanHouseholdMeshTimestamp, validate_transport_text);
bounded_transport_text_value!(LanHouseholdMeshPublicKeyBase64, validate_transport_text);
bounded_transport_text_value!(LanHouseholdMeshPublicKeyId, validate_transport_text);
bounded_transport_text_value!(LanHouseholdMeshPublicKeySha256, validate_transport_sha256);
bounded_transport_text_value!(LanHouseholdMeshSignatureBase64, validate_transport_text);
bounded_transport_text_value!(LanHouseholdMeshSignatureAlgorithm, validate_transport_text);

/// BRAND-INVARIANT: construction accepts only the single supported schema
/// version, so deserialization cannot create an unsupported version value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LanHouseholdMeshIngressSchemaVersionDto(
    #[serde(deserialize_with = "deserialize_schema_version")] u16,
);

impl LanHouseholdMeshIngressSchemaVersionDto {
    /// Returns the current supported schema version.
    pub fn current() -> Self {
        Self(LAN_SIGNED_HOUSEHOLD_MESH_INGRESS_SCHEMA_VERSION)
    }

    /// Validates and creates a signed ingress schema version.
    pub fn try_new(value: u16) -> Result<Self, EventingError> {
        if value != LAN_SIGNED_HOUSEHOLD_MESH_INGRESS_SCHEMA_VERSION {
            return Err(invalid_transport_value(
                "LanHouseholdMeshIngressSchemaVersionDto",
            ));
        }
        Ok(Self(value))
    }

    /// Returns the protocol version number.
    pub fn value(self) -> u16 {
        self.0
    }
}

/// BRAND-INVARIANT: zero is not a valid signed replay sequence; every accepted
/// value identifies a concrete monotonic child-message position.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LanHouseholdMeshSequenceDto(#[serde(deserialize_with = "deserialize_sequence")] u64);

impl LanHouseholdMeshSequenceDto {
    /// Validates and creates a nonzero signed replay sequence.
    pub fn try_new(value: u64) -> Result<Self, EventingError> {
        if value == 0 {
            return Err(invalid_transport_value("LanHouseholdMeshSequenceDto"));
        }
        Ok(Self(value))
    }

    /// Returns the monotonic sequence number.
    pub fn value(self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for LanHouseholdMeshSequenceDto {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

#[derive(Clone, Copy)]
struct LanSignedHouseholdMeshProtocolValues {
    message_type: &'static str,
    event_ref: &'static str,
}

const LAN_SIGNED_HOUSEHOLD_MESH_PROTOCOL_VALUES: [LanSignedHouseholdMeshProtocolValues; 13] = [
    LanSignedHouseholdMeshProtocolValues {
        message_type: household_mesh::LAN_MESSAGE_DEVICE_DISCOVERY,
        event_ref: household_mesh::LOCAL_EVENT_DEVICE_DISCOVERY,
    },
    LanSignedHouseholdMeshProtocolValues {
        message_type: household_mesh::LAN_MESSAGE_PROVIDER_ADVERTISEMENT,
        event_ref: household_mesh::LOCAL_EVENT_PROVIDER_ADVERTISEMENT,
    },
    LanSignedHouseholdMeshProtocolValues {
        message_type: household_mesh::LAN_MESSAGE_PROVIDER_HEARTBEAT,
        event_ref: household_mesh::LOCAL_EVENT_PROVIDER_HEARTBEAT,
    },
    LanSignedHouseholdMeshProtocolValues {
        message_type: household_mesh::LAN_MESSAGE_PROVIDER_CAPABILITY,
        event_ref: household_mesh::LOCAL_EVENT_PROVIDER_CAPABILITY,
    },
    LanSignedHouseholdMeshProtocolValues {
        message_type: household_mesh::LAN_MESSAGE_AI_WORK_OFFER,
        event_ref: household_mesh::LOCAL_EVENT_AI_WORK_OFFER,
    },
    LanSignedHouseholdMeshProtocolValues {
        message_type: household_mesh::LAN_MESSAGE_AI_WORK_CLAIM_REQUEST,
        event_ref: household_mesh::LOCAL_EVENT_AI_WORK_CLAIM_REQUEST,
    },
    LanSignedHouseholdMeshProtocolValues {
        message_type: household_mesh::LAN_MESSAGE_AI_WORK_CLAIM_DECISION,
        event_ref: household_mesh::LOCAL_EVENT_AI_WORK_CLAIM_DECISION,
    },
    LanSignedHouseholdMeshProtocolValues {
        message_type: household_mesh::LAN_MESSAGE_AI_WORK_LEASE_STATE,
        event_ref: household_mesh::LOCAL_EVENT_AI_WORK_LEASE_STATE,
    },
    LanSignedHouseholdMeshProtocolValues {
        message_type: household_mesh::LAN_MESSAGE_AI_JOB_PAYLOAD_TRANSFER,
        event_ref: household_mesh::LOCAL_EVENT_AI_JOB_PAYLOAD_TRANSFER,
    },
    LanSignedHouseholdMeshProtocolValues {
        message_type: household_mesh::LAN_MESSAGE_AI_RESULT_RETURN,
        event_ref: household_mesh::LOCAL_EVENT_AI_RESULT_RETURN,
    },
    LanSignedHouseholdMeshProtocolValues {
        message_type: household_mesh::LAN_MESSAGE_CONFIG_COMMAND,
        event_ref: household_mesh::LOCAL_EVENT_CONFIG_COMMAND,
    },
    LanSignedHouseholdMeshProtocolValues {
        message_type: household_mesh::LAN_MESSAGE_APPROVAL_OVERRIDE_COMMAND,
        event_ref: household_mesh::LOCAL_EVENT_APPROVAL_OVERRIDE_COMMAND,
    },
    LanSignedHouseholdMeshProtocolValues {
        message_type: household_mesh::LAN_MESSAGE_READ_MODEL_QUERY_REQUEST,
        event_ref: household_mesh::LOCAL_EVENT_READ_MODEL_QUERY_REQUEST,
    },
];

bounded_transport_text_value!(
    LanSignedHouseholdMeshMessageType,
    validate_transport_message_type
);

impl LanSignedHouseholdMeshMessageType {
    /// Returns the local-event reference paired with this validated message type.
    pub fn local_event_ref(&self) -> Option<&'static str> {
        LAN_SIGNED_HOUSEHOLD_MESH_PROTOCOL_VALUES
            .iter()
            .find(|values| values.message_type == self.as_str())
            .map(|values| values.event_ref)
    }
}

/// Complete identity and payload-digest statement signed by the child peer.
/// SERIALIZATION-DOC: camelCase field names and declaration order are the v1
/// canonical JSON signing contract; schema_version gates incompatible changes.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LanSignedHouseholdMeshTransportClaimDto {
    pub schema_version: LanHouseholdMeshIngressSchemaVersionDto,
    pub message_kind: LanSignedChildAgentMessageKind,
    pub message_id: LanHouseholdMeshMessageId,
    pub idempotency_key: LanHouseholdMeshIdempotencyKey,
    pub local_event_ref: LanHouseholdMeshLocalEventRef,
    pub lan_message_type: LanSignedHouseholdMeshMessageType,
    pub canonical_payload_sha256: LanHouseholdMeshPayloadSha256,
    pub family_hash: LanHouseholdMeshFamilyHash,
    pub parent_device_id: LanHouseholdMeshParentDeviceId,
    pub child_device_id: LanHouseholdMeshChildDeviceId,
    pub target_device_id: LanHouseholdMeshTargetDeviceId,
    pub install_id: LanHouseholdMeshInstallId,
    pub route_id: LanHouseholdMeshRouteId,
    pub pairing_id: LanHouseholdMeshPairingId,
    pub registry_proof_digest: LanHouseholdMeshRegistryProofDigest,
    pub nonce: LanHouseholdMeshNonce,
    pub sequence: LanHouseholdMeshSequenceDto,
    pub issued_at: LanHouseholdMeshTimestamp,
    pub expires_at: LanHouseholdMeshTimestamp,
}

impl std::fmt::Debug for LanSignedHouseholdMeshTransportClaimDto {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("LanSignedHouseholdMeshTransportClaimDto")
            .field("schema_version", &self.schema_version)
            .field("message_kind", &self.message_kind)
            .field("sequence", &self.sequence.value())
            .field("signed_identity", &"[redacted]")
            .finish()
    }
}

/// Wire envelope carrying one signed transport claim and its canonical payload.
/// SERIALIZATION-DOC: camelCase fields form the bounded v1 LAN ingress envelope;
/// unknown or incompatible schema versions are rejected by LAN verification.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LanSignedHouseholdMeshTransportEnvelope {
    pub schema_version: LanHouseholdMeshIngressSchemaVersionDto,
    pub claim: LanSignedHouseholdMeshTransportClaimDto,
    pub payload: HouseholdMeshTransportEnvelope,
    pub public_key_base64: LanHouseholdMeshPublicKeyBase64,
    pub public_key_id: LanHouseholdMeshPublicKeyId,
    pub signature_base64: LanHouseholdMeshSignatureBase64,
    pub signature_algorithm: LanHouseholdMeshSignatureAlgorithm,
}

impl std::fmt::Debug for LanSignedHouseholdMeshTransportEnvelope {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("LanSignedHouseholdMeshTransportEnvelope")
            .field("schema_version", &self.schema_version)
            .field("claim", &self.claim)
            .field("payload", &"[redacted]")
            .field("key_material", &"[redacted]")
            .field("signature", &"[redacted]")
            .finish()
    }
}

/// One child beacon plus the separately signed household-mesh transport packet.
/// SERIALIZATION-DOC: camelCase fields are the v1 peer-ingress packet contract;
/// both nested signatures are verified before this packet can yield any result.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LanSignedChildBeaconIngressEnvelope {
    pub schema_version: LanHouseholdMeshIngressSchemaVersionDto,
    pub signed_child_agent: LanSignedChildAgentEnvelope,
    pub signed_transport: LanSignedHouseholdMeshTransportEnvelope,
}

impl std::fmt::Debug for LanSignedChildBeaconIngressEnvelope {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("LanSignedChildBeaconIngressEnvelope")
            .field("schema_version", &self.schema_version)
            .field("signed_child_agent", &"[redacted]")
            .field("signed_transport", &self.signed_transport)
            .finish()
    }
}

/// Canonical, domain-separated bytes signed by child transport producers and
/// verified by LAN ingress. Keeping this construction in the wire-protocol
/// owner prevents producer/verifier drift.
pub fn lan_signed_household_mesh_transport_signing_bytes(
    claim: &LanSignedHouseholdMeshTransportClaimDto,
) -> Result<Vec<u8>, serde_json::Error> {
    let claim = serde_json::to_vec(claim)?;
    let mut payload = Vec::with_capacity(
        LAN_SIGNED_HOUSEHOLD_MESH_TRANSPORT_SIGNATURE_DOMAIN.len() + claim.len(),
    );
    payload.extend_from_slice(LAN_SIGNED_HOUSEHOLD_MESH_TRANSPORT_SIGNATURE_DOMAIN);
    payload.extend_from_slice(&claim);
    Ok(payload)
}
