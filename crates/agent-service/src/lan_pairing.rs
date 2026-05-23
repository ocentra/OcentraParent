use std::sync::{Arc, Mutex};

use ocentra_parent_agent_core::TrustedDeviceRegistry;
use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope, AgentEventName,
    AgentRoute, LanPairingDeviceRef, LanPairingIntentKind, LanPairingProof,
    LanPairingRejectionReason, LanParentIntentEnvelope, LogFieldValue, LogFields, LogLevel,
};

use crate::{
    event_builder::build_event,
    fields::fields_from_pairs,
    lan_pairing_audit::{accepted_control_audit_fields, rejected_control_audit_fields},
    time::timestamp_now,
};

#[derive(Clone, Debug)]
pub struct LanPairingRuntime {
    registry: Arc<Mutex<TrustedDeviceRegistry>>,
}

pub enum LanCommandDecision {
    Continue {
        command: AgentCommandEnvelope,
        audit_fields: Option<LogFields>,
    },
    Respond(AgentEventEnvelope),
}

impl LanPairingRuntime {
    pub fn empty() -> Self {
        Self {
            registry: Arc::new(Mutex::new(TrustedDeviceRegistry::empty())),
        }
    }

    pub fn trusted_device_count(&self) -> usize {
        self.registry
            .lock()
            .map(|registry| registry.entries().len())
            .unwrap_or(0)
    }

    #[cfg(test)]
    pub fn revoke_pairing_for_test(&self, pairing_id: &str, revoked_at: &str) -> bool {
        self.registry
            .lock()
            .map(|mut registry| registry.revoke_pairing(pairing_id, revoked_at))
            .unwrap_or(false)
    }
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
    let (state, count) = pairing_state(&runtime);
    pairing_status_event(command, state, count)
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
            let count = runtime
                .registry
                .lock()
                .map(|mut registry| {
                    registry.accept_pairing_proof(&proof, child_device, parent_device, &trusted_at);
                    registry.entries().len()
                })
                .unwrap_or(0);
            pairing_status_event(command, constants::value::LAN_PAIRING_PAIRED, count)
        }
        Err(reason) => rejection_event(command, reason, None, None),
    }
}

fn lan_pairing_status_get(
    runtime: LanPairingRuntime,
    origin: Option<String>,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let observed_origin = origin.as_deref();
    match parse_intent(&command.payload, &command) {
        Ok(intent) => match validate_intent_result(&runtime, observed_origin, &intent) {
            Ok(()) => {
                let audit_fields =
                    accepted_control_audit_fields(&command, &intent, observed_origin);
                let (state, count) = pairing_state(&runtime);
                let mut event = pairing_status_event(command, state, count);
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

fn parse_pairing_proof(fields: &LogFields) -> Result<LanPairingProof, LanPairingRejectionReason> {
    Ok(LanPairingProof {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        pairing_id: required_string(fields, constants::field::LAN_PAIRING_ID)?,
        challenge_id: required_string(fields, constants::field::LAN_CHALLENGE_ID)?,
        child_device_id: required_string(fields, constants::field::LAN_CHILD_DEVICE_ID)?,
        parent_device_id: required_string(fields, constants::field::LAN_PARENT_DEVICE_ID)?,
        route_id: required_string(fields, constants::field::LAN_ROUTE_ID)?,
        origin: required_string(fields, constants::field::ORIGIN)?,
        proof_digest: required_string(fields, constants::field::LAN_PROOF_DIGEST)?,
        issued_at: required_string(fields, constants::field::STARTED_AT)?,
        expires_at: required_string(fields, constants::field::STALE_AT)?,
    })
}

fn parse_intent(
    fields: &LogFields,
    command: &AgentCommandEnvelope,
) -> Result<LanParentIntentEnvelope, LanPairingRejectionReason> {
    let pairing_id = required_anonymous_string(fields, constants::field::LAN_PAIRING_ID)?;
    let proof_digest = required_anonymous_string(fields, constants::field::LAN_PROOF_DIGEST)?;
    Ok(LanParentIntentEnvelope {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        intent_id: required_string(fields, constants::field::LAN_INTENT_ID)?,
        intent_kind: LanPairingIntentKind::HealthQuery,
        target_child_device_id: command.target.device_id.clone(),
        route_id: required_string(fields, constants::field::LAN_ROUTE_ID)?,
        pairing_id,
        proof_digest,
        origin: required_string(fields, constants::field::ORIGIN)?,
        issued_at: required_string(fields, constants::field::STARTED_AT)?,
        expires_at: required_string(fields, constants::field::STALE_AT)?,
    })
}

fn required_anonymous_string(
    fields: &LogFields,
    key: &str,
) -> Result<String, LanPairingRejectionReason> {
    required_string(fields, key).map_err(|_| LanPairingRejectionReason::Anonymous)
}

fn required_string(fields: &LogFields, key: &str) -> Result<String, LanPairingRejectionReason> {
    match fields.get(key) {
        Some(LogFieldValue::String(value)) if !value.is_empty() => Ok(value.clone()),
        _ => Err(LanPairingRejectionReason::Malformed),
    }
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

fn pairing_status_event(
    command: AgentCommandEnvelope,
    state: &str,
    count: usize,
) -> AgentEventEnvelope {
    build_event(
        constants::lan_pairing::EVENT_STATUS_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentLanPairingStatusReported,
        LogLevel::Info,
        fields_from_pairs(vec![
            (
                constants::field::TRANSPORT,
                LogFieldValue::String(constants::value::TRANSPORT_WEBSOCKET.to_string()),
            ),
            (
                constants::field::LAN_SUPPORTED_WEBSOCKET_COMMANDS,
                LogFieldValue::String(
                    constants::lan_pairing::SUPPORTED_WEBSOCKET_COMMANDS
                        .join(&constants::delimiter::LIST.to_string()),
                ),
            ),
            (
                constants::field::LAN_UNSUPPORTED_HTTP_ENDPOINTS,
                LogFieldValue::String(
                    constants::lan_pairing::PLANNED_HTTP_ENDPOINT_PATHS
                        .join(&constants::delimiter::LIST.to_string()),
                ),
            ),
            (
                constants::field::LAN_PERSISTENCE_MODE,
                LogFieldValue::String(
                    constants::value::LAN_PERSISTENCE_IN_MEMORY_FAIL_CLOSED.to_string(),
                ),
            ),
            (
                constants::field::LAN_PROOF_MODE,
                LogFieldValue::String(constants::value::LAN_PROOF_DIRECT_PROOF_SUBMIT.to_string()),
            ),
            (
                constants::field::LAN_ROUTE_REQUIREMENTS,
                LogFieldValue::String(
                    constants::lan_pairing::ROUTE_REQUIREMENTS
                        .join(&constants::delimiter::LIST.to_string()),
                ),
            ),
            (
                constants::field::LAN_MANUAL_PROOF_GAPS,
                LogFieldValue::String(
                    constants::lan_pairing::MANUAL_PROOF_GAPS
                        .join(&constants::delimiter::LIST.to_string()),
                ),
            ),
            (
                constants::field::LAN_PAIRING_STATE,
                LogFieldValue::String(state.to_string()),
            ),
            (
                constants::field::LAN_TRUSTED_DEVICE_COUNT,
                LogFieldValue::Number(count as f64),
            ),
        ]),
        None,
    )
}

fn pairing_state(runtime: &LanPairingRuntime) -> (&'static str, usize) {
    let count = runtime.trusted_device_count();
    let state = if count > 0 {
        constants::value::LAN_PAIRING_PAIRED
    } else {
        constants::value::LAN_PAIRING_UNPAIRED
    };
    (state, count)
}

fn device_ref(device_id: &str, platform: &str) -> LanPairingDeviceRef {
    LanPairingDeviceRef {
        device_id: device_id.to_string(),
        child_profile_id: None,
        label: device_id.to_string(),
        platform: platform.to_string(),
    }
}
