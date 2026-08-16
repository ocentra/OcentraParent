use super::super::super::*;

struct PayloadField(String);

macro_rules! payload_field {
    ($value:expr) => {
        PayloadField($value.to_string())
    };
}

struct NetworkFlowResponsePayload(std::collections::BTreeMap<String, LogFieldValue>);

impl NetworkFlowResponsePayload {
    fn new() -> Self {
        Self(std::collections::BTreeMap::new())
    }

    fn insert(&mut self, field: PayloadField, value: LogFieldValue) {
        self.0.insert(field.0, value);
    }
}

pub(crate) fn sample_screen_read_model() -> ActivityScreenReadModel {
    ActivityScreenReadModel {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        request: ActivitySurfaceRequest {
            schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
            scope: ActivitySurfaceScope {
                scope_kind: ActivitySurfaceScopeKind::Device,
                family_id: Some("family-1".to_string()),
                device_id: Some("child-device-1".to_string()),
            },
            requested_at: "2026-06-27T17:40:00Z".to_string(),
            range_start: "2026-06-27T17:35:00Z".to_string(),
            range_end: "2026-06-27T17:40:00Z".to_string(),
        },
        state: ActivityReadModelState::Ready,
        generated_at: "2026-06-27T17:40:00Z".to_string(),
        summary: "1 screen row ready".to_string(),
        rows: vec![ActivityScreenReadModelRow {
            row_id: "screen-ready-row-1".to_string(),
            label: "screen-ready-row".to_string(),
            device_id: "child-device-1".to_string(),
            state: ActivityReadModelState::Ready,
            total_ms: 120_000,
            foreground_ms: 90_000,
            background_ms: 30_000,
            capture_reason: "scheduled-capture".to_string(),
            capture_scope: "foreground".to_string(),
            capability_status: "screen-capture-ready".to_string(),
            queue_job_id: "screen-job-1".to_string(),
            model_runtime_ref: "screen-runtime-1".to_string(),
            model_id: "gpt-4.1-mini".to_string(),
            provider_kind: "openai".to_string(),
            prompt_or_template_version: "screen-v1".to_string(),
            primary_category: Some("video".to_string()),
            confidence: 0.93,
            image_deletion_state: "kept".to_string(),
            raw_image_retained: true,
            policy_eligible: true,
            image_digest: "digest-screen-1".to_string(),
            custody_state: "child-device-local-summary".to_string(),
            evidence: vec![ActivityEvidenceRef {
                evidence_id: "evidence.screen.1".to_string(),
                kind: ActivityEvidenceKind::Screenshot,
                digest: Some("digest-screen-1".to_string()),
                uri: Some("file:///captures/screen-1.png".to_string()),
            }],
            policy_decision_ref: Some("policy.screen.1".to_string()),
            policy_action: Some("allow".to_string()),
            policy_reason_codes: vec!["screen-policy-ready".to_string()],
            parent_rule_refs: vec!["rule.screen.1".to_string()],
            local_model_runtime_refs: vec!["screen-runtime-1".to_string()],
            parent_explanation_refs: vec!["screen-explanation.1".to_string()],
            explanation_reasons: vec!["family-rules-reviewed".to_string()],
            deletion_reasons: vec!["retention-window-open".to_string()],
            ocr_text_snippets: vec!["Sample OCR text".to_string()],
            redaction_notes: vec!["Faces blurred".to_string()],
        }],
    }
}

pub(crate) fn screen_read_model_response_event() -> AgentEventEnvelope {
    let read_model = sample_screen_read_model();
    let mut payload = std::collections::BTreeMap::new();
    payload.insert(
        constants::field::ACTIVITY_READ_MODEL_KIND.to_string(),
        LogFieldValue::String(constants::activity_surface::READ_MODEL_SCREEN.to_string()),
    );
    payload.insert(
        constants::field::ACTIVITY_READ_MODEL.to_string(),
        LogFieldValue::String(require_ok(
            serde_json::to_string(&read_model),
            "screen read model serializes",
        )),
    );

    AgentEventEnvelope {
        schema_version: 1,
        event_id: "agent.activity.screen.read-model.reported-1".to_string(),
        correlation_id: "screen-read-model".to_string(),
        sent_at: "2026-06-27T17:40:00Z".to_string(),
        source: AgentPeer {
            peer_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            role: AgentPeerRole::AgentService,
        },
        target: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        event: AgentEventName::AgentActivityScreenReadModelReported,
        severity: LogLevel::Info,
        payload: payload.into(),
        snapshot: None,
    }
}

pub(crate) fn network_flow_response_event() -> AgentEventEnvelope {
    let read_model = sample_network_flow_read_model();
    let row = &read_model.rows[0];
    let digest = ActivityNetworkFlowDigest {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        generated_at: read_model.generated_at.clone(),
        custody: read_model.custody.clone(),
        evidence: row.evidence.clone(),
        top_processes: Vec::new(),
        top_destinations: Vec::new(),
        unusual_indicators: Vec::new(),
    };
    let payload = network_flow_response_event_payload(&read_model, row, &digest);

    AgentEventEnvelope {
        schema_version: 1,
        event_id: "agent.network.flow.read-model.reported-1".to_string(),
        correlation_id: "network-flow".to_string(),
        sent_at: "2026-06-23T00:00:00Z".to_string(),
        source: AgentPeer {
            peer_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            role: AgentPeerRole::AgentService,
        },
        target: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        event: AgentEventName::AgentNetworkFlowReadModelReported,
        severity: LogLevel::Info,
        payload: payload.0.into(),
        snapshot: None,
    }
}

fn network_flow_response_event_payload(
    read_model: &ActivityNetworkFlowReadModel,
    row: &ActivityNetworkFlowObservation,
    digest: &ActivityNetworkFlowDigest,
) -> NetworkFlowResponsePayload {
    let mut payload = NetworkFlowResponsePayload::new();
    insert_network_flow_response_event_summary_fields(&mut payload, read_model, row, digest);
    insert_network_flow_response_event_endpoint_fields(&mut payload, row);
    payload
}

fn insert_network_flow_response_event_summary_fields(
    payload: &mut NetworkFlowResponsePayload,
    read_model: &ActivityNetworkFlowReadModel,
    row: &ActivityNetworkFlowObservation,
    digest: &ActivityNetworkFlowDigest,
) {
    payload.insert(
        payload_field!(constants::field::GENERATED_AT),
        LogFieldValue::String(read_model.generated_at.clone()),
    );
    payload.insert(
        payload_field!(constants::field::CUSTODY),
        LogFieldValue::String(read_model.custody.clone()),
    );
    payload.insert(
        payload_field!(constants::field::LIMIT),
        LogFieldValue::Number(read_model.limit as f64),
    );
    payload.insert(
        payload_field!(constants::field::RETURNED),
        LogFieldValue::Number(read_model.returned as f64),
    );
    payload.insert(
        payload_field!(NETWORK_FLOW_READ_MODEL_FIELD_ACTIVE_ROWS),
        LogFieldValue::Number(read_model.active_rows as f64),
    );
    payload.insert(
        payload_field!(NETWORK_FLOW_READ_MODEL_FIELD_TOMBSTONE_ROWS),
        LogFieldValue::Number(read_model.tombstone_rows as f64),
    );
    payload.insert(
        payload_field!(NETWORK_FLOW_READ_MODEL_FIELD_EXPORTABLE_ROWS),
        LogFieldValue::Number(read_model.exportable_rows as f64),
    );
    payload.insert(
        payload_field!(constants::field::CAPABILITY_STATUS),
        LogFieldValue::String(read_model.capability_status.clone()),
    );
    payload.insert(
        payload_field!(constants::field::LATEST_EVENT_ID),
        LogFieldValue::String(row.event_id.clone()),
    );
    payload.insert(
        payload_field!(constants::field::LATEST_OBSERVED_AT),
        LogFieldValue::String(row.observed_at.clone()),
    );
    payload.insert(
        payload_field!(NETWORK_FLOW_READ_MODEL_FIELD_LATEST_TOMBSTONE_EVENT_ID),
        LogFieldValue::String(String::new()),
    );
    payload.insert(
        payload_field!(NETWORK_FLOW_READ_MODEL_FIELD_LATEST_TOMBSTONE_OBSERVED_AT),
        LogFieldValue::String(String::new()),
    );
    payload.insert(
        payload_field!(NETWORK_FLOW_READ_MODEL_FIELD_DELETED_EVIDENCE_REFERENCE_IDS),
        LogFieldValue::String(read_model.deleted_evidence_reference_ids.join(",")),
    );
    payload.insert(
        payload_field!(constants::field::ACTIVITY_DIGEST),
        LogFieldValue::String(require_ok(
            serde_json::to_string(&digest),
            "network flow digest serializes",
        )),
    );
}

fn insert_network_flow_response_event_endpoint_fields(
    payload: &mut NetworkFlowResponsePayload,
    row: &ActivityNetworkFlowObservation,
) {
    payload.insert(
        payload_field!(constants::field::OBSERVER),
        LogFieldValue::String(row.observer.clone()),
    );
    payload.insert(
        payload_field!(constants::field::ADAPTER_ID),
        LogFieldValue::String(row.adapter_id.clone()),
    );
    payload.insert(
        payload_field!(constants::field::NETWORK_PROTOCOL),
        LogFieldValue::String(row.protocol.clone().unwrap_or_default()),
    );
    payload.insert(
        payload_field!(constants::field::TCP_STATE),
        LogFieldValue::String(row.tcp_state.clone().unwrap_or_default()),
    );
    payload.insert(
        payload_field!(constants::field::LOCAL_IP),
        LogFieldValue::String(row.local_endpoint.ip.clone().unwrap_or_default()),
    );
    payload.insert(
        payload_field!(constants::field::LOCAL_PORT),
        LogFieldValue::Number(row.local_endpoint.port.unwrap_or_default() as f64),
    );
    payload.insert(
        payload_field!(constants::field::DESTINATION_IP),
        LogFieldValue::String(row.destination_endpoint.ip.clone().unwrap_or_default()),
    );
    payload.insert(
        payload_field!(constants::field::DESTINATION_PORT),
        LogFieldValue::Number(row.destination_endpoint.port.unwrap_or_default() as f64),
    );
    payload.insert(
        payload_field!(constants::field::DESTINATION_DOMAIN),
        LogFieldValue::String(row.destination_domain.clone().unwrap_or_default()),
    );
    payload.insert(
        payload_field!(constants::field::DOMAIN_ATTRIBUTION_STATUS),
        LogFieldValue::String(row.domain_attribution_status.clone()),
    );
    payload.insert(
        payload_field!(constants::field::PROCESS_ATTRIBUTION_STATUS),
        LogFieldValue::String(row.process_attribution_status.clone()),
    );
    payload.insert(
        payload_field!(constants::field::PROCESS_ID),
        LogFieldValue::Number(row.process_id.unwrap_or_default() as f64),
    );
    payload.insert(
        payload_field!(constants::field::PROCESS_NAME),
        LogFieldValue::String(row.process_name.clone().unwrap_or_default()),
    );
    payload.insert(
        payload_field!(constants::field::CONNECTION_COUNT),
        LogFieldValue::Number(row.counters.connection_count as f64),
    );
    payload.insert(
        payload_field!(constants::field::BYTES_SENT),
        LogFieldValue::Number(row.counters.bytes_sent.unwrap_or_default() as f64),
    );
    payload.insert(
        payload_field!(constants::field::BYTES_RECEIVED),
        LogFieldValue::Number(row.counters.bytes_received.unwrap_or_default() as f64),
    );
    payload.insert(
        payload_field!(constants::field::FIRST_SEEN_AT),
        LogFieldValue::String(row.counters.first_seen_at.clone().unwrap_or_default()),
    );
    payload.insert(
        payload_field!(constants::field::LAST_SEEN_AT),
        LogFieldValue::String(row.counters.last_seen_at.clone().unwrap_or_default()),
    );
}

pub(crate) fn sample_network_flow_read_model() -> ActivityNetworkFlowReadModel {
    ActivityNetworkFlowReadModel {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        generated_at: "2026-06-25T15:00:43.552Z".to_string(),
        custody: "parent-owned-export".to_string(),
        limit: 50,
        returned: 1,
        active_rows: 1,
        tombstone_rows: 0,
        exportable_rows: 1,
        capability_status: "reported".to_string(),
        latest_event_id: Some("network-ui-flow-1".to_string()),
        latest_observed_at: Some("2026-06-25T15:00:43.552Z".to_string()),
        latest_tombstone_event_id: None,
        latest_tombstone_observed_at: None,
        deleted_evidence_reference_ids: vec!["network-ui-journal-1".to_string()],
        rows: vec![ActivityNetworkFlowObservation {
            schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
            event_id: "network-ui-flow-1".to_string(),
            observed_at: "2026-06-25T15:00:43.552Z".to_string(),
            observer: "windows-wfp-gate".to_string(),
            capability_status: "reported".to_string(),
            adapter_id: "activity-store".to_string(),
            protocol: Some("tcp".to_string()),
            tcp_state: Some("established".to_string()),
            local_endpoint: ActivityNetworkEndpoint {
                ip: Some("192.168.1.10".to_string()),
                port: Some(49712),
            },
            destination_endpoint: ActivityNetworkEndpoint {
                ip: Some("93.184.216.34".to_string()),
                port: Some(443),
            },
            destination_domain: Some("example-network.test".to_string()),
            domain_attribution_status: "reported".to_string(),
            process_attribution_status: "reported".to_string(),
            process_id: Some(4420),
            process_name: Some("notepad.exe".to_string()),
            counters: ActivityNetworkFlowCounters {
                connection_count: 1,
                bytes_sent: Some(2048),
                bytes_received: Some(4096),
                first_seen_at: Some("2026-06-25T14:58:12.000Z".to_string()),
                last_seen_at: Some("2026-06-25T15:00:43.552Z".to_string()),
            },
            evidence: vec![ActivityEvidenceRef {
                evidence_id: "network-ui-evidence-1".to_string(),
                kind: ActivityEvidenceKind::LocalDbRow,
                digest: Some("sha256:network-ui-evidence-1".to_string()),
                uri: Some("file://network-ui-evidence-1".to_string()),
            }],
        }],
    }
}

pub(crate) fn sample_tracking_read_model() -> TrackingReadModel {
    require_ok(
        serde_json::from_str(
            r#"{
                "schemaVersion": 1,
                "generatedAt": "2026-06-25T15:00:43.552Z",
                "custodyLabel": "child-device-query-store",
                "limit": 20,
                "returned": 1,
                "activeRows": 1,
                "tombstoneRows": 0,
                "capabilityStatus": "recent",
                "latestEventId": "tracking-read-model-event-1",
                "latestObservedAt": "2026-06-25T15:00:43.552Z",
                "latestActiveEventId": "tracking-read-model-event-1",
                "latestActiveObservedAt": "2026-06-25T15:00:43.552Z",
                "latestTombstoneEventId": null,
                "latestTombstoneObservedAt": null,
                "activeKindCounts": [{ "value": "tracking.expected-place.evaluated", "count": 1 }],
                "activeDeviceCounts": [{ "value": "child-device-1", "count": 1 }],
                "activeCapabilityStatusCounts": [{ "value": "recent", "count": 1 }],
                "deletedEvidenceReferenceIds": [],
                "rows": [{
                    "schemaVersion": 1,
                    "eventId": "tracking-read-model-event-1",
                    "observedAt": "2026-06-25T15:00:43.552Z",
                    "deviceId": "child-device-1",
                    "platform": "android",
                    "observer": "tracking-engine",
                    "kind": "tracking.expected-place.evaluated",
                    "subjectKind": "tracking-rule",
                    "subjectId": "expected-place-school",
                    "subjectDisplayName": "School",
                    "capabilityStatus": "recent",
                    "queryVisibility": "active",
                    "deletedAt": null,
                    "evidenceReferenceIds": ["tracking-evidence-1"],
                    "deletedEvidenceReferenceIds": [],
                    "evidence": [{
                        "evidenceId": "tracking-evidence-1",
                        "kind": "local-db-row",
                        "digest": "sha256:tracking-evidence-1",
                        "uri": null
                    }]
                }]
            }"#,
        ),
        "tracking read model fixture deserializes",
    )
}
