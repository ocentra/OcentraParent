use ocentra_eventing::expect_value::ExpectValue;
use serde_json::{json, Value};

use ocentra_parent_agent_protocol::social_parent_notification_delivery_read_model::{
    SocialParentNotificationDeliveryReadinessRow, SocialParentNotificationDeliveryReadinessSnapshot,
};
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_AUDIT_REF;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_EVIDENCE_REF;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_ENFORCEMENT;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_EXTERNAL_RUNTIME;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_FINAL_POLICY;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_PARENT_NOTIFICATION_UI;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_PROVIDER_DELIVERY;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_PROVIDER_RECEIPT;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_POLICY_REF;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_READINESS_ID;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_SOURCE_REPORT_WRITER_PROOF_REF;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_REPORT_READY;

#[test]
fn social_parent_notification_delivery_snapshot_serializes_claim_boundaries() {
    let snapshot = snapshot();
    let value = serde_json::to_value(snapshot).expect_value("snapshot serializes: {error:?}");

    assert_eq!(
        value["schemaVersion"],
        json!(SOCIAL_PARENT_NOTIFICATION_DELIVERY_SCHEMA_VERSION)
    );
    assert_eq!(
        value["readinessId"],
        json!(SOCIAL_PARENT_NOTIFICATION_DELIVERY_READINESS_ID)
    );
    assert_eq!(value["parentReportStatusReadyCount"], json!(1));
    assert_eq!(value["parentNotificationUiDeliveryClaimed"], json!(false));
    assert_eq!(value["externalRuntimeReportDeliveryClaimed"], json!(false));
    assert_eq!(value["finalPolicyExecutionClaimed"], json!(false));
    assert_eq!(value["enforcementClaimed"], json!(false));
    assert_eq!(value["nonClaims"].as_array().map(Vec::len), Some(6));
    assert_ready_row(value["rows"].as_array().and_then(|rows| rows.first()));
}

fn assert_ready_row(row: Option<&Value>) {
    let row = row.expect_value("ready row exists");
    assert_eq!(
        row["notificationDeliveryReadinessState"],
        json!(SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_REPORT_READY)
    );
    assert_eq!(row["parentNotificationUiRef"], Value::Null);
    assert_eq!(row["parentNotificationUiDelivered"], json!(false));
    assert_eq!(row["externalRuntimeReportDeliveryClaimed"], json!(false));
    assert_eq!(row["providerDeliveryAttempted"], json!(false));
    assert_eq!(row["providerReceiptIngested"], json!(false));
    assert_eq!(row["finalPolicyDecisionClaimed"], json!(false));
    assert_eq!(row["enforcementClaimed"], json!(false));
}

fn snapshot() -> SocialParentNotificationDeliveryReadinessSnapshot {
    SocialParentNotificationDeliveryReadinessSnapshot {
        schema_version: SOCIAL_PARENT_NOTIFICATION_DELIVERY_SCHEMA_VERSION.to_string(),
        readiness_id: SOCIAL_PARENT_NOTIFICATION_DELIVERY_READINESS_ID.to_string(),
        generated_at: "2026-06-08T11:45:00Z".to_string(),
        source_report_writer_proof_ref:
            SOCIAL_PARENT_NOTIFICATION_DELIVERY_SOURCE_REPORT_WRITER_PROOF_REF.to_string(),
        rows: vec![row()],
        non_claims: vec![
            SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_PARENT_NOTIFICATION_UI.to_string(),
            SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_EXTERNAL_RUNTIME.to_string(),
            SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_PROVIDER_DELIVERY.to_string(),
            SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_PROVIDER_RECEIPT.to_string(),
            SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_FINAL_POLICY.to_string(),
            SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_ENFORCEMENT.to_string(),
        ],
        parent_report_status_ready_count: 1,
        manual_required_count: 0,
        unavailable_count: 0,
        parent_notification_ui_delivery_claimed: false,
        external_runtime_report_delivery_claimed: false,
        final_policy_execution_claimed: false,
        enforcement_claimed: false,
    }
}

fn row() -> SocialParentNotificationDeliveryReadinessRow {
    SocialParentNotificationDeliveryReadinessRow {
        notification_delivery_readiness_row_id:
            "social-parent-notification-ready-high-risk-service".to_string(),
        source_report_writer_delivery_row_ref: "social-report-writer-delivery-row-service"
            .to_string(),
        source_intent_ref: "social-alert-report-high-risk-service".to_string(),
        parent_visible_report_status_ref: Some(
            "social-parent-visible-report-status-high-risk-service".to_string(),
        ),
        parent_notification_ui_ref: None,
        parent_report_ref: Some("social-parent-report-high-risk-service".to_string()),
        report_artifact_ref: Some("social-report-artifact-high-risk-service".to_string()),
        report_receipt_ref: Some("social-report-receipt-high-risk-service".to_string()),
        source_evidence_refs: vec![SOCIAL_PARENT_NOTIFICATION_DELIVERY_EVIDENCE_REF.to_string()],
        source_policy_refs: vec![SOCIAL_PARENT_NOTIFICATION_DELIVERY_POLICY_REF.to_string()],
        source_audit_refs: vec![SOCIAL_PARENT_NOTIFICATION_DELIVERY_AUDIT_REF.to_string()],
        manual_proof_requirements: Vec::new(),
        notification_delivery_readiness_state:
            SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_REPORT_READY.to_string(),
        report_delivery_execution_state: "parent-owned-report-ready".to_string(),
        parent_owned_report_artifact_written: true,
        parent_owned_report_receipt_recorded: true,
        parent_notification_ui_delivered: false,
        external_runtime_report_delivery_claimed: false,
        provider_delivery_attempted: false,
        provider_receipt_ingested: false,
        final_policy_decision_claimed: false,
        enforcement_claimed: false,
        created_at: "2026-06-08T11:45:00Z".to_string(),
    }
}
