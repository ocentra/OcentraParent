use serde::{de::Error as _, Deserialize, Deserializer};

use ocentra_schema::remote_capability_fabric::RemoteActorRole;
use ocentra_schema::remote_capability_fabric::RemoteRoute;

use super::{
    validation, RemoteAccessGrant, RemoteAccessGrantCapability, RemoteAccessGrantDisclosureState,
    RemoteAccessGrantParentGrant, RemoteAccessGrantState,
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct RemoteAccessGrantSnapshot {
    grant_id: String,
    household_ref: String,
    child_device_ref: String,
    route: RemoteRoute,
    parent_actor_ref: String,
    capability: RemoteAccessGrantCapability,
    actor_role: RemoteActorRole,
    state: RemoteAccessGrantState,
    disclosure_state: RemoteAccessGrantDisclosureState,
    #[serde(default = "default_parent_grant")]
    parent_grant: RemoteAccessGrantParentGrant,
    audit_ref: String,
    #[serde(default)]
    attempts: Vec<super::RemoteAccessGrantAuditMilestone>,
}

fn default_parent_grant() -> RemoteAccessGrantParentGrant {
    RemoteAccessGrantParentGrant::NotGranted
}

impl<'de> Deserialize<'de> for RemoteAccessGrant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let snapshot = RemoteAccessGrantSnapshot::deserialize(deserializer)?;
        let grant = RemoteAccessGrant {
            grant_id: snapshot.grant_id,
            household_ref: snapshot.household_ref,
            child_device_ref: snapshot.child_device_ref,
            route: snapshot.route,
            parent_actor_ref: snapshot.parent_actor_ref,
            capability: snapshot.capability,
            actor_role: snapshot.actor_role,
            state: snapshot.state,
            disclosure_state: snapshot.disclosure_state,
            parent_grant: snapshot.parent_grant,
            audit_ref: snapshot.audit_ref,
            attempts: snapshot.attempts,
        };
        validation::serialized(&grant).map_err(D::Error::custom)?;
        Ok(grant)
    }
}
