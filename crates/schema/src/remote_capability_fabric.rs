//! Rust-owned remote live-view capability contract.
//!
//! This module deliberately models a narrow, view-only authorization boundary.
//! It does not create a relay, capture stream, remote-control path, or custody
//! policy; those require their owning runtime workpacks.

use serde::{Deserialize, Serialize};

pub const REMOTE_CAPABILITY_FABRIC_SCHEMA_VERSION: &str = "remote-capability-fabric-v1";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RemoteCapabilityType {
    LiveView,
    ScreenshotRequest,
    Diagnostic,
    RemoteControlDeferred,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RemoteRoute {
    Localhost,
    LocalNetwork,
    CloudRelay,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RemoteActorRole {
    ParentOwner,
    CoParent,
    SupportAdmin,
    ChildAgent,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RemotePairingState {
    Requested,
    Paired,
    Denied,
    Failed,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RemoteGrantState {
    Requested,
    Active,
    Revoked,
    Removed,
    Denied,
    Failed,
    Expired,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RemoteSessionState {
    Requested,
    Authorized,
    Paired,
    Connecting,
    Active,
    Degraded,
    Stopped,
    Removed,
    Revoked,
    Denied,
    Failed,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RemoteDeviceTrustState {
    Trusted,
    Missing,
    Expired,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RemoteCapabilityAuthorizationError {
    UnsupportedSchemaVersion,
    CapabilityDeferred,
    WrongHousehold,
    WrongActorRole,
    WrongParentActor,
    PairingRequired,
    GrantNotActive,
    Revoked,
    DeviceRemoved,
    WrongChildDevice,
    WrongRoute,
    DeviceTrustRequired,
    MissingAuditRef,
    SessionNotLiveViewEligible,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteCapabilityGrant {
    pub schema_version: String,
    pub grant_ref: String,
    pub household_ref: String,
    pub child_device_ref: String,
    pub route: RemoteRoute,
    pub parent_actor_ref: String,
    pub capability_type: RemoteCapabilityType,
    pub actor_role: RemoteActorRole,
    pub pairing_state: RemotePairingState,
    pub grant_state: RemoteGrantState,
    pub session_state: RemoteSessionState,
    pub device_trust_state: RemoteDeviceTrustState,
    pub audit_ref: String,
    pub diagnostic_redaction_state: String,
    pub no_claim: String,
}

impl RemoteCapabilityGrant {
    pub fn authorize_live_view(
        &self,
        expected_household_ref: &str,
        requesting_parent_actor_ref: &str,
        requested_child_device_ref: &str,
        expected_route: RemoteRoute,
    ) -> Result<(), RemoteCapabilityAuthorizationError> {
        if self.schema_version != REMOTE_CAPABILITY_FABRIC_SCHEMA_VERSION {
            return Err(RemoteCapabilityAuthorizationError::UnsupportedSchemaVersion);
        }
        if self.capability_type != RemoteCapabilityType::LiveView {
            return Err(RemoteCapabilityAuthorizationError::CapabilityDeferred);
        }
        if self.household_ref != expected_household_ref {
            return Err(RemoteCapabilityAuthorizationError::WrongHousehold);
        }
        if !matches!(
            self.actor_role,
            RemoteActorRole::ParentOwner | RemoteActorRole::CoParent
        ) {
            return Err(RemoteCapabilityAuthorizationError::WrongActorRole);
        }
        if let Some(error) = (self.parent_actor_ref != requesting_parent_actor_ref)
            .then_some(RemoteCapabilityAuthorizationError::WrongParentActor)
            .or_else(|| {
                (self.child_device_ref != requested_child_device_ref)
                    .then_some(RemoteCapabilityAuthorizationError::WrongChildDevice)
            })
            .or_else(|| {
                (self.route != expected_route)
                    .then_some(RemoteCapabilityAuthorizationError::WrongRoute)
            })
        {
            return Err(error);
        }
        if self.pairing_state != RemotePairingState::Paired {
            return Err(RemoteCapabilityAuthorizationError::PairingRequired);
        }
        if self.grant_state == RemoteGrantState::Revoked {
            return Err(RemoteCapabilityAuthorizationError::Revoked);
        }
        if self.grant_state == RemoteGrantState::Removed {
            return Err(RemoteCapabilityAuthorizationError::DeviceRemoved);
        }
        if self.grant_state != RemoteGrantState::Active {
            return Err(RemoteCapabilityAuthorizationError::GrantNotActive);
        }
        if self.device_trust_state != RemoteDeviceTrustState::Trusted {
            return Err(RemoteCapabilityAuthorizationError::DeviceTrustRequired);
        }
        if self.audit_ref.trim().is_empty() {
            return Err(RemoteCapabilityAuthorizationError::MissingAuditRef);
        }
        if !matches!(
            self.session_state,
            RemoteSessionState::Authorized
                | RemoteSessionState::Paired
                | RemoteSessionState::Connecting
                | RemoteSessionState::Active
                | RemoteSessionState::Degraded
        ) {
            return Err(RemoteCapabilityAuthorizationError::SessionNotLiveViewEligible);
        }
        Ok(())
    }
}
