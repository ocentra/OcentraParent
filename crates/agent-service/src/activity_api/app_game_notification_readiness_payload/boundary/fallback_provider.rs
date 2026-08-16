use ocentra_parent_agent_protocol::notification_provider_status_boundary::{
    V08NotificationEscalationReadiness, V08NotificationProviderDeliveryClaim,
    V08NotificationProviderStatus, V08NotificationProviderStatusBoundaryEntry,
    V08NotificationProviderStatusProofState, V08NotificationQuietHoursReadiness,
    V08_NOTIFICATION_PROVIDER_STATUS_BOUNDARY_SCHEMA_VERSION,
};
use ocentra_parent_agent_protocol::{
    AppGameNotificationReadinessRow, APP_GAME_NOTIFICATION_READINESS_STATE_UNAVAILABLE,
};

use super::super::constants::*;

pub(super) fn build(
    row: &AppGameNotificationReadinessRow,
    generated_at: &str,
    scheduler_evidence_invalid: bool,
) -> V08NotificationProviderStatusBoundaryEntry {
    let unavailable = row.readiness_state == APP_GAME_NOTIFICATION_READINESS_STATE_UNAVAILABLE;
    let scheduler_evidence_ref = scheduler_evidence_ref(unavailable, scheduler_evidence_invalid);
    V08NotificationProviderStatusBoundaryEntry {
        schema_version: V08_NOTIFICATION_PROVIDER_STATUS_BOUNDARY_SCHEMA_VERSION.to_string(),
        status_entry_id: format!("app-game-provider-status-entry:{}", row.row_id),
        provider_status: provider_status(unavailable),
        status_proof_state: proof_state(unavailable),
        quiet_hours_readiness: quiet_hours_readiness(unavailable),
        escalation_readiness: escalation_readiness(unavailable),
        delivery_claim_state: V08NotificationProviderDeliveryClaim::NotImplemented,
        notification_intent_ref: row.row_id.clone(),
        notification_status_ref: format!("notification-status:{}", row.row_id),
        provider_attempt_ref: format!("provider-attempt-not-observed:{}", row.row_id),
        audit_refs: row.evidence_reference_ids.clone(),
        preference_refs: Vec::new(),
        readiness_refs: vec![row.row_id.clone()],
        provider_receipt_refs: Vec::new(),
        manual_proof_requirements: proof_requirements(unavailable, scheduler_evidence_ref),
        minimal_payload_boundary: fallback_payload_boundary(),
        provider_delivery_implemented: false,
        provider_delivery_observed: false,
        delivered_notification_claimed: false,
        sensitive_provider_payload_claimed: false,
        provider_stores_child_evidence_claimed: false,
        last_checked_at: generated_at.to_string(),
    }
}

pub(super) fn scheduler_evidence_ref(unavailable: bool, invalid: bool) -> &'static str {
    match (invalid, unavailable) {
        (true, true) => SCHEDULER_INVALID_UNAVAILABLE,
        (true, false) => SCHEDULER_INVALID_MANUAL,
        (false, true) => SCHEDULER_MISSING_UNAVAILABLE,
        (false, false) => SCHEDULER_MISSING_MANUAL,
    }
}

fn provider_status(unavailable: bool) -> V08NotificationProviderStatus {
    if unavailable {
        V08NotificationProviderStatus::Unavailable
    } else {
        V08NotificationProviderStatus::ManualRequired
    }
}

fn proof_state(unavailable: bool) -> V08NotificationProviderStatusProofState {
    if unavailable {
        V08NotificationProviderStatusProofState::ProviderUnavailableContract
    } else {
        V08NotificationProviderStatusProofState::ManualActionRequired
    }
}

fn quiet_hours_readiness(unavailable: bool) -> V08NotificationQuietHoursReadiness {
    if unavailable {
        V08NotificationQuietHoursReadiness::Unavailable
    } else {
        V08NotificationQuietHoursReadiness::ManualRequired
    }
}

fn escalation_readiness(unavailable: bool) -> V08NotificationEscalationReadiness {
    if unavailable {
        V08NotificationEscalationReadiness::Unavailable
    } else {
        V08NotificationEscalationReadiness::ManualRequired
    }
}

fn proof_requirements(unavailable: bool, scheduler_evidence_ref: &str) -> Vec<String> {
    if unavailable {
        vec![
            MANUAL_PROVIDER_AVAILABILITY.to_string(),
            scheduler_evidence_ref.to_string(),
        ]
    } else {
        vec![
            MANUAL_PROVIDER_CREDENTIALS.to_string(),
            provider_delivery_receipt(),
            scheduler_evidence_ref.to_string(),
        ]
    }
}
