use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::schema_domain_mirrors::{
    family::{
        ChildProfileReference, FamilyReference, ParentActionReference, ParentActorReference,
        ParentActorRole, ParentDevicePlatform, ParentDeviceReference, ParentEvidenceReference,
        ParentEvidenceReferenceKind,
    },
    notification::{
        NotificationLocalOutboxAdapterProof, NotificationLocalOutboxAdapterProofSchemaVersion,
        NotificationLocalOutboxDeliveryClaimState, NotificationLocalOutboxMinimalAlertEnvelope,
        NotificationLocalOutboxNonClaim, NotificationLocalOutboxRecord,
        NotificationLocalOutboxSchedulerRecord, NotificationLocalOutboxSchedulerState,
        NotificationLocalOutboxSeverity, NotificationLocalOutboxState,
        V3NotificationProviderChannel, V3NotificationRuleReasonCode,
    },
    policy::{
        FamilyPolicySet, PolicyAction, PolicyDecision, PolicyDecisionHandoffState, PolicyPreview,
        PolicyPreviewBudgetBoundaryState, PolicyPreviewConfirmationState, PolicyPreviewOrigin,
        PolicyScheduleBoundary, PolicyScheduleBoundaryState, PolicyScheduleClockSource,
        PolicyScheduleDstBoundary, PolicyScheduleDstResolution, PolicyScheduleDstTransition,
        PolicyScheduleOfflineRecoveryState, PolicyScheduleOfflineRecoveryStatus,
        PolicyScheduleTimeBudgetStatus, PolicyTarget, PolicyTargetType,
    },
};

fn to_json<T: serde::Serialize>(value: T) -> serde_json::Value {
    serde_json::to_value(value).expect_value("schema domain mirror fixture must serialize")
}

#[test]
fn schema_domain_mirrors_family_policy_set_serializes_to_typescript_shape() {
    let value = to_json(FamilyPolicySet {
        schema_version: "v0.6".to_string().into(),
        family: FamilyReference {
            family_id: "family-1".to_string(),
        },
        child_profiles: vec![ChildProfileReference {
            child_profile_id: "child-1".to_string(),
            display_name: "Child One".to_string(),
        }],
        devices: vec![ParentDeviceReference {
            device_id: "device-1".to_string().into(),
            child_profile_id: Some("child-1".to_string().into()),
            label: "Parent Laptop".to_string(),
            platform: ParentDevicePlatform::Windows,
        }],
        policy_version: "policy-v1".to_string().into(),
        rules: vec![],
        schedules: vec![],
    });

    assert_eq!(value["schemaVersion"], "v0.6");
    assert_eq!(value["family"]["familyId"], "family-1");
    assert_eq!(value["childProfiles"][0]["childProfileId"], "child-1");
    assert_eq!(value["childProfiles"][0]["displayName"], "Child One");
    assert!(value["childProfiles"][0].get("familyId").is_none());
    assert_eq!(value["devices"][0]["platform"], "windows");
    assert_eq!(value["policyVersion"], "policy-v1");
}
#[test]
fn schema_domain_mirrors_parent_references_use_current_typescript_literals() {
    let evidence = to_json(ParentEvidenceReference {
        evidence_reference_id: "evidence-1".to_string(),
        kind: ParentEvidenceReferenceKind::QueryStoreSummary,
        observed_at: "2026-06-20T18:00:00.000Z".to_string(),
    });

    let action = to_json(ParentActionReference {
        action_reference_id: "action-1".to_string(),
        actor: ParentActorReference {
            actor_id: "actor-1".to_string(),
            role: ParentActorRole::Guardian,
        },
        policy_version: "policy-v1".to_string(),
        created_at: "2026-06-20T18:01:00.000Z".to_string(),
    });

    assert_eq!(evidence["evidenceReferenceId"], "evidence-1");
    assert_eq!(evidence["kind"], "query-store-summary");
    assert_eq!(evidence["observedAt"], "2026-06-20T18:00:00.000Z");
    assert_eq!(action["actor"]["role"], "guardian");
    assert_eq!(action["policyVersion"], "policy-v1");
}

#[test]
fn schema_domain_mirrors_notification_outbox_proof_uses_current_typescript_literals() {
    let value = to_json(notification_outbox_adapter_proof_fixture());

    assert_eq!(
        value["schemaVersion"],
        "notification-local-outbox-adapter-proof"
    );
    assert_eq!(value["records"][0]["state"], "queued-local");
    assert_eq!(
        value["records"][0]["deliveryClaimState"],
        "local-outbox-only"
    );
    assert_eq!(
        value["records"][0]["envelope"]["alertRef"],
        "notification-alert-ref-1"
    );
    assert_eq!(value["records"][0]["envelope"]["providerChannel"], "push");
    assert_eq!(
        value["records"][0]["envelope"]["reasonCode"],
        "policy-violation"
    );
    assert_eq!(value["records"].as_array().map(|rows| rows.len()), Some(1));
    assert_eq!(
        value["records"][0]["visibleAfterAt"],
        serde_json::Value::Null
    );
    assert!(value["records"][0]["manualActionRequired"]
        .as_bool()
        .is_some_and(|manual_action_required| !manual_action_required));
    assert_eq!(
        value["nonClaims"],
        serde_json::json!([
            "no-provider-delivery",
            "no-provider-receipt-ingestion",
            "no-provider-credentials",
            "no-cloud-routing",
            "no-parent-notification-ui",
            "no-raw-child-evidence",
            "no-sensitive-provider-metadata",
        ])
    );
}

#[test]
fn schema_domain_mirrors_notification_scheduler_record_uses_shared_shape() {
    let source = notification_outbox_record_fixture();
    let value = to_json(NotificationLocalOutboxSchedulerRecord {
        scheduler_entry_id: "scheduler-entry-1".into(),
        source_entry_id: source.entry_id,
        source_state: source.state,
        scheduler_state: NotificationLocalOutboxSchedulerState::DueLocal,
        reason_code: source.envelope.reason_code,
        provider_channel: source.envelope.provider_channel,
        severity: source.envelope.severity,
        scheduler_decision_ref: "scheduler-decision-1".into(),
        scheduler_artifact_ref: "scheduler-artifact-1".into(),
        source_outbox_file_ref: source.outbox_file_ref,
        local_data_path_ref: source.local_data_path_ref,
        scheduler_now_at: "2026-08-15T00:01:00Z".into(),
        next_attempt_at: Some("2026-08-15T00:01:00Z".into()),
        quiet_hours_window: None,
        retry_window: None,
        dead_letter_review_ref: None,
        provider_receipt_ref: None,
        manual_proof_requirements: Vec::new(),
        manual_action_required: false,
        parent_owned_artifact_written: false,
        raw_child_evidence_included: false,
        raw_url_or_title_included: false,
        raw_message_text_included: false,
        screenshot_or_report_included: false,
        provider_delivery_attempted: false,
        provider_delivery_observed: false,
        provider_receipt_ingested: false,
        provider_credentials_stored: false,
        cloud_routing_claimed: false,
        parent_notification_ui_claimed: false,
        production_durable_outbox_storage_claimed: false,
        sensitive_provider_metadata_stored: false,
        scheduler_payload_preview: "controlled scheduler preview".into(),
    });

    assert_eq!(value["schedulerEntryId"], "scheduler-entry-1");
    assert_eq!(value["sourceState"], "queued-local");
    assert_eq!(value["schedulerState"], "due-local");
    assert_eq!(value["reasonCode"], "policy-violation");
    assert_eq!(value["providerChannel"], "push");
    assert_eq!(value["nextAttemptAt"], "2026-08-15T00:01:00Z");
    assert_eq!(value["parentOwnedArtifactWritten"], false);
    assert_eq!(value["providerDeliveryAttempted"], false);
    assert_eq!(value["productionDurableOutboxStorageClaimed"], false);
}

fn notification_outbox_adapter_proof_fixture() -> NotificationLocalOutboxAdapterProof {
    NotificationLocalOutboxAdapterProof {
        schema_version:
            NotificationLocalOutboxAdapterProofSchemaVersion::NotificationLocalOutboxAdapterProof,
        contract_version: "v0.6".to_string().into(),
        read_model_id: "notification-local-outbox-adapter-proof".to_string().into(),
        generated_at: "2026-06-21T09:46:00.000Z".to_string().into(),
        outbox_root_ref: "notification-outbox-root-ref-1".to_string().into(),
        records: vec![notification_outbox_record_fixture()],
        non_claims: vec![
            NotificationLocalOutboxNonClaim::NoProviderDelivery,
            NotificationLocalOutboxNonClaim::NoProviderReceiptIngestion,
            NotificationLocalOutboxNonClaim::NoProviderCredentials,
            NotificationLocalOutboxNonClaim::NoCloudRouting,
            NotificationLocalOutboxNonClaim::NoParentNotificationUi,
            NotificationLocalOutboxNonClaim::NoRawChildEvidence,
            NotificationLocalOutboxNonClaim::NoSensitiveProviderMetadata,
        ],
        provider_delivery_runtime_claimed: false,
        provider_receipt_ingestion_claimed: false,
        provider_credentials_claimed: false,
        cloud_routing_claimed: false,
        parent_notification_ui_claimed: false,
    }
}

fn notification_outbox_record_fixture() -> NotificationLocalOutboxRecord {
    NotificationLocalOutboxRecord {
        entry_id: "notification-outbox-entry-1".to_string().into(),
        state: NotificationLocalOutboxState::QueuedLocal,
        envelope: notification_outbox_envelope_fixture(),
        outbox_file_ref: "outbox-file-ref-1".to_string().into(),
        local_data_path_ref: "local-data-path-ref-1".to_string().into(),
        delivery_claim_state: NotificationLocalOutboxDeliveryClaimState::LocalOutboxOnly,
        visible_after_at: None,
        retry_attempt_count: 0,
        quiet_hours_ref: None,
        retry_policy_ref: None,
        dead_letter_ref: None,
        provider_receipt_ref: None,
        manual_proof_requirements: vec![],
        manual_action_required: false,
        provider_delivery_attempted: false,
        provider_delivery_observed: false,
        provider_receipt_ingested: false,
        provider_credentials_stored: false,
        cloud_routing_claimed: false,
        parent_notification_ui_claimed: false,
        sensitive_provider_metadata_stored: false,
    }
}

fn notification_outbox_envelope_fixture() -> NotificationLocalOutboxMinimalAlertEnvelope {
    NotificationLocalOutboxMinimalAlertEnvelope {
        alert_ref: "notification-alert-ref-1".to_string().into(),
        family: FamilyReference {
            family_id: "family-1".to_string(),
        },
        device: ParentDeviceReference {
            device_id: "device-1".to_string().into(),
            child_profile_id: Some("child-1".to_string().into()),
            label: "Parent Laptop".to_string(),
            platform: ParentDevicePlatform::Windows,
        },
        parent_action: ParentActionReference {
            action_reference_id: "action-1".to_string(),
            actor: ParentActorReference {
                actor_id: "actor-1".to_string(),
                role: ParentActorRole::Parent,
            },
            policy_version: "policy-v1".to_string(),
            created_at: "2026-06-21T09:45:00.000Z".to_string(),
        },
        severity: NotificationLocalOutboxSeverity::Urgent,
        reason_code: V3NotificationRuleReasonCode::PolicyViolation,
        provider_channel: V3NotificationProviderChannel::Push,
        evidence_refs: vec![ParentEvidenceReference {
            evidence_reference_id: "evidence-1".to_string(),
            kind: ParentEvidenceReferenceKind::PolicyDecision,
            observed_at: "2026-06-21T09:44:00.000Z".to_string(),
        }],
        policy_refs: vec!["policy-ref-1".to_string().into()],
        audit_refs: vec!["audit-ref-1".to_string().into()],
        payload_template_ref: "payload-template-ref-1".to_string().into(),
        provider_payload_preview: "minimal payload preview".to_string().into(),
        sensitive_detail_minimized: true,
        raw_child_evidence_included: false,
        raw_url_or_title_included: false,
        raw_message_text_included: false,
        screenshot_or_report_included: false,
    }
}

#[test]
fn schema_domain_mirrors_policy_preview_boundary_uses_current_typescript_shape() {
    let preview = to_json(policy_preview_fixture());
    let budget_boundary_state = to_json(PolicyPreviewBudgetBoundaryState::BonusTimeExpiring);

    assert_eq!(preview["origin"], "assistant-preview");
    assert_eq!(preview["confirmationState"], "confirmed");
    assert_eq!(preview["requestedAction"], "time-limit");
    assert_eq!(preview["scheduleBoundary"]["state"], "dst-gap");
    assert_eq!(
        preview["scheduleBoundary"]["dstBoundary"]["transition"],
        "spring-forward"
    );
    assert_eq!(
        preview["scheduleBoundary"]["dstBoundary"]["resolution"],
        "manual-required"
    );
    assert_eq!(
        preview["scheduleBoundary"]["timeBudget"]["clockSource"],
        "trusted-service"
    );
    assert_eq!(
        preview["scheduleBoundary"]["timeBudget"]["offlineRecovery"]["state"],
        "recomputed-from-journal"
    );
    assert_eq!(preview["decision"]["enforcementHandoffState"], "disabled");
    assert_eq!(
        budget_boundary_state,
        serde_json::json!("bonus-time-expiring")
    );
}

fn policy_preview_fixture() -> PolicyPreview {
    PolicyPreview {
        preview_id: "preview-1".to_string().into(),
        origin: PolicyPreviewOrigin::AssistantPreview,
        confirmation_state: PolicyPreviewConfirmationState::Confirmed,
        confirmed_by: Some(ParentActorReference {
            actor_id: "actor-1".to_string(),
            role: ParentActorRole::Parent,
        }),
        confirmed_at: Some("2026-06-21T10:05:00.000Z".to_string().into()),
        target: PolicyTarget {
            target_id: "target-1".to_string(),
            target_type: PolicyTargetType::Domain,
            target_value: "example.test".to_string(),
        },
        requested_action: PolicyAction::TimeLimit,
        schedule_boundary: Some(policy_schedule_boundary_fixture()),
        decision: policy_decision_fixture(),
    }
}

fn policy_schedule_boundary_fixture() -> PolicyScheduleBoundary {
    PolicyScheduleBoundary {
        schedule_id: "schedule-1".to_string().into(),
        time_zone: "America/Toronto".to_string().into(),
        evaluated_at: "2026-06-21T10:00:00.000Z".to_string().into(),
        local_time: "06:00".to_string().into(),
        state: PolicyScheduleBoundaryState::DstGap,
        dst_boundary: Some(PolicyScheduleDstBoundary {
            transition: PolicyScheduleDstTransition::SpringForward,
            local_time: "02:00".to_string().into(),
            offset_before_minutes: -300.0,
            offset_after_minutes: -240.0,
            resolution: PolicyScheduleDstResolution::ManualRequired,
        }),
        clock_skew: None,
        exception: None,
        expiry: None,
        time_budget: Some(policy_schedule_time_budget_fixture()),
    }
}

fn policy_schedule_time_budget_fixture() -> PolicyScheduleTimeBudgetStatus {
    PolicyScheduleTimeBudgetStatus {
        budget_window_minutes: 60.0,
        used_minutes: 15.0,
        remaining_minutes: 45.0,
        carryover_minutes: 10.0,
        grace_period_minutes: 5.0,
        reset_at: "2026-06-22T00:00:00.000Z".to_string().into(),
        clock_source: PolicyScheduleClockSource::TrustedService,
        offline_recovery: PolicyScheduleOfflineRecoveryStatus {
            state: PolicyScheduleOfflineRecoveryState::RecomputedFromJournal,
            recovered_at: Some("2026-06-21T09:58:00.000Z".to_string().into()),
            recovered_offline_minutes: 12.0,
        },
        bonus_time_minutes: Some(20.0),
        bonus_time_remaining_minutes: Some(5.0),
        bonus_time_expires_at: Some("2026-06-21T10:20:00.000Z".to_string().into()),
    }
}

fn policy_decision_fixture() -> PolicyDecision {
    PolicyDecision {
        schema_version: "v0.6".to_string(),
        decision_id: "decision-1".to_string(),
        action: PolicyAction::Warn,
        reason_codes: vec!["reason-1".to_string()],
        evidence_references: vec![ParentEvidenceReference {
            evidence_reference_id: "evidence-1".to_string(),
            kind: ParentEvidenceReferenceKind::PolicyDecision,
            observed_at: "2026-06-21T09:55:00.000Z".to_string(),
        }],
        rule_ids: vec!["rule-1".to_string()],
        local_ai_result_id: Some("local-ai-result-1".to_string()),
        dry_run: true,
        enforcement_handoff_state: PolicyDecisionHandoffState::Disabled,
        expires_at: Some("2026-06-21T11:00:00.000Z".to_string()),
    }
}
