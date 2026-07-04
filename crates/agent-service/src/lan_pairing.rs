use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

#[path = "lan_pairing/authority.rs"]
pub(crate) mod authority;
#[path = "lan_pairing/command_routing.rs"]
pub(crate) mod command_routing;
#[path = "lan_pairing/controller_lease.rs"]
pub(crate) mod controller_lease;
#[path = "lan_pairing/controller_lease_flow.rs"]
mod controller_lease_flow;
#[path = "lan_pairing/lan_ai_job.rs"]
pub(crate) mod lan_ai_job;
#[path = "lan_pairing/lan_ai_job_flow.rs"]
mod lan_ai_job_flow;
#[path = "lan_pairing/lan_ai_job_lease_events.rs"]
pub(crate) mod lan_ai_job_lease_events;
#[path = "lan_pairing/lan_ai_route_metadata.rs"]
pub(crate) mod lan_ai_route_metadata;
#[path = "lan_pairing/runtime_commands.rs"]
mod runtime_commands;
#[path = "lan_pairing/runtime_device_ref.rs"]
mod runtime_device_ref;
#[path = "lan_pairing/runtime_rejection.rs"]
mod runtime_rejection;
#[path = "lan_pairing/runtime_validation.rs"]
mod runtime_validation;

use ocentra_lan_core::lan_pairing::LanSignedChildAgentReplayGuard;
use ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoveryListenerState;
use ocentra_parent_agent_core::trusted_device_registry::TrustedDeviceRegistry;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::DeviceRoleRuntimeReadModel;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingOptionalText;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingProof;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;
use ocentra_parent_agent_protocol::lan_pairing::LanParentIntentEnvelope;
use ocentra_parent_agent_protocol::lan_pairing_authority::LanPairingParentAuthority;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use crate::{
    event_builder::build_event,
    lan_pairing_audit::{
        accepted_control_audit_fields, accepted_pairing_audit_fields,
        rejected_control_audit_fields, rejected_pairing_audit_fields, revoked_route_audit_fields,
        selected_route_audit_fields,
    },
    lan_pairing_payload::{
        is_challenge_request, parse_intent, parse_pairing_proof, parse_signed_child_agent_envelope,
    },
    lan_pairing_runtime_state::{
        job_leases::LanAiJobLeaseState, provider_heartbeat::LanAiProviderHeartbeatState,
    },
    lan_pairing_status::{pairing_challenge_status_event, pairing_status_event},
    time::timestamp_now,
};

use self::authority::{
    is_write_intent, validate_registry_selection_intent, validate_write_authority,
};
use self::command_routing::LanCommandOrigin;
use self::controller_lease::LanControllerLeaseState;

#[derive(Clone, Debug)]
pub struct LanPairingRuntime {
    pub(crate) registry: Arc<Mutex<TrustedDeviceRegistry>>,
    pub(crate) challenges: Arc<Mutex<Vec<LanPairingChallengeState>>>,
    pub(crate) controller_lease: Arc<Mutex<Option<LanControllerLeaseState>>>,
    pub(crate) signed_child_agent_replay_guard: Arc<Mutex<LanSignedChildAgentReplayGuard>>,
    pub(crate) passive_discovery_listener_state: Arc<Mutex<LanPassiveDiscoveryListenerState>>,
    pub(crate) lan_ai_provider_heartbeat: Arc<Mutex<Option<LanAiProviderHeartbeatState>>>,
    pub(crate) lan_ai_job_leases: Arc<Mutex<Vec<LanAiJobLeaseState>>>,
    pub(crate) persistence: LanPairingRegistryPersistence,
    pub(crate) local_child_device_id: Option<String>,
    pub(crate) signed_child_agent_parent_device_id: Option<String>,
    pub(crate) signed_child_agent_family_hash: Option<String>,
    pub(crate) signed_child_agent_route_id: String,
    pub(crate) device_roles: DeviceRoleRuntimeReadModel,
    pub(crate) lan_ai_provider_capabilities: Vec<String>,
}

#[derive(Clone, Debug)]
pub(crate) struct LanPairingChallengeState {
    pub(crate) challenge_id: String,
    pub(crate) child_device_id: String,
    pub(crate) parent_device_id: String,
    pub(crate) route_id: String,
    pub(crate) origin: String,
    pub(crate) proof_digest: String,
    pub(crate) issued_at: String,
    pub(crate) expires_at: String,
    pub(crate) accepted: bool,
}

#[derive(Clone, Debug)]
pub(crate) enum LanPairingRegistryPersistence {
    InMemory,
    LocalJsonRegistry(PathBuf),
}

pub enum LanCommandDecision {
    Continue {
        command: AgentCommandEnvelope,
        audit_fields: Option<LogFields>,
    },
    Respond(AgentEventEnvelope),
}

pub(crate) fn extend_log_fields(target: &mut LogFields, fields: LogFields) {
    for (key, value) in fields.into_inner() {
        target.insert(key, value);
    }
}

pub(crate) fn log_fields_contains_key(fields: &LogFields, key: LanPairingText) -> bool {
    fields.get(key.0.as_str()).is_some()
}

pub(crate) fn log_fields_is_empty(fields: &LogFields) -> bool {
    fields.iter().next().is_none()
}

impl Default for LanPairingRuntime {
    fn default() -> Self {
        Self::empty()
    }
}

pub fn build_lan_pairing_status_report(
    runtime: &LanPairingRuntime,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    pairing_status_event(runtime, command)
}

fn lan_pairing_route_select(
    runtime: LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    runtime_commands::lan_pairing_route_select(runtime, origin, command)
}

fn lan_pairing_status_get(
    runtime: LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    runtime_commands::lan_pairing_status_get(runtime, origin, command)
}

fn lan_pairing_route_revoke(
    runtime: LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    runtime_commands::lan_pairing_route_revoke(runtime, origin, command)
}

fn signed_child_agent_observed(
    runtime: &LanPairingRuntime,
    origin: &LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    runtime_commands::signed_child_agent_observed(runtime, origin, command)
}

async fn submit_pairing_proof(
    runtime: LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    runtime_commands::submit_pairing_proof(runtime, origin, command).await
}

fn validate_control_intent(
    runtime: LanPairingRuntime,
    origin: &LanPairingOptionalText,
    command: AgentCommandEnvelope,
    intent: LanParentIntentEnvelope,
) -> LanCommandDecision {
    runtime_commands::validate_control_intent(runtime, origin, command, intent)
}

pub(crate) fn validate_command_target(
    runtime: &LanPairingRuntime,
    command: &AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
) -> Result<(), LanPairingRejectionReason> {
    runtime_validation::validate_command_target(runtime, command, intent)
}

pub(crate) fn validate_local_child_target(
    runtime: &LanPairingRuntime,
    command: &AgentCommandEnvelope,
) -> Result<(), LanPairingRejectionReason> {
    runtime_validation::validate_local_child_target(runtime, command)
}

fn validate_pairing_proof_target(
    runtime: &LanPairingRuntime,
    command: &AgentCommandEnvelope,
    proof: &LanPairingProof,
    origin: &LanPairingOptionalText,
) -> Result<(), LanPairingRejectionReason> {
    runtime_validation::validate_pairing_proof_target(runtime, command, proof, origin)
}

fn validate_intent_result(
    runtime: &LanPairingRuntime,
    origin: &LanPairingOptionalText,
    intent: &LanParentIntentEnvelope,
) -> Result<(), LanPairingRejectionReason> {
    runtime_validation::validate_intent_result(runtime, origin, intent)
}

fn validate_selection_intent_result(
    runtime: &LanPairingRuntime,
    origin: &LanPairingOptionalText,
    intent: &LanParentIntentEnvelope,
) -> Result<(), LanPairingRejectionReason> {
    runtime_validation::validate_selection_intent_result(runtime, origin, intent)
}

fn select_pairing_result(
    runtime: &LanPairingRuntime,
    intent: &LanParentIntentEnvelope,
) -> Result<(), LanPairingRejectionReason> {
    runtime_validation::select_pairing_result(runtime, intent)
}

fn revoke_pairing(runtime: &LanPairingRuntime, intent: &LanParentIntentEnvelope) -> bool {
    runtime_validation::revoke_pairing(runtime, intent)
}

pub(crate) fn rejection_event(
    command: AgentCommandEnvelope,
    reason: &LanPairingRejectionReason,
    intent: Option<&LanParentIntentEnvelope>,
    origin: &LanPairingOptionalText,
) -> AgentEventEnvelope {
    runtime_rejection::rejection_event(command, reason, intent, origin)
}

fn pairing_rejection_event(
    command: AgentCommandEnvelope,
    reason: &LanPairingRejectionReason,
) -> AgentEventEnvelope {
    runtime_rejection::pairing_rejection_event(command, reason)
}

fn device_ref(paired_device_id: LanPairingText, platform: LanPairingText) -> LanPairingDeviceRef {
    runtime_device_ref::device_ref(paired_device_id, platform)
}
