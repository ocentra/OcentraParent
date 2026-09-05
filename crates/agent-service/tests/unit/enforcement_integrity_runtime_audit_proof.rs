use std::path::PathBuf as TestPathBuf;
use std::primitive::str as TestStr;
use std::string::String as TestString;
use std::{collections::BTreeMap, fs};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::constants::v08_enforcement_integrity_runtime_audit as proof;
use ocentra_parent_agent_protocol::constants::v08_integrity_alert_status_bridge as bridge;
use ocentra_parent_agent_protocol::constants::v08_notification_provider_status_boundary as boundary;
use ocentra_parent_agent_protocol::enforcement_integrity_runtime_audit::V08EnforcementIntegrityRuntimeAuditEntry;
use ocentra_parent_agent_protocol::policy_constants;
use serde_json::{json, Value};

use super::enforcement_api::enforcement_integrity_runtime_audit_read_model::{
    v08_enforcement_integrity_runtime_audit_read_model, GeneratedAtTextRef,
};
use crate::test_require_ok::require_ok;
use crate::test_require_some::require_some;

#[test]
fn enforcement_integrity_runtime_audit_read_model_writes_proof_artifact() {
    let read_model = v08_enforcement_integrity_runtime_audit_read_model(GeneratedAtTextRef(
        policy_constants::TEST_EVALUATED_AT,
    ));
    let proof_artifact = proof_artifact_path();
    let summary = build_proof_summary(&read_model);
    write_proof_summary(&proof_artifact, &summary);
    assert_summary_overview(&summary);

    let rows = summary_rows(&summary, "rows");
    assert_eq!(rows.len(), 14);
    assert_row_summary(
        rows,
        proof::ENTRY_TAMPER_MANUAL,
        proof::RESULT_MANUAL_REQUIRED,
        proof::INTEGRITY_TAMPER_SIGNAL_MANUAL_REQUIRED,
        &[
            proof::REQUIREMENT_SERVICE_MANAGER_STOP_PROOF,
            proof::REQUIREMENT_UNINSTALL_DETECTION_ARTIFACT,
            proof::REQUIREMENT_SECURITY_REVIEW,
        ],
    );
    assert_row_summary(
        rows,
        proof::ENTRY_PERMISSION_LOSS,
        proof::RESULT_UNAVAILABLE,
        proof::INTEGRITY_PERMISSION_MISSING,
        &[
            proof::REQUIREMENT_PERMISSION_RESTORE,
            proof::REQUIREMENT_OPERATOR_PERMISSION_STATE,
        ],
    );
    assert_row_summary(
        rows,
        proof::ENTRY_STALE_HEARTBEAT,
        proof::RESULT_UNAVAILABLE,
        proof::INTEGRITY_STALE_HEARTBEAT,
        &[
            proof::REQUIREMENT_FRESH_HEARTBEAT,
            proof::REQUIREMENT_PARENT_VISIBLE_STALE_ALERT,
        ],
    );
    assert_row_summary(
        rows,
        proof::ENTRY_ADAPTER_UNAVAILABLE,
        proof::RESULT_UNAVAILABLE,
        proof::INTEGRITY_ADAPTER_UNAVAILABLE,
        &[
            proof::REQUIREMENT_ADAPTER_RECOVERY,
            proof::REQUIREMENT_SERVICE_RESTART_RECOVERY,
        ],
    );
    assert_row_summary(
        rows,
        proof::ENTRY_MOBILE_UNSUPPORTED,
        proof::RESULT_UNSUPPORTED,
        proof::INTEGRITY_NOT_APPLICABLE,
        &[
            proof::REQUIREMENT_IOS_FAMILY_CONTROLS,
            proof::REQUIREMENT_IOS_DEVICE_ACTIVITY,
        ],
    );

    assert_bridge_and_boundary_rows(&summary);
}

fn build_proof_summary(
    read_model: &ocentra_parent_agent_protocol::enforcement_integrity_runtime_audit::V08EnforcementIntegrityRuntimeAuditReadModel,
) -> Value {
    json!({
        "schemaVersion": 1,
        "proofMode": "tamper-integrity-audit-contract-proof",
        "generatedAt": read_model.generated_at,
        "readModelId": read_model.read_model_id,
        "entryCount": read_model.entries.len(),
        "bridgeReadModelId": read_model.integrity_alert_status_bridge.read_model_id,
        "bridgeEntryCount": read_model.integrity_alert_status_bridge.entries.len(),
        "boundaryReadModelId": read_model.notification_provider_status_boundary.read_model_id,
        "boundaryEntryCount": read_model.notification_provider_status_boundary.entries.len(),
        "sourceReadModelIds": read_model.source_read_model_ids,
        "rows": read_model.entries.iter().map(entry_summary).collect::<Vec<_>>(),
        "resultCounts": count_results(&read_model.entries),
        "integrityCounts": count_integrity_states(&read_model.entries),
        "negativeClaims": count_negative_claims(read_model),
        "bridgeRows": read_model.integrity_alert_status_bridge.entries.iter().map(bridge_row_summary).collect::<Vec<_>>(),
        "boundaryRows": read_model.notification_provider_status_boundary.entries.iter().map(boundary_row_summary).collect::<Vec<_>>(),
    })
}

fn entry_summary(entry: &V08EnforcementIntegrityRuntimeAuditEntry) -> Value {
    json!({
        "auditEntryId": entry.audit_entry_id,
        "surface": entry.surface,
        "result": entry.result,
        "integrityState": entry.integrity_state,
        "boundary": entry.boundary,
        "manualProofRequirements": entry.manual_proof_requirements,
        "claimFlags": claim_flags(entry),
    })
}

fn bridge_row_summary(
    entry: &ocentra_parent_agent_protocol::integrity_alert_status_bridge::V08IntegrityAlertStatusBridgeEntry,
) -> Value {
    json!({
        "bridgeEntryId": entry.bridge_entry_id,
        "statusEntryId": entry.status_ref,
        "providerDeliveryClaimed": entry.provider_delivery_claimed,
        "tamperResistanceClaimed": entry.tamper_resistance_claimed,
    })
}

fn boundary_row_summary(
    entry: &ocentra_parent_agent_protocol::notification_provider_status_boundary::V08NotificationProviderStatusBoundaryEntry,
) -> Value {
    json!({
        "statusEntryId": entry.status_entry_id,
        "providerDeliveryObserved": entry.provider_delivery_observed,
        "deliveredNotificationClaimed": entry.delivered_notification_claimed,
    })
}

fn claim_flags(entry: &V08EnforcementIntegrityRuntimeAuditEntry) -> Value {
    json!({
        "broadInstalledAppBlockingClaimed": entry.broad_installed_app_blocking_claimed,
        "hostNetworkDomainBlockingClaimed": entry.host_network_domain_blocking_claimed,
        "exactActiveTabEnforcementClaimed": entry.exact_active_tab_enforcement_claimed,
        "notificationDeliveryClaimed": entry.notification_delivery_claimed,
        "tamperHardeningClaimed": entry.tamper_hardening_claimed,
        "mobilePrivilegeClaimed": entry.mobile_privilege_claimed,
        "stealthPersistenceClaimed": entry.stealth_persistence_claimed,
        "privilegeEscalationClaimed": entry.privilege_escalation_claimed,
    })
}

fn count_results(
    entries: &[V08EnforcementIntegrityRuntimeAuditEntry],
) -> BTreeMap<TestString, usize> {
    entries.iter().fold(BTreeMap::new(), |mut counts, entry| {
        *counts.entry(protocol_text(entry.result)).or_default() += 1;
        counts
    })
}

fn count_integrity_states(
    entries: &[V08EnforcementIntegrityRuntimeAuditEntry],
) -> BTreeMap<TestString, usize> {
    entries.iter().fold(BTreeMap::new(), |mut counts, entry| {
        *counts
            .entry(protocol_text(entry.integrity_state))
            .or_default() += 1;
        counts
    })
}

fn count_negative_claims(
    read_model: &ocentra_parent_agent_protocol::enforcement_integrity_runtime_audit::V08EnforcementIntegrityRuntimeAuditReadModel,
) -> Value {
    json!({
        "notificationDeliveryClaimed": read_model
            .entries
            .iter()
            .filter(|entry| entry.notification_delivery_claimed)
            .count(),
        "tamperHardeningClaimed": read_model
            .entries
            .iter()
            .filter(|entry| entry.tamper_hardening_claimed)
            .count(),
        "mobilePrivilegeClaimed": read_model
            .entries
            .iter()
            .filter(|entry| entry.mobile_privilege_claimed)
            .count(),
        "stealthPersistenceClaimed": read_model
            .entries
            .iter()
            .filter(|entry| entry.stealth_persistence_claimed)
            .count(),
        "privilegeEscalationClaimed": read_model
            .entries
            .iter()
            .filter(|entry| entry.privilege_escalation_claimed)
            .count(),
        "providerDeliveryClaimed": read_model
            .integrity_alert_status_bridge
            .entries
            .iter()
            .filter(|entry| entry.provider_delivery_claimed)
            .count(),
        "tamperResistanceClaimed": read_model
            .integrity_alert_status_bridge
            .entries
            .iter()
            .filter(|entry| entry.tamper_resistance_claimed)
            .count(),
        "providerDeliveryObserved": read_model
            .notification_provider_status_boundary
            .entries
            .iter()
            .filter(|entry| entry.provider_delivery_observed)
            .count(),
        "deliveredNotificationClaimed": read_model
            .notification_provider_status_boundary
            .entries
            .iter()
            .filter(|entry| entry.delivered_notification_claimed)
            .count(),
    })
}

fn protocol_text<T: serde::Serialize>(value: T) -> TestString {
    require_ok(
        serde_json::to_string(&value),
        constants::error::AGENT_EVENT_SERIALIZES,
    )
    .trim_matches('"')
    .to_owned()
}

fn assert_row_summary(
    rows: &[Value],
    audit_entry_id: &'static TestStr,
    result: &'static TestStr,
    integrity_state: &'static TestStr,
    manual_proof_requirements: &[&'static TestStr],
) {
    let row = require_some(
        rows.iter()
            .find(|candidate| candidate["auditEntryId"] == audit_entry_id),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    assert_eq!(row["result"], result);
    assert_eq!(row["integrityState"], integrity_state);
    assert_eq!(
        row["manualProofRequirements"],
        json!(manual_proof_requirements)
    );
}

fn proof_artifact_path() -> TestPathBuf {
    TestPathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../test-results/tamper-integrity-audit-contract-proof/rust-proof.json")
}

fn write_proof_summary(proof_artifact: &TestPathBuf, summary: &Value) {
    if let Some(parent) = proof_artifact.parent() {
        require_ok(
            fs::create_dir_all(parent),
            constants::error::AGENT_EVENT_SERIALIZES,
        );
    }

    let serialized_summary = require_ok(
        serde_json::to_string_pretty(summary),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    require_ok(
        fs::write(proof_artifact, serialized_summary),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
}

fn assert_summary_overview(summary: &Value) {
    assert_eq!(summary["schemaVersion"], 1);
    assert_eq!(
        summary["proofMode"],
        "tamper-integrity-audit-contract-proof"
    );
    assert_eq!(summary["readModelId"], proof::READ_MODEL_ID);
    assert_eq!(summary["entryCount"], 14);
    assert_eq!(summary["bridgeEntryCount"], 4);
    assert_eq!(summary["boundaryEntryCount"], 5);
    assert_eq!(summary["negativeClaims"]["notificationDeliveryClaimed"], 0);
    assert_eq!(summary["negativeClaims"]["tamperHardeningClaimed"], 0);
    assert_eq!(summary["negativeClaims"]["mobilePrivilegeClaimed"], 0);
    assert_eq!(summary["negativeClaims"]["stealthPersistenceClaimed"], 0);
    assert_eq!(summary["negativeClaims"]["privilegeEscalationClaimed"], 0);
    assert_eq!(summary["negativeClaims"]["providerDeliveryClaimed"], 0);
    assert_eq!(summary["negativeClaims"]["tamperResistanceClaimed"], 0);
    assert_eq!(summary["negativeClaims"]["providerDeliveryObserved"], 0);
    assert_eq!(summary["negativeClaims"]["deliveredNotificationClaimed"], 0);
}

fn assert_bridge_and_boundary_rows(summary: &Value) {
    let bridge_rows = summary_rows(summary, "bridgeRows");
    assert_eq!(bridge_rows.len(), 4);
    assert!(bridge_rows
        .iter()
        .any(|row| row["bridgeEntryId"] == bridge::ENTRY_STOPPED_OR_REMOVED));

    let boundary_rows = summary_rows(summary, "boundaryRows");
    assert_eq!(boundary_rows.len(), 5);
    assert!(boundary_rows
        .iter()
        .any(|row| row["statusEntryId"] == boundary::ENTRY_DELIVERED));
}

fn summary_rows<'a>(summary: &'a Value, key: &TestStr) -> &'a [Value] {
    require_some(
        summary[key].as_array().map(Vec::as_slice),
        constants::error::AGENT_EVENT_SERIALIZES,
    )
}
