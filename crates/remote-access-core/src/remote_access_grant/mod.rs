#![forbid(unsafe_code)]

//! Rust-owned pairing and standing-access lifecycle for remote live view.
//!
//! This module owns the deterministic lifecycle boundary only. Persistence,
//! relay transport, portal disclosure rendering, device-trust enrollment, and
//! audit storage remain with their owning crates. A terminal revoke or device
//! removal is intentionally irreversible so reconnect cannot resurrect access.

mod audit;
mod errors;
mod serialization;
mod transition;
mod validation;

use ocentra_schema::remote_capability_fabric::RemoteActorRole;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RemoteAccessGrantCapability {
    LiveView,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RemoteAccessGrantDisclosureState {
    Undisclosed,
    Disclosed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RemoteAccessGrantParentGrant {
    NotGranted,
    Granted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[repr(u8)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[repr(u8)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum RemoteAccessGrantError {
    EmptyField,
    WrongHousehold,
    WrongActor,
    WrongDevice,
    ParentAuthorityRequired,
    ChildDisclosureRequired,
    SupportAccessRequiresParentGrant,
    InvalidTransition,
    InvalidSerializedState,
    ReconnectDenied,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RemoteAccessGrant {
    grant_id: String,
    household_ref: String,
    child_device_ref: String,
    parent_actor_ref: String,
    capability: RemoteAccessGrantCapability,
    actor_role: RemoteActorRole,
    state: RemoteAccessGrantState,
    disclosure_state: RemoteAccessGrantDisclosureState,
    parent_grant: RemoteAccessGrantParentGrant,
    audit_ref: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RemoteAccessGrantContext<'a> {
    pub household_ref: &'a str,
    pub actor_ref: &'a str,
    pub child_device_ref: &'a str,
    pub parent_authorized: bool,
    pub child_disclosed: bool,
    pub parent_grant_approved: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[repr(u8)]
pub enum RemoteAccessGrantAuditOutcome {
    Accepted,
    Denied,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteAccessGrantAuditMilestone {
    pub grant_id: String,
    pub household_ref: String,
    pub actor_ref: String,
    pub transition: RemoteAccessGrantTransition,
    pub outcome: RemoteAccessGrantAuditOutcome,
    pub resulting_state: RemoteAccessGrantState,
    pub error: Option<RemoteAccessGrantError>,
    pub audit_ref: String,
}

pub const REMOTE_ACCESS_GRANT_AUDIT_EVENT_TYPE: &str = "remote-access.grant.transition";
pub const REMOTE_ACCESS_GRANT_AUDIT_SCHEMA_VERSION: u16 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteAccessGrantTransitionReport {
    pub result: Result<RemoteAccessGrantState, RemoteAccessGrantError>,
    pub audit: RemoteAccessGrantAuditMilestone,
}

impl RemoteAccessGrant {
    pub fn request(
        grant_id: impl Into<String>,
        household_ref: impl Into<String>,
        child_device_ref: impl Into<String>,
        parent_actor_ref: impl Into<String>,
        actor_role: RemoteActorRole,
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
            parent_grant: RemoteAccessGrantParentGrant::NotGranted,
            audit_ref: audit_ref.into(),
        };
        validation::fields(&grant)?;
        validation::actor_role(&grant.actor_role)?;
        Ok(grant)
    }

    pub fn transition(
        &mut self,
        transition: RemoteAccessGrantTransition,
        context: RemoteAccessGrantContext<'_>,
    ) -> Result<RemoteAccessGrantState, RemoteAccessGrantError> {
        self.transition_with_audit(transition, context).result
    }

    pub fn transition_with_audit(
        &mut self,
        transition: RemoteAccessGrantTransition,
        context: RemoteAccessGrantContext<'_>,
    ) -> RemoteAccessGrantTransitionReport {
        let result = self.apply_transition(transition, context);
        let (outcome, error, resulting_state) = match result {
            Ok(state) => (RemoteAccessGrantAuditOutcome::Accepted, None, state),
            Err(error) => (
                RemoteAccessGrantAuditOutcome::Denied,
                Some(error),
                self.state,
            ),
        };
        RemoteAccessGrantTransitionReport {
            result,
            audit: RemoteAccessGrantAuditMilestone {
                grant_id: self.grant_id.clone(),
                household_ref: self.household_ref.clone(),
                actor_ref: context.actor_ref.to_owned(),
                transition,
                outcome,
                resulting_state,
                error,
                audit_ref: self.audit_ref.clone(),
            },
        }
    }

    fn apply_transition(
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

    pub fn grant_id(&self) -> &str {
        &self.grant_id
    }

    pub fn household_ref(&self) -> &str {
        &self.household_ref
    }

    pub fn child_device_ref(&self) -> &str {
        &self.child_device_ref
    }

    pub fn parent_actor_ref(&self) -> &str {
        &self.parent_actor_ref
    }

    pub fn capability(&self) -> RemoteAccessGrantCapability {
        self.capability
    }

    pub fn actor_role(&self) -> RemoteActorRole {
        self.actor_role.clone()
    }

    pub fn state(&self) -> RemoteAccessGrantState {
        self.state
    }

    pub fn disclosure_state(&self) -> RemoteAccessGrantDisclosureState {
        self.disclosure_state
    }

    pub fn parent_grant(&self) -> RemoteAccessGrantParentGrant {
        self.parent_grant
    }

    pub fn audit_ref(&self) -> &str {
        &self.audit_ref
    }
}
