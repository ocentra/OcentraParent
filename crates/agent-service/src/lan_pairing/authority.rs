use ocentra_parent_agent_protocol::{
    LanPairingIntentKind, LanPairingParentAuthority, LanPairingRejectionReason,
    LanParentIntentEnvelope,
};

use crate::{lan_pairing::LanPairingRuntime, time::timestamp_now};

pub(crate) fn validate_registry_selection_intent(
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

pub(crate) fn validate_authorized_lan_ai_job(
    runtime: &LanPairingRuntime,
    origin: Option<&str>,
    intent: &LanParentIntentEnvelope,
) -> Result<(), LanPairingRejectionReason> {
    validate_write_authority(intent)?;
    runtime.validate_controller_lease(intent, timestamp_now().as_str())?;
    validate_registry_control_intent(runtime, origin, intent)
}

pub(crate) fn validate_observer_read_intent(
    runtime: &LanPairingRuntime,
    origin: Option<&str>,
    intent: &LanParentIntentEnvelope,
) -> Result<(), LanPairingRejectionReason> {
    validate_registry_selection_intent(runtime, origin, intent)
}

pub(crate) fn validate_registry_control_intent(
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

pub(crate) fn validate_write_authority(
    intent: &LanParentIntentEnvelope,
) -> Result<(), LanPairingRejectionReason> {
    if intent.parent_authority == LanPairingParentAuthority::Observer {
        Err(LanPairingRejectionReason::ObserverReadOnly)
    } else {
        Ok(())
    }
}

pub(crate) fn is_write_intent(intent: &LanParentIntentEnvelope) -> bool {
    matches!(
        intent.intent_kind,
        LanPairingIntentKind::RuleUpdate
            | LanPairingIntentKind::ApprovalDecision
            | LanPairingIntentKind::ConfigurationUpdate
            | LanPairingIntentKind::ControllerLeaseRenew
            | LanPairingIntentKind::ControllerLeaseRelease
            | LanPairingIntentKind::ControllerLeaseTakeover
            | LanPairingIntentKind::LanAiJobSubmit
    )
}
