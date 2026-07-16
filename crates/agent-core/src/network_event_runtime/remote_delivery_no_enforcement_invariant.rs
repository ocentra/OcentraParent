use ocentra_eventing::ids::SourceComponent;
use ocentra_parent_agent_protocol::constants;

use super::remote_delivery_dispatch_readiness::prove_network_runtime_remote_delivery_dispatch_readiness;
use super::remote_delivery_dispatch_readiness_types::NetworkRuntimeRemoteDeliveryDispatchReadinessReport;
use super::remote_delivery_event_chain_store::source_component;
use super::remote_delivery_no_enforcement_invariant_types::{
    NetworkRuntimeRemoteDeliveryNoEnforcementInvariantError,
    NetworkRuntimeRemoteDeliveryNoEnforcementInvariantReport,
    NetworkRuntimeRemoteDeliveryNoEnforcementInvariantState,
    NetworkRuntimeRemoteDeliveryNoEnforcementStage,
};

pub async fn prove_network_runtime_remote_delivery_no_enforcement_invariant() -> Result<
    NetworkRuntimeRemoteDeliveryNoEnforcementInvariantReport,
    NetworkRuntimeRemoteDeliveryNoEnforcementInvariantError,
> {
    let dispatch_readiness = prove_network_runtime_remote_delivery_dispatch_readiness()
        .await
        .map_err(NetworkRuntimeRemoteDeliveryNoEnforcementInvariantError::DispatchReadiness)?;
    prove_network_runtime_remote_delivery_no_enforcement_invariant_from_dispatch_readiness(
        dispatch_readiness,
    )
}

pub fn prove_network_runtime_remote_delivery_no_enforcement_invariant_from_dispatch_readiness(
    dispatch_readiness: NetworkRuntimeRemoteDeliveryDispatchReadinessReport,
) -> Result<
    NetworkRuntimeRemoteDeliveryNoEnforcementInvariantReport,
    NetworkRuntimeRemoteDeliveryNoEnforcementInvariantError,
> {
    if has_unsupported_claims(&dispatch_readiness) {
        return Err(NetworkRuntimeRemoteDeliveryNoEnforcementInvariantError::UnsupportedClaim);
    }
    let available_metadata_refs = available_metadata_refs(&dispatch_readiness);
    if available_metadata_refs.is_empty() {
        return Err(
            NetworkRuntimeRemoteDeliveryNoEnforcementInvariantError::MissingAvailableMetadata,
        );
    }
    let stages = invariant_stages();
    Ok(NetworkRuntimeRemoteDeliveryNoEnforcementInvariantReport {
        invariant_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_NO_ENFORCEMENT_INVARIANT_REF,
        )?,
        available_metadata_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_AVAILABLE_METADATA_REF,
        )?,
        state:
            NetworkRuntimeRemoteDeliveryNoEnforcementInvariantState::AvailableMetadataNonEnforcing,
        remote_metadata_stage_count: stages.len(),
        available_metadata_ref_count: available_metadata_refs.len(),
        manual_required_candidate_count: dispatch_readiness.manual_required_candidate_count,
        dispatch_ready_candidate_count: dispatch_readiness.dispatch_ready_candidate_count,
        dispatch_attempt_count: dispatch_readiness.dispatch_attempt_count,
        remote_ack_count: dispatch_readiness.remote_ack_count,
        broker_delivery_implemented: dispatch_readiness.broker_delivery_implemented,
        family_hub_delivery_implemented: dispatch_readiness.family_hub_delivery_implemented,
        remote_delivery_ack_implemented: dispatch_readiness.remote_delivery_ack_implemented,
        provider_delivery_implemented: dispatch_readiness.provider_delivery_implemented,
        child_device_delivery_implemented: dispatch_readiness.child_device_delivery_implemented,
        remote_delete_export_propagation_implemented: dispatch_readiness
            .remote_delete_export_propagation_implemented,
        product_ready_remote_delivery: dispatch_readiness.product_ready_remote_delivery,
        policy_authority: dispatch_readiness.policy_authority,
        side_effect_authority: dispatch_readiness.side_effect_authority,
        enforcement_command_event_count: dispatch_readiness.enforcement_command_event_count,
        adapter_action_executed_count: dispatch_readiness.adapter_action_executed_count,
        raw_pcap_available_count: dispatch_readiness.raw_pcap_available_count,
        exact_url_available_count: dispatch_readiness.exact_url_available_count,
        decrypted_payload_available_count: dispatch_readiness.decrypted_payload_available_count,
        page_content_available_count: dispatch_readiness.page_content_available_count,
        video_content_available_count: dispatch_readiness.video_content_available_count,
        private_message_content_available_count: dispatch_readiness
            .private_message_content_available_count,
        search_query_available_count: dispatch_readiness.search_query_available_count,
        stages,
        available_metadata_refs,
        dispatch_readiness,
    })
}

fn invariant_stages() -> Vec<NetworkRuntimeRemoteDeliveryNoEnforcementStage> {
    vec![
        NetworkRuntimeRemoteDeliveryNoEnforcementStage::RemoteDeliveryStatus,
        NetworkRuntimeRemoteDeliveryNoEnforcementStage::EventChainJournal,
        NetworkRuntimeRemoteDeliveryNoEnforcementStage::ReceiptLedger,
        NetworkRuntimeRemoteDeliveryNoEnforcementStage::DurableEnvelope,
        NetworkRuntimeRemoteDeliveryNoEnforcementStage::OutboxHandoff,
        NetworkRuntimeRemoteDeliveryNoEnforcementStage::DispatchReadiness,
    ]
}

fn available_metadata_refs(
    dispatch_readiness: &NetworkRuntimeRemoteDeliveryDispatchReadinessReport,
) -> Vec<SourceComponent> {
    let outbox = &dispatch_readiness.outbox_handoff;
    let durable = &outbox.durable_envelope;
    let receipt = &durable.receipt_ledger;
    let remote_status = &receipt.remote_delivery_status;
    vec![
        remote_status.custody_proof_ref.clone(),
        remote_status.publisher_auth_ref.clone(),
        remote_status.subscriber_auth_ref.clone(),
        remote_status.encryption_ref.clone(),
        remote_status.retention_policy_ref.clone(),
        remote_status.replay_plan_ref.clone(),
        remote_status.deletion_plan_ref.clone(),
        remote_status.offset_policy_ref.clone(),
        remote_status.dedupe_policy_ref.clone(),
        remote_status.transport_config_ref.clone(),
        remote_status.relay_identity_ref.clone(),
        remote_status.relay_policy_ref.clone(),
        receipt.event_chain_journal_ref.clone(),
        receipt.event_chain_export_ref.clone(),
        receipt.receipt_ledger_ref.clone(),
        receipt.local_receipt_ack_ref.clone(),
        receipt.receipt_replay_ref.clone(),
        receipt.receipt_support_status_ref.clone(),
        durable.durable_envelope_ref.clone(),
        durable.durable_store_ref.clone(),
        durable.durable_replay_ref.clone(),
        durable.delete_export_readiness_ref.clone(),
        durable.durable_support_status_ref.clone(),
        outbox.outbox_ref.clone(),
        outbox.handoff_ref.clone(),
        outbox.outbox_replay_ref.clone(),
        outbox.outbox_support_status_ref.clone(),
        dispatch_readiness.dispatch_readiness_ref.clone(),
        dispatch_readiness.transport_requirements_ref.clone(),
        dispatch_readiness.broker_gate.gate_ref.clone(),
        dispatch_readiness.family_hub_gate.gate_ref.clone(),
    ]
}

fn has_unsupported_claims(report: &NetworkRuntimeRemoteDeliveryDispatchReadinessReport) -> bool {
    let outbox = &report.outbox_handoff;
    let durable = &outbox.durable_envelope;
    let receipt = &durable.receipt_ledger;
    let remote_status = &receipt.remote_delivery_status;
    report.dispatch_attempt_count > 0
        || report.remote_ack_count > 0
        || outbox.dispatch_attempt_count > 0
        || outbox.remote_ack_count > 0
        || report.broker_delivery_implemented
        || report.family_hub_delivery_implemented
        || report.remote_delivery_ack_implemented
        || report.provider_delivery_implemented
        || report.child_device_delivery_implemented
        || report.remote_delete_export_propagation_implemented
        || report.product_ready_remote_delivery
        || report.policy_authority
        || report.side_effect_authority
        || remote_status.external_transport_delivery_implemented
        || remote_status.family_hub_delivery_implemented
        || remote_status.cross_process_replay_implemented
        || remote_status.remote_retention_delete_export_propagation_implemented
        || remote_status.policy_authority
        || remote_status.side_effect_authority
        || receipt.broker_delivery_implemented
        || receipt.family_hub_delivery_implemented
        || receipt.remote_delivery_ack_implemented
        || receipt.policy_authority
        || receipt.side_effect_authority
        || durable.broker_delivery_implemented
        || durable.family_hub_delivery_implemented
        || durable.remote_delivery_ack_implemented
        || durable.provider_delivery_implemented
        || durable.child_device_delivery_implemented
        || durable.remote_delete_export_propagation_implemented
        || durable.product_ready_remote_delivery
        || durable.policy_authority
        || durable.side_effect_authority
        || report.enforcement_command_event_count > 0
        || report.adapter_action_executed_count > 0
        || report.raw_pcap_available_count > 0
        || report.exact_url_available_count > 0
        || report.decrypted_payload_available_count > 0
        || report.page_content_available_count > 0
        || report.video_content_available_count > 0
        || report.private_message_content_available_count > 0
        || report.search_query_available_count > 0
        || remote_status.enforcement_command_event_count > 0
        || remote_status.adapter_action_executed_count > 0
        || receipt.enforcement_command_event_count > 0
        || receipt.adapter_action_executed_count > 0
        || receipt.raw_pcap_available_count > 0
        || receipt.exact_url_available_count > 0
        || receipt.decrypted_payload_available_count > 0
        || receipt.page_content_available_count > 0
        || receipt.video_content_available_count > 0
        || receipt.private_message_content_available_count > 0
        || receipt.search_query_available_count > 0
        || durable.enforcement_command_event_count > 0
        || durable.adapter_action_executed_count > 0
        || durable.raw_pcap_available_count > 0
        || durable.exact_url_available_count > 0
        || durable.decrypted_payload_available_count > 0
        || durable.page_content_available_count > 0
        || durable.video_content_available_count > 0
        || durable.private_message_content_available_count > 0
        || durable.search_query_available_count > 0
}
