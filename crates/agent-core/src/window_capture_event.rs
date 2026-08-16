use ocentra_parent_agent_protocol::activity::{
    ActivityEvent, ActivityEventKind, ActivityObserver, ActivitySource, ActivitySubject,
    ActivitySubjectKind, ACTIVITY_SCHEMA_VERSION,
};
use ocentra_parent_agent_protocol::activity_capture::{
    ActivityCaptureCapabilityStatus, ActivityObservationMode,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

use crate::window_capture::{collect_foreground_window_observation, ForegroundWindowObservation};

pub fn foreground_window_event(observed_at: &str) -> ActivityEvent {
    foreground_window_observation_event(collect_foreground_window_observation(), observed_at)
}

pub fn foreground_window_observation_event(
    observation: ForegroundWindowObservation,
    observed_at: &str,
) -> ActivityEvent {
    let ForegroundWindowObservation {
        status,
        pid,
        app_name,
        process_path,
        title,
        window_id,
    } = observation;
    let observation = ForegroundWindowObservation {
        status,
        pid,
        app_name,
        process_path,
        title,
        window_id,
    };
    let mut fields = base_fields(&observation);
    insert_optional_number(&mut fields, constants::field::PID, observation.pid);
    insert_optional_text(
        &mut fields,
        constants::field::APP_NAME,
        &observation.app_name,
    );
    insert_optional_text(
        &mut fields,
        constants::field::PROCESS_PATH,
        &observation.process_path,
    );
    insert_optional_text(
        &mut fields,
        constants::field::WINDOW_ID,
        &observation.window_id,
    );
    insert_optional_text(
        &mut fields,
        constants::field::WINDOW_TITLE,
        &observation.title,
    );

    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: window_event_id(&observation, observed_at),
        observed_at: observed_at.to_string(),
        source: ActivitySource {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: std::env::consts::OS.to_string(),
            observer: ActivityObserver::WindowsWindow,
            source_id: constants::activity_capture::WINDOWS_WINDOW_SOURCE_ID.to_string(),
        },
        kind: ActivityEventKind::WindowFocused,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Window,
            subject_id: window_subject_id(&observation),
            display_name: observation
                .title
                .clone()
                .or_else(|| observation.app_name.clone()),
        },
        fields,
        evidence: Vec::new(),
    }
}

fn base_fields(observation: &ForegroundWindowObservation) -> LogFields {
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::OBSERVATION_MODE.to_string(),
        LogFieldValue::String(
            ActivityObservationMode::ActiveWindow
                .as_protocol_str()
                .to_string(),
        ),
    );
    fields.insert(
        constants::field::CAPABILITY_STATUS.to_string(),
        LogFieldValue::String(observation.status.as_protocol_str().to_string()),
    );
    fields.insert(
        constants::field::ADAPTER_ID.to_string(),
        LogFieldValue::String(constants::activity_capture::WINDOW_ADAPTER_ID.to_string()),
    );
    fields.insert(
        constants::field::FOREGROUND.to_string(),
        LogFieldValue::Boolean(observation.status == ActivityCaptureCapabilityStatus::Available),
    );
    fields
}

fn insert_optional_number(fields: &mut LogFields, key: &str, value: Option<u32>) {
    if let Some(value) = value {
        fields.insert(key.to_string(), LogFieldValue::Number(f64::from(value)));
    }
}

fn insert_optional_text(fields: &mut LogFields, key: &str, value: &Option<String>) {
    if let Some(value) = value {
        fields.insert(key.to_string(), LogFieldValue::String(value.clone()));
    }
}

fn window_event_id(observation: &ForegroundWindowObservation, observed_at: &str) -> String {
    let mut event_id = String::from(constants::activity_capture::WINDOW_EVENT_ID_PREFIX);
    event_id.push_str(observation.status.as_protocol_str());
    event_id.push(constants::delimiter::HYPHEN);
    event_id.push_str(
        observation
            .window_id
            .as_deref()
            .unwrap_or_else(|| observation.status.as_protocol_str()),
    );
    event_id.push(constants::delimiter::HYPHEN);
    event_id.push_str(observed_at);
    event_id
}

fn window_subject_id(observation: &ForegroundWindowObservation) -> String {
    match &observation.window_id {
        Some(window_id) => {
            let mut subject_id =
                String::from(constants::activity_capture::WINDOW_SUBJECT_ID_PREFIX);
            subject_id.push_str(window_id);
            subject_id
        }
        None => {
            let mut subject_id =
                String::from(constants::activity_capture::WINDOW_STATUS_SUBJECT_ID_PREFIX);
            subject_id.push_str(observation.status.as_protocol_str());
            subject_id
        }
    }
}
