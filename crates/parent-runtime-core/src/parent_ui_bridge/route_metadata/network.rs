use super::common::{parse_identifier, parse_identifier_list, parse_optional_identifier};
use super::*;

pub(super) fn network_flow_read_model_snapshot(
    read_model: &ActivityNetworkFlowReadModel,
) -> ParentActivityNetworkFlowReadModelSnapshot {
    ParentActivityNetworkFlowReadModelSnapshot {
        schema_version: read_model.schema_version,
        generated_at: read_model.generated_at.clone(),
        custody: read_model.custody.clone(),
        limit: read_model.limit,
        returned: read_model.returned,
        active_rows: read_model.active_rows,
        tombstone_rows: read_model.tombstone_rows,
        exportable_rows: read_model.exportable_rows,
        capability_status: read_model.capability_status.clone(),
        latest_event_id: parse_optional_identifier(read_model.latest_event_id.clone(), |value| {
            ParentRouteEventId::parse(value)
        }),
        latest_observed_at: read_model.latest_observed_at.clone(),
        latest_tombstone_event_id: parse_optional_identifier(
            read_model.latest_tombstone_event_id.clone(),
            ParentRouteEventId::parse,
        ),
        latest_tombstone_observed_at: read_model.latest_tombstone_observed_at.clone(),
        deleted_evidence_reference_ids: parse_identifier_list(
            &read_model.deleted_evidence_reference_ids,
            "network deleted_evidence_reference_ids",
            ParentEvidenceReferenceId::parse,
        ),
        rows: read_model
            .rows
            .iter()
            .map(parent_activity_network_flow_observation_snapshot)
            .collect(),
    }
}

pub(super) fn network_evidence_summary_snapshot(
    network_runtime_event_chain_stream: Option<&ParentNetworkRuntimeEventChainStreamSnapshot>,
    policy_preview_read_model: Option<&ParentPolicyPreviewReadModelSnapshot>,
) -> Option<ParentNetworkEvidenceSummarySnapshot> {
    let ai_audit_ref = latest_runtime_event_ref(network_runtime_event_chain_stream, |value| {
        value.ai_analysis_ref.clone()
    })
    .or_else(|| {
        policy_preview_read_model
            .and_then(|read_model| read_model.local_ai_result_id.as_ref())
            .map(ToString::to_string)
            .and_then(ParentContractReferenceId::parse)
    });
    let policy_decision_ref =
        latest_runtime_event_ref(network_runtime_event_chain_stream, |value| {
            value.policy_decision_ref.clone()
        })
        .or_else(|| {
            policy_preview_read_model
                .and_then(|read_model| read_model.decision_id.as_ref())
                .map(ToString::to_string)
                .and_then(ParentContractReferenceId::parse)
        });
    let network_evidence_grade =
        policy_preview_read_model.and_then(|read_model| read_model.network_evidence_grade.clone());
    let intervention_result_ref =
        latest_runtime_event_ref(network_runtime_event_chain_stream, |value| {
            value.enforcement_result_ref.clone()
        });

    if ai_audit_ref.is_none()
        && policy_decision_ref.is_none()
        && network_evidence_grade.is_none()
        && intervention_result_ref.is_none()
    {
        return None;
    }

    Some(ParentNetworkEvidenceSummarySnapshot {
        ai_audit_ref,
        policy_decision_ref,
        network_evidence_grade,
        intervention_result_ref,
    })
}

fn parent_activity_network_flow_observation_snapshot(
    row: &ActivityNetworkFlowObservation,
) -> ParentActivityNetworkFlowObservationSnapshot {
    ParentActivityNetworkFlowObservationSnapshot {
        schema_version: row.schema_version,
        event_id: parse_identifier(row.event_id.clone(), "network event_id", |value| {
            ParentRouteEventId::parse(value)
        }),
        observed_at: row.observed_at.clone(),
        observer: row.observer.clone(),
        capability_status: row.capability_status.clone(),
        adapter_id: parse_identifier(row.adapter_id.clone(), "network adapter_id", |value| {
            ParentRouteAdapterId::parse(value)
        }),
        protocol: row.protocol.clone(),
        tcp_state: row.tcp_state.clone(),
        local_endpoint: parent_activity_network_endpoint_snapshot(&row.local_endpoint),
        destination_endpoint: parent_activity_network_endpoint_snapshot(&row.destination_endpoint),
        destination_domain: row.destination_domain.clone(),
        domain_attribution_status: row.domain_attribution_status.clone(),
        process_attribution_status: row.process_attribution_status.clone(),
        process_id: row.process_id,
        process_name: row.process_name.clone(),
        counters: parent_activity_network_flow_counters_snapshot(&row.counters),
        evidence: row
            .evidence
            .iter()
            .map(|evidence| ParentActivityEvidenceRefSnapshot {
                evidence_id: parse_identifier(
                    evidence.evidence_id.clone(),
                    "network evidence_id",
                    ParentEvidenceId::parse,
                ),
                kind: serialized_enum_label(&evidence.kind),
                digest: evidence.digest.clone(),
                uri: evidence.uri.clone(),
            })
            .collect(),
    }
}

fn parent_activity_network_endpoint_snapshot(
    endpoint: &ActivityNetworkEndpoint,
) -> ParentActivityNetworkEndpointSnapshot {
    ParentActivityNetworkEndpointSnapshot {
        ip: endpoint.ip.clone(),
        port: endpoint.port,
    }
}

fn parent_activity_network_flow_counters_snapshot(
    counters: &ActivityNetworkFlowCounters,
) -> ParentActivityNetworkFlowCountersSnapshot {
    ParentActivityNetworkFlowCountersSnapshot {
        connection_count: counters.connection_count,
        bytes_sent: counters.bytes_sent,
        bytes_received: counters.bytes_received,
        first_seen_at: counters.first_seen_at.clone(),
        last_seen_at: counters.last_seen_at.clone(),
    }
}

fn latest_runtime_event_ref(
    stream: Option<&ParentNetworkRuntimeEventChainStreamSnapshot>,
    select: impl Fn(
        &ocentra_schema::parent_ui_bridge::ParentNetworkRuntimeEventValueSnapshot,
    ) -> Option<ParentContractReferenceId>,
) -> Option<ParentContractReferenceId> {
    let stream = stream?;
    for event in stream.events.iter().rev() {
        if !event.ok {
            continue;
        }
        if let Some(value) = event.value.as_ref() {
            if let Some(reference_id) = select(value) {
                return Some(reference_id);
            }
        }
    }
    None
}
