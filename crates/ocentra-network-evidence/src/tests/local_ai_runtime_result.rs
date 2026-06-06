use crate::{
    bridge_network_local_ai_runtime_result, build_network_cross_slice_evidence_bundle,
    plan_network_local_ai_queue, NetworkCascadeSignalStrength, NetworkCascadeSourceKind,
    NetworkCrossSliceEvidenceBundle, NetworkCrossSliceEvidenceBundleInput,
    NetworkCrossSliceEvidenceSource, NetworkEvidenceGrade, NetworkLocalAiQueueInput,
    NetworkLocalAiQueuePlan, NetworkLocalAiQueueStatus, NetworkLocalAiRuntimeBridgeState,
    NetworkLocalAiRuntimeGenerationState, NetworkLocalAiRuntimeResultError,
    NetworkLocalAiRuntimeResultInput, NetworkLocalAiRuntimeResultRef,
};

#[test]
fn local_ai_runtime_result_bridges_queued_job_to_audit_ready_refs() {
    let bridge = bridge_network_local_ai_runtime_result(bridge_input(
        queued_plan(weak_managed_browser_bundle()),
        Some(complete_runtime_result()),
    ))
    .expect("queued job with complete runtime result should bridge");

    assert_eq!(
        bridge.bridge_state,
        NetworkLocalAiRuntimeBridgeState::ResultReady
    );
    assert_eq!(bridge.queue_status, NetworkLocalAiQueueStatus::Queued);
    assert_eq!(bridge.trigger_ref, "network-trigger-managed-browser");
    assert_eq!(bridge.queue_job_ref, Some("local-ai-job-1".to_owned()));
    assert_eq!(bridge.queue_ref, Some("local-ai-queue-1".to_owned()));
    assert_eq!(
        bridge.model_runtime_ref,
        Some("local-model-runtime-1".to_owned())
    );
    assert_eq!(
        bridge.local_ai_result_ref,
        Some("local-ai-result-network-1".to_owned())
    );
    assert_eq!(
        bridge.managed_browser_exact_url_evidence_refs,
        vec!["managed-browser-url-ref"]
    );
    assert_eq!(
        bridge.output_summary_ref,
        Some("local-ai-output-summary-1".to_owned())
    );
    assert!(bridge.local_runtime_result_observed);
    assert!(bridge.audit_input_ready);
    assert!(bridge.local_model_output_available);
    assert_no_unsupported_claims(&bridge);
}

#[test]
fn local_ai_runtime_result_keeps_unavailable_runtime_non_audit_ready() {
    let bridge = bridge_network_local_ai_runtime_result(bridge_input(
        queued_plan(weak_transfer_bundle()),
        Some(runtime_result(
            "local-ai-result-network-unavailable",
            NetworkLocalAiRuntimeGenerationState::Unavailable,
            None,
        )),
    ))
    .expect("unavailable runtime result should stay explicit");

    assert_eq!(
        bridge.bridge_state,
        NetworkLocalAiRuntimeBridgeState::RuntimeUnavailable
    );
    assert_eq!(
        bridge.local_ai_result_ref,
        Some("local-ai-result-network-unavailable".to_owned())
    );
    assert!(bridge.local_runtime_result_observed);
    assert!(!bridge.audit_input_ready);
    assert!(!bridge.local_model_output_available);
    assert_eq!(bridge.output_summary_ref, None);
    assert_no_unsupported_claims(&bridge);
}

#[test]
fn local_ai_runtime_result_keeps_non_queued_plan_without_runtime_result() {
    let bridge = bridge_network_local_ai_runtime_result(bridge_input(not_recommended_plan(), None))
        .expect("not recommended queue plan should bridge as queue-not-ready");

    assert_eq!(
        bridge.bridge_state,
        NetworkLocalAiRuntimeBridgeState::QueueNotReady
    );
    assert_eq!(
        bridge.queue_status,
        NetworkLocalAiQueueStatus::NotRecommended
    );
    assert_eq!(bridge.trigger_ref, "network-trigger-confirmed");
    assert_eq!(bridge.evidence_refs, vec!["domain-category-1"]);
    assert!(!bridge.local_runtime_result_observed);
    assert_eq!(bridge.local_ai_result_ref, None);
    assert_eq!(bridge.output_summary_ref, None);
    assert_no_unsupported_claims(&bridge);
}

#[test]
fn local_ai_runtime_result_rejects_queue_and_result_mismatches() {
    assert_eq!(
        bridge_network_local_ai_runtime_result(bridge_input(
            not_recommended_plan(),
            Some(complete_runtime_result())
        )),
        Err(NetworkLocalAiRuntimeResultError::ResultWithoutQueuedJob)
    );
    assert_eq!(
        bridge_network_local_ai_runtime_result(bridge_input(
            queued_plan(weak_transfer_bundle()),
            None
        )),
        Err(NetworkLocalAiRuntimeResultError::MissingRuntimeResultForQueuedJob)
    );

    let mut missing_output = complete_runtime_result();
    missing_output.output_summary_ref = None;
    assert_eq!(
        bridge_network_local_ai_runtime_result(bridge_input(
            queued_plan(weak_transfer_bundle()),
            Some(missing_output)
        )),
        Err(NetworkLocalAiRuntimeResultError::CompleteMissingOutputSummaryRef)
    );

    let mut unavailable_with_output = runtime_result(
        "local-ai-result-network-unavailable",
        NetworkLocalAiRuntimeGenerationState::Unavailable,
        None,
    );
    unavailable_with_output.output_summary_ref = Some("local-ai-output-summary-1".to_owned());
    assert_eq!(
        bridge_network_local_ai_runtime_result(bridge_input(
            queued_plan(weak_transfer_bundle()),
            Some(unavailable_with_output)
        )),
        Err(NetworkLocalAiRuntimeResultError::NonCompleteOutputSummaryRef)
    );
}

#[test]
fn local_ai_runtime_result_rejects_content_remote_and_authority_claims() {
    assert_claim_rejected(
        |input| input.remote_ai_claimed = true,
        NetworkLocalAiRuntimeResultError::RemoteAiClaimRejected,
    );
    assert_claim_rejected(
        |input| input.raw_pcap_input_claimed = true,
        NetworkLocalAiRuntimeResultError::RawPcapInputRejected,
    );
    assert_claim_rejected(
        |input| input.decrypted_payload_claimed = true,
        NetworkLocalAiRuntimeResultError::DecryptedPayloadClaimRejected,
    );
    assert_claim_rejected(
        |input| input.page_content_claimed = true,
        NetworkLocalAiRuntimeResultError::PageContentClaimRejected,
    );
    assert_claim_rejected(
        |input| input.exact_url_claimed = true,
        NetworkLocalAiRuntimeResultError::ExactUrlClaimRejected,
    );
    assert_claim_rejected(
        |input| input.private_message_claimed = true,
        NetworkLocalAiRuntimeResultError::PrivateMessageClaimRejected,
    );
    assert_claim_rejected(
        |input| input.search_query_claimed = true,
        NetworkLocalAiRuntimeResultError::SearchQueryClaimRejected,
    );
    assert_claim_rejected(
        |input| input.policy_authority_claimed = true,
        NetworkLocalAiRuntimeResultError::PolicyAuthorityClaimRejected,
    );
    assert_claim_rejected(
        |input| input.adapter_authority_claimed = true,
        NetworkLocalAiRuntimeResultError::AdapterAuthorityClaimRejected,
    );
    assert_claim_rejected(
        |input| input.enforcement_command_claimed = true,
        NetworkLocalAiRuntimeResultError::EnforcementCommandClaimRejected,
    );
}

fn assert_claim_rejected(
    mutate: impl FnOnce(&mut NetworkLocalAiRuntimeResultInput),
    expected: NetworkLocalAiRuntimeResultError,
) {
    let mut input = bridge_input(
        queued_plan(weak_transfer_bundle()),
        Some(complete_runtime_result()),
    );
    mutate(&mut input);
    assert_eq!(bridge_network_local_ai_runtime_result(input), Err(expected));
}

fn bridge_input(
    queue_plan: NetworkLocalAiQueuePlan,
    runtime_result: Option<NetworkLocalAiRuntimeResultRef>,
) -> NetworkLocalAiRuntimeResultInput {
    NetworkLocalAiRuntimeResultInput {
        queue_plan,
        runtime_result,
        prompt_template_ref: "network.local-ai.prompt-template.33a".to_owned(),
        policy_context_ref: "network.local-ai.policy-context.33a".to_owned(),
        parent_rule_refs: vec![
            "parent-rule-network-local-ai-review".to_owned(),
            "parent-rule-network-local-ai-review".to_owned(),
        ],
        remote_ai_claimed: false,
        raw_pcap_input_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        exact_url_claimed: false,
        private_message_claimed: false,
        search_query_claimed: false,
        policy_authority_claimed: false,
        adapter_authority_claimed: false,
        enforcement_command_claimed: false,
    }
}

fn complete_runtime_result() -> NetworkLocalAiRuntimeResultRef {
    runtime_result(
        "local-ai-result-network-1",
        NetworkLocalAiRuntimeGenerationState::Complete,
        Some("local-ai-output-summary-1"),
    )
}

fn runtime_result(
    local_ai_result_ref: &str,
    generation_state: NetworkLocalAiRuntimeGenerationState,
    output_summary_ref: Option<&str>,
) -> NetworkLocalAiRuntimeResultRef {
    NetworkLocalAiRuntimeResultRef {
        local_ai_result_ref: local_ai_result_ref.to_owned(),
        runtime_reference_id: "local-ai-runtime-ref-1".to_owned(),
        model_reference: "local-ai-model-ref-1".to_owned(),
        model_version_ref: "local-ai-model-version-1".to_owned(),
        generation_state,
        output_summary_ref: output_summary_ref.map(str::to_owned),
    }
}

fn queued_plan(bundle: NetworkCrossSliceEvidenceBundle) -> NetworkLocalAiQueuePlan {
    plan_network_local_ai_queue(queue_input(bundle)).expect("bundle should queue local AI")
}

fn not_recommended_plan() -> NetworkLocalAiQueuePlan {
    plan_network_local_ai_queue(queue_input(bundle(
        "network-trigger-confirmed",
        source(
            NetworkCascadeSourceKind::DomainCategory,
            NetworkCascadeSignalStrength::Confirmed,
            NetworkEvidenceGrade::B,
            "domain-category-1",
            false,
        ),
    )))
    .expect("confirmed domain bundle should not queue local AI")
}

fn queue_input(bundle: NetworkCrossSliceEvidenceBundle) -> NetworkLocalAiQueueInput {
    NetworkLocalAiQueueInput {
        queue_job_ref: "local-ai-job-1".to_owned(),
        queue_ref: "local-ai-queue-1".to_owned(),
        model_runtime_ref: "local-model-runtime-1".to_owned(),
        bundle,
        summary_refs: vec![
            "network-summary-1".to_owned(),
            "screen-summary-ref-1".to_owned(),
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
        "network-trigger-weak",
        source(
            NetworkCascadeSourceKind::TransferCandidate,
            NetworkCascadeSignalStrength::WeakHint,
            NetworkEvidenceGrade::D,
            "transfer-hint-1",
            false,
        ),
    )
}

fn weak_managed_browser_bundle() -> NetworkCrossSliceEvidenceBundle {
    bundle(
        "network-trigger-managed-browser",
        source(
            NetworkCascadeSourceKind::ManagedBrowserExactUrl,
            NetworkCascadeSignalStrength::WeakHint,
            NetworkEvidenceGrade::C,
            "managed-browser-url-ref",
            true,
        ),
    )
}

fn bundle(
    trigger_ref: &str,
    source: NetworkCrossSliceEvidenceSource,
) -> NetworkCrossSliceEvidenceBundle {
    build_network_cross_slice_evidence_bundle(NetworkCrossSliceEvidenceBundleInput {
        trigger_ref: trigger_ref.to_owned(),
        sources: vec![source],
    })
    .expect("test bundle should be valid")
}

fn source(
    source_kind: NetworkCascadeSourceKind,
    signal_strength: NetworkCascadeSignalStrength,
    evidence_grade: NetworkEvidenceGrade,
    evidence_ref: &str,
    exact_url_available: bool,
) -> NetworkCrossSliceEvidenceSource {
    NetworkCrossSliceEvidenceSource {
        source_kind,
        signal_strength,
        evidence_grade,
        evidence_ref: evidence_ref.to_owned(),
        exact_url_available,
        decrypted_payload_available: false,
        policy_action_authority: false,
        adapter_action_authority: false,
    }
}

fn assert_no_unsupported_claims(bridge: &crate::NetworkLocalAiRuntimeResultBridge) {
    assert!(!bridge.model_execution_proved);
    assert!(!bridge.raw_pcap_available);
    assert!(!bridge.exact_url_claimed);
    assert!(!bridge.decrypted_payload_available);
    assert!(!bridge.page_content_available);
    assert!(!bridge.private_message_available);
    assert!(!bridge.search_query_available);
    assert!(!bridge.remote_ai_used);
    assert!(!bridge.policy_authority);
    assert!(!bridge.adapter_authority);
    assert_eq!(bridge.enforcement_commands_published, 0);
}
