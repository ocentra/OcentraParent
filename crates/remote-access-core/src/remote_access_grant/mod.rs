#![forbid(unsafe_code)]

//! Rust-owned pairing and standing-access lifecycle for remote live view.
//!
//! This module owns the deterministic lifecycle boundary only. Persistence,
//! relay transport, portal disclosure rendering, device-trust enrollment, and
//! audit storage remain with their owning crates. A terminal revoke or device
//! removal is intentionally irreversible so reconnect cannot resurrect access.

mod audit;
mod errors;
mod replay;
mod replay_capacity;
mod replay_identity;
mod serialization;
mod transition;
mod validation;
mod validation_context;
mod validation_history;

/// Number of transition attempts retained for idempotent replay after a grant
/// is persisted. Once the bounded window is full, a new attempt fails closed;
/// retaining every recorded identity prevents an old accepted transition from
/// being applied again after eviction.
pub(super) const MAX_REPLAY_ATTEMPTS: usize = 64;

use ocentra_schema::remote_capability_fabric::{
    RemoteActorRole, RemoteDeviceTrustState, RemoteRoute,
};
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
    Denied,
    Failed,
    Superseded,
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
    Deny,
    Fail,
    Supersede,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RemoteAccessGrantStopRecoveryState {
    #[default]
    NotRequired,
    Pending,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum RemoteAccessGrantError {
    EmptyField,
    WrongHousehold,
    WrongActor,
    WrongDevice,
    WrongRoute,
    DeviceTrustRequired,
    ParentAuthorityRequired,
    ChildDisclosureRequired,
    SupportAccessRequiresParentGrant,
    InvalidTransition,
    InvalidSerializedState,
    ReconnectDenied,
    SupersedingGrantRequired,
    SupersedingGrantMismatch,
    ReplayWindowExhausted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RemoteAccessGrant {
    grant_id: String,
    household_ref: String,
    child_device_ref: String,
    route: RemoteRoute,
    parent_actor_ref: String,
    capability: RemoteAccessGrantCapability,
    actor_role: RemoteActorRole,
    state: RemoteAccessGrantState,
    disclosure_state: RemoteAccessGrantDisclosureState,
    parent_grant: RemoteAccessGrantParentGrant,
    audit_ref: String,
    attempts: Vec<RemoteAccessGrantAuditMilestone>,
    superseded_by: Option<String>,
    stop_recovery: RemoteAccessGrantStopRecoveryState,
    #[serde(skip)]
    pending_supersession: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RemoteAccessGrantContext<'a> {
    pub household_ref: &'a str,
    pub actor_ref: &'a str,
    pub child_device_ref: &'a str,
    pub route: RemoteRoute,
    pub attempt_ref: &'a str,
    pub transition_authority: RemoteAccessGrantTransitionAuthority,
    pub device_trust_state: RemoteDeviceTrustState,
    pub parent_authorized: bool,
    pub child_disclosed: bool,
    pub parent_grant_approved: bool,
    pub recovery_proof: RemoteAccessGrantRecoveryProof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoteAccessGrantTransitionAuthority {
    Parent,
    SystemFailure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoteAccessGrantRecoveryProof {
    NotRequired,
    SystemConditionCleared,
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
    #[serde(default)]
    pub child_device_ref: String,
    pub route: RemoteRoute,
    pub attempt_ref: String,
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
        route: RemoteRoute,
        parent_actor_ref: impl Into<String>,
        actor_role: RemoteActorRole,
        audit_ref: impl Into<String>,
    ) -> Result<Self, RemoteAccessGrantError> {
        let grant = Self {
            grant_id: grant_id.into(),
            household_ref: household_ref.into(),
            child_device_ref: child_device_ref.into(),
            route,
            parent_actor_ref: parent_actor_ref.into(),
            capability: RemoteAccessGrantCapability::LiveView,
            actor_role,
            state: RemoteAccessGrantState::Requested,
            disclosure_state: RemoteAccessGrantDisclosureState::Undisclosed,
            parent_grant: RemoteAccessGrantParentGrant::NotGranted,
            audit_ref: audit_ref.into(),
            attempts: Vec::new(),
            superseded_by: None,
            stop_recovery: RemoteAccessGrantStopRecoveryState::NotRequired,
            pending_supersession: None,
        };
        validation::fields(&grant)?;
        validation::actor_role(&grant.actor_role)?;
        Ok(grant)
    }

    pub fn transition(
        &mut self,
        transition: RemoteAccessGrantTransition,
        context: RemoteAccessGrantContext<'_>,
    ) -> RemoteAccessGrantTransitionReport {
        self.transition_with_audit(transition, context)
    }

    pub fn transition_with_audit(
        &mut self,
        transition: RemoteAccessGrantTransition,
        context: RemoteAccessGrantContext<'_>,
    ) -> RemoteAccessGrantTransitionReport {
        let actor_ref = context.actor_ref.to_owned();
        let route = replay_identity::audit_route(self, transition, &context);
        let attempt_ref = context.attempt_ref.to_owned();
        if let Some(previous) = self
            .attempts
            .iter()
            .find(|attempt| attempt.attempt_ref == attempt_ref)
        {
            let child_device_retry = previous.outcome == RemoteAccessGrantAuditOutcome::Denied
                && previous.error == Some(RemoteAccessGrantError::WrongDevice)
                && previous.child_device_ref != context.child_device_ref;
            if !child_device_retry {
                return replay::existing_report(self, previous.clone(), transition, context);
            }
        }
        if !replay_capacity::prepare(self, transition) {
            self.pending_supersession = None;
            return replay::denied_report(
                self,
                transition,
                context,
                RemoteAccessGrantError::ReplayWindowExhausted,
            );
        }
        let result = self.apply_transition(transition, &context);
        self.pending_supersession = None;
        let (outcome, error, resulting_state) = match result {
            Ok(state) => (RemoteAccessGrantAuditOutcome::Accepted, None, state),
            Err(error) => (
                RemoteAccessGrantAuditOutcome::Denied,
                Some(error),
                self.state,
            ),
        };
        let report = RemoteAccessGrantTransitionReport {
            result,
            audit: RemoteAccessGrantAuditMilestone {
                grant_id: self.grant_id.clone(),
                household_ref: self.household_ref.clone(),
                actor_ref,
                child_device_ref: context.child_device_ref.to_owned(),
                route,
                attempt_ref,
                transition,
                outcome,
                resulting_state,
                error,
                audit_ref: self.audit_ref.clone(),
            },
        };
        self.attempts.push(report.audit.clone());
        report
    }

    fn apply_transition(
        &mut self,
        transition: RemoteAccessGrantTransition,
        context: &RemoteAccessGrantContext<'_>,
    ) -> Result<RemoteAccessGrantState, RemoteAccessGrantError> {
        validation::fields(self)?;
        validation_context::context(self, transition, context)?;
        transition::apply(self, transition, context)
    }

    /// Atomically marks this grant as superseded by a newer grant with the
    /// same household, device, route, and capability scope. The owning grant
    /// store/service is responsible for creating the replacement; this
    /// boundary makes the old grant unusable before the replacement is used.
    pub fn supersede_with(
        &mut self,
        replacement: &Self,
        context: RemoteAccessGrantContext<'_>,
    ) -> RemoteAccessGrantTransitionReport {
        if self.grant_id == replacement.grant_id
            || self.household_ref != replacement.household_ref
            || self.child_device_ref != replacement.child_device_ref
            || self.route != replacement.route
            || self.capability != replacement.capability
        {
            return replay::denied_report(
                self,
                RemoteAccessGrantTransition::Supersede,
                context,
                RemoteAccessGrantError::SupersedingGrantMismatch,
            );
        }

        self.pending_supersession = Some(replacement.grant_id.clone());
        self.transition_with_audit(RemoteAccessGrantTransition::Supersede, context)
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

    pub fn route(&self) -> RemoteRoute {
        self.route
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

    pub fn superseded_by(&self) -> Option<&str> {
        self.superseded_by.as_deref()
    }

    pub fn audit_ref(&self) -> &str {
        &self.audit_ref
    }
}
