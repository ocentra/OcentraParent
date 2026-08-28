use std::collections::BTreeMap;
use std::primitive::str as TestStr;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::constants::v08_enforcement_integrity_runtime_audit as proof;
use ocentra_parent_agent_protocol::constants::v08_integrity_alert_status_bridge as bridge;
use ocentra_parent_agent_protocol::constants::v08_notification_provider_status_boundary as boundary;
use ocentra_parent_agent_protocol::enforcement_integrity_runtime_audit::V08EnforcementIntegrityRuntimeAuditEntry;
use ocentra_parent_agent_protocol::enforcement_integrity_runtime_audit::V08EnforcementIntegrityRuntimeAuditIntegrityState;
use ocentra_parent_agent_protocol::enforcement_integrity_runtime_audit::V08EnforcementIntegrityRuntimeAuditReadModel;
use ocentra_parent_agent_protocol::enforcement_integrity_runtime_audit::V08EnforcementIntegrityRuntimeAuditResult;
use ocentra_parent_agent_protocol::policy_constants;

use super::enforcement_api::enforcement_integrity_runtime_audit_read_model::{
    v08_enforcement_integrity_runtime_audit_read_model, GeneratedAtTextRef,
};
use crate::test_invariants::require_some;

#[test]
fn enforcement_integrity_runtime_audit_read_model_covers_required_states() {
    let read_model = v08_enforcement_integrity_runtime_audit_read_model(GeneratedAtTextRef(
        policy_constants::TEST_EVALUATED_AT,
    ));
    let result_counts = count_results(&read_model.entries);
    let integrity_counts = count_integrity_states(&read_model.entries);

    assert_eq!(read_model.read_model_id, proof::READ_MODEL_ID);
    assert_eq!(read_model.entries.len(), 14);
    assert_eq!(
        read_model.integrity_alert_status_bridge.read_model_id,
        bridge::READ_MODEL_ID
    );
    assert_eq!(read_model.integrity_alert_status_bridge.entries.len(), 4);
    assert_eq!(
        read_model
            .notification_provider_status_boundary
            .read_model_id,
        boundary::READ_MODEL_ID
    );
    assert_eq!(
        read_model
            .notification_provider_status_boundary
            .entries
            .len(),
        5
    );
    for (result, expected_count) in [
        (V08EnforcementIntegrityRuntimeAuditResult::Succeeded, 1),
        (V08EnforcementIntegrityRuntimeAuditResult::Failed, 2),
        (V08EnforcementIntegrityRuntimeAuditResult::Unavailable, 3),
        (V08EnforcementIntegrityRuntimeAuditResult::Expired, 1),
        (V08EnforcementIntegrityRuntimeAuditResult::RolledBack, 1),
        (V08EnforcementIntegrityRuntimeAuditResult::Superseded, 1),
        (V08EnforcementIntegrityRuntimeAuditResult::NoOp, 1),
        (V08EnforcementIntegrityRuntimeAuditResult::ManualRequired, 2),
        (V08EnforcementIntegrityRuntimeAuditResult::Unsupported, 1),
        (V08EnforcementIntegrityRuntimeAuditResult::ObserveOnly, 1),
    ] {
        assert_eq!(result_count(&result_counts, result), expected_count);
    }
    for (state, expected_count) in [
        (
            V08EnforcementIntegrityRuntimeAuditIntegrityState::Running,
            8,
        ),
        (
            V08EnforcementIntegrityRuntimeAuditIntegrityState::PermissionMissing,
            1,
        ),
        (
            V08EnforcementIntegrityRuntimeAuditIntegrityState::AdapterUnavailable,
            1,
        ),
        (
            V08EnforcementIntegrityRuntimeAuditIntegrityState::StaleHeartbeat,
            1,
        ),
        (
            V08EnforcementIntegrityRuntimeAuditIntegrityState::TamperSignalManualRequired,
            1,
        ),
    ] {
        assert_eq!(integrity_count(&integrity_counts, state), expected_count);
    }
}

#[test]
fn enforcement_integrity_runtime_audit_preserves_no_claim_flags() {
    let read_model = v08_enforcement_integrity_runtime_audit_read_model(GeneratedAtTextRef(
        policy_constants::TEST_EVALUATED_AT,
    ));

    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.broad_installed_app_blocking_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.host_network_domain_blocking_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.exact_active_tab_enforcement_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.notification_delivery_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.tamper_hardening_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.mobile_privilege_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.stealth_persistence_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.privilege_escalation_claimed));
    assert!(read_model
        .integrity_alert_status_bridge
        .entries
        .iter()
        .all(|entry| !entry.provider_delivery_claimed));
    assert!(read_model
        .integrity_alert_status_bridge
        .entries
        .iter()
        .all(|entry| !entry.tamper_resistance_claimed));
    assert!(read_model
        .notification_provider_status_boundary
        .entries
        .iter()
        .all(|entry| !entry.provider_delivery_observed));
    assert!(read_model
        .notification_provider_status_boundary
        .entries
        .iter()
        .all(|entry| !entry.delivered_notification_claimed));
    assert!(read_model
        .notification_provider_status_boundary
        .entries
        .iter()
        .any(|entry| entry.status_entry_id == boundary::ENTRY_DELIVERED));
}

fn assert_entry_integrity(
    entries: &[V08EnforcementIntegrityRuntimeAuditEntry],
    audit_entry_id: &TestStr,
    integrity_state: V08EnforcementIntegrityRuntimeAuditIntegrityState,
) {
    let entry = require_some(
        entries
            .iter()
            .find(|candidate| candidate.audit_entry_id == audit_entry_id),
        proof::READ_MODEL_ID,
    );

    assert_eq!(entry.integrity_state, integrity_state);
}

fn count_results(
    entries: &[V08EnforcementIntegrityRuntimeAuditEntry],
) -> BTreeMap<V08EnforcementIntegrityRuntimeAuditResult, usize> {
    count_by(entries, |entry| entry.result)
}

fn count_integrity_states(
    entries: &[V08EnforcementIntegrityRuntimeAuditEntry],
) -> BTreeMap<V08EnforcementIntegrityRuntimeAuditIntegrityState, usize> {
    count_by(entries, |entry| entry.integrity_state)
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

fn result_count(
    counts: &BTreeMap<V08EnforcementIntegrityRuntimeAuditResult, usize>,
    result: V08EnforcementIntegrityRuntimeAuditResult,
) -> usize {
    *counts.get(&result).unwrap_or(&0)
}

fn integrity_count(
    counts: &BTreeMap<V08EnforcementIntegrityRuntimeAuditIntegrityState, usize>,
    state: V08EnforcementIntegrityRuntimeAuditIntegrityState,
) -> usize {
    *counts.get(&state).unwrap_or(&0)
}
