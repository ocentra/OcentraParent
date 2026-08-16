use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::enforcement_policy_dispatch::EnforcementPolicyDispatchReadModel;
use ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlSpineReadModel;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;

use crate::{fields::fields_from_pairs, json_contract::serialize_json_string};

pub(crate) fn enforcement_product_control_spine_payload(
    read_model: &V08EnforcementProductControlSpineReadModel,
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
            constants::field::ENFORCEMENT_PRODUCT_CONTROL_SPINE_READ_MODEL,
            read_model_json(read_model),
        ),
    ])
}

fn read_model_json(read_model: &V08EnforcementProductControlSpineReadModel) -> LogFieldValue {
    LogFieldValue::String(serialize_json_string(read_model).0)
}

pub(crate) fn enforcement_policy_dispatch_payload(
    read_model: &EnforcementPolicyDispatchReadModel,
) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::READ_MODEL_ID,
            LogFieldValue::String(read_model.read_model_id.clone()),
        ),
        (
            constants::field::RETURNED,
            LogFieldValue::Number(read_model.entries.len() as f64),
        ),
        (
            constants::field::ENFORCEMENT_POLICY_DISPATCH_READ_MODEL,
            policy_dispatch_read_model_json(read_model),
        ),
    ])
}

fn policy_dispatch_read_model_json(
    read_model: &EnforcementPolicyDispatchReadModel,
) -> LogFieldValue {
    LogFieldValue::String(serialize_json_string(read_model).0)
}
