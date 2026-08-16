use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::bundle::*;
use ocentra_network_evidence::cascade::*;
use ocentra_network_evidence::dns::types::*;
use ocentra_network_evidence::local_ai_queue::*;

#[derive(Clone, Copy)]
struct TriggerRef(&'static str);
#[derive(Clone, Copy)]
struct EvidenceRef(&'static str);

#[test]
fn local_ai_queue_enqueues_weak_network_bundle_with_refs_only() {
    let plan = plan_network_local_ai_queue(queue_input(weak_transfer_bundle()))
        .expect_value("weak transfer bundle should queue local AI review");
    let job = plan.job.expect_value("queued plan should contain a job");

    assert_eq!(plan.status, NetworkLocalAiQueueStatus::Queued);
    assert!(plan.local_ai_review_recommended);
    assert_eq!(plan.trigger_ref, "network-trigger-weak");
    assert_eq!(job.queue_job_ref, "local-ai-job-1");
    assert_eq!(job.queue_ref, "local-ai-queue-1");
    assert_eq!(job.model_runtime_ref, "local-model-runtime-1");
    assert_eq!(job.evidence_refs, vec!["transfer-hint-1"]);
    assert_eq!(
        job.summary_refs,
        vec![
            "network-summary-1".to_owned(),
            "screen-summary-ref-1".to_owned()
        ]
    );
    assert_eq!(
        job.input_kinds,
        vec![
            NetworkLocalAiQueueInputKind::EvidenceRefs,
            NetworkLocalAiQueueInputKind::SummaryRefs,
        ]
    );
    assert!(!job.raw_network_payload_available);
    assert!(!job.page_content_available);
    assert!(!job.decrypted_payload_available);
    assert!(!job.policy_action_authority);
    assert!(!job.adapter_action_authority);
    assert!(!plan.policy_action_authority);
    assert!(!plan.adapter_action_authority);
}

#[test]
fn local_ai_queue_keeps_managed_browser_exact_url_as_evidence_ref_only() {
    let plan = plan_network_local_ai_queue(queue_input(weak_managed_browser_bundle()))
        .expect_value("weak managed browser evidence should queue local AI review");
    let job = plan.job.expect_value("queued plan should contain a job");

    assert_eq!(plan.status, NetworkLocalAiQueueStatus::Queued);
    assert_eq!(job.exact_url_evidence_refs, vec!["managed-browser-url-ref"]);
    assert_eq!(
        job.summary_refs,
        vec![
            "network-summary-1".to_owned(),
            "screen-summary-ref-1".to_owned()
        ]
    );
    assert!(!job.page_content_available);
    assert!(!job.raw_network_payload_available);
    assert!(!job.decrypted_payload_available);
}

#[test]
fn local_ai_queue_skips_when_review_not_recommended() {
    let plan = plan_network_local_ai_queue(queue_input(confirmed_domain_bundle()))
        .expect_value("confirmed domain bundle should produce a skip plan");

    assert_eq!(plan.status, NetworkLocalAiQueueStatus::NotRecommended);
    assert!(!plan.local_ai_review_recommended);
    assert_eq!(plan.evidence_refs, vec!["domain-category-1"]);
    assert!(plan.job.is_none());
    assert!(!plan.policy_action_authority);
    assert!(!plan.adapter_action_authority);
}

#[test]
fn local_ai_queue_disabled_or_unavailable_states_cannot_carry_job() {
    let disabled = plan_network_local_ai_queue(NetworkLocalAiQueueInput {
        local_ai_enabled: false,
        ..queue_input(weak_transfer_bundle())
    })
    .expect_value("disabled state should be explicit");
    assert_eq!(disabled.status, NetworkLocalAiQueueStatus::DisabledByParent);
    assert!(disabled.job.is_none());

    let model_unavailable = plan_network_local_ai_queue(NetworkLocalAiQueueInput {
        model_runtime_available: false,
        ..queue_input(weak_transfer_bundle())
    })
    .expect_value("model unavailable state should be explicit");
    assert_eq!(
        model_unavailable.status,
        NetworkLocalAiQueueStatus::ModelUnavailable
    );
    assert!(model_unavailable.job.is_none());

    let queue_unavailable = plan_network_local_ai_queue(NetworkLocalAiQueueInput {
        queue_available: false,
        ..queue_input(weak_transfer_bundle())
    })
    .expect_value("queue unavailable state should be explicit");
    assert_eq!(
        queue_unavailable.status,
        NetworkLocalAiQueueStatus::QueueUnavailable
    );
    assert!(queue_unavailable.job.is_none());
}

#[test]
fn local_ai_queue_rejects_raw_content_or_authority_claims() {
    assert_eq!(
        plan_network_local_ai_queue(NetworkLocalAiQueueInput {
            raw_network_payload_available: true,
            ..queue_input(weak_transfer_bundle())
        }),
        Err(NetworkLocalAiQueueError::RawNetworkPayloadRejected)
    );
    assert_eq!(
        plan_network_local_ai_queue(NetworkLocalAiQueueInput {
            page_content_available: true,
            ..queue_input(weak_transfer_bundle())
        }),
        Err(NetworkLocalAiQueueError::PageContentRejected)
    );

    let mut decrypted = weak_transfer_bundle();
    decrypted.decrypted_payload_available = true;
    assert_eq!(
        plan_network_local_ai_queue(queue_input(decrypted)),
        Err(NetworkLocalAiQueueError::DecryptedPayloadRejected)
    );

    let mut policy = weak_transfer_bundle();
    policy.policy_action_authority = true;
    assert_eq!(
        plan_network_local_ai_queue(queue_input(policy)),
        Err(NetworkLocalAiQueueError::PolicyAuthorityRejected)
    );

    let mut adapter = weak_transfer_bundle();
    adapter.adapter_action_authorized = true;
    assert_eq!(
        plan_network_local_ai_queue(queue_input(adapter)),
        Err(NetworkLocalAiQueueError::AdapterAuthorityRejected)
    );
}

#[test]
fn local_ai_queue_rejects_empty_queue_and_summary_refs() {
    assert_eq!(
        plan_network_local_ai_queue(NetworkLocalAiQueueInput {
            queue_job_ref: " ".to_owned(),
            ..queue_input(weak_transfer_bundle())
        }),
        Err(NetworkLocalAiQueueError::EmptyQueueJobRef)
    );
    assert_eq!(
        plan_network_local_ai_queue(NetworkLocalAiQueueInput {
            queue_ref: " ".to_owned(),
            ..queue_input(weak_transfer_bundle())
        }),
        Err(NetworkLocalAiQueueError::EmptyQueueRef)
    );
    assert_eq!(
        plan_network_local_ai_queue(NetworkLocalAiQueueInput {
            model_runtime_ref: " ".to_owned(),
            ..queue_input(weak_transfer_bundle())
        }),
        Err(NetworkLocalAiQueueError::EmptyModelRuntimeRef)
    );
    assert_eq!(
        plan_network_local_ai_queue(NetworkLocalAiQueueInput {
            summary_refs: vec![" ".to_owned()],
            ..queue_input(weak_transfer_bundle())
        }),
        Err(NetworkLocalAiQueueError::EmptySummaryRef)
    );
}

fn queue_input(bundle: NetworkCrossSliceEvidenceBundle) -> NetworkLocalAiQueueInput {
    NetworkLocalAiQueueInput {
        queue_job_ref: " local-ai-job-1 ".to_owned(),
        queue_ref: "local-ai-queue-1".to_owned(),
        model_runtime_ref: "local-model-runtime-1".to_owned(),
        bundle,
        summary_refs: vec![
            " network-summary-1 ".to_owned(),
            "screen-summary-ref-1".to_owned(),
            "network-summary-1".to_owned(),
        ],
        local_ai_enabled: true,
        model_runtime_available: true,
        queue_available: true,
        raw_network_payload_available: false,
        page_content_available: false,
        policy_action_authority: false,
        adapter_action_authority: false,
    }
}

fn weak_transfer_bundle() -> NetworkCrossSliceEvidenceBundle {
    bundle(
        TriggerRef("network-trigger-weak"),
        source(
            NetworkCascadeSourceKind::TransferCandidate,
            NetworkCascadeSignalStrength::WeakHint,
            NetworkEvidenceGrade::D,
            EvidenceRef("transfer-hint-1"),
            false,
        ),
    )
}

fn weak_managed_browser_bundle() -> NetworkCrossSliceEvidenceBundle {
    bundle(
        TriggerRef("network-trigger-managed-browser"),
        source(
            NetworkCascadeSourceKind::ManagedBrowserExactUrl,
            NetworkCascadeSignalStrength::WeakHint,
            NetworkEvidenceGrade::C,
            EvidenceRef("managed-browser-url-ref"),
            true,
        ),
    )
}

fn confirmed_domain_bundle() -> NetworkCrossSliceEvidenceBundle {
    bundle(
        TriggerRef("network-trigger-confirmed"),
        source(
            NetworkCascadeSourceKind::DomainCategory,
            NetworkCascadeSignalStrength::Confirmed,
            NetworkEvidenceGrade::B,
            EvidenceRef("domain-category-1"),
            false,
        ),
    )
}

fn bundle(
    trigger_ref: TriggerRef,
    source: NetworkCrossSliceEvidenceSource,
) -> NetworkCrossSliceEvidenceBundle {
    build_network_cross_slice_evidence_bundle(NetworkCrossSliceEvidenceBundleInput {
        trigger_ref: trigger_ref.0.to_owned(),
        sources: vec![source],
    })
    .expect_value("test bundle should be valid")
}

fn source(
    source_kind: NetworkCascadeSourceKind,
    signal_strength: NetworkCascadeSignalStrength,
    evidence_grade: NetworkEvidenceGrade,
    evidence_ref: EvidenceRef,
    exact_url_available: bool,
) -> NetworkCrossSliceEvidenceSource {
    NetworkCrossSliceEvidenceSource {
        source_kind,
        signal_strength,
        evidence_grade,
        evidence_ref: evidence_ref.0.to_owned(),
        exact_url_available,
        decrypted_payload_available: false,
        policy_action_authority: false,
        adapter_action_authority: false,
    }
}
