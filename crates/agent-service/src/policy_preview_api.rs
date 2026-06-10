use ocentra_parent_agent_core::ActivityStore;
use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentEventEnvelope, AgentEventName, LogLevel,
    PolicyPreviewReadModel,
};

use crate::{
    activity_payload::activity_store_error_payload, activity_store_path::activity_db_path,
    event_builder::build_event, policy_preview_payload::policy_preview_read_model_payload,
    time::timestamp_now,
};

pub async fn build_policy_preview_read_model_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    match load_policy_preview_read_model().await {
        Some(read_model) => build_event(
            constants::event_id::POLICY_PREVIEW_READ_MODEL_REPORTED,
            &command.message_id,
            command.source,
            AgentEventName::AgentPolicyPreviewReadModelReported,
            LogLevel::Info,
            policy_preview_read_model_payload(&read_model),
            None,
        ),
        None => build_event(
            constants::event_id::POLICY_PREVIEW_READ_MODEL_REPORTED,
            &command.message_id,
            command.source,
            AgentEventName::AgentPolicyPreviewReadModelReported,
            LogLevel::Error,
            activity_store_error_payload(),
            None,
        ),
    }
}

pub(crate) async fn load_policy_preview_read_model() -> Option<PolicyPreviewReadModel> {
    let path = activity_db_path();
    let generated_at = timestamp_now();
    tokio::task::spawn_blocking(move || {
        let store = ActivityStore::open(path).ok()?;
        store
            .policy_preview_read_model(
                constants::activity_store::DEFAULT_RECENT_LIMIT,
                &generated_at,
            )
            .ok()
    })
    .await
    .ok()
    .flatten()
}
