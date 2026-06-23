use std::path::PathBuf;

use ocentra_parent_agent_protocol::activity::ACTIVITY_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::activity::{
    ActivityEvent, ActivityEventKind, ActivityObserver, ActivitySource, ActivitySubject,
    ActivitySubjectKind,
};
use ocentra_parent_agent_protocol::activity_capture::{
    ActivityCaptureCapabilityStatus, ActivityObservationMode,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use sysinfo::{ProcessesToUpdate, System};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProcessObservation {
    pub pid: u32,
    pub name: String,
    pub executable_path: Option<PathBuf>,
}

pub fn collect_process_snapshot(limit: usize) -> Vec<ProcessObservation> {
    let mut system = System::new();
    system.refresh_processes(ProcessesToUpdate::All, true);
    let mut observations = system
        .processes()
        .values()
        .map(|process| ProcessObservation {
            pid: process.pid().as_u32(),
            name: process.name().to_string_lossy().into_owned(),
            executable_path: process.exe().map(std::path::Path::to_path_buf),
        })
        .collect::<Vec<_>>();
    observations.sort_by(|left, right| {
        left.pid
            .cmp(&right.pid)
            .then_with(|| left.name.cmp(&right.name))
    });
    observations.truncate(limit);
    observations
}

pub fn process_snapshot_events(observed_at: &str, limit: usize) -> Vec<ActivityEvent> {
    collect_process_snapshot(limit)
        .into_iter()
        .enumerate()
        .map(|(index, observation)| process_observation_event(observation, observed_at, index))
        .collect()
}

pub fn process_observation_event(
    observation: ProcessObservation,
    observed_at: &str,
    sequence_index: usize,
) -> ActivityEvent {
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::PID.to_string(),
        LogFieldValue::Number(f64::from(observation.pid)),
    );
    fields.insert(
        constants::field::PROCESS_NAME.to_string(),
        LogFieldValue::String(observation.name.clone()),
    );
    fields.insert(
        constants::field::OBSERVATION_MODE.to_string(),
        LogFieldValue::String(
            ActivityObservationMode::Snapshot
                .as_protocol_str()
                .to_string(),
        ),
    );
    fields.insert(
        constants::field::CAPABILITY_STATUS.to_string(),
        LogFieldValue::String(
            ActivityCaptureCapabilityStatus::Available
                .as_protocol_str()
                .to_string(),
        ),
    );
    fields.insert(
        constants::field::ADAPTER_ID.to_string(),
        LogFieldValue::String(constants::activity_capture::PROCESS_ADAPTER_ID.to_string()),
    );

    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: process_event_id(&observation, observed_at, sequence_index),
        observed_at: observed_at.to_string(),
        source: ActivitySource {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: std::env::consts::OS.to_string(),
            observer: ActivityObserver::WindowsProcess,
            source_id: constants::activity_capture::WINDOWS_PROCESS_SOURCE_ID.to_string(),
        },
        kind: ActivityEventKind::ProcessObserved,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Process,
            subject_id: process_subject_id(observation.pid),
            display_name: Some(observation.name),
        },
        fields,
        evidence: Vec::new(),
    }
}

fn process_event_id(
    observation: &ProcessObservation,
    observed_at: &str,
    sequence_index: usize,
) -> String {
    let mut event_id = String::from(constants::activity_capture::PROCESS_EVENT_ID_PREFIX);
    event_id.push_str(&observation.pid.to_string());
    event_id.push(constants::delimiter::HYPHEN);
    event_id.push_str(&sequence_index.to_string());
    event_id.push(constants::delimiter::HYPHEN);
    event_id.push_str(observed_at);
    event_id
}

fn process_subject_id(pid: u32) -> String {
    let mut subject_id = String::from(constants::activity_capture::PROCESS_SUBJECT_ID_PREFIX);
    subject_id.push_str(&pid.to_string());
    subject_id
}
