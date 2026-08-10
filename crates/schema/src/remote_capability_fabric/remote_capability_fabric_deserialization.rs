use serde::{de::Error as _, Deserialize, Deserializer};

use super::{
    RemoteActorRole, RemoteCapabilityGrant, RemoteCapabilityType, RemoteDeviceTrustState,
    RemoteGrantState, RemotePairingState, RemoteParentGrantState, RemoteRoute, RemoteSessionState,
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RemoteCapabilityGrantWire {
    schema_version: String,
    grant_ref: String,
    household_ref: String,
    child_device_ref: String,
    route: Option<RemoteRoute>,
    parent_actor_ref: String,
    #[serde(default)]
    parent_grant: Option<RemoteParentGrantState>,
    capability_type: RemoteCapabilityType,
    actor_role: RemoteActorRole,
    pairing_state: RemotePairingState,
    grant_state: RemoteGrantState,
    session_state: RemoteSessionState,
    device_trust_state: RemoteDeviceTrustState,
    audit_ref: String,
    diagnostic_redaction_state: String,
    no_claim: String,
}

impl<'de> Deserialize<'de> for RemoteCapabilityGrant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = RemoteCapabilityGrantWire::deserialize(deserializer)?;
        let route = match (wire.schema_version.as_str(), wire.route) {
            ("remote-capability-fabric-v1", Some(route)) => route,
            ("remote-capability-fabric-v1", None) => RemoteRoute::Localhost,
            (_, Some(route)) => route,
            (_, None) => {
                return Err(D::Error::custom(
                    "remote capability fabric v2 payload must include route",
                ));
            }
        };
        Ok(Self {
            schema_version: wire.schema_version,
            grant_ref: wire.grant_ref,
            household_ref: wire.household_ref,
            child_device_ref: wire.child_device_ref,
            route,
            parent_actor_ref: wire.parent_actor_ref,
            parent_grant: wire
                .parent_grant
                .unwrap_or(RemoteParentGrantState::NotGranted),
            capability_type: wire.capability_type,
            actor_role: wire.actor_role,
            pairing_state: wire.pairing_state,
            grant_state: wire.grant_state,
            session_state: wire.session_state,
            device_trust_state: wire.device_trust_state,
            audit_ref: wire.audit_ref,
            diagnostic_redaction_state: wire.diagnostic_redaction_state,
            no_claim: wire.no_claim,
        })
    }
}
