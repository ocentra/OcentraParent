use ocentra_network_evidence::{
    bridge_network_local_ai_runtime_result, build_network_cross_slice_evidence_bundle,
    plan_network_local_ai_queue, NetworkCascadeSignalStrength, NetworkCascadeSourceKind,
    NetworkCrossSliceEvidenceBundleInput, NetworkCrossSliceEvidenceSource, NetworkEvidenceGrade,
    NetworkLocalAiQueueInput, NetworkLocalAiQueueStatus as EvidenceLocalAiQueueStatus,
    NetworkLocalAiRuntimeBridgeState as EvidenceLocalAiRuntimeBridgeState,
    NetworkLocalAiRuntimeGenerationState, NetworkLocalAiRuntimeResultBridge,
    NetworkLocalAiRuntimeResultInput, NetworkLocalAiRuntimeResultRef,
};
use ocentra_parent_agent_protocol::{
    constants, NetworkLocalAiRuntimeResultBridgeState, NetworkLocalAiRuntimeResultQueueStatus,
    NetworkLocalAiRuntimeResultStatus,
};

pub(crate) fn local_ai_runtime_result_status() -> NetworkLocalAiRuntimeResultStatus {
    let queue_plan = plan_network_local_ai_queue(local_ai_queue_input())
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    let bridge = bridge_network_local_ai_runtime_result(NetworkLocalAiRuntimeResultInput {
        queue_plan,
        runtime_result: Some(local_ai_runtime_result_ref()),
        prompt_template_ref: constants::network_flow::TEST_LOCAL_AI_PROMPT_TEMPLATE_REF.to_owned(),
        policy_context_ref: constants::network_flow::TEST_LOCAL_AI_POLICY_CONTEXT_REF.to_owned(),
        parent_rule_refs: vec![constants::network_flow::TEST_PARENT_RULE_REF.to_owned()],
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
    })
    .expect(constants::error::AGENT_EVENT_SERIALIZES);

    local_ai_runtime_result_status_from_bridge(bridge)
}

fn local_ai_queue_input() -> NetworkLocalAiQueueInput {
    let bundle = build_network_cross_slice_evidence_bundle(NetworkCrossSliceEvidenceBundleInput {
        trigger_ref: constants::network_flow::TEST_LOCAL_AI_TRIGGER_REF.to_owned(),
        sources: vec![NetworkCrossSliceEvidenceSource {
            source_kind: NetworkCascadeSourceKind::ManagedBrowserExactUrl,
            signal_strength: NetworkCascadeSignalStrength::WeakHint,
            evidence_grade: NetworkEvidenceGrade::C,
            evidence_ref:
                constants::network_flow::TEST_LOCAL_AI_MANAGED_BROWSER_EXACT_URL_EVIDENCE_REF
                    .to_owned(),
            exact_url_available: true,
            decrypted_payload_available: false,
            policy_action_authority: false,
            adapter_action_authority: false,
        }],
    })
    .expect(constants::error::AGENT_EVENT_SERIALIZES);

    NetworkLocalAiQueueInput {
        queue_job_ref: constants::network_flow::TEST_LOCAL_AI_QUEUE_JOB_REF.to_owned(),
        queue_ref: constants::network_flow::TEST_LOCAL_AI_QUEUE_REF.to_owned(),
        model_runtime_ref: constants::network_flow::TEST_LOCAL_AI_MODEL_RUNTIME_REF.to_owned(),
        bundle,
        summary_refs: vec![
            constants::network_flow::TEST_LOCAL_AI_NETWORK_SUMMARY_REF.to_owned(),
            constants::network_flow::TEST_LOCAL_AI_SCREEN_SUMMARY_REF.to_owned(),
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

fn local_ai_runtime_result_ref() -> NetworkLocalAiRuntimeResultRef {
    NetworkLocalAiRuntimeResultRef {
        local_ai_result_ref: constants::network_flow::TEST_LOCAL_AI_RESULT_REF.to_owned(),
        runtime_reference_id: constants::network_flow::TEST_LOCAL_AI_RUNTIME_REFERENCE_ID
            .to_owned(),
        model_reference: constants::network_flow::TEST_LOCAL_AI_MODEL_REF.to_owned(),
        model_version_ref: constants::network_flow::TEST_LOCAL_AI_MODEL_VERSION_REF.to_owned(),
        generation_state: NetworkLocalAiRuntimeGenerationState::Complete,
        output_summary_ref: Some(
            constants::network_flow::TEST_LOCAL_AI_OUTPUT_SUMMARY_REF.to_owned(),
        ),
    }
}

fn local_ai_runtime_result_status_from_bridge(
    bridge: NetworkLocalAiRuntimeResultBridge,
) -> NetworkLocalAiRuntimeResultStatus {
    NetworkLocalAiRuntimeResultStatus {
        status_ref: constants::network_flow::TEST_LOCAL_AI_RUNTIME_RESULT_STATUS_REF.to_owned(),
        bridge_state: local_ai_bridge_state(bridge.bridge_state),
        queue_status: local_ai_queue_status(bridge.queue_status),
        trigger_ref: bridge.trigger_ref,
        queue_job_ref: bridge.queue_job_ref,
        queue_ref: bridge.queue_ref,
        model_runtime_ref: bridge.model_runtime_ref,
        local_ai_result_ref: bridge.local_ai_result_ref,
        runtime_reference_id: bridge.runtime_reference_id,
        model_reference: bridge.model_reference,
        model_version_ref: bridge.model_version_ref,
        prompt_template_ref: bridge.prompt_template_ref,
        policy_context_ref: bridge.policy_context_ref,
        parent_rule_refs: bridge.parent_rule_refs,
        evidence_refs: bridge.evidence_refs,
        summary_refs: bridge.summary_refs,
        managed_browser_exact_url_evidence_refs: bridge.managed_browser_exact_url_evidence_refs,
        output_summary_ref: bridge.output_summary_ref,
        local_runtime_result_observed: bridge.local_runtime_result_observed,
        audit_input_ready: bridge.audit_input_ready,
        local_model_output_available: bridge.local_model_output_available,
        model_execution_proved: bridge.model_execution_proved,
        raw_pcap_available: bridge.raw_pcap_available,
        exact_url_claimed: bridge.exact_url_claimed,
        decrypted_payload_available: bridge.decrypted_payload_available,
        page_content_available: bridge.page_content_available,
        private_message_available: bridge.private_message_available,
        search_query_available: bridge.search_query_available,
        remote_ai_used: bridge.remote_ai_used,
        policy_authority: bridge.policy_authority,
        adapter_authority: bridge.adapter_authority,
        enforcement_commands_published: bridge.enforcement_commands_published as u64,
    }
}

fn local_ai_bridge_state(
    bridge_state: EvidenceLocalAiRuntimeBridgeState,
) -> NetworkLocalAiRuntimeResultBridgeState {
    match bridge_state {
        EvidenceLocalAiRuntimeBridgeState::ResultReady => {
            NetworkLocalAiRuntimeResultBridgeState::ResultReady
        }
        EvidenceLocalAiRuntimeBridgeState::RuntimeUnavailable => {
            NetworkLocalAiRuntimeResultBridgeState::RuntimeUnavailable
        }
        EvidenceLocalAiRuntimeBridgeState::RuntimeFailed => {
            NetworkLocalAiRuntimeResultBridgeState::RuntimeFailed
        }
        EvidenceLocalAiRuntimeBridgeState::RuntimeTimedOut => {
            NetworkLocalAiRuntimeResultBridgeState::RuntimeTimedOut
        }
        EvidenceLocalAiRuntimeBridgeState::QueueNotReady => {
            NetworkLocalAiRuntimeResultBridgeState::QueueNotReady
        }
    }
}

fn local_ai_queue_status(
    queue_status: EvidenceLocalAiQueueStatus,
) -> NetworkLocalAiRuntimeResultQueueStatus {
    match queue_status {
        EvidenceLocalAiQueueStatus::Queued => NetworkLocalAiRuntimeResultQueueStatus::Queued,
        EvidenceLocalAiQueueStatus::NotRecommended => {
            NetworkLocalAiRuntimeResultQueueStatus::NotRecommended
        }
        EvidenceLocalAiQueueStatus::DisabledByParent => {
            NetworkLocalAiRuntimeResultQueueStatus::DisabledByParent
        }
        EvidenceLocalAiQueueStatus::ModelUnavailable => {
            NetworkLocalAiRuntimeResultQueueStatus::ModelUnavailable
        }
        EvidenceLocalAiQueueStatus::QueueUnavailable => {
            NetworkLocalAiRuntimeResultQueueStatus::QueueUnavailable
        }
    }
}
