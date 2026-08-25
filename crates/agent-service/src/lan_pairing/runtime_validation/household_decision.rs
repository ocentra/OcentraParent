use chrono::{DateTime, Utc};
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingOptionalText, LanPairingRejectionReason, LanPairingText, LanParentIntentEnvelope,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanHouseholdDeviceActionKind, LanHouseholdDeviceDecision,
};
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::LanPairingParentAuthority;

use crate::lan_pairing::authority::{
    is_household_device_decision_intent, validate_write_authority,
};
use crate::lan_pairing::LanPairingRuntime;

pub(crate) fn validate_household_device_decision(
    runtime: &LanPairingRuntime,
    command: &AgentCommandEnvelope,
    origin: &LanPairingOptionalText,
    intent: &LanParentIntentEnvelope,
    decision: &LanHouseholdDeviceDecision,
) -> Result<(), LanPairingRejectionReason> {
    validate_shape(intent, decision)?;
    if origin.0.as_deref() != Some(intent.origin.as_str()) {
        return Err(LanPairingRejectionReason::WrongOrigin);
    }
    validate_timestamps(intent, decision)?;
    super::validate_command_target(runtime, command, intent)?;
    validate_write_authority(intent)?;
    Ok(())
}

fn validate_shape(
    intent: &LanParentIntentEnvelope,
    decision: &LanHouseholdDeviceDecision,
) -> Result<(), LanPairingRejectionReason> {
    if decision.action_id.is_empty()
        || decision.canonical_device_id.is_empty()
        || decision.parent_actor_id.is_empty()
        || decision.decided_at.is_empty()
    {
        return Err(LanPairingRejectionReason::Malformed);
    }
    if !is_household_device_decision_intent(intent) {
        return Err(LanPairingRejectionReason::Malformed);
    }
    if intent.parent_authority != LanPairingParentAuthority::ActiveController {
        return Err(LanPairingRejectionReason::ObserverReadOnly);
    }
    if decision.action_id != intent.intent_id || decision.parent_actor_id != intent.parent_actor_id
    {
        return Err(LanPairingRejectionReason::Malformed);
    }
    Ok(())
}

fn validate_timestamps(
    intent: &LanParentIntentEnvelope,
    decision: &LanHouseholdDeviceDecision,
) -> Result<(), LanPairingRejectionReason> {
    let decided_at = strict_timestamp(&LanPairingText(decision.decided_at.clone()))
        .ok_or(LanPairingRejectionReason::Malformed)?;
    let issued_at = strict_timestamp(&LanPairingText(intent.issued_at.clone()))
        .ok_or(LanPairingRejectionReason::Malformed)?;
    let expires_at = strict_timestamp(&LanPairingText(intent.expires_at.clone()))
        .ok_or(LanPairingRejectionReason::Malformed)?;
    let now = Utc::now();
    if issued_at > now
        || expires_at <= now
        || issued_at > decided_at
        || decided_at >= expires_at
        || decided_at > now
    {
        return Err(LanPairingRejectionReason::Stale);
    }
    match &decision.action_kind {
        LanHouseholdDeviceActionKind::Revoke => {
            let revoked_at = decision
                .revoked_at
                .as_deref()
                .map(|value| strict_timestamp(&LanPairingText(value.to_owned())))
                .flatten()
                .ok_or(LanPairingRejectionReason::Malformed)?;
            if revoked_at < decided_at || revoked_at >= expires_at || revoked_at > now {
                return Err(LanPairingRejectionReason::Stale);
            }
        }
        LanHouseholdDeviceActionKind::Assign
        | LanHouseholdDeviceActionKind::Rename
        | LanHouseholdDeviceActionKind::Ignore
        | LanHouseholdDeviceActionKind::Restore
        | LanHouseholdDeviceActionKind::Trust
            if decision.revoked_at.is_some() =>
        {
            return Err(LanPairingRejectionReason::Malformed);
        }
        _ => {}
    }
    Ok(())
}

fn strict_timestamp(value: &LanPairingText) -> Option<DateTime<Utc>> {
    let parsed = DateTime::parse_from_rfc3339(value.0.as_str())
        .ok()?
        .with_timezone(&Utc);
    (parsed.to_rfc3339_opts(chrono::SecondsFormat::Millis, true) == value.0).then_some(parsed)
}
