use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::bundle::*;
use ocentra_network_evidence::cascade::{
    NetworkCascadeNextCheck, NetworkCascadeSignalStrength, NetworkCascadeSourceKind,
};
use ocentra_network_evidence::dns::types::NetworkEvidenceGrade;
use ocentra_network_evidence::screen_summary::*;

#[derive(Clone, Copy)]
struct TriggerRef(&'static str);
#[derive(Clone, Copy)]
struct EvidenceRef(&'static str);

#[test]
fn screen_summary_trigger_queues_when_recommended_enabled_and_custody_ready() {
    let plan = plan_network_screen_summary_trigger(input(weak_network_bundle()))
        .expect_value("recommended screen summary should queue with custody refs");
    let job = plan
        .job
        .as_ref()
        .expect_value("queued plan should include job");

    assert_eq!(plan.status, NetworkScreenSummaryTriggerStatus::Queued);
    assert_eq!(
        plan.privacy_mode,
        NetworkScreenSummaryPrivacyMode::ActiveWindowScreenIfEnabled
    );
    assert!(plan.screen_summary_recommended);
    assert_eq!(job.queue_job_ref, "screen-job-1");
    assert_eq!(job.parent_setting_ref, "screen-parent-setting-1");
    assert_eq!(job.retention_policy_ref, "screen-retention-1");
    assert_eq!(job.source_evidence_refs, vec!["transfer-hint-1"]);
    assert!(job.encrypted_temporary_custody_required);
    assert!(job.delete_after_analysis_required);
    assert_no_capture_or_authority_claims(&plan);
}

#[test]
fn screen_summary_trigger_skips_when_cascade_does_not_recommend_screen() {
    let plan = plan_network_screen_summary_trigger(input(confirmed_domain_bundle()))
        .expect_value("confirmed source should not queue screen summary");

    assert_eq!(
        plan.status,
        NetworkScreenSummaryTriggerStatus::NotRecommended
    );
    assert_eq!(
        plan.privacy_mode,
        NetworkScreenSummaryPrivacyMode::NetworkOnly
    );
    assert!(!plan.screen_summary_recommended);
    assert!(plan.job.is_none());
    assert_no_capture_or_authority_claims(&plan);
}

#[test]
fn screen_summary_trigger_preserves_disabled_unavailable_protected_and_debounced_states() {
    let disabled = plan_network_screen_summary_trigger(NetworkScreenSummaryTriggerInput {
        screen_summary_enabled: false,
        ..input(weak_network_bundle())
    })
    .expect_value("disabled state should be explicit");
    assert_eq!(
        disabled.status,
        NetworkScreenSummaryTriggerStatus::DisabledByParent
    );
    assert!(disabled.job.is_none());

    let queue_unavailable = plan_network_screen_summary_trigger(NetworkScreenSummaryTriggerInput {
        queue_available: false,
        ..input(weak_network_bundle())
    })
    .expect_value("queue unavailable should be explicit");
    assert_eq!(
        queue_unavailable.status,
        NetworkScreenSummaryTriggerStatus::QueueUnavailable
    );

    let protected = plan_network_screen_summary_trigger(NetworkScreenSummaryTriggerInput {
        protected_surface_detected: true,
        ..input(weak_network_bundle())
    })
    .expect_value("protected surface should not queue");
    assert_eq!(
        protected.status,
        NetworkScreenSummaryTriggerStatus::ProtectedSurfaceUnavailable
    );

    let debounced = plan_network_screen_summary_trigger(NetworkScreenSummaryTriggerInput {
        debounce_clear: false,
        ..input(weak_network_bundle())
    })
    .expect_value("debounced trigger should not queue");
    assert_eq!(
        debounced.status,
        NetworkScreenSummaryTriggerStatus::Debounced
    );
}

#[test]
fn screen_summary_trigger_requires_local_custody_deletion_and_runtime() {
    let custody = plan_network_screen_summary_trigger(NetworkScreenSummaryTriggerInput {
        encrypted_temporary_custody_available: false,
        ..input(weak_network_bundle())
    })
    .expect_value("missing custody should be manual-required");
    assert_eq!(
        custody.status,
        NetworkScreenSummaryTriggerStatus::CustodyManualRequired
    );

    let deletion = plan_network_screen_summary_trigger(NetworkScreenSummaryTriggerInput {
        delete_after_analysis_available: false,
        ..input(weak_network_bundle())
    })
    .expect_value("missing deletion policy should be manual-required");
    assert_eq!(
        deletion.status,
        NetworkScreenSummaryTriggerStatus::CustodyManualRequired
    );

    let runtime = plan_network_screen_summary_trigger(NetworkScreenSummaryTriggerInput {
        local_only_runtime_available: false,
        ..input(weak_network_bundle())
    })
    .expect_value("missing local runtime should be manual-required");
    assert_eq!(
        runtime.status,
        NetworkScreenSummaryTriggerStatus::CustodyManualRequired
    );
}

#[test]
fn screen_summary_trigger_rejects_raw_remote_content_and_authority_claims() {
    assert_eq!(
        plan_network_screen_summary_trigger(NetworkScreenSummaryTriggerInput {
            raw_image_retention_requested: true,
            ..input(weak_network_bundle())
        }),
        Err(NetworkScreenSummaryTriggerError::RawImageRetentionRejected)
    );
    assert_eq!(
        plan_network_screen_summary_trigger(NetworkScreenSummaryTriggerInput {
            remote_upload_requested: true,
            ..input(weak_network_bundle())
        }),
        Err(NetworkScreenSummaryTriggerError::RemoteUploadRejected)
    );
    assert_eq!(
        plan_network_screen_summary_trigger(NetworkScreenSummaryTriggerInput {
            screen_content_available: true,
            ..input(weak_network_bundle())
        }),
        Err(NetworkScreenSummaryTriggerError::ScreenContentRejected)
    );
    assert_eq!(
        plan_network_screen_summary_trigger(NetworkScreenSummaryTriggerInput {
            policy_action_authority: true,
            ..input(weak_network_bundle())
        }),
        Err(NetworkScreenSummaryTriggerError::PolicyAuthorityRejected)
    );
    assert_eq!(
        plan_network_screen_summary_trigger(NetworkScreenSummaryTriggerInput {
            enforcement_command_published: true,
            ..input(weak_network_bundle())
        }),
        Err(NetworkScreenSummaryTriggerError::EnforcementCommandRejected)
    );
}

#[test]
fn screen_summary_trigger_rejects_empty_queue_and_setting_refs() {
    assert_eq!(
        plan_network_screen_summary_trigger(NetworkScreenSummaryTriggerInput {
            queue_job_ref: " ".to_owned(),
            ..input(weak_network_bundle())
        }),
        Err(NetworkScreenSummaryTriggerError::EmptyQueueJobRef)
    );
    assert_eq!(
        plan_network_screen_summary_trigger(NetworkScreenSummaryTriggerInput {
            screen_queue_ref: " ".to_owned(),
            ..input(weak_network_bundle())
        }),
        Err(NetworkScreenSummaryTriggerError::EmptyScreenQueueRef)
    );
    assert_eq!(
        plan_network_screen_summary_trigger(NetworkScreenSummaryTriggerInput {
            parent_setting_ref: " ".to_owned(),
            ..input(weak_network_bundle())
        }),
        Err(NetworkScreenSummaryTriggerError::EmptyParentSettingRef)
    );
}

fn input(bundle: NetworkCrossSliceEvidenceBundle) -> NetworkScreenSummaryTriggerInput {
    NetworkScreenSummaryTriggerInput {
        queue_job_ref: " screen-job-1 ".to_owned(),
        screen_queue_ref: "screen-queue-1".to_owned(),
        parent_setting_ref: "screen-parent-setting-1".to_owned(),
        retention_policy_ref: "screen-retention-1".to_owned(),
        bundle,
        screen_summary_enabled: true,
        queue_available: true,
        encrypted_temporary_custody_available: true,
        delete_after_analysis_available: true,
        local_only_runtime_available: true,
        protected_surface_detected: false,
        debounce_clear: true,
        raw_image_retention_requested: false,
        remote_upload_requested: false,
        screen_content_available: false,
        policy_action_authority: false,
        adapter_action_authority: false,
        enforcement_command_published: false,
    }
}

fn weak_network_bundle() -> NetworkCrossSliceEvidenceBundle {
    bundle(
        TriggerRef("network-trigger-screen"),
        NetworkCascadeSourceKind::TransferCandidate,
        NetworkCascadeSignalStrength::WeakHint,
        EvidenceRef("transfer-hint-1"),
    )
}

fn confirmed_domain_bundle() -> NetworkCrossSliceEvidenceBundle {
    bundle(
        TriggerRef("network-trigger-confirmed"),
        NetworkCascadeSourceKind::DomainCategory,
        NetworkCascadeSignalStrength::Confirmed,
        EvidenceRef("domain-category-1"),
    )
}

fn bundle(
    trigger_ref: TriggerRef,
    source_kind: NetworkCascadeSourceKind,
    signal_strength: NetworkCascadeSignalStrength,
    evidence_ref: EvidenceRef,
) -> NetworkCrossSliceEvidenceBundle {
    let bundle = build_network_cross_slice_evidence_bundle(NetworkCrossSliceEvidenceBundleInput {
        trigger_ref: trigger_ref.0.to_owned(),
        sources: vec![NetworkCrossSliceEvidenceSource {
            source_kind,
            signal_strength,
            evidence_grade: NetworkEvidenceGrade::D,
            evidence_ref: evidence_ref.0.to_owned(),
            exact_url_available: false,
            decrypted_payload_available: false,
            policy_action_authority: false,
            adapter_action_authority: false,
        }],
    })
    .expect_value("test bundle should be valid");
    if signal_strength == NetworkCascadeSignalStrength::WeakHint {
        assert!(bundle
            .next_checks
            .contains(&NetworkCascadeNextCheck::ScreenSummary));
    }
    bundle
}

fn assert_no_capture_or_authority_claims(plan: &NetworkScreenSummaryTriggerPlan) {
    assert!(!plan.capture_executed);
    assert!(!plan.raw_image_available);
    assert!(!plan.raw_image_retained);
    assert!(!plan.remote_upload_authorized);
    assert!(!plan.policy_action_authority);
    assert!(!plan.adapter_action_authority);
    assert!(!plan.enforcement_command_published);
}
