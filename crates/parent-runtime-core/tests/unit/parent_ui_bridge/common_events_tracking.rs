use super::super::super::*;
use super::samples::sample_tracking_read_model;
pub(crate) fn tracking_read_model_response_event() -> AgentEventEnvelope {
    let read_model = sample_tracking_read_model();
    let mut payload = std::collections::BTreeMap::new();
    payload.insert(
        constants::field::ACTIVITY_TRACKING_READ_MODEL.to_string(),
        LogFieldValue::String(require_ok(
            serde_json::to_string(&read_model),
            "tracking read model serializes",
        )),
    );

    AgentEventEnvelope {
        schema_version: 1,
        event_id: "agent.activity.tracking.read-model.reported-1".to_string(),
        correlation_id: "tracking-read-model".to_string(),
        sent_at: "2026-06-25T15:00:43.552Z".to_string(),
        source: AgentPeer {
            peer_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            role: AgentPeerRole::AgentService,
        },
        target: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        event: AgentEventName::AgentActivityTrackingReadModelReported,
        severity: LogLevel::Info,
        payload: payload.into(),
        snapshot: None,
    }
}
