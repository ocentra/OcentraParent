use ocentra_parent_agent_protocol::constants::v08_notification_provider_status_boundary as boundary;
use ocentra_parent_agent_protocol::notification_provider_status_boundary::V08NotificationEscalationReadiness;
use ocentra_parent_agent_protocol::notification_provider_status_boundary::V08NotificationProviderDeliveryClaim;
use ocentra_parent_agent_protocol::notification_provider_status_boundary::V08NotificationProviderStatus;
use ocentra_parent_agent_protocol::notification_provider_status_boundary::V08NotificationProviderStatusBoundaryEntry;
use ocentra_parent_agent_protocol::notification_provider_status_boundary::V08NotificationProviderStatusBoundaryReadModel;
use ocentra_parent_agent_protocol::notification_provider_status_boundary::V08NotificationProviderStatusProofState;
use ocentra_parent_agent_protocol::notification_provider_status_boundary::V08NotificationQuietHoursReadiness;
use ocentra_parent_agent_protocol::policy_constants;

#[derive(Clone, Copy)]
pub(crate) struct GeneratedAtTextRef<'a>(pub(crate) &'a str);

#[derive(Clone, Copy)]
struct StaticTextRefs(&'static [&'static str]);

struct BoundaryTextList(Vec<String>);

pub(crate) fn v08_notification_provider_status_boundary_read_model<'a>(
    generated_at: impl Into<GeneratedAtTextRef<'a>>,
) -> V08NotificationProviderStatusBoundaryReadModel {
    let generated_at = generated_at.into();
    V08NotificationProviderStatusBoundaryReadModel {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: boundary::READ_MODEL_ID.to_string(),
        generated_at: generated_at.0.to_string(),
        source_read_model_ids: vec![
            boundary::SOURCE_REPORTS_NOTIFICATIONS_SYNC.to_string(),
            boundary::SOURCE_INTEGRITY_ALERT_STATUS_BRIDGE.to_string(),
            boundary::SOURCE_DATA_CUSTODY.to_string(),
        ],
        entries: entry_specs()
            .iter()
            .map(|spec| entry_from_spec(spec, generated_at))
            .collect(),
    }
}

struct EntrySpec {
    status_entry_id: &'static str,
    provider_status: V08NotificationProviderStatus,
    status_proof_state: V08NotificationProviderStatusProofState,
    quiet_hours_readiness: V08NotificationQuietHoursReadiness,
    escalation_readiness: V08NotificationEscalationReadiness,
    delivery_claim_state: V08NotificationProviderDeliveryClaim,
    notification_status_ref: &'static str,
    provider_attempt_ref: &'static str,
    readiness_refs: &'static [&'static str],
    provider_receipt_refs: &'static [&'static str],
    manual_proof_requirements: &'static [&'static str],
    minimal_payload_boundary: &'static str,
}

fn entry_specs() -> Vec<EntrySpec> {
    vec![
        queued_spec(),
        delivered_spec(),
        failed_spec(),
        unavailable_spec(),
        manual_required_spec(),
    ]
}

fn queued_spec() -> EntrySpec {
    EntrySpec {
        status_entry_id: boundary::ENTRY_QUEUED,
        provider_status: V08NotificationProviderStatus::Queued,
        status_proof_state: V08NotificationProviderStatusProofState::QueuedContractOnly,
        quiet_hours_readiness: V08NotificationQuietHoursReadiness::Ready,
        escalation_readiness: V08NotificationEscalationReadiness::Ready,
        delivery_claim_state: V08NotificationProviderDeliveryClaim::NotImplemented,
        notification_status_ref: boundary::REF_STATUS_QUEUED,
        provider_attempt_ref: boundary::REF_ATTEMPT_QUEUED,
        readiness_refs: &[boundary::REF_QUIET_READY, boundary::REF_ESCALATION_READY],
        provider_receipt_refs: &[],
        manual_proof_requirements: &[],
        minimal_payload_boundary: boundary::BOUNDARY_QUEUED,
    }
}

fn delivered_spec() -> EntrySpec {
    EntrySpec {
        status_entry_id: boundary::ENTRY_DELIVERED,
        provider_status: V08NotificationProviderStatus::Delivered,
        status_proof_state: V08NotificationProviderStatusProofState::DeliveryReceiptRequired,
        quiet_hours_readiness: V08NotificationQuietHoursReadiness::DeferNoncritical,
        escalation_readiness: V08NotificationEscalationReadiness::WaitingWindow,
        delivery_claim_state: V08NotificationProviderDeliveryClaim::ReceiptRequired,
        notification_status_ref: boundary::REF_STATUS_DELIVERED,
        provider_attempt_ref: boundary::REF_ATTEMPT_DELIVERED,
        readiness_refs: &[
            boundary::REF_QUIET_DEFER_NONCRITICAL,
            boundary::REF_ESCALATION_WAITING_WINDOW,
        ],
        provider_receipt_refs: &[boundary::REF_PROVIDER_RECEIPT_REQUIRED],
        manual_proof_requirements: &[boundary::REQUIREMENT_PROVIDER_RECEIPT_ARTIFACT],
        minimal_payload_boundary: boundary::BOUNDARY_DELIVERED,
    }
}

fn failed_spec() -> EntrySpec {
    EntrySpec {
        status_entry_id: boundary::ENTRY_FAILED,
        provider_status: V08NotificationProviderStatus::Failed,
        status_proof_state: V08NotificationProviderStatusProofState::FailureContractOnly,
        quiet_hours_readiness: V08NotificationQuietHoursReadiness::Ready,
        escalation_readiness: V08NotificationEscalationReadiness::ManualRequired,
        delivery_claim_state: V08NotificationProviderDeliveryClaim::NotObserved,
        notification_status_ref: boundary::REF_STATUS_FAILED,
        provider_attempt_ref: boundary::REF_ATTEMPT_FAILED,
        readiness_refs: &[
            boundary::REF_QUIET_READY,
            boundary::REF_ESCALATION_MANUAL_REQUIRED,
        ],
        provider_receipt_refs: &[],
        manual_proof_requirements: &[boundary::REQUIREMENT_PROVIDER_ERROR_ARTIFACT],
        minimal_payload_boundary: boundary::BOUNDARY_FAILED,
    }
}

fn unavailable_spec() -> EntrySpec {
    EntrySpec {
        status_entry_id: boundary::ENTRY_UNAVAILABLE,
        provider_status: V08NotificationProviderStatus::Unavailable,
        status_proof_state: V08NotificationProviderStatusProofState::ProviderUnavailableContract,
        quiet_hours_readiness: V08NotificationQuietHoursReadiness::Unavailable,
        escalation_readiness: V08NotificationEscalationReadiness::Unavailable,
        delivery_claim_state: V08NotificationProviderDeliveryClaim::NotImplemented,
        notification_status_ref: boundary::REF_STATUS_UNAVAILABLE,
        provider_attempt_ref: boundary::REF_ATTEMPT_UNAVAILABLE,
        readiness_refs: &[
            boundary::REF_QUIET_UNAVAILABLE,
            boundary::REF_ESCALATION_UNAVAILABLE,
        ],
        provider_receipt_refs: &[],
        manual_proof_requirements: &[boundary::REQUIREMENT_PROVIDER_CONFIGURATION],
        minimal_payload_boundary: boundary::BOUNDARY_UNAVAILABLE,
    }
}

fn manual_required_spec() -> EntrySpec {
    EntrySpec {
        status_entry_id: boundary::ENTRY_MANUAL_REQUIRED,
        provider_status: V08NotificationProviderStatus::ManualRequired,
        status_proof_state: V08NotificationProviderStatusProofState::ManualActionRequired,
        quiet_hours_readiness: V08NotificationQuietHoursReadiness::ManualRequired,
        escalation_readiness: V08NotificationEscalationReadiness::ManualRequired,
        delivery_claim_state: V08NotificationProviderDeliveryClaim::NotObserved,
        notification_status_ref: boundary::REF_STATUS_MANUAL_REQUIRED,
        provider_attempt_ref: boundary::REF_ATTEMPT_MANUAL_REQUIRED,
        readiness_refs: &[
            boundary::REF_QUIET_MANUAL_REQUIRED,
            boundary::REF_ESCALATION_MANUAL_REQUIRED,
        ],
        provider_receipt_refs: &[],
        manual_proof_requirements: &[
            boundary::REQUIREMENT_PARENT_PROVIDER_SETUP,
            boundary::REQUIREMENT_PROVIDER_SECURITY_REVIEW,
        ],
        minimal_payload_boundary: boundary::BOUNDARY_MANUAL_REQUIRED,
    }
}

fn entry_from_spec(
    spec: &EntrySpec,
    generated_at: GeneratedAtTextRef<'_>,
) -> V08NotificationProviderStatusBoundaryEntry {
    V08NotificationProviderStatusBoundaryEntry {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        status_entry_id: spec.status_entry_id.to_string(),
        provider_status: spec.provider_status,
        status_proof_state: spec.status_proof_state,
        quiet_hours_readiness: spec.quiet_hours_readiness,
        escalation_readiness: spec.escalation_readiness,
        delivery_claim_state: spec.delivery_claim_state,
        notification_intent_ref: boundary::REF_NOTIFICATION_INTENT.to_string(),
        notification_status_ref: spec.notification_status_ref.to_string(),
        provider_attempt_ref: spec.provider_attempt_ref.to_string(),
        audit_refs: vec![boundary::REF_AUDIT.to_string()],
        preference_refs: vec![boundary::REF_PARENT_PREFERENCES.to_string()],
        readiness_refs: to_strings(StaticTextRefs(spec.readiness_refs)).0,
        provider_receipt_refs: to_strings(StaticTextRefs(spec.provider_receipt_refs)).0,
        manual_proof_requirements: to_strings(StaticTextRefs(spec.manual_proof_requirements)).0,
        minimal_payload_boundary: spec.minimal_payload_boundary.to_string(),
        provider_delivery_implemented: false,
        provider_delivery_observed: false,
        delivered_notification_claimed: false,
        sensitive_provider_payload_claimed: false,
        provider_stores_child_evidence_claimed: false,
        last_checked_at: generated_at.0.to_string(),
    }
}

fn to_strings(values: StaticTextRefs) -> BoundaryTextList {
    BoundaryTextList(values.0.iter().map(|value| (*value).to_string()).collect())
}
