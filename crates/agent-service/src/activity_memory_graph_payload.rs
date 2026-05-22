use ocentra_parent_agent_protocol::{
    constants, ActivityMemoryGraphReadModel, LogFieldValue, LogFields,
};

use crate::fields::fields_from_pairs;

pub fn activity_memory_graph_payload(read_model: &ActivityMemoryGraphReadModel) -> LogFields {
    let pairs = vec![
        (
            constants::field::GENERATED_AT,
            LogFieldValue::String(read_model.generated_at.clone()),
        ),
        (
            constants::field::CUSTODY,
            LogFieldValue::String(read_model.custody.clone()),
        ),
        (
            constants::field::LIMIT,
            LogFieldValue::Number(read_model.query.limit as f64),
        ),
        (
            constants::field::RETURNED,
            LogFieldValue::Number(read_model.returned_edge_count as f64),
        ),
        (
            constants::field::CAPABILITY_STATUS,
            LogFieldValue::String(read_model.capability_status.clone()),
        ),
        (
            constants::field::ACTIVITY_DIGEST,
            LogFieldValue::String(
                serde_json::to_string(read_model).expect(constants::error::AGENT_EVENT_SERIALIZES),
            ),
        ),
    ];
    fields_from_pairs(pairs)
}
