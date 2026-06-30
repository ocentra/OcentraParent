use serde::{
    de::{self, Deserializer},
    ser::SerializeStruct,
    Deserialize, Serialize, Serializer,
};
use serde_json::{Map, Value};

use ocentra_parent_agent_core::browser_event_runtime::BrowserRuntimeReport;
use ocentra_parent_agent_protocol::browser::BrowserRuntimeEventPayload;
use ocentra_parent_agent_protocol::constants;

use crate::json_contract::serialize_json_value;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct BrowserRuntimeServiceStreamEntry {
    pub(crate) runtime_event_name: String,
    pub(crate) event_ref: String,
    pub(crate) payload: Value,
}

impl<'de> Deserialize<'de> for BrowserRuntimeServiceStreamEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut entry = Value::deserialize(deserializer)
            .and_then(|value| match value {
                Value::Object(fields) => Ok(fields),
                _ => Err(de::Error::custom(constants::error::AGENT_EVENT_SERIALIZES)),
            })
            .map_err(de::Error::custom)?;
        let runtime_event_name = take_string_field(&mut entry, constants::field::EVENT_TYPE)?;
        let event_ref = take_string_field(&mut entry, constants::field::EVENT_REF)?;
        let payload = entry
            .remove(constants::field::PAYLOAD)
            .ok_or_else(|| de::Error::missing_field(constants::field::PAYLOAD))?;
        Ok(Self {
            runtime_event_name,
            event_ref,
            payload,
        })
    }
}

fn take_string_field<E>(
    fields: &mut serde_json::Map<String, Value>,
    field_name: &'static str,
) -> Result<String, E>
where
    E: de::Error,
{
    fields
        .remove(field_name)
        .ok_or_else(|| E::missing_field(field_name))
        .and_then(|value| match value {
            Value::String(text) => Ok(text),
            _ => Err(E::invalid_type(
                de::Unexpected::Other(field_name),
                &field_name,
            )),
        })
}

impl Serialize for BrowserRuntimeServiceStreamEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut entry =
            serializer.serialize_struct(constants::field::BROWSER_RUNTIME_EVENT_CHAIN_STREAM, 3)?;
        entry.serialize_field(constants::field::EVENT_TYPE, &self.runtime_event_name)?;
        entry.serialize_field(constants::field::EVENT_REF, &self.event_ref)?;
        entry.serialize_field(constants::field::PAYLOAD, &self.payload)?;
        entry.end()
    }
}

pub(crate) fn stream_entries_from_report(
    report: &BrowserRuntimeReport,
) -> Vec<BrowserRuntimeServiceStreamEntry> {
    report
        .stored_events
        .iter()
        .filter_map(|event| {
            let decoded = event.decode::<BrowserRuntimeEventPayload>().ok()?;
            let runtime_event_name = event.contract.event_type.as_str().to_string();
            let event_ref = event_ref(
                event.correlation_id.as_str(),
                event.contract.event_type.as_str(),
            );
            let payload = protocol_payload(&decoded.payload);
            Some(BrowserRuntimeServiceStreamEntry {
                runtime_event_name,
                event_ref,
                payload,
            })
        })
        .collect()
}

fn protocol_payload(payload: &BrowserRuntimeEventPayload) -> Value {
    let mut fields = Map::new();
    insert_payload_refs(&mut fields, payload);
    insert_policy_refs(&mut fields, payload);
    insert_payload_flags(&mut fields, payload);
    Value::Object(fields)
}

fn insert_payload_refs(fields: &mut Map<String, Value>, payload: &BrowserRuntimeEventPayload) {
    insert_payload_field(fields, constants::field::PHASE, payload.phase);
    insert_payload_field(fields, constants::field::SOURCE_REF, &payload.source_ref);
    insert_payload_field(
        fields,
        constants::field::EVIDENCE_REF,
        &payload.evidence_ref,
    );
    insert_payload_field(
        fields,
        constants::field::CAPABILITY_STATUS,
        &payload.capability_status,
    );
    insert_payload_field(
        fields,
        constants::field::CUSTODY_LABEL,
        &payload.custody_label,
    );
    insert_payload_field(
        fields,
        constants::field::QUERY_VISIBILITY,
        &payload.query_visibility,
    );
    insert_payload_field(
        fields,
        constants::field::DEGRADED_REASON,
        &payload.degraded_reason,
    );
    insert_payload_field(fields, constants::field::JOURNAL_REF, &payload.journal_ref);
    insert_payload_field(
        fields,
        constants::field::AI_REQUEST_REF,
        &payload.ai_request_ref,
    );
}

fn insert_policy_refs(fields: &mut Map<String, Value>, payload: &BrowserRuntimeEventPayload) {
    insert_payload_field(
        fields,
        constants::field::AI_ANALYSIS_REF,
        &payload.ai_analysis_ref,
    );
    insert_payload_field(
        fields,
        constants::field::POLICY_EVALUATION_REF,
        &payload.policy_evaluation_ref,
    );
    insert_payload_field(
        fields,
        constants::field::POLICY_DECISION_REF,
        &payload.policy_decision_ref,
    );
    insert_payload_field(
        fields,
        constants::field::POLICY_PREVIEW_ID,
        &payload.policy_preview_id,
    );
    insert_payload_field(
        fields,
        constants::field::ACTION_INTENT_ID,
        &payload.action_intent_id,
    );
    insert_payload_field(
        fields,
        constants::field::INTERVENTION_COMMAND_REF,
        &payload.intervention_command_ref,
    );
    insert_payload_field(
        fields,
        constants::field::INTERVENTION_RESULT_REF,
        &payload.intervention_result_ref,
    );
    insert_payload_field(
        fields,
        constants::field::AUDIT_ENTRY_REF,
        &payload.audit_entry_ref,
    );
    insert_payload_field(
        fields,
        constants::field::READ_MODEL_REF,
        &payload.read_model_ref,
    );
    insert_payload_field(
        fields,
        constants::field::PREVIOUS_PHASE_REF,
        &payload.previous_phase_ref,
    );
}

fn insert_payload_flags(fields: &mut Map<String, Value>, payload: &BrowserRuntimeEventPayload) {
    insert_payload_field(
        fields,
        constants::field::EXACT_URL_CLAIMED,
        payload.exact_url_claimed,
    );
    insert_payload_field(fields, constants::field::AI_AUTHORITY, payload.ai_authority);
    insert_payload_field(
        fields,
        constants::field::POLICY_AUTHORITY,
        payload.policy_authority,
    );
    insert_payload_field(fields, constants::field::POLICY_DRY_RUN, payload.dry_run);
    insert_payload_field(
        fields,
        constants::field::ADAPTER_DISPATCH_CLAIMED,
        payload.adapter_dispatch_claimed,
    );
    insert_payload_field(
        fields,
        constants::field::INTERVENTION_COMMAND_ALLOWED,
        payload.intervention_command_allowed,
    );
    insert_payload_field(fields, constants::field::OBSERVED_AT, &payload.observed_at);
}

fn insert_payload_field<T: Serialize>(fields: &mut Map<String, Value>, field_name: &str, value: T) {
    fields.insert(field_name.to_string(), payload_json_value(value));
}

fn payload_json_value<T: Serialize>(value: T) -> Value {
    serialize_json_value(value)
}

fn event_ref(correlation_id: &str, runtime_event_name: &str) -> String {
    let mut value = String::from(correlation_id);
    value.push(constants::delimiter::HYPHEN);
    value.push_str(runtime_event_name);
    value
}
