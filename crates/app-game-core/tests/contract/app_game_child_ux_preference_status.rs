use ocentra_app_game_core::{
    app_game_child_ux_preference_preflight_types::{
        AppGameChildUxPreferencePreflightRow, AppGameChildUxPreferencePreflightStatus,
    },
    app_game_child_ux_preference_status::build_app_game_child_ux_preference_status_handoff,
    app_game_child_ux_preference_status_types::AppGameChildUxPreferenceStatusInput,
};
use ocentra_parent_agent_protocol::{
    activity::policy::{ParentEvidenceReference, ParentEvidenceReferenceKind},
    schema_domain_mirrors::notification::{
        NotificationLocalOutboxSchedulerState, NotificationLocalOutboxSeverity,
        V3NotificationProviderChannel, V3NotificationRuleReasonCode,
    },
};

#[test]
fn preference_status_maps_manual_and_unavailable_rows_without_delivery_claims(
) -> Result<(), Box<dyn std::error::Error>> {
    let manual = build_app_game_child_ux_preference_status_handoff(status_input(preflight(
        AppGameChildUxPreferencePreflightStatus::ParentPreferenceRequired,
    )))?;
    let manual_entry = &manual
        .preference_status_handoff_row
        .notification_preference_status_entry;
    assert_eq!(manual_entry.delivery_result_state, "manual-required");
    assert_eq!(
        manual_entry.parent_preference_state,
        "manual-setup-required"
    );
    assert_eq!(manual_entry.quiet_hours_decision, "manual-required");
    assert_eq!(manual_entry.provider_channel, "in-app");
    assert_eq!(manual.evidence_refs.len(), 1);
    assert_eq!(manual.manual_proof_requirements.len(), 3);
    assert!(!manual.parent_preference_mutation_runtime_claimed);
    assert!(!manual.quiet_hours_timer_runtime_claimed);
    assert!(!manual.retry_execution_runtime_claimed);
    assert!(!manual.provider_delivery_runtime_claimed);

    let unavailable = build_app_game_child_ux_preference_status_handoff(status_input(preflight(
        AppGameChildUxPreferencePreflightStatus::Unavailable,
    )))?;
    let unavailable_entry = &unavailable
        .preference_status_handoff_row
        .notification_preference_status_entry;
    assert_eq!(unavailable_entry.delivery_result_state, "not-sent");
    assert_eq!(
        unavailable_entry.parent_preference_state,
        "channel-disabled"
    );
    assert_eq!(unavailable_entry.quiet_hours_decision, "allow");
    assert_eq!(unavailable_entry.provider_channel, "in-app");
    assert_eq!(
        unavailable
            .preference_status_handoff_row
            .source_scheduler_entry_ref
            .as_deref(),
        Some("scheduler-entry-1")
    );
    assert_eq!(
        unavailable
            .preference_status_handoff_row
            .source_outbox_record_ref,
        None
    );
    assert!(!unavailable.child_delivery_claimed);
    Ok(())
}

#[test]
fn preference_status_rejects_claimed_malformed_and_duplicate_context(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut claimed = preflight(AppGameChildUxPreferencePreflightStatus::ManualRequired);
    claimed.parent_preference_mutation_runtime_claimed = true;
    assert_eq!(
        build_app_game_child_ux_preference_status_handoff(status_input(claimed)),
        Err(ocentra_eventing::error::EventingError::InvalidValue {
            field: "app_game.child_ux_preference_status.source",
            value: "preference-preflight-row-1".to_string(),
        })
    );

    let mut missing_channel = preflight(AppGameChildUxPreferencePreflightStatus::Unavailable);
    missing_channel.provider_channel = None;
    assert_eq!(
        build_app_game_child_ux_preference_status_handoff(status_input(missing_channel)),
        Err(ocentra_eventing::error::EventingError::InvalidValue {
            field: "app_game.child_ux_preference_status.source",
            value: "preference-preflight-row-1".to_string(),
        })
    );

    let mut duplicate = status_input(preflight(
        AppGameChildUxPreferencePreflightStatus::ParentPreferenceRequired,
    ));
    duplicate.retry_policy_ref = duplicate.notification_rule_ref.clone();
    assert_eq!(
        build_app_game_child_ux_preference_status_handoff(duplicate),
        Err(ocentra_eventing::error::EventingError::InvalidValue {
            field: "app_game.child_ux_preference_status.context",
            value: "preference-status-handoff-row-1".to_string(),
        })
    );
    Ok(())
}

fn status_input(
    preflight_row: AppGameChildUxPreferencePreflightRow,
) -> AppGameChildUxPreferenceStatusInput {
    AppGameChildUxPreferenceStatusInput {
        preflight_row,
        handoff_row_id: "preference-status-handoff-row-1".into(),
        notification_rule_ref: "notification-rule-1".into(),
        notification_intent_ref: "notification-intent-1".into(),
        delivery_attempt_ref: "delivery-attempt-not-executed-1".into(),
        delivery_result_ref: "delivery-result-not-sent-1".into(),
        retry_policy_ref: "retry-policy-manual-review-1".into(),
        quiet_hours_policy_ref: "quiet-hours-policy-manual-1".into(),
        escalation_policy_ref: "escalation-policy-manual-1".into(),
        parent_preference_ref: "parent-preference-required-1".into(),
        last_checked_at: "2026-08-15T00:03:00Z".to_string(),
    }
}

fn preflight(
    status: AppGameChildUxPreferencePreflightStatus,
) -> AppGameChildUxPreferencePreflightRow {
    let ready = status == AppGameChildUxPreferencePreflightStatus::ParentPreferenceRequired;
    AppGameChildUxPreferencePreflightRow {
        preflight_row_id: "preference-preflight-row-1".into(),
        source_scheduler_entry_id: "scheduler-entry-1".into(),
        source_scheduler_state: if ready {
            NotificationLocalOutboxSchedulerState::DueLocal
        } else if status == AppGameChildUxPreferencePreflightStatus::Unavailable {
            NotificationLocalOutboxSchedulerState::DeadLetterReview
        } else {
            NotificationLocalOutboxSchedulerState::ManualRequired
        },
        status,
        source_local_outbox_record_ref: ready.then(|| "entry-1".into()),
        source_outbox_file_ref: ready.then(|| "outbox-file-1".into()),
        local_data_path_ref: ready.then(|| "local-data-path-1".into()),
        scheduler_decision_ref: "scheduler-decision-1".into(),
        scheduler_artifact_ref: "scheduler-artifact-1".into(),
        provider_channel: Some(V3NotificationProviderChannel::InApp),
        reason_code: Some(V3NotificationRuleReasonCode::ParentRequest),
        severity: Some(NotificationLocalOutboxSeverity::Attention),
        evidence_refs: vec![ParentEvidenceReference {
            evidence_reference_id: "evidence-1".to_string(),
            kind: ParentEvidenceReferenceKind::ActivityEvent,
            observed_at: "2026-08-15T00:00:00Z".to_string(),
        }],
        policy_refs: vec!["policy-1".into()],
        audit_refs: vec!["audit-1".into()],
        parent_preference_requirement_refs: ready
            .then(|| "parent-preference-required-1".into())
            .into_iter()
            .collect(),
        notification_frequency_requirement_refs: ready
            .then(|| "notification-frequency-required-1".into())
            .into_iter()
            .collect(),
        quiet_hours_requirement_refs: ready
            .then(|| "quiet-hours-required-1".into())
            .into_iter()
            .collect(),
        manual_proof_requirements: vec![
            "parent-preference-required-1".into(),
            "notification-frequency-required-1".into(),
            "quiet-hours-required-1".into(),
        ],
        parent_preference_mutation_runtime_claimed: false,
        parent_frequency_control_ui_claimed: false,
        quiet_hours_timer_runtime_claimed: false,
        provider_delivery_runtime_claimed: false,
        provider_receipt_ingestion_claimed: false,
        provider_credentials_claimed: false,
        cloud_routing_claimed: false,
        parent_notification_ui_claimed: false,
        child_delivery_claimed: false,
        adapter_dispatch_claimed: false,
        platform_enforcement_claimed: false,
    }
}
