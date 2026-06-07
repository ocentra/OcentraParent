use ocentra_parent_agent_protocol::constants;

use super::prove_network_runtime_remote_delivery_no_enforcement_invariant;
use super::remote_delivery_event_chain_store::source_component;
use super::remote_delivery_transport_dispatch_state_types::{
    NetworkRuntimeRemoteDeliveryBlockedDispatchRecord,
    NetworkRuntimeRemoteDeliveryTransportDispatchState,
    NetworkRuntimeRemoteDeliveryTransportDispatchStateError,
    NetworkRuntimeRemoteDeliveryTransportDispatchStateReport,
};
use super::{
    NetworkRuntimeRemoteDeliveryNoEnforcementInvariantReport,
    NetworkRuntimeRemoteDeliveryOutboxState,
};

pub async fn prove_network_runtime_remote_delivery_transport_dispatch_state() -> Result<
    NetworkRuntimeRemoteDeliveryTransportDispatchStateReport,
    NetworkRuntimeRemoteDeliveryTransportDispatchStateError,
> {
    let no_enforcement_invariant = prove_network_runtime_remote_delivery_no_enforcement_invariant()
        .await
        .map_err(NetworkRuntimeRemoteDeliveryTransportDispatchStateError::NoEnforcementInvariant)?;
    prove_network_runtime_remote_delivery_transport_dispatch_state_from_invariant(
        no_enforcement_invariant,
    )
}

fn prove_network_runtime_remote_delivery_transport_dispatch_state_from_invariant(
    no_enforcement_invariant: NetworkRuntimeRemoteDeliveryNoEnforcementInvariantReport,
) -> Result<
    NetworkRuntimeRemoteDeliveryTransportDispatchStateReport,
    NetworkRuntimeRemoteDeliveryTransportDispatchStateError,
> {
    if has_unsupported_claims(&no_enforcement_invariant) {
        return Err(NetworkRuntimeRemoteDeliveryTransportDispatchStateError::UnsupportedClaim);
    }
    let outbox = &no_enforcement_invariant.dispatch_readiness.outbox_handoff;
    if outbox.candidates.is_empty() {
        return Err(NetworkRuntimeRemoteDeliveryTransportDispatchStateError::EmptyOutbox);
    }
    let dispatch_state_ref = source_component(
        constants::network_flow::TEST_REMOTE_DELIVERY_TRANSPORT_DISPATCH_STATE_REF,
    )?;
    let blocked_dispatch_ref = source_component(
        constants::network_flow::TEST_REMOTE_DELIVERY_DISPATCH_BLOCKED_MANUAL_REF,
    )?;
    let future_transport_seam_ref =
        source_component(constants::network_flow::TEST_REMOTE_DELIVERY_FUTURE_TRANSPORT_SEAM_REF)?;
    let blocked_dispatch_records = outbox
        .candidates
        .iter()
        .map(
            |candidate| NetworkRuntimeRemoteDeliveryBlockedDispatchRecord {
                sequence: candidate.sequence,
                event_id: candidate.event_id.clone(),
                event_type: candidate.event_type.clone(),
                correlation_id: candidate.correlation_id.clone(),
                source_outbox_state: candidate.state,
                blocked_state:
                    NetworkRuntimeRemoteDeliveryTransportDispatchState::ManualRequiredBlocked,
                outbox_ref: candidate.outbox_ref.clone(),
                handoff_ref: candidate.handoff_ref.clone(),
                dispatch_state_ref: dispatch_state_ref.clone(),
                blocked_dispatch_ref: blocked_dispatch_ref.clone(),
                future_transport_seam_ref: future_transport_seam_ref.clone(),
            },
        )
        .collect::<Vec<_>>();

    Ok(NetworkRuntimeRemoteDeliveryTransportDispatchStateReport {
        source_outbox_candidate_count: outbox.outbox_candidate_count,
        blocked_dispatch_record_count: blocked_dispatch_records.len(),
        dispatch_ready_candidate_count: no_enforcement_invariant.dispatch_ready_candidate_count,
        dispatch_attempt_count: no_enforcement_invariant.dispatch_attempt_count,
        remote_ack_count: no_enforcement_invariant.remote_ack_count,
        broker_delivery_implemented: no_enforcement_invariant.broker_delivery_implemented,
        family_hub_delivery_implemented: no_enforcement_invariant.family_hub_delivery_implemented,
        remote_delivery_ack_implemented: no_enforcement_invariant.remote_delivery_ack_implemented,
        provider_delivery_implemented: no_enforcement_invariant.provider_delivery_implemented,
        child_device_delivery_implemented: no_enforcement_invariant
            .child_device_delivery_implemented,
        remote_delete_export_propagation_implemented: no_enforcement_invariant
            .remote_delete_export_propagation_implemented,
        product_ready_remote_delivery: no_enforcement_invariant.product_ready_remote_delivery,
        policy_authority: no_enforcement_invariant.policy_authority,
        side_effect_authority: no_enforcement_invariant.side_effect_authority,
        enforcement_command_event_count: no_enforcement_invariant.enforcement_command_event_count,
        adapter_action_executed_count: no_enforcement_invariant.adapter_action_executed_count,
        raw_pcap_available_count: no_enforcement_invariant.raw_pcap_available_count,
        exact_url_available_count: no_enforcement_invariant.exact_url_available_count,
        decrypted_payload_available_count: no_enforcement_invariant
            .decrypted_payload_available_count,
        page_content_available_count: no_enforcement_invariant.page_content_available_count,
        video_content_available_count: no_enforcement_invariant.video_content_available_count,
        private_message_content_available_count: no_enforcement_invariant
            .private_message_content_available_count,
        search_query_available_count: no_enforcement_invariant.search_query_available_count,
        dispatch_state_ref,
        blocked_dispatch_ref,
        future_transport_seam_ref,
        state: NetworkRuntimeRemoteDeliveryTransportDispatchState::ManualRequiredBlocked,
        blocked_dispatch_records,
        no_enforcement_invariant,
    })
}

fn has_unsupported_claims(
    report: &NetworkRuntimeRemoteDeliveryNoEnforcementInvariantReport,
) -> bool {
    report.dispatch_ready_candidate_count > 0
        || report.dispatch_attempt_count > 0
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn network_runtime_remote_delivery_transport_dispatch_state_blocks_without_transport() {
        let report = prove_network_runtime_remote_delivery_transport_dispatch_state()
            .await
            .expect(constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_TRANSPORT_DISPATCH_STATE);

        assert_eq!(
            report.dispatch_state_ref.as_str(),
            constants::network_flow::TEST_REMOTE_DELIVERY_TRANSPORT_DISPATCH_STATE_REF
        );
        assert_eq!(
            report.blocked_dispatch_ref.as_str(),
            constants::network_flow::TEST_REMOTE_DELIVERY_DISPATCH_BLOCKED_MANUAL_REF
        );
        assert_eq!(
            report.future_transport_seam_ref.as_str(),
            constants::network_flow::TEST_REMOTE_DELIVERY_FUTURE_TRANSPORT_SEAM_REF
        );
        assert_eq!(
            report.state,
            NetworkRuntimeRemoteDeliveryTransportDispatchState::ManualRequiredBlocked
        );
        assert_eq!(
            report.source_outbox_candidate_count,
            report.blocked_dispatch_record_count
        );
        assert_eq!(
            report.blocked_dispatch_record_count,
            report
                .no_enforcement_invariant
                .manual_required_candidate_count
        );
        assert_eq!(report.dispatch_ready_candidate_count, 0);
        assert_eq!(report.dispatch_attempt_count, 0);
        assert_eq!(report.remote_ack_count, 0);
        assert!(!report.broker_delivery_implemented);
        assert!(!report.family_hub_delivery_implemented);
        assert!(!report.remote_delivery_ack_implemented);
        assert!(!report.provider_delivery_implemented);
        assert!(!report.child_device_delivery_implemented);
        assert!(!report.remote_delete_export_propagation_implemented);
        assert!(!report.product_ready_remote_delivery);
        assert!(!report.policy_authority);
        assert!(!report.side_effect_authority);
        assert_eq!(report.enforcement_command_event_count, 0);
        assert_eq!(report.adapter_action_executed_count, 0);
        assert_eq!(report.raw_pcap_available_count, 0);
        assert_eq!(report.exact_url_available_count, 0);
        assert_eq!(report.decrypted_payload_available_count, 0);
        assert_eq!(report.page_content_available_count, 0);
        assert_eq!(report.video_content_available_count, 0);
        assert_eq!(report.private_message_content_available_count, 0);
        assert_eq!(report.search_query_available_count, 0);
        for record in &report.blocked_dispatch_records {
            assert_eq!(
                record.source_outbox_state,
                NetworkRuntimeRemoteDeliveryOutboxState::PreparedNotDispatched
            );
            assert_eq!(
                record.blocked_state,
                NetworkRuntimeRemoteDeliveryTransportDispatchState::ManualRequiredBlocked
            );
            assert_eq!(record.dispatch_state_ref, report.dispatch_state_ref);
            assert_eq!(record.blocked_dispatch_ref, report.blocked_dispatch_ref);
            assert_eq!(
                record.future_transport_seam_ref,
                report.future_transport_seam_ref
            );
        }
    }

    #[tokio::test]
    async fn network_runtime_remote_delivery_transport_dispatch_state_rejects_action_claims() {
        let mut no_enforcement_invariant =
            prove_network_runtime_remote_delivery_no_enforcement_invariant()
                .await
                .expect(
                    constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_NO_ENFORCEMENT_INVARIANT,
                );
        no_enforcement_invariant.dispatch_attempt_count = 1;
        no_enforcement_invariant.remote_ack_count = 1;

        let proof_result =
            prove_network_runtime_remote_delivery_transport_dispatch_state_from_invariant(
                no_enforcement_invariant,
            );

        assert!(matches!(
            proof_result,
            Err(NetworkRuntimeRemoteDeliveryTransportDispatchStateError::UnsupportedClaim)
        ));
    }
}
