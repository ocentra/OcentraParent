use std::collections::BTreeMap;

use ocentra_parent_agent_protocol::{
    constants::v08_notification_provider_status_boundary as boundary, policy_constants,
    V08NotificationEscalationReadiness, V08NotificationProviderStatus,
    V08NotificationProviderStatusBoundaryEntry, V08NotificationQuietHoursReadiness,
};

use super::notification_provider_status_boundary_read_model::v08_notification_provider_status_boundary_read_model;

#[test]
fn notification_provider_status_boundary_read_model_covers_provider_states() {
    let read_model =
        v08_notification_provider_status_boundary_read_model(policy_constants::TEST_EVALUATED_AT);
    let status_counts = count_statuses(&read_model.entries);

    assert_eq!(read_model.read_model_id, boundary::READ_MODEL_ID);
    assert_eq!(read_model.entries.len(), 5);
    assert_eq!(status_count(&status_counts, boundary::STATUS_QUEUED), 1);
    assert_eq!(status_count(&status_counts, boundary::STATUS_DELIVERED), 1);
    assert_eq!(status_count(&status_counts, boundary::STATUS_FAILED), 1);
    assert_eq!(
        status_count(&status_counts, boundary::STATUS_UNAVAILABLE),
        1
    );
    assert_eq!(
        status_count(&status_counts, boundary::STATUS_MANUAL_REQUIRED),
        1
    );
    assert!(read_model
        .source_read_model_ids
        .contains(&boundary::SOURCE_DATA_CUSTODY.to_string()));
}

#[test]
fn notification_provider_status_boundary_read_model_preserves_readiness_and_non_claims() {
    let read_model =
        v08_notification_provider_status_boundary_read_model(policy_constants::TEST_EVALUATED_AT);
    let quiet_counts = count_quiet_hours(&read_model.entries);
    let escalation_counts = count_escalation(&read_model.entries);

    assert_eq!(quiet_count(&quiet_counts, boundary::QUIET_HOURS_READY), 2);
    assert_eq!(
        quiet_count(&quiet_counts, boundary::QUIET_HOURS_DEFER_NONCRITICAL),
        1
    );
    assert_eq!(
        escalation_count(&escalation_counts, boundary::ESCALATION_MANUAL_REQUIRED),
        2
    );
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.provider_delivery_implemented));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.provider_delivery_observed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.delivered_notification_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.sensitive_provider_payload_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.provider_stores_child_evidence_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| entry.audit_refs.contains(&boundary::REF_AUDIT.to_string())));
}

fn count_statuses(
    entries: &[V08NotificationProviderStatusBoundaryEntry],
) -> BTreeMap<&'static str, usize> {
    entries.iter().fold(BTreeMap::new(), |mut counts, entry| {
        *counts
            .entry(status_name(entry.provider_status))
            .or_default() += 1;
        counts
    })
}

fn count_quiet_hours(
    entries: &[V08NotificationProviderStatusBoundaryEntry],
) -> BTreeMap<&'static str, usize> {
    entries.iter().fold(BTreeMap::new(), |mut counts, entry| {
        *counts
            .entry(quiet_hours_name(entry.quiet_hours_readiness))
            .or_default() += 1;
        counts
    })
}

fn count_escalation(
    entries: &[V08NotificationProviderStatusBoundaryEntry],
) -> BTreeMap<&'static str, usize> {
    entries.iter().fold(BTreeMap::new(), |mut counts, entry| {
        *counts
            .entry(escalation_name(entry.escalation_readiness))
            .or_default() += 1;
        counts
    })
}

fn status_name(status: V08NotificationProviderStatus) -> &'static str {
    match status {
        V08NotificationProviderStatus::Queued => boundary::STATUS_QUEUED,
        V08NotificationProviderStatus::Delivered => boundary::STATUS_DELIVERED,
        V08NotificationProviderStatus::Failed => boundary::STATUS_FAILED,
        V08NotificationProviderStatus::Unavailable => boundary::STATUS_UNAVAILABLE,
        V08NotificationProviderStatus::ManualRequired => boundary::STATUS_MANUAL_REQUIRED,
    }
}

fn quiet_hours_name(readiness: V08NotificationQuietHoursReadiness) -> &'static str {
    match readiness {
        V08NotificationQuietHoursReadiness::Ready => boundary::QUIET_HOURS_READY,
        V08NotificationQuietHoursReadiness::DeferNoncritical => {
            boundary::QUIET_HOURS_DEFER_NONCRITICAL
        }
        V08NotificationQuietHoursReadiness::ManualRequired => boundary::QUIET_HOURS_MANUAL_REQUIRED,
        V08NotificationQuietHoursReadiness::Unavailable => boundary::QUIET_HOURS_UNAVAILABLE,
    }
}

fn escalation_name(readiness: V08NotificationEscalationReadiness) -> &'static str {
    match readiness {
        V08NotificationEscalationReadiness::Ready => boundary::ESCALATION_READY,
        V08NotificationEscalationReadiness::WaitingWindow => boundary::ESCALATION_WAITING_WINDOW,
        V08NotificationEscalationReadiness::ManualRequired => boundary::ESCALATION_MANUAL_REQUIRED,
        V08NotificationEscalationReadiness::Unavailable => boundary::ESCALATION_UNAVAILABLE,
    }
}

fn status_count(counts: &BTreeMap<&'static str, usize>, status: &'static str) -> usize {
    *counts.get(status).unwrap_or(&0)
}

fn quiet_count(counts: &BTreeMap<&'static str, usize>, readiness: &'static str) -> usize {
    *counts.get(readiness).unwrap_or(&0)
}

fn escalation_count(counts: &BTreeMap<&'static str, usize>, readiness: &'static str) -> usize {
    *counts.get(readiness).unwrap_or(&0)
}
