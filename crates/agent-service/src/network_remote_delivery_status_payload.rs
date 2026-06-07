use ocentra_parent_agent_core::{
    prove_network_runtime_remote_delivery_durable_envelope,
    NetworkRuntimeRemoteDeliveryDurableEnvelopeReport, NetworkRuntimeRemoteDeliveryState,
};
use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentEventEnvelope, AgentEventName, LogFieldValue, LogFields,
    LogLevel, NetworkRemoteDeliveryStatus, NetworkRemoteDeliveryStatusState,
};

use crate::{event_builder::build_event, fields::fields_from_pairs};

pub(crate) async fn build_network_remote_delivery_status_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let correlation_id = command.message_id.clone();
    let target = command.source;
    match network_remote_delivery_status_payload().await {
        Ok(payload) => build_event(
            constants::event_id::NETWORK_REMOTE_DELIVERY_STATUS_REPORTED,
            &correlation_id,
            target,
            AgentEventName::AgentNetworkRemoteDeliveryStatusReported,
            LogLevel::Info,
            payload,
            None,
        ),
        Err(()) => build_event(
            constants::event_id::COMMAND_REJECTED,
            &correlation_id,
            target,
            AgentEventName::AgentCommandRejected,
            LogLevel::Warn,
            fields_from_pairs(vec![(
                constants::field::REASON,
                LogFieldValue::String(
                    constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_DURABLE_ENVELOPE
                        .to_string(),
                ),
            )]),
            None,
        ),
    }
}

pub(crate) async fn network_remote_delivery_status_payload() -> Result<LogFields, ()> {
    let status = network_remote_delivery_status().await?;
    let serialized = serde_json::to_string(&status).map_err(|_| ())?;
    Ok(fields_from_pairs(vec![(
        constants::field::NETWORK_REMOTE_DELIVERY_STATUS,
        LogFieldValue::String(serialized),
    )]))
}

async fn network_remote_delivery_status() -> Result<NetworkRemoteDeliveryStatus, ()> {
    let handle = tokio::runtime::Handle::current();
    let report = tokio::task::spawn_blocking(move || {
        handle.block_on(prove_network_runtime_remote_delivery_durable_envelope())
    })
    .await
    .map_err(|_| ())?
    .map_err(|_| ())?;
    Ok(status_from_report(&report))
}

fn status_from_report(
    report: &NetworkRuntimeRemoteDeliveryDurableEnvelopeReport,
) -> NetworkRemoteDeliveryStatus {
    let remote_status = &report.receipt_ledger.remote_delivery_status;
    NetworkRemoteDeliveryStatus {
        status_ref: constants::network_flow::TEST_REMOTE_DELIVERY_STATUS_BRIDGE_REF.to_string(),
        broker_status: protocol_remote_delivery_state(remote_status.broker_status),
        family_hub_status: protocol_remote_delivery_state(remote_status.family_hub_status),
        custody_proof_ref: remote_status.custody_proof_ref.as_str().to_string(),
        publisher_auth_ref: remote_status.publisher_auth_ref.as_str().to_string(),
        subscriber_auth_ref: remote_status.subscriber_auth_ref.as_str().to_string(),
        encryption_ref: remote_status.encryption_ref.as_str().to_string(),
        retention_policy_ref: remote_status.retention_policy_ref.as_str().to_string(),
        replay_plan_ref: remote_status.replay_plan_ref.as_str().to_string(),
        deletion_plan_ref: remote_status.deletion_plan_ref.as_str().to_string(),
        offset_policy_ref: remote_status.offset_policy_ref.as_str().to_string(),
        dedupe_policy_ref: remote_status.dedupe_policy_ref.as_str().to_string(),
        transport_config_ref: remote_status.transport_config_ref.as_str().to_string(),
        relay_identity_ref: remote_status.relay_identity_ref.as_str().to_string(),
        relay_policy_ref: remote_status.relay_policy_ref.as_str().to_string(),
        broker_missing_artifact_count: count(remote_status.broker_missing_artifact_count),
        family_hub_missing_artifact_count: count(remote_status.family_hub_missing_artifact_count),
        accepted_event_type_count: count(remote_status.accepted_event_type_count),
        local_idempotency_queue_proved: remote_status.local_idempotency_queue_proved,
        dropped_event_dead_letter_count: count(remote_status.dropped_event_dead_letter_count),
        queued_duplicate_rejected: remote_status.queued_duplicate_rejected,
        completed_duplicate_rejected: remote_status.completed_duplicate_rejected,
        event_chain_journal_ref: report
            .receipt_ledger
            .event_chain_journal_ref
            .as_str()
            .to_string(),
        receipt_ledger_ref: report
            .receipt_ledger
            .receipt_ledger_ref
            .as_str()
            .to_string(),
        local_receipt_ack_ref: report
            .receipt_ledger
            .local_receipt_ack_ref
            .as_str()
            .to_string(),
        durable_envelope_ref: report.durable_envelope_ref.as_str().to_string(),
        durable_store_ref: report.durable_store_ref.as_str().to_string(),
        durable_replay_ref: report.durable_replay_ref.as_str().to_string(),
        durable_delete_export_ref: report.delete_export_readiness_ref.as_str().to_string(),
        durable_support_status_ref: report.durable_support_status_ref.as_str().to_string(),
        durable_envelope_ready: report.durable_store_ready
            && report.durable_replay_ready
            && report.delete_export_readiness_recorded
            && report.durable_records_match_receipts,
        durable_envelope_missing_artifact_count: 0,
        broker_delivery_implemented: report.broker_delivery_implemented,
        family_hub_delivery_implemented: report.family_hub_delivery_implemented,
        remote_delivery_ack_implemented: report.remote_delivery_ack_implemented,
        provider_delivery_implemented: report.provider_delivery_implemented,
        child_device_delivery_implemented: report.child_device_delivery_implemented,
        cross_process_replay_implemented: remote_status.cross_process_replay_implemented,
        remote_delete_export_propagation_implemented: report
            .remote_delete_export_propagation_implemented,
        product_ready_remote_delivery: report.product_ready_remote_delivery,
        policy_authority: report.policy_authority,
        side_effect_authority: report.side_effect_authority,
        enforcement_command_event_count: count(report.enforcement_command_event_count),
        adapter_action_executed_count: count(report.adapter_action_executed_count),
        raw_pcap_available_count: count(report.raw_pcap_available_count),
        exact_url_available_count: count(report.exact_url_available_count),
        decrypted_payload_available_count: count(report.decrypted_payload_available_count),
        page_content_available_count: count(report.page_content_available_count),
        video_content_available_count: count(report.video_content_available_count),
        private_message_content_available_count: count(
            report.private_message_content_available_count,
        ),
        search_query_available_count: count(report.search_query_available_count),
    }
}

fn protocol_remote_delivery_state(
    state: NetworkRuntimeRemoteDeliveryState,
) -> NetworkRemoteDeliveryStatusState {
    match state {
        NetworkRuntimeRemoteDeliveryState::FixtureRequirementsRecordedButNotImplemented => {
            NetworkRemoteDeliveryStatusState::FixtureRequirementsRecordedButNotImplemented
        }
        NetworkRuntimeRemoteDeliveryState::ManualRequired => {
            NetworkRemoteDeliveryStatusState::ManualRequired
        }
    }
}

fn count(value: usize) -> u64 {
    u64::try_from(value).unwrap_or(u64::MAX)
}
