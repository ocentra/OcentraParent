#![forbid(unsafe_code)]

//! Rust-owned pairing and standing-access lifecycle for remote live view.
//!
//! This module owns the deterministic lifecycle boundary only. Persistence,
//! relay transport, portal disclosure rendering, device-trust enrollment, and
//! audit storage remain with their owning crates. A terminal revoke or device
//! removal is intentionally irreversible so reconnect cannot resurrect access.

mod transition;
mod validation;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RemoteAccessGrantCapability {
    LiveView,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RemoteAccessGrantActorRole {
    ParentOwner,
    CoParent,
    SupportAdmin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RemoteAccessGrantDisclosureState {
    Undisclosed,
    Disclosed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RemoteAccessGrantState {
    Requested,
    ParentConfirmed,
    Paired,
    Active,
    Paused,
    Stopped,
    ReconnectPending,
    Revoked,
    Removed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoteAccessGrantTransition {
    ConfirmParent,
    Pair,
    Activate,
    Pause,
    Stop,
    RequestReconnect,
    Reconnect,
    Revoke,
    RemoveDevice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoteAccessGrantError {
    EmptyField,
    WrongHousehold,
    WrongActor,
    WrongDevice,
    ParentAuthorityRequired,
    ChildDisclosureRequired,
    SupportAccessRequiresParentGrant,
    InvalidTransition,
    ReconnectDenied,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoteAccessGrant {
    pub grant_id: String,
    pub household_ref: String,
    pub child_device_ref: String,
    pub parent_actor_ref: String,
    pub capability: RemoteAccessGrantCapability,
    pub actor_role: RemoteAccessGrantActorRole,
    pub state: RemoteAccessGrantState,
    pub disclosure_state: RemoteAccessGrantDisclosureState,
    pub audit_ref: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RemoteAccessGrantContext<'a> {
    pub household_ref: &'a str,
    pub actor_ref: &'a str,
    pub child_device_ref: &'a str,
    pub parent_authorized: bool,
    pub child_disclosed: bool,
}

impl RemoteAccessGrant {
    pub fn request(
        grant_id: impl Into<String>,
        household_ref: impl Into<String>,
        child_device_ref: impl Into<String>,
        parent_actor_ref: impl Into<String>,
        actor_role: RemoteAccessGrantActorRole,
        audit_ref: impl Into<String>,
    ) -> Result<Self, RemoteAccessGrantError> {
        let grant = Self {
            grant_id: grant_id.into(),
            household_ref: household_ref.into(),
            child_device_ref: child_device_ref.into(),
            parent_actor_ref: parent_actor_ref.into(),
            capability: RemoteAccessGrantCapability::LiveView,
            actor_role,
            state: RemoteAccessGrantState::Requested,
            disclosure_state: RemoteAccessGrantDisclosureState::Undisclosed,
            audit_ref: audit_ref.into(),
        };
        validation::fields(&grant)?;
        Ok(grant)
    }

    pub fn transition(
        &mut self,
        transition: RemoteAccessGrantTransition,
        context: RemoteAccessGrantContext<'_>,
    ) -> Result<RemoteAccessGrantState, RemoteAccessGrantError> {
        validation::fields(self)?;
        validation::context(self, context)?;
        transition::apply(self, transition, context)
    }

    pub fn can_reconnect(&self) -> bool {
        matches!(
            self.state,
            RemoteAccessGrantState::Paused
                | RemoteAccessGrantState::Stopped
                | RemoteAccessGrantState::ReconnectPending
        ) && self.disclosure_state == RemoteAccessGrantDisclosureState::Disclosed
    }
}
