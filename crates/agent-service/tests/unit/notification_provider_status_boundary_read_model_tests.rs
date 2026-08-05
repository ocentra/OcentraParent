use std::collections::BTreeMap;

use ocentra_parent_agent_protocol::constants::v08_notification_provider_status_boundary as boundary;
use ocentra_parent_agent_protocol::notification_provider_status_boundary::V08NotificationEscalationReadiness;
use ocentra_parent_agent_protocol::notification_provider_status_boundary::V08NotificationProviderStatus;
use ocentra_parent_agent_protocol::notification_provider_status_boundary::V08NotificationProviderStatusBoundaryEntry;
use ocentra_parent_agent_protocol::notification_provider_status_boundary::V08NotificationQuietHoursReadiness;
use ocentra_parent_agent_protocol::policy_constants;

use super::enforcement_api::notification_provider_status_boundary_read_model::{
    v08_notification_provider_status_boundary_read_model, GeneratedAtTextRef,
};

#[test]
fn notification_provider_status_boundary_read_model_covers_provider_states() {
    let read_model = v08_notification_provider_status_boundary_read_model(GeneratedAtTextRef(
        policy_constants::TEST_EVALUATED_AT,
    ));
    let status_counts = count_statuses(&read_model.entries);

    assert_eq!(read_model.read_model_id, boundary::READ_MODEL_ID);
    assert_eq!(read_model.entries.len(), 5);
    assert_eq!(
        status_count(&status_counts, V08NotificationProviderStatus::Queued),
        1
    );
    assert_eq!(
        status_count(&status_counts, V08NotificationProviderStatus::Delivered),
        1
    );
    assert_eq!(
        status_count(&status_counts, V08NotificationProviderStatus::Failed),
        1
    );
    assert_eq!(
        status_count(&status_counts, V08NotificationProviderStatus::Unavailable),
        1
    );
    assert_eq!(
        status_count(
            &status_counts,
            V08NotificationProviderStatus::ManualRequired
        ),
        1
    );
    assert!(read_model
        .source_read_model_ids
        .contains(&boundary::SOURCE_DATA_CUSTODY.to_string()));
}

#[test]
fn notification_provider_status_boundary_read_model_preserves_readiness_and_non_claims() {
    let read_model = v08_notification_provider_status_boundary_read_model(GeneratedAtTextRef(
        policy_constants::TEST_EVALUATED_AT,
    ));
    let quiet_counts = count_quiet_hours(&read_model.entries);
    let escalation_counts = count_escalation(&read_model.entries);

    assert_eq!(
        quiet_count(&quiet_counts, V08NotificationQuietHoursReadiness::Ready),
        2
    );
    assert_eq!(
        quiet_count(
            &quiet_counts,
            V08NotificationQuietHoursReadiness::DeferNoncritical
        ),
        1
    );
    assert_eq!(
        escalation_count(
            &escalation_counts,
            V08NotificationEscalationReadiness::ManualRequired
        ),
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
) -> BTreeMap<V08NotificationProviderStatus, usize> {
    count_by(entries, |entry| entry.provider_status)
}

fn count_quiet_hours(
    entries: &[V08NotificationProviderStatusBoundaryEntry],
) -> BTreeMap<V08NotificationQuietHoursReadiness, usize> {
    count_by(entries, |entry| entry.quiet_hours_readiness)
}

fn count_escalation(
    entries: &[V08NotificationProviderStatusBoundaryEntry],
) -> BTreeMap<V08NotificationEscalationReadiness, usize> {
    count_by(entries, |entry| entry.escalation_readiness)
}

fn count_by<TEntry, TKey>(
    entries: &[TEntry],
    key_for: impl Fn(&TEntry) -> TKey,
) -> BTreeMap<TKey, usize>
where
    TKey: Copy + Ord,
{
    entries.iter().fold(BTreeMap::new(), |mut counts, entry| {
        *counts.entry(key_for(entry)).or_default() += 1;
        counts
    })
}

fn status_count(
    counts: &BTreeMap<V08NotificationProviderStatus, usize>,
    status: V08NotificationProviderStatus,
) -> usize {
    *counts.get(&status).unwrap_or(&0)
}

fn quiet_count(
    counts: &BTreeMap<V08NotificationQuietHoursReadiness, usize>,
    readiness: V08NotificationQuietHoursReadiness,
) -> usize {
    *counts.get(&readiness).unwrap_or(&0)
}

fn escalation_count(
    counts: &BTreeMap<V08NotificationEscalationReadiness, usize>,
    readiness: V08NotificationEscalationReadiness,
) -> usize {
    *counts.get(&readiness).unwrap_or(&0)
}
