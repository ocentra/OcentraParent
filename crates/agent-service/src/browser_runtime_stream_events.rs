use serde::{ser::SerializeStruct, Serialize, Serializer};
use serde_json::{Map, Value};

use ocentra_parent_agent_core::{BrowserRuntimeEventPayload, BrowserRuntimeReport};
use ocentra_parent_agent_protocol::constants;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct BrowserRuntimeServiceStreamEntry {
    pub(crate) event_type: String,
    pub(crate) event_ref: String,
    pub(crate) payload: Value,
}

impl Serialize for BrowserRuntimeServiceStreamEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut entry =
            serializer.serialize_struct(constants::field::BROWSER_RUNTIME_EVENT_CHAIN_STREAM, 3)?;
        entry.serialize_field(constants::field::EVENT_TYPE, &self.event_type)?;
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
            let event_type = event.contract.event_type.as_str().to_string();
            let event_ref = event_ref(
                event.correlation_id.as_str(),
                event.contract.event_type.as_str(),
            );
            let payload = protocol_payload(&decoded.payload);
            Some(BrowserRuntimeServiceStreamEntry {
                event_type,
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
    insert_payload_field(
        fields,
        constants::field::INTERVENTION_COMMAND_ALLOWED,
        payload.intervention_command_allowed,
    );
    insert_payload_field(fields, constants::field::OBSERVED_AT, &payload.observed_at);
}

fn insert_payload_field<T: Serialize>(fields: &mut Map<String, Value>, key: &str, value: T) {
    fields.insert(
        key.to_string(),
        serde_json::to_value(value).expect(constants::error::AGENT_EVENT_SERIALIZES),
    );
}

fn event_ref(correlation_id: &str, event_type: &str) -> String {
    let mut value = String::from(correlation_id);
    value.push(constants::delimiter::HYPHEN);
    value.push_str(event_type);
    value
}
