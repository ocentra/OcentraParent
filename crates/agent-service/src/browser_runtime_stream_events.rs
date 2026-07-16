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

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct BrowserRuntimeText(pub(crate) String);

impl From<BrowserRuntimeText> for String {
    fn from(value: BrowserRuntimeText) -> Self {
        value.0
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct BrowserRuntimeFieldName(&'static str);

#[derive(Clone, Debug, Default)]
struct BrowserRuntimeFields(Map<String, Value>);

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct BrowserRuntimeServiceStreamEntry {
    pub(crate) runtime_event_name: BrowserRuntimeText,
    pub(crate) event_ref: BrowserRuntimeText,
    pub(crate) payload: Value,
}

impl<'de> Deserialize<'de> for BrowserRuntimeServiceStreamEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let entry = Value::deserialize(deserializer)
            .and_then(|value| match value {
                Value::Object(fields) => Ok(fields),
                _ => Err(de::Error::custom(constants::error::AGENT_EVENT_SERIALIZES)),
            })
            .map_err(de::Error::custom)?;
        let mut fields = BrowserRuntimeFields(entry);
        let runtime_event_name = take_string_field(
            &mut fields,
            BrowserRuntimeFieldName(constants::field::EVENT_TYPE),
        )?;
        let event_ref = take_string_field(
            &mut fields,
            BrowserRuntimeFieldName(constants::field::EVENT_REF),
        )?;
        let payload = fields
            .0
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
    fields: &mut BrowserRuntimeFields,
    field_name: BrowserRuntimeFieldName,
) -> Result<BrowserRuntimeText, E>
where
    E: de::Error,
{
    fields
        .0
        .remove(field_name.0)
        .ok_or_else(|| E::missing_field(field_name.0))
        .and_then(|value| match value {
            Value::String(text) => Ok(BrowserRuntimeText(text)),
            _ => Err(E::invalid_type(
                de::Unexpected::Other(field_name.0),
                &field_name.0,
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
            let runtime_event_name =
                BrowserRuntimeText(event.contract.event_type.as_str().to_string());
            let event_ref = event_ref(
                BrowserRuntimeText(event.correlation_id.as_str().to_string()),
                &BrowserRuntimeText(event.contract.event_type.as_str().to_string()),
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
    let mut fields = BrowserRuntimeFields(Map::new());
    insert_payload_refs(&mut fields, payload);
    insert_policy_refs(&mut fields, payload);
    insert_payload_flags(&mut fields, payload);
    Value::Object(fields.0)
}

fn insert_payload_refs(fields: &mut BrowserRuntimeFields, payload: &BrowserRuntimeEventPayload) {
    insert_payload_field(
        fields,
        BrowserRuntimeFieldName(constants::field::PHASE),
        payload.phase,
    );
    insert_payload_field(
        fields,
        BrowserRuntimeFieldName(constants::field::SOURCE_REF),
        &payload.source_ref,
    );
    insert_payload_field(
        fields,
        BrowserRuntimeFieldName(constants::field::EVIDENCE_REF),
        &payload.evidence_ref,
    );
    insert_payload_field(
        fields,
        BrowserRuntimeFieldName(constants::field::CAPABILITY_STATUS),
        &payload.capability_status,
    );
    insert_payload_field(
        fields,
        BrowserRuntimeFieldName(constants::field::CUSTODY_LABEL),
        &payload.custody_label,
    );
    insert_payload_field(
        fields,
        BrowserRuntimeFieldName(constants::field::QUERY_VISIBILITY),
        &payload.query_visibility,
    );
    insert_payload_field(
        fields,
        BrowserRuntimeFieldName(constants::field::DEGRADED_REASON),
        &payload.degraded_reason,
    );
    insert_payload_field(
        fields,
        BrowserRuntimeFieldName(constants::field::JOURNAL_REF),
        &payload.journal_ref,
    );
    insert_payload_field(
        fields,
        BrowserRuntimeFieldName(constants::field::AI_REQUEST_REF),
        &payload.ai_request_ref,
    );
}

fn insert_policy_refs(fields: &mut BrowserRuntimeFields, payload: &BrowserRuntimeEventPayload) {
    insert_payload_field(
        fields,
        BrowserRuntimeFieldName(constants::field::AI_ANALYSIS_REF),
        &payload.ai_analysis_ref,
    );
    insert_payload_field(
        fields,
        BrowserRuntimeFieldName(constants::field::POLICY_EVALUATION_REF),
        &payload.policy_evaluation_ref,
    );
    insert_payload_field(
        fields,
        BrowserRuntimeFieldName(constants::field::POLICY_DECISION_REF),
        &payload.policy_decision_ref,
    );
    insert_payload_field(
        fields,
        BrowserRuntimeFieldName(constants::field::POLICY_PREVIEW_ID),
        &payload.policy_preview_id,
    );
    insert_payload_field(
        fields,
        BrowserRuntimeFieldName(constants::field::ACTION_INTENT_ID),
        &payload.action_intent_id,
    );
    insert_payload_field(
        fields,
        BrowserRuntimeFieldName(constants::field::INTERVENTION_COMMAND_REF),
        &payload.intervention_command_ref,
    );
    insert_payload_field(
        fields,
        BrowserRuntimeFieldName(constants::field::INTERVENTION_RESULT_REF),
        &payload.intervention_result_ref,
    );
    insert_payload_field(
        fields,
        BrowserRuntimeFieldName(constants::field::AUDIT_ENTRY_REF),
        &payload.audit_entry_ref,
    );
    insert_payload_field(
        fields,
        BrowserRuntimeFieldName(constants::field::READ_MODEL_REF),
        &payload.read_model_ref,
    );
    insert_payload_field(
        fields,
        BrowserRuntimeFieldName(constants::field::PREVIOUS_PHASE_REF),
        &payload.previous_phase_ref,
    );
}

fn insert_payload_flags(fields: &mut BrowserRuntimeFields, payload: &BrowserRuntimeEventPayload) {
    insert_payload_field(
        fields,
        BrowserRuntimeFieldName(constants::field::EXACT_URL_CLAIMED),
        payload.exact_url_claimed,
    );
    insert_payload_field(
        fields,
        BrowserRuntimeFieldName(constants::field::AI_AUTHORITY),
        payload.ai_authority,
    );
    insert_payload_field(
        fields,
        BrowserRuntimeFieldName(constants::field::POLICY_AUTHORITY),
        payload.policy_authority,
    );
    insert_payload_field(
        fields,
        BrowserRuntimeFieldName(constants::field::POLICY_DRY_RUN),
        payload.dry_run,
    );
    insert_payload_field(
        fields,
        BrowserRuntimeFieldName(constants::field::ADAPTER_DISPATCH_CLAIMED),
        payload.adapter_dispatch_claimed,
    );
    insert_payload_field(
        fields,
        BrowserRuntimeFieldName(constants::field::INTERVENTION_COMMAND_ALLOWED),
        payload.intervention_command_allowed,
    );
    insert_payload_field(
        fields,
        BrowserRuntimeFieldName(constants::field::OBSERVED_AT),
        &payload.observed_at,
    );
}

fn insert_payload_field<T: Serialize>(
    fields: &mut BrowserRuntimeFields,
    field_name: BrowserRuntimeFieldName,
    value: T,
) {
    fields
        .0
        .insert(field_name.0.to_string(), payload_json_value(value));
}

fn payload_json_value<T: Serialize>(value: T) -> Value {
    serialize_json_value(value)
}

fn event_ref(
    correlation_id: BrowserRuntimeText,
    runtime_event_name: &BrowserRuntimeText,
) -> BrowserRuntimeText {
    let mut value = correlation_id.0;
    value.push(constants::delimiter::HYPHEN);
    value.push_str(&runtime_event_name.0);
    BrowserRuntimeText(value)
}
