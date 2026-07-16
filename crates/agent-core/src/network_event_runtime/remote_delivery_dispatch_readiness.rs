use ocentra_eventing::delivery::validation::EventDeliveryDecisionProof;
use ocentra_parent_agent_protocol::constants;

use super::remote_delivery_dispatch_readiness_types::{
    NetworkRuntimeRemoteDeliveryDispatchGate, NetworkRuntimeRemoteDeliveryDispatchReadinessError,
    NetworkRuntimeRemoteDeliveryDispatchReadinessReport,
    NetworkRuntimeRemoteDeliveryDispatchReadinessState,
};
use super::remote_delivery_event_chain_store::source_component;
use super::remote_delivery_outbox_handoff::prove_network_runtime_remote_delivery_outbox_handoff;
use super::remote_delivery_outbox_handoff_types::NetworkRuntimeRemoteDeliveryOutboxHandoffReport;

pub async fn prove_network_runtime_remote_delivery_dispatch_readiness() -> Result<
    NetworkRuntimeRemoteDeliveryDispatchReadinessReport,
    NetworkRuntimeRemoteDeliveryDispatchReadinessError,
> {
    let outbox_handoff = prove_network_runtime_remote_delivery_outbox_handoff()
        .await
        .map_err(NetworkRuntimeRemoteDeliveryDispatchReadinessError::OutboxHandoff)?;
    if outbox_handoff.outbox_candidate_count == 0 {
        return Err(NetworkRuntimeRemoteDeliveryDispatchReadinessError::EmptyOutbox);
    }
    if has_unsupported_claims(&outbox_handoff) {
        return Err(NetworkRuntimeRemoteDeliveryDispatchReadinessError::UnsupportedClaim);
    }
    build_dispatch_readiness_report(outbox_handoff)
}

fn build_dispatch_readiness_report(
    outbox_handoff: NetworkRuntimeRemoteDeliveryOutboxHandoffReport,
) -> Result<
    NetworkRuntimeRemoteDeliveryDispatchReadinessReport,
    NetworkRuntimeRemoteDeliveryDispatchReadinessError,
> {
    let remote_status = &outbox_handoff
        .durable_envelope
        .receipt_ledger
        .remote_delivery_status;
    let broker_gate = dispatch_gate(
        &remote_status.broker_semantics.delivery_decision,
        constants::network_flow::TEST_REMOTE_DELIVERY_BROKER_DISPATCH_GATE_REF,
        outbox_handoff.broker_delivery_implemented,
    )?;
    let family_hub_gate = dispatch_gate(
        &remote_status.family_hub_decision,
        constants::network_flow::TEST_REMOTE_DELIVERY_FAMILY_HUB_DISPATCH_GATE_REF,
        outbox_handoff.family_hub_delivery_implemented,
    )?;
    let candidate_count = outbox_handoff.outbox_candidate_count;
    Ok(NetworkRuntimeRemoteDeliveryDispatchReadinessReport {
        dispatch_readiness_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_DISPATCH_READINESS_REF,
        )?,
        transport_requirements_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_TRANSPORT_REQUIREMENTS_REF,
        )?,
        broker_gate,
        family_hub_gate,
        state: NetworkRuntimeRemoteDeliveryDispatchReadinessState::ManualRequiredTransportNotImplemented,
        source_outbox_candidate_count: candidate_count,
        prepared_not_dispatched_count: outbox_handoff.prepared_not_dispatched_count,
        manual_required_candidate_count: candidate_count,
        dispatch_ready_candidate_count: 0,
        dispatch_attempt_count: 0,
        remote_ack_count: 0,
        broker_delivery_implemented: outbox_handoff.broker_delivery_implemented,
        family_hub_delivery_implemented: outbox_handoff.family_hub_delivery_implemented,
        remote_delivery_ack_implemented: outbox_handoff.remote_delivery_ack_implemented,
        provider_delivery_implemented: outbox_handoff.provider_delivery_implemented,
        child_device_delivery_implemented: outbox_handoff.child_device_delivery_implemented,
        remote_delete_export_propagation_implemented: outbox_handoff
            .remote_delete_export_propagation_implemented,
        product_ready_remote_delivery: outbox_handoff.product_ready_remote_delivery,
        policy_authority: outbox_handoff.policy_authority,
        side_effect_authority: outbox_handoff.side_effect_authority,
        enforcement_command_event_count: outbox_handoff.enforcement_command_event_count,
        adapter_action_executed_count: outbox_handoff.adapter_action_executed_count,
        raw_pcap_available_count: outbox_handoff.raw_pcap_available_count,
        exact_url_available_count: outbox_handoff.exact_url_available_count,
        decrypted_payload_available_count: outbox_handoff.decrypted_payload_available_count,
        page_content_available_count: outbox_handoff.page_content_available_count,
        video_content_available_count: outbox_handoff.video_content_available_count,
        private_message_content_available_count: outbox_handoff
            .private_message_content_available_count,
        search_query_available_count: outbox_handoff.search_query_available_count,
        outbox_handoff,
    })
}

fn dispatch_gate(
    decision: &EventDeliveryDecisionProof,
    gate_ref: &str,
    transport_implemented: bool,
) -> Result<
    NetworkRuntimeRemoteDeliveryDispatchGate,
    NetworkRuntimeRemoteDeliveryDispatchReadinessError,
> {
    let fixture_requirements_satisfied = decision.missing_artifacts.is_empty();
    let dispatch_ready = fixture_requirements_satisfied && transport_implemented;
    Ok(NetworkRuntimeRemoteDeliveryDispatchGate {
        gate_ref: source_component(gate_ref)?,
        route_kind: decision.route_kind,
        required_artifacts: decision.required_artifacts.clone(),
        required_artifact_count: decision.required_artifacts.len(),
        missing_artifact_count: decision.missing_artifacts.len(),
        fixture_requirements_satisfied,
        transport_implemented,
        dispatch_ready,
        manual_required: !dispatch_ready,
    })
}

fn has_unsupported_claims(report: &NetworkRuntimeRemoteDeliveryOutboxHandoffReport) -> bool {
    report.dispatch_attempt_count > 0
        || report.remote_ack_count > 0
        || report.broker_delivery_implemented
        || report.family_hub_delivery_implemented
        || report.remote_delivery_ack_implemented
        || report.provider_delivery_implemented
        || report.child_device_delivery_implemented
        || report.remote_delete_export_propagation_implemented
        || report.product_ready_remote_delivery
        || report.policy_authority
        || report.side_effect_authority
        || report.enforcement_command_event_count > 0
        || report.adapter_action_executed_count > 0
        || report.raw_pcap_available_count > 0
        || report.exact_url_available_count > 0
        || report.decrypted_payload_available_count > 0
        || report.page_content_available_count > 0
        || report.video_content_available_count > 0
        || report.private_message_content_available_count > 0
        || report.search_query_available_count > 0
}
