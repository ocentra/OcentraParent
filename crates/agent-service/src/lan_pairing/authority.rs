use ocentra_parent_agent_core::trusted_device_registry::controller_lease::LanControllerLeaseMutation;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingIntentKind;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingOptionalText;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;
use ocentra_parent_agent_protocol::lan_pairing::LanParentIntentEnvelope;
use ocentra_parent_agent_protocol::lan_pairing_authority::LanPairingParentAuthority;

use crate::{
    lan_pairing::{LanPairingRegistryPersistence, LanPairingRuntime},
    time::timestamp_now,
};

pub(crate) fn validate_registry_selection_intent(
    runtime: &LanPairingRuntime,
    origin: Option<impl Into<LanPairingText>>,
    intent: &LanParentIntentEnvelope,
) -> Result<(), LanPairingRejectionReason> {
    let origin = optional_text(origin);
    let observed_at = LanPairingText(timestamp_now());
    validate_registry_intent_at(runtime, &origin, intent, &observed_at, false)
}

pub(crate) fn validate_authorized_lan_ai_job(
    runtime: &LanPairingRuntime,
    origin: Option<impl Into<LanPairingText>>,
    intent: &LanParentIntentEnvelope,
) -> Result<(), LanPairingRejectionReason> {
    validate_write_authority(intent)?;
    let origin = optional_text(origin);
    let observed_at = LanPairingText(timestamp_now());
    validate_registry_intent_without_acceptance_at(runtime, &origin, intent, &observed_at, true)?;
    // Job execution is owned by a separate runtime store. Until that owner
    // exposes an atomic pending/applied handoff with the registry replay
    // record, accepting here could suppress a job that never ran after a
    // crash. Keep the production route fail-closed instead.
    Err(LanPairingRejectionReason::SignedChildAgentContextUnavailable)
}

pub(crate) fn validate_observer_read_intent(
    runtime: &LanPairingRuntime,
    origin: Option<impl Into<LanPairingText>>,
    intent: &LanParentIntentEnvelope,
) -> Result<(), LanPairingRejectionReason> {
    let origin = optional_text(origin);
    let observed_at = LanPairingText(timestamp_now());
    validate_registry_intent_at(runtime, &origin, intent, &observed_at, false)
}

pub(crate) fn validate_registry_control_intent(
    runtime: &LanPairingRuntime,
    origin: Option<impl Into<LanPairingText>>,
    intent: &LanParentIntentEnvelope,
) -> Result<(), LanPairingRejectionReason> {
    let origin = optional_text(origin);
    let observed_at = LanPairingText(timestamp_now());
    validate_registry_intent_without_acceptance_at(runtime, &origin, intent, &observed_at, true)?;
    // Generic LAN commands execute in dependency-owned stores after this
    // router returns. Without a durable pending/applied handoff, terminally
    // consuming the intent here creates a crash window. Exact registry-owned
    // route, lease, decision, and status operations use the atomic paths
    // above; all other control remains contract-only and fail-closed.
    Err(LanPairingRejectionReason::SignedChildAgentContextUnavailable)
}

fn validate_registry_intent_without_acceptance_at(
    runtime: &LanPairingRuntime,
    origin: &LanPairingOptionalText,
    intent: &LanParentIntentEnvelope,
    observed_at: &LanPairingText,
    require_selected_pairing: bool,
) -> Result<(), LanPairingRejectionReason> {
    runtime
        .registry
        .lock()
        .map(|registry| {
            registry.validate_intent_without_acceptance(
                intent,
                origin.0.as_deref(),
                observed_at.0.as_str(),
                require_selected_pairing,
            )
        })
        .unwrap_or(Err(LanPairingRejectionReason::Malformed))
}

fn validate_registry_intent_at(
    runtime: &LanPairingRuntime,
    origin: &LanPairingOptionalText,
    intent: &LanParentIntentEnvelope,
    observed_at: &LanPairingText,
    require_selected_pairing: bool,
) -> Result<(), LanPairingRejectionReason> {
    runtime
        .registry
        .lock()
        .map(|mut registry| match &runtime.persistence {
            LanPairingRegistryPersistence::InMemory => registry.apply_intent(
                intent,
                origin.0.as_deref(),
                observed_at.0.as_str(),
                require_selected_pairing,
                |candidate| ensure_controller_lease(candidate, intent, observed_at),
            ),
            LanPairingRegistryPersistence::LocalJsonRegistry(path) => registry
                .apply_intent_persisted(
                    path.as_path(),
                    intent,
                    origin.0.as_deref(),
                    observed_at.0.as_str(),
                    require_selected_pairing,
                    |candidate| ensure_controller_lease(candidate, intent, observed_at),
                )
                .map_err(|_error| LanPairingRejectionReason::SignedChildAgentContextUnavailable)
                .and_then(|result| result),
            LanPairingRegistryPersistence::UnavailableLocalJsonRegistry => {
                Err(LanPairingRejectionReason::SignedChildAgentContextUnavailable)
            }
        })
        .unwrap_or(Err(LanPairingRejectionReason::Malformed))
}

fn ensure_controller_lease(
    registry: &mut ocentra_parent_agent_core::trusted_device_registry::TrustedDeviceRegistry,
    intent: &LanParentIntentEnvelope,
    observed_at: &LanPairingText,
) -> Result<(), LanPairingRejectionReason> {
    if intent.parent_authority == LanPairingParentAuthority::ActiveController {
        registry.apply_controller_lease(
            intent,
            observed_at.0.as_str(),
            LanControllerLeaseMutation::Ensure,
        )?;
    }
    Ok(())
}

fn optional_text(origin: Option<impl Into<LanPairingText>>) -> LanPairingOptionalText {
    LanPairingOptionalText(origin.map(Into::into).map(|value| value.0))
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

pub(crate) fn is_household_device_decision_intent(intent: &LanParentIntentEnvelope) -> bool {
    matches!(
        intent.intent_kind,
        LanPairingIntentKind::RuleUpdate
            | LanPairingIntentKind::ApprovalDecision
            | LanPairingIntentKind::ConfigurationUpdate
    )
}
