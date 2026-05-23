use std::sync::{Arc, Mutex};

use ocentra_parent_agent_core::TrustedDeviceRegistry;
use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope, AgentEventName,
    AgentRoute, LanPairingDeviceRef, LanPairingRejectionReason, LanParentIntentEnvelope, LogFields,
    LogLevel,
};

use crate::{
    event_builder::build_event,
    lan_pairing_audit::{
        accepted_control_audit_fields, rejected_control_audit_fields, revoked_route_audit_fields,
        selected_route_audit_fields,
    },
    lan_pairing_payload::{parse_intent, parse_pairing_proof},
    lan_pairing_status::pairing_status_event,
    time::timestamp_now,
};

#[derive(Clone, Debug)]
pub struct LanPairingRuntime {
    pub(crate) registry: Arc<Mutex<TrustedDeviceRegistry>>,
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
        return LanCommandDecision::Respond(submit_pairing_proof(runtime, command).await);
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
    match parse_intent(&command.payload, &command) {
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
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    match parse_pairing_proof(&command.payload) {
        Ok(proof) => {
            let child_device = device_ref(&proof.child_device_id, &command.target.platform);
            let parent_device = device_ref(&proof.parent_device_id, &command.target.platform);
            let trusted_at = timestamp_now();
            runtime
                .registry
                .lock()
                .map(|mut registry| {
                    registry.accept_pairing_proof(&proof, child_device, parent_device, &trusted_at);
                    registry.entries().len()
                })
                .unwrap_or(0);
            pairing_status_event(&runtime, command)
        }
        Err(reason) => rejection_event(command, reason, None, None),
    }
}

fn lan_pairing_route_select(
    runtime: LanPairingRuntime,
    origin: Option<String>,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let observed_origin = origin.as_deref();
    match parse_intent(&command.payload, &command) {
        Ok(intent) => match validate_selection_intent_result(&runtime, observed_origin, &intent) {
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
    match parse_intent(&command.payload, &command) {
        Ok(intent) => match validate_selection_intent_result(&runtime, observed_origin, &intent) {
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
    match parse_intent(&command.payload, &command) {
        Ok(intent) => match validate_selection_intent_result(&runtime, observed_origin, &intent) {
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
    match validate_intent_result(&runtime, origin, &intent) {
        Ok(()) => LanCommandDecision::Continue {
            audit_fields: Some(accepted_control_audit_fields(&command, &intent, origin)),
            command,
        },
        Err(reason) => {
            LanCommandDecision::Respond(rejection_event(command, reason, Some(&intent), origin))
        }
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
        .map(|mut registry| registry.revoke_pairing(&intent.pairing_id, &revoked_at))
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

fn device_ref(device_id: &str, platform: &str) -> LanPairingDeviceRef {
    LanPairingDeviceRef {
        device_id: device_id.to_string(),
        child_profile_id: None,
        label: device_id.to_string(),
        platform: platform.to_string(),
    }
}
