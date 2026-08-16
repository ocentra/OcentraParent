use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::enforcement_broad_adapter_proof::V08BroadAdapterRuntimeProofReadModel;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;

use crate::{fields::fields_from_pairs, json_contract::serialize_json_string};

pub(crate) fn enforcement_broad_adapter_proof_payload(
    read_model: &V08BroadAdapterRuntimeProofReadModel,
) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::READ_MODEL_ID,
            LogFieldValue::String(read_model.read_model_id.clone()),
        ),
        (
            constants::field::GENERATED_AT,
            LogFieldValue::String(read_model.generated_at.clone()),
        ),
        (
            constants::field::RETURNED,
            LogFieldValue::Number(read_model.entries.len() as f64),
        ),
        (
            constants::field::ENFORCEMENT_BROAD_ADAPTER_PROOF_READ_MODEL,
            read_model_json(read_model),
        ),
    ])
}

fn read_model_json(read_model: &V08BroadAdapterRuntimeProofReadModel) -> LogFieldValue {
    LogFieldValue::String(serialize_json_string(read_model).0)
}
