use ocentra_parent_agent_protocol::schema_domain_mirrors::{
    family::{
        FamilyReference, ParentActionReference, ParentActorReference, ParentActorRole,
        ParentDevicePlatform, ParentDeviceReference, ParentEvidenceReference,
        ParentEvidenceReferenceKind,
    },
    notification::{
        NotificationLocalOutboxAdapterProof, NotificationLocalOutboxDeliveryClaimState,
        NotificationLocalOutboxMinimalAlertEnvelope, NotificationLocalOutboxNonClaim,
        NotificationLocalOutboxRecord, NotificationLocalOutboxSeverity,
        NotificationLocalOutboxState, V3NotificationProviderChannel, V3NotificationRuleReasonCode,
    },
};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct NotificationLocalOutboxProofRow {
    entry_id: String,
    state: String,
    reason_code: String,
    provider_channel: String,
    severity: String,
    delivery_claim_state: String,
    visible_after_at: Option<String>,
    retry_attempt_count: u64,
    quiet_hours_ref: Option<String>,
    retry_policy_ref: Option<String>,
    dead_letter_ref: Option<String>,
    provider_receipt_ref: Option<String>,
    manual_proof_requirements: Vec<String>,
    manual_action_required: bool,
    provider_payload_preview: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct NotificationLocalOutboxQuietHoursWindow {
    quiet_hours_window_ref: String,
    starts_at: String,
    ends_at: String,
    hold_reason_ref: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct NotificationLocalOutboxRetryWindow {
    retry_window_ref: String,
    opens_at: String,
    closes_at: String,
    attempt_number: u64,
    max_attempts: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct NotificationLocalOutboxSchedulerProofRow {
    source_entry_id: String,
    scheduler_state: String,
    scheduler_decision_ref: String,
    next_attempt_at: Option<String>,
    quiet_hours_window: Option<NotificationLocalOutboxQuietHoursWindow>,
    retry_window: Option<NotificationLocalOutboxRetryWindow>,
    dead_letter_review_ref: Option<String>,
    provider_receipt_ref: Option<String>,
    manual_proof_requirements: Vec<String>,
    manual_action_required: bool,
    scheduler_payload_preview: String,
}

fn push_export<T: Serialize>(output: &mut String, name: &str, value: &T) {
    let json = crate::schema_result_or_unreachable(serde_json::to_string_pretty(value), name);
    let typescript = if json.trim_start().starts_with('[') {
        let compact = crate::typescript_literal::json_object_to_typescript_literal(&json);
        let compact_line = format!("export const {name} = {compact} as const;");
        if compact.lines().count() == 1 && compact_line.len() <= 120 {
            compact
        } else {
            crate::typescript_literal::json_array_to_typescript_literal(&json)
        }
    } else {
        crate::typescript_literal::json_object_to_typescript_literal(&json)
    };
    let export_line = format!("export const {name} = {typescript} as const;");
    let export_line = if export_line.len() > 120 && json.trim_start().starts_with('"') {
        format!("export const {name} =\n  {typescript} as const;")
    } else {
        export_line
    };
    output.push_str(&export_line);
    output.push_str("\n\n");
}

fn proof_family() -> FamilyReference {
    FamilyReference {
        family_id: "family-notification-local-outbox-proof-1".to_string(),
    }
}

fn proof_device() -> ParentDeviceReference {
    ParentDeviceReference {
        device_id: "windows-child-device-notification-outbox-proof-1"
            .to_string()
            .into(),
        child_profile_id: Some("child-notification-outbox-proof-1".to_string().into()),
        label: "Windows child device notification outbox proof".to_string(),
        platform: ParentDevicePlatform::Windows,
    }
}

fn proof_parent_action() -> ParentActionReference {
    ParentActionReference {
        action_reference_id: "parent-action-notification-outbox-proof-1".to_string(),
        actor: ParentActorReference {
            actor_id: "parent-notification-outbox-proof-1".to_string(),
            role: ParentActorRole::Parent,
        },
        policy_version: "notification-local-outbox-proof-v1".to_string(),
        created_at: "2026-06-04T01:31:47.023Z".to_string(),
    }
}

fn proof_evidence_ref() -> ParentEvidenceReference {
    ParentEvidenceReference {
        evidence_reference_id: "notification-local-outbox-evidence-ref-1".to_string(),
        kind: ParentEvidenceReferenceKind::PolicyDecision,
        observed_at: "2026-06-04T01:31:47.023Z".to_string(),
    }
}

fn notification_outbox_proof_rows() -> Vec<NotificationLocalOutboxProofRow> {
    vec![
        NotificationLocalOutboxProofRow {
            entry_id: "notification-local-outbox-policy-violation-push-queued".to_string(),
            state: "queued-local".to_string(),
            reason_code: "policy-violation".to_string(),
            provider_channel: "push".to_string(),
            severity: "urgent".to_string(),
            delivery_claim_state: "local-outbox-only".to_string(),
            visible_after_at: None,
            retry_attempt_count: 0,
            quiet_hours_ref: None,
            retry_policy_ref: None,
            dead_letter_ref: None,
            provider_receipt_ref: None,
            manual_proof_requirements: vec![],
            manual_action_required: false,
            provider_payload_preview: "alert id, urgent severity, policy reason, evidence ref, policy ref, parent action link".to_string(),
        },
        NotificationLocalOutboxProofRow {
            entry_id: "notification-local-outbox-sync-failure-whatsapp-deferred".to_string(),
            state: "deferred-quiet-hours".to_string(),
            reason_code: "sync-failure".to_string(),
            provider_channel: "whatsapp".to_string(),
            severity: "attention".to_string(),
            delivery_claim_state: "local-outbox-only".to_string(),
            visible_after_at: Some("2026-06-04T12:00:00.000Z".to_string()),
            retry_attempt_count: 0,
            quiet_hours_ref: Some("quiet-hours-defer-noncritical-ref".to_string()),
            retry_policy_ref: None,
            dead_letter_ref: None,
            provider_receipt_ref: None,
            manual_proof_requirements: vec![],
            manual_action_required: false,
            provider_payload_preview: "alert id, sync failure reason, parent-owned storage ref, authenticated drill-in".to_string(),
        },
        NotificationLocalOutboxProofRow {
            entry_id: "notification-local-outbox-suspicious-unknown-email-retry".to_string(),
            state: "retry-scheduled".to_string(),
            reason_code: "suspicious-unknown".to_string(),
            provider_channel: "email".to_string(),
            severity: "attention".to_string(),
            delivery_claim_state: "local-outbox-only".to_string(),
            visible_after_at: Some("2026-06-04T01:41:47.023Z".to_string()),
            retry_attempt_count: 1,
            quiet_hours_ref: None,
            retry_policy_ref: Some("retry-policy-exponential-backoff-ref".to_string()),
            dead_letter_ref: None,
            provider_receipt_ref: None,
            manual_proof_requirements: vec![],
            manual_action_required: false,
            provider_payload_preview: "alert id, suspicious unknown reason, evidence ref, retry window ref, parent action link".to_string(),
        },
        NotificationLocalOutboxProofRow {
            entry_id: "notification-local-outbox-provider-failure-sms-dead-letter".to_string(),
            state: "dead-lettered".to_string(),
            reason_code: "provider-failure".to_string(),
            provider_channel: "sms".to_string(),
            severity: "info".to_string(),
            delivery_claim_state: "local-outbox-only".to_string(),
            visible_after_at: None,
            retry_attempt_count: 3,
            quiet_hours_ref: None,
            retry_policy_ref: Some("retry-policy-dead-letter-ref".to_string()),
            dead_letter_ref: Some("dead-letter-provider-setup-required-ref".to_string()),
            provider_receipt_ref: None,
            manual_proof_requirements: vec!["provider setup review required".to_string()],
            manual_action_required: true,
            provider_payload_preview: "alert id, provider failure reason, dead letter ref, manual review link".to_string(),
        },
        NotificationLocalOutboxProofRow {
            entry_id: "notification-local-outbox-parent-request-in-app-receipt-required".to_string(),
            state: "receipt-required".to_string(),
            reason_code: "parent-request".to_string(),
            provider_channel: "in-app".to_string(),
            severity: "attention".to_string(),
            delivery_claim_state: "provider-receipt-required".to_string(),
            visible_after_at: None,
            retry_attempt_count: 0,
            quiet_hours_ref: None,
            retry_policy_ref: None,
            dead_letter_ref: None,
            provider_receipt_ref: Some("provider-receipt-required-ref".to_string()),
            manual_proof_requirements: vec![
                "real provider receipt artifact required before delivery can be claimed".to_string(),
            ],
            manual_action_required: true,
            provider_payload_preview: "alert id, parent request reason, receipt required marker, parent action link".to_string(),
        },
        NotificationLocalOutboxProofRow {
            entry_id: "notification-local-outbox-device-offline-in-app-manual".to_string(),
            state: "manual-required".to_string(),
            reason_code: "device-offline".to_string(),
            provider_channel: "in-app".to_string(),
            severity: "urgent".to_string(),
            delivery_claim_state: "manual-required".to_string(),
            visible_after_at: None,
            retry_attempt_count: 0,
            quiet_hours_ref: None,
            retry_policy_ref: None,
            dead_letter_ref: None,
            provider_receipt_ref: None,
            manual_proof_requirements: vec![
                "parent/provider preference setup required before send path can be enabled".to_string(),
            ],
            manual_action_required: true,
            provider_payload_preview: "alert id, device offline reason, manual required marker, authenticated parent link".to_string(),
        },
    ]
}

fn notification_outbox_scheduler_proof_rows() -> Vec<NotificationLocalOutboxSchedulerProofRow> {
    vec![
        NotificationLocalOutboxSchedulerProofRow {
            source_entry_id: "notification-local-outbox-policy-violation-push-queued".to_string(),
            scheduler_state: "due-local".to_string(),
            scheduler_decision_ref: "scheduler-due-policy-violation-ref".to_string(),
            next_attempt_at: Some("2026-06-04T02:28:51.667Z".to_string()),
            quiet_hours_window: None,
            retry_window: None,
            dead_letter_review_ref: None,
            provider_receipt_ref: None,
            manual_proof_requirements: vec![],
            manual_action_required: false,
            scheduler_payload_preview: "alert id, push channel, policy reason, evidence ref, scheduler due marker".to_string(),
        },
        NotificationLocalOutboxSchedulerProofRow {
            source_entry_id: "notification-local-outbox-sync-failure-whatsapp-deferred".to_string(),
            scheduler_state: "held-quiet-hours".to_string(),
            scheduler_decision_ref: "scheduler-quiet-hours-sync-failure-ref".to_string(),
            next_attempt_at: Some("2026-06-04T12:00:00.000Z".to_string()),
            quiet_hours_window: Some(NotificationLocalOutboxQuietHoursWindow {
                quiet_hours_window_ref: "quiet-hours-household-night-window-ref".to_string(),
                starts_at: "2026-06-04T02:00:00.000Z".to_string(),
                ends_at: "2026-06-04T12:00:00.000Z".to_string(),
                hold_reason_ref: "quiet-hours-noncritical-sync-failure-hold-ref".to_string(),
            }),
            retry_window: None,
            dead_letter_review_ref: None,
            provider_receipt_ref: None,
            manual_proof_requirements: vec![],
            manual_action_required: false,
            scheduler_payload_preview: "alert id, whatsapp channel, sync failure reason, quiet-hours hold, authenticated drill-in".to_string(),
        },
        NotificationLocalOutboxSchedulerProofRow {
            source_entry_id: "notification-local-outbox-suspicious-unknown-email-retry".to_string(),
            scheduler_state: "retry-window-scheduled".to_string(),
            scheduler_decision_ref: "scheduler-retry-suspicious-unknown-ref".to_string(),
            next_attempt_at: Some("2026-06-04T02:38:51.667Z".to_string()),
            quiet_hours_window: None,
            retry_window: Some(NotificationLocalOutboxRetryWindow {
                retry_window_ref: "retry-window-exponential-backoff-attempt-2-ref".to_string(),
                opens_at: "2026-06-04T02:38:51.667Z".to_string(),
                closes_at: "2026-06-04T02:43:51.667Z".to_string(),
                attempt_number: 2,
                max_attempts: 3,
            }),
            dead_letter_review_ref: None,
            provider_receipt_ref: None,
            manual_proof_requirements: vec![],
            manual_action_required: false,
            scheduler_payload_preview: "alert id, email channel, suspicious unknown reason, retry window, parent action link".to_string(),
        },
        NotificationLocalOutboxSchedulerProofRow {
            source_entry_id: "notification-local-outbox-provider-failure-sms-dead-letter".to_string(),
            scheduler_state: "dead-letter-review".to_string(),
            scheduler_decision_ref: "scheduler-dead-letter-provider-failure-ref".to_string(),
            next_attempt_at: None,
            quiet_hours_window: None,
            retry_window: None,
            dead_letter_review_ref: Some("dead-letter-provider-setup-review-ref".to_string()),
            provider_receipt_ref: None,
            manual_proof_requirements: vec![
                "provider setup review required before retry worker can be enabled".to_string(),
            ],
            manual_action_required: true,
            scheduler_payload_preview: "alert id, sms channel, provider failure reason, dead-letter review, manual link".to_string(),
        },
        NotificationLocalOutboxSchedulerProofRow {
            source_entry_id: "notification-local-outbox-parent-request-in-app-receipt-required".to_string(),
            scheduler_state: "receipt-required".to_string(),
            scheduler_decision_ref: "scheduler-receipt-required-parent-request-ref".to_string(),
            next_attempt_at: None,
            quiet_hours_window: None,
            retry_window: None,
            dead_letter_review_ref: None,
            provider_receipt_ref: Some("provider-receipt-required-ref".to_string()),
            manual_proof_requirements: vec![
                "real provider receipt artifact required before delivered state can be claimed".to_string(),
            ],
            manual_action_required: true,
            scheduler_payload_preview: "alert id, in-app channel, parent request reason, receipt required, parent action link".to_string(),
        },
        NotificationLocalOutboxSchedulerProofRow {
            source_entry_id: "notification-local-outbox-device-offline-in-app-manual".to_string(),
            scheduler_state: "manual-required".to_string(),
            scheduler_decision_ref: "scheduler-manual-required-device-offline-ref".to_string(),
            next_attempt_at: None,
            quiet_hours_window: None,
            retry_window: None,
            dead_letter_review_ref: None,
            provider_receipt_ref: None,
            manual_proof_requirements: vec![
                "parent/provider preference setup required before notification worker can be enabled".to_string(),
            ],
            manual_action_required: true,
            scheduler_payload_preview: "alert id, in-app channel, device offline reason, manual required, authenticated parent link".to_string(),
        },
    ]
}

pub fn notification_local_outbox_typescript() -> String {
    let mut output =
        String::from("/* generated from crates/schema/src/notification_local_outbox_ts.rs */\n\n");

    let _ = (
        std::mem::size_of::<NotificationLocalOutboxAdapterProof>(),
        std::mem::size_of::<NotificationLocalOutboxMinimalAlertEnvelope>(),
        std::mem::size_of::<NotificationLocalOutboxRecord>(),
        std::mem::size_of::<NotificationLocalOutboxDeliveryClaimState>(),
        std::mem::size_of::<NotificationLocalOutboxNonClaim>(),
        std::mem::size_of::<NotificationLocalOutboxState>(),
        std::mem::size_of::<NotificationLocalOutboxSeverity>(),
        std::mem::size_of::<V3NotificationProviderChannel>(),
        std::mem::size_of::<V3NotificationRuleReasonCode>(),
    );

    push_export(
        &mut output,
        "GeneratedRequiredNotificationLocalOutboxStates",
        &[
            "queued-local",
            "deferred-quiet-hours",
            "retry-scheduled",
            "dead-lettered",
            "receipt-required",
            "manual-required",
        ],
    );
    push_export(
        &mut output,
        "GeneratedRequiredNotificationLocalOutboxNonClaims",
        &[
            "no-provider-delivery",
            "no-provider-receipt-ingestion",
            "no-provider-credentials",
            "no-cloud-routing",
            "no-parent-notification-ui",
            "no-raw-child-evidence",
            "no-sensitive-provider-metadata",
        ],
    );
    push_export(
        &mut output,
        "GeneratedNotificationLocalOutboxForbiddenDetailFragments",
        &[
            "http://",
            "https://",
            "screenshot-bytes",
            "raw-title-value",
            "raw-message-body",
            "sqlite-private-path",
            "oauth-secret",
            "provider-token",
            "report-body",
        ],
    );
    push_export(
        &mut output,
        "GeneratedNotificationLocalOutboxKnownGaps",
        &[
            "No push, email, SMS, WhatsApp, or in-app provider adapter is implemented by this rust-parent-runtime proof.",
            "No provider delivery execution, webhook receipt ingestion, credentials, cloud routing, or parent notification UI is claimed.",
            "No raw child evidence, raw URLs, titles, message text, screenshots, reports, provider tokens, or private paths are stored in the local outbox artifact.",
            "Quiet-hours scheduling, retry execution, dead-letter review, and receipt/manual-required handling remain adapter/runtime work.",
            "Durable production outbox storage, retention controls, parent-visible history, and physical provider smoke proof remain future work.",
        ],
    );
    push_export(
        &mut output,
        "GeneratedNotificationLocalOutboxProviderChannels",
        &["push", "email", "sms", "whatsapp", "in-app"],
    );
    push_export(
        &mut output,
        "GeneratedNotificationLocalOutboxProofTimestamp",
        &"2026-06-04T01:31:47.023Z",
    );
    push_export(
        &mut output,
        "GeneratedNotificationLocalOutboxProofFamily",
        &proof_family(),
    );
    push_export(
        &mut output,
        "GeneratedNotificationLocalOutboxProofDevice",
        &proof_device(),
    );
    push_export(
        &mut output,
        "GeneratedNotificationLocalOutboxProofParentAction",
        &proof_parent_action(),
    );
    push_export(
        &mut output,
        "GeneratedNotificationLocalOutboxProofEvidenceRef",
        &proof_evidence_ref(),
    );
    push_export(
        &mut output,
        "GeneratedNotificationLocalOutboxProofRows",
        &notification_outbox_proof_rows(),
    );
    push_export(
        &mut output,
        "GeneratedRequiredNotificationLocalOutboxSchedulerStates",
        &[
            "due-local",
            "held-quiet-hours",
            "retry-window-scheduled",
            "dead-letter-review",
            "receipt-required",
            "manual-required",
        ],
    );
    push_export(
        &mut output,
        "GeneratedRequiredNotificationLocalOutboxSchedulerNonClaims",
        &[
            "no-provider-delivery-execution",
            "no-provider-receipt-ingestion",
            "no-provider-credentials",
            "no-cloud-routing",
            "no-parent-notification-ui",
            "no-production-durable-outbox-storage",
            "no-sensitive-detail-storage",
        ],
    );
    push_export(
        &mut output,
        "GeneratedNotificationLocalOutboxSchedulerProofTimestamp",
        &"2026-06-04T02:28:51.667Z",
    );
    push_export(
        &mut output,
        "GeneratedNotificationLocalOutboxSchedulerProofNow",
        &"2026-06-04T02:28:51.667Z",
    );
    push_export(
        &mut output,
        "GeneratedNotificationLocalOutboxSchedulerArtifactRef",
        &"parent-owned-local-notification-outbox-scheduler-jsonl-ref",
    );
    push_export(
        &mut output,
        "GeneratedNotificationLocalOutboxSchedulerKnownGaps",
        &[
            "No push, email, SMS, WhatsApp, or in-app provider adapter is implemented by this rust-parent-runtime scheduler proof.",
            "No provider delivery execution, webhook receipt ingestion, provider credentials, cloud routing, or parent notification UI is claimed.",
            "No raw child evidence, raw URLs, titles, message text, screenshots, reports, provider tokens, or private paths are stored in the scheduler artifact.",
            "Scheduler decisions are deterministic rust-parent-runtime proof rows; no production timer loop, durable outbox database, provider retry worker, or receipt webhook is implemented.",
            "Parent-visible history, preferences, escalation controls, retention controls, and physical provider smoke proof remain future work.",
        ],
    );
    push_export(
        &mut output,
        "GeneratedNotificationLocalOutboxSchedulerProofRows",
        &notification_outbox_scheduler_proof_rows(),
    );

    format!("{}\n", output.trim_end())
}
