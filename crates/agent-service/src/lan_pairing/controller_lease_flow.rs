use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingAuditEventType;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingOptionalText;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason;
use ocentra_parent_agent_protocol::lan_pairing::LanParentIntentEnvelope;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use crate::event_builder::build_event;
use crate::lan_pairing::authority::{validate_registry_selection_intent, validate_write_authority};
use crate::lan_pairing::{
    extend_log_fields, runtime_rejection::rejection_event,
    runtime_validation::validate_command_target, LanPairingRuntime,
};
use crate::lan_pairing_audit::controller_lease_audit_fields;
use crate::lan_pairing_payload::parse_intent;
use crate::lan_pairing_status::pairing_status_event;
use crate::time::timestamp_now;

pub(crate) fn controller_lease_lifecycle_command(
    runtime: LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
    audit_event_type: LanPairingAuditEventType,
    apply: fn(
        &LanPairingRuntime,
        &LanParentIntentEnvelope,
        ocentra_parent_agent_protocol::lan_pairing::LanPairingText,
    ) -> Result<(), LanPairingRejectionReason>,
) -> AgentEventEnvelope {
    let origin = LanPairingOptionalText(origin.0);
    let origin_text = origin.0.as_deref();
    let event = match parse_intent(&command.payload) {
        Ok(intent) => match validate_command_target(&runtime, &command, &intent)
            .and_then(|()| validate_write_authority(&intent))
            .and_then(|()| validate_registry_selection_intent(&runtime, origin_text, &intent))
            .and_then(|()| apply(&runtime, &intent, timestamp_now::<String>().into()))
        {
            Ok(()) => controller_lease_success_event(
                &runtime,
                command,
                &intent,
                &origin,
                audit_event_type,
            ),
            Err(reason) => rejection_event(command, &reason, Some(&intent), &origin),
        },
        Err(reason) => rejection_event(command, &reason, None, &origin),
    };
    drop(runtime);
    event
}

pub(crate) fn controller_lease_renew(
    runtime: &LanPairingRuntime,
    intent: &LanParentIntentEnvelope,
    observed_at: ocentra_parent_agent_protocol::lan_pairing::LanPairingText,
) -> Result<(), LanPairingRejectionReason> {
    runtime.renew_controller_lease(intent, observed_at)
}

pub(crate) fn controller_lease_release(
    runtime: &LanPairingRuntime,
    intent: &LanParentIntentEnvelope,
    observed_at: ocentra_parent_agent_protocol::lan_pairing::LanPairingText,
) -> Result<(), LanPairingRejectionReason> {
    runtime.release_controller_lease(intent, observed_at)
}

pub(crate) fn controller_lease_takeover(
    runtime: LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let origin = LanPairingOptionalText(origin.0);
    let origin_text = origin.0.as_deref();
    let event = match parse_intent(&command.payload) {
        Ok(intent) => match validate_command_target(&runtime, &command, &intent)
            .and_then(|()| validate_write_authority(&intent))
            .and_then(|()| validate_registry_selection_intent(&runtime, origin_text, &intent))
            .and_then(|()| {
                runtime.takeover_controller_lease(&intent, timestamp_now::<String>().as_str())
            }) {
            Ok(()) => controller_lease_success_event(
                &runtime,
                command,
                &intent,
                &origin,
                LanPairingAuditEventType::ControllerLeaseTakeoverAccepted,
            ),
            Err(reason) => controller_lease_rejection_event(command, &intent, origin, &reason),
        },
        Err(reason) => rejection_event(command, &reason, None, &origin),
    };
    drop(runtime);
    event
}

fn controller_lease_success_event(
    runtime: &LanPairingRuntime,
    command: AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
    origin: &LanPairingOptionalText,
    audit_event_type: LanPairingAuditEventType,
) -> AgentEventEnvelope {
    let audit_fields =
        controller_lease_audit_fields(&command, intent, origin, audit_event_type, None);
    let mut event = pairing_status_event(runtime, command);
    extend_log_fields(&mut event.payload, audit_fields);
    event
}

fn controller_lease_rejection_event(
    command: AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
    origin: LanPairingOptionalText,
    reason: &LanPairingRejectionReason,
) -> AgentEventEnvelope {
    let origin = LanPairingOptionalText(origin.0);
    let payload = controller_lease_audit_fields(
        &command,
        intent,
        &origin,
        if *reason == LanPairingRejectionReason::TakeoverDenied {
            LanPairingAuditEventType::ControllerLeaseTakeoverRejected
        } else {
            LanPairingAuditEventType::ControlRejected
        },
        Some(reason),
    );
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
