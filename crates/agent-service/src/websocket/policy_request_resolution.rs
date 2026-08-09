use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogLevel};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
};

use crate::{
    event_builder::build_event, fields::fields_from_pairs, json_contract::serialize_json_string,
};

#[path = "policy_request_resolution/apply.rs"]
mod apply;
#[path = "policy_request_resolution/audit.rs"]
mod audit;
#[path = "policy_request_resolution/domain.rs"]
mod domain;
#[path = "policy_request_resolution/execute.rs"]
mod execute;
#[path = "policy_request_resolution/mapping.rs"]
mod mapping;
#[path = "policy_request_resolution/result.rs"]
mod result;
#[path = "policy_request_resolution/snapshot.rs"]
mod snapshot;
#[path = "policy_request_resolution/store.rs"]
mod store;
#[path = "policy_request_resolution/types.rs"]
mod types;

pub(crate) async fn build_policy_request_parent_resolution_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let result = execute::execute(&command).await;
    let result_text = serialize_json_string(&result).0;

    build_event(
        command.message_id.as_str(),
        &command.message_id,
        command.source,
        AgentEventName::AgentPolicyRequestParentResolutionResolved,
        LogLevel::Info,
        fields_from_pairs(vec![(
            constants::field::POLICY_REQUEST_PARENT_RESOLUTION_RESULT,
            LogFieldValue::String(result_text),
        )]),
        None,
    )
}
