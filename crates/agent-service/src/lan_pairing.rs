use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use ocentra_parent_agent_core::TrustedDeviceRegistry;
use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope, AgentEventName,
    AgentRoute, LanPairingDeviceRef, LanPairingProof, LanPairingRejectionReason,
    LanParentIntentEnvelope, LogFields, LogLevel,
};

use crate::{
    event_builder::build_event,
    lan_pairing_audit::{
        accepted_control_audit_fields, accepted_pairing_audit_fields,
        rejected_control_audit_fields, rejected_pairing_audit_fields, revoked_route_audit_fields,
        selected_route_audit_fields,
    },
    lan_pairing_payload::{is_challenge_request, parse_intent, parse_pairing_proof},
    lan_pairing_status::{pairing_challenge_status_event, pairing_status_event},
    time::timestamp_now,
};

#[derive(Clone, Debug)]
pub struct LanPairingRuntime {
    pub(crate) registry: Arc<Mutex<TrustedDeviceRegistry>>,
    pub(crate) challenges: Arc<Mutex<Vec<LanPairingChallengeState>>>,
    pub(crate) persistence: LanPairingRegistryPersistence,
    pub(crate) local_child_device_id: Option<String>,
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

impl Default for LanPairingRuntime {
    fn default() -> Self {
        Self::empty()
    }
}

pub async fn route_lan_command(
    runtime: LanPairingRuntime,
    origin: Option<String>,
    command: AgentCommandEnvelope,
) -> LanCommandDecision {
    if command.target.route != AgentRoute::LocalNetwork {
        return LanCommandDecision::Continue {
            command,
            audit_fields: None,
        };
    }

    if command.command == AgentCommandName::AgentLanPairingProofSubmit {
        return LanCommandDecision::Respond(submit_pairing_proof(runtime, origin, command).await);
    }

    if command.command == AgentCommandName::AgentLanPairingRouteSelect {
        return LanCommandDecision::Respond(lan_pairing_route_select(runtime, origin, command));
    }

    if command.command == AgentCommandName::AgentLanPairingRouteRevoke {
        return LanCommandDecision::Respond(lan_pairing_route_revoke(runtime, origin, command));
    }

    if command.command == AgentCommandName::AgentLanPairingStatusGet {
        return LanCommandDecision::Respond(lan_pairing_status_get(runtime, origin, command));
    }

    let observed_origin = origin.as_deref();
    match parse_intent(&command.payload) {
        Ok(intent) => validate_control_intent(runtime, observed_origin, command, intent),
        Err(reason) => {
            LanCommandDecision::Respond(rejection_event(command, reason, None, observed_origin))
        }
    }
}

pub fn build_lan_pairing_status_report(
    runtime: LanPairingRuntime,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    pairing_status_event(&runtime, command)
}

async fn submit_pairing_proof(
    runtime: LanPairingRuntime,
    origin: Option<String>,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let observed_origin = origin.as_deref();
    match parse_pairing_proof(&command.payload) {
        Ok(proof) => {
            if let Err(reason) =
                validate_pairing_proof_target(&runtime, &command, &proof, observed_origin)
            {
                return pairing_rejection_event(command, reason);
            }
            let child_device = device_ref(&proof.child_device_id, &command.target.platform);
            let parent_device = device_ref(&proof.parent_device_id, &command.target.platform);
            let trusted_at = timestamp_now();
            runtime
                .registry
                .lock()
                .map(|mut registry| {
                    registry.accept_pairing_proof(&proof, child_device, parent_device, &trusted_at);
                    runtime.persist_registry(&registry);
                    registry.entries().len()
                })
                .unwrap_or(0);
            let audit_fields = accepted_pairing_audit_fields(&command, &proof);
            let mut event = pairing_status_event(&runtime, command);
            event.payload.extend(audit_fields);
            event
        }
        Err(reason) => pairing_rejection_event(command, reason),
    }
}

fn lan_pairing_route_select(
    runtime: LanPairingRuntime,
    origin: Option<String>,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let observed_origin = origin.as_deref();
    match parse_intent(&command.payload) {
        Ok(intent) => match validate_command_target(&runtime, &command, &intent)
            .and_then(|()| validate_selection_intent_result(&runtime, observed_origin, &intent))
        {
            Ok(()) => match select_pairing_result(&runtime, &intent) {
                Ok(()) => {
                    let audit_fields =
                        selected_route_audit_fields(&command, &intent, observed_origin);
                    let mut event = pairing_status_event(&runtime, command);
                    event.payload.extend(audit_fields);
                    event
                }
                Err(reason) => rejection_event(command, reason, Some(&intent), observed_origin),
            },
            Err(reason) => rejection_event(command, reason, Some(&intent), observed_origin),
        },
        Err(reason) => rejection_event(command, reason, None, observed_origin),
    }
}

fn lan_pairing_status_get(
    runtime: LanPairingRuntime,
    origin: Option<String>,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let observed_origin = origin.as_deref();
    if is_challenge_request(&command.payload) {
        return pairing_challenge_status_event(&runtime, observed_origin, command);
    }
    match parse_intent(&command.payload) {
        Ok(intent) => match validate_command_target(&runtime, &command, &intent)
            .and_then(|()| validate_selection_intent_result(&runtime, observed_origin, &intent))
        {
            Ok(()) => {
                let audit_fields =
                    accepted_control_audit_fields(&command, &intent, observed_origin);
                let mut event = pairing_status_event(&runtime, command);
                event.payload.extend(audit_fields);
                event
            }
            Err(reason) => rejection_event(command, reason, Some(&intent), observed_origin),
        },
        Err(reason) => rejection_event(command, reason, None, observed_origin),
    }
}

fn lan_pairing_route_revoke(
    runtime: LanPairingRuntime,
    origin: Option<String>,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let observed_origin = origin.as_deref();
    match parse_intent(&command.payload) {
        Ok(intent) => match validate_command_target(&runtime, &command, &intent)
            .and_then(|()| validate_selection_intent_result(&runtime, observed_origin, &intent))
        {
            Ok(()) => {
                revoke_pairing(&runtime, &intent);
                let audit_fields = revoked_route_audit_fields(&command, &intent, observed_origin);
                let mut event = pairing_status_event(&runtime, command);
                event.payload.extend(audit_fields);
                event
            }
            Err(reason) => rejection_event(command, reason, Some(&intent), observed_origin),
        },
        Err(reason) => rejection_event(command, reason, None, observed_origin),
    }
}

fn validate_control_intent(
    runtime: LanPairingRuntime,
    origin: Option<&str>,
    command: AgentCommandEnvelope,
    intent: LanParentIntentEnvelope,
) -> LanCommandDecision {
    match validate_command_target(&runtime, &command, &intent)
        .and_then(|()| validate_intent_result(&runtime, origin, &intent))
    {
        Ok(()) => LanCommandDecision::Continue {
            audit_fields: Some(accepted_control_audit_fields(&command, &intent, origin)),
            command,
        },
        Err(reason) => {
            LanCommandDecision::Respond(rejection_event(command, reason, Some(&intent), origin))
        }
    }
}

fn validate_pairing_proof_target(
    runtime: &LanPairingRuntime,
    command: &AgentCommandEnvelope,
    proof: &LanPairingProof,
    origin: Option<&str>,
) -> Result<(), LanPairingRejectionReason> {
    validate_local_child_target(runtime, command)?;
    if origin != Some(proof.origin.as_str()) {
        return Err(LanPairingRejectionReason::WrongOrigin);
    }
    if command.target.device_id.as_str() == proof.child_device_id.as_str() {
        runtime.validate_challenge_proof(proof, &timestamp_now())
    } else {
        Err(LanPairingRejectionReason::WrongDevice)
    }
}

fn validate_command_target(
    runtime: &LanPairingRuntime,
    command: &AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
) -> Result<(), LanPairingRejectionReason> {
    validate_local_child_target(runtime, command)?;
    if command.target.device_id.as_str() == intent.target_child_device_id.as_str() {
        Ok(())
    } else {
        Err(LanPairingRejectionReason::WrongDevice)
    }
}

pub(crate) fn validate_local_child_target(
    runtime: &LanPairingRuntime,
    command: &AgentCommandEnvelope,
) -> Result<(), LanPairingRejectionReason> {
    match runtime.local_child_device_id.as_deref() {
        Some(local_child_device_id)
            if command.target.device_id.as_str() != local_child_device_id =>
        {
            Err(LanPairingRejectionReason::WrongDevice)
        }
        _ => Ok(()),
    }
}

fn validate_intent_result(
    runtime: &LanPairingRuntime,
    origin: Option<&str>,
    intent: &LanParentIntentEnvelope,
) -> Result<(), LanPairingRejectionReason> {
    let observed_at = timestamp_now();
    runtime
        .registry
        .lock()
        .map(|mut registry| registry.validate_intent(intent, origin, &observed_at))
        .unwrap_or(Err(LanPairingRejectionReason::Malformed))
}

fn validate_selection_intent_result(
    runtime: &LanPairingRuntime,
    origin: Option<&str>,
    intent: &LanParentIntentEnvelope,
) -> Result<(), LanPairingRejectionReason> {
    let observed_at = timestamp_now();
    runtime
        .registry
        .lock()
        .map(|mut registry| registry.validate_selection_intent(intent, origin, &observed_at))
        .unwrap_or(Err(LanPairingRejectionReason::Malformed))
}

fn select_pairing_result(
    runtime: &LanPairingRuntime,
    intent: &LanParentIntentEnvelope,
) -> Result<(), LanPairingRejectionReason> {
    runtime
        .registry
        .lock()
        .map(|mut registry| {
            registry.select_pairing(
                &intent.pairing_id,
                &intent.target_child_device_id,
                &intent.route_id,
                &intent.expires_at,
            )
        })
        .unwrap_or(Err(LanPairingRejectionReason::Malformed))
        .map(|_| ())
}

fn revoke_pairing(runtime: &LanPairingRuntime, intent: &LanParentIntentEnvelope) -> bool {
    let revoked_at = timestamp_now();
    runtime
        .registry
        .lock()
        .map(|mut registry| {
            let revoked = registry.revoke_pairing(&intent.pairing_id, &revoked_at);
            if revoked {
                runtime.persist_registry(&registry);
            }
            revoked
        })
        .unwrap_or(false)
}

fn rejection_event(
    command: AgentCommandEnvelope,
    reason: LanPairingRejectionReason,
    intent: Option<&LanParentIntentEnvelope>,
    origin: Option<&str>,
) -> AgentEventEnvelope {
    let payload = rejected_control_audit_fields(&command, &reason, intent, origin);
    build_event(
        constants::event_id::COMMAND_REJECTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentCommandRejected,
        LogLevel::Warn,
        payload,
        None,
    )
}

fn pairing_rejection_event(
    command: AgentCommandEnvelope,
    reason: LanPairingRejectionReason,
) -> AgentEventEnvelope {
    let payload = rejected_pairing_audit_fields(&command, &reason);
    build_event(
        constants::event_id::COMMAND_REJECTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentCommandRejected,
        LogLevel::Warn,
        payload,
        None,
    )
}

fn device_ref(device_id: &str, platform: &str) -> LanPairingDeviceRef {
    LanPairingDeviceRef {
        device_id: device_id.to_string(),
        child_profile_id: None,
        label: device_id.to_string(),
        platform: platform.to_string(),
    }
}
