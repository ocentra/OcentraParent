use ocentra_parent_agent_protocol::activity::ACTIVITY_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::activity::{
    ActivityEvent, ActivityEventKind, ActivityEvidenceRef, ActivityObserver, ActivitySource,
    ActivitySubject, ActivitySubjectKind,
};
use ocentra_parent_agent_protocol::app_game::{
    APP_GAME_JOURNAL_CUSTODY_LOCAL_JOURNAL, APP_GAME_JOURNAL_FIELD_CLASSIFICATION_STATE,
    APP_GAME_JOURNAL_FIELD_CUSTODY_LABEL, APP_GAME_JOURNAL_FIELD_REPLAY_STATE,
    APP_GAME_JOURNAL_FIELD_ROW_JSON, APP_GAME_JOURNAL_FIELD_ROW_KIND,
    APP_GAME_JOURNAL_REPLAY_STATE_STORED, APP_GAME_JOURNAL_SOURCE_ID,
};
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

pub(super) fn fields_for_row(
    row_kind: &str,
    row_json: &str,
    classification_state: Option<&str>,
) -> LogFields {
    let mut fields = LogFields::new();
    insert_string(&mut fields, APP_GAME_JOURNAL_FIELD_ROW_KIND, row_kind);
    insert_string(
        &mut fields,
        APP_GAME_JOURNAL_FIELD_CUSTODY_LABEL,
        APP_GAME_JOURNAL_CUSTODY_LOCAL_JOURNAL,
    );
    insert_string(
        &mut fields,
        APP_GAME_JOURNAL_FIELD_REPLAY_STATE,
        APP_GAME_JOURNAL_REPLAY_STATE_STORED,
    );
    if let Some(classification_state) = classification_state {
        insert_string(
            &mut fields,
            APP_GAME_JOURNAL_FIELD_CLASSIFICATION_STATE,
            classification_state,
        );
    }
    insert_string(&mut fields, APP_GAME_JOURNAL_FIELD_ROW_JSON, row_json);
    fields
}

pub(super) struct ActivityEventInput<'a> {
    pub event_id: String,
    pub observed_at: String,
    pub observer: ActivityObserver,
    pub kind: ActivityEventKind,
    pub subject_kind: ActivitySubjectKind,
    pub subject_id: String,
    pub display_name: Option<String>,
    pub device_id: &'a str,
    pub platform: &'a str,
    pub fields: LogFields,
    pub evidence: Vec<ActivityEvidenceRef>,
}

pub(super) fn activity_event(input: ActivityEventInput<'_>) -> ActivityEvent {
    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: input.event_id,
        observed_at: input.observed_at,
        source: ActivitySource {
            device_id: input.device_id.to_string(),
            platform: input.platform.to_string(),
            observer: input.observer,
            source_id: APP_GAME_JOURNAL_SOURCE_ID.to_string(),
        },
        kind: input.kind,
        subject: ActivitySubject {
            kind: input.subject_kind,
            subject_id: input.subject_id,
            display_name: input.display_name,
        },
        fields: input.fields,
        evidence: input.evidence,
    }
}

pub(super) fn insert_string(fields: &mut LogFields, key: &str, value: &str) {
    fields.insert(key.to_string(), LogFieldValue::String(value.to_string()));
}

pub(super) fn insert_number(fields: &mut LogFields, key: &str, value: u64) {
    fields.insert(key.to_string(), LogFieldValue::Number(value as f64));
}

pub(super) fn insert_boolean(fields: &mut LogFields, key: &str, value: bool) {
    fields.insert(key.to_string(), LogFieldValue::Boolean(value));
}
