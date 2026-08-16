use crate::{
    constants::{self, v08_notification_provider_status_boundary as boundary},
    notification_provider_status_boundary::{
        V08NotificationEscalationReadiness, V08NotificationProviderDeliveryClaim,
        V08NotificationProviderStatus, V08NotificationProviderStatusBoundaryEntry,
        V08NotificationProviderStatusBoundaryReadModel, V08NotificationProviderStatusProofState,
        V08NotificationQuietHoursReadiness,
    },
    policy_constants,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn notification_provider_status_boundary_serializes_stable_state_values() {
    assert_eq!(
        serde_json::to_value(V08NotificationProviderStatus::Delivered)
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        boundary::STATUS_DELIVERED
    );
    assert_eq!(
        serde_json::to_value(V08NotificationProviderStatusProofState::DeliveryReceiptRequired)
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        boundary::PROOF_DELIVERY_RECEIPT_REQUIRED
    );
    assert_eq!(
        serde_json::to_value(V08NotificationQuietHoursReadiness::DeferNoncritical)
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        boundary::QUIET_HOURS_DEFER_NONCRITICAL
    );
    assert_eq!(
        serde_json::to_value(V08NotificationEscalationReadiness::WaitingWindow)
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        boundary::ESCALATION_WAITING_WINDOW
    );
}

#[test]
fn notification_provider_status_boundary_preserves_no_delivery_claims() {
    let read_model = V08NotificationProviderStatusBoundaryReadModel {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: boundary::READ_MODEL_ID.to_string(),
        generated_at: policy_constants::TEST_EVALUATED_AT.to_string(),
        source_read_model_ids: vec![boundary::SOURCE_REPORTS_NOTIFICATIONS_SYNC.to_string()],
        entries: vec![delivered_entry()],
    };
    let reparsed = serde_json::from_value::<V08NotificationProviderStatusBoundaryReadModel>(
        serde_json::to_value(read_model).expect_value(constants::error::AGENT_EVENT_SERIALIZES),
    )
    .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let entry = reparsed
        .entries
        .first()
        .expect_value(boundary::READ_MODEL_ID);

    assert_eq!(reparsed.read_model_id, boundary::READ_MODEL_ID);
    assert_eq!(
        entry.provider_status,
        V08NotificationProviderStatus::Delivered
    );
    assert_eq!(
        entry.status_proof_state,
        V08NotificationProviderStatusProofState::DeliveryReceiptRequired
    );
    assert!(!entry.provider_delivery_implemented);
    assert!(!entry.provider_delivery_observed);
    assert!(!entry.delivered_notification_claimed);
    assert!(!entry.sensitive_provider_payload_claimed);
    assert!(!entry.provider_stores_child_evidence_claimed);
    assert_eq!(
        entry.provider_receipt_refs,
        vec![boundary::REF_PROVIDER_RECEIPT_REQUIRED.to_string()]
    );
}

fn delivered_entry() -> V08NotificationProviderStatusBoundaryEntry {
    V08NotificationProviderStatusBoundaryEntry {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        status_entry_id: boundary::ENTRY_DELIVERED.to_string(),
        provider_status: V08NotificationProviderStatus::Delivered,
        status_proof_state: V08NotificationProviderStatusProofState::DeliveryReceiptRequired,
        quiet_hours_readiness: V08NotificationQuietHoursReadiness::DeferNoncritical,
        escalation_readiness: V08NotificationEscalationReadiness::WaitingWindow,
        delivery_claim_state: V08NotificationProviderDeliveryClaim::ReceiptRequired,
        notification_intent_ref: boundary::REF_NOTIFICATION_INTENT.to_string(),
        notification_status_ref: boundary::REF_STATUS_DELIVERED.to_string(),
        provider_attempt_ref: boundary::REF_ATTEMPT_DELIVERED.to_string(),
        audit_refs: vec![boundary::REF_AUDIT.to_string()],
        preference_refs: vec![boundary::REF_PARENT_PREFERENCES.to_string()],
        readiness_refs: vec![
            boundary::REF_QUIET_DEFER_NONCRITICAL.to_string(),
            boundary::REF_ESCALATION_WAITING_WINDOW.to_string(),
        ],
        provider_receipt_refs: vec![boundary::REF_PROVIDER_RECEIPT_REQUIRED.to_string()],
        manual_proof_requirements: vec![boundary::REQUIREMENT_PROVIDER_RECEIPT_ARTIFACT.to_string()],
        minimal_payload_boundary: boundary::BOUNDARY_DELIVERED.to_string(),
        provider_delivery_implemented: false,
        provider_delivery_observed: false,
        delivered_notification_claimed: false,
        sensitive_provider_payload_claimed: false,
        provider_stores_child_evidence_claimed: false,
        last_checked_at: policy_constants::TEST_EVALUATED_AT.to_string(),
    }
}
