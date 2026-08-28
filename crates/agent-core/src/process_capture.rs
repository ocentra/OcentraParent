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
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System, UpdateKind};

pub type ProcessSnapshotSystem = System;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProcessObservation {
    pub pid: u32,
    pub name: String,
    pub executable_path: Option<PathBuf>,
}

pub fn live_process_snapshot_system() -> ProcessSnapshotSystem {
    let mut system = System::new();
    refresh_process_snapshot(&mut system, ProcessesToUpdate::All);
    system
}

pub fn live_process_snapshot_system_for_pid(process_id: u32) -> ProcessSnapshotSystem {
    let mut system = System::new();
    let pid = Pid::from_u32(process_id);
    let pids = [pid];
    refresh_process_snapshot(&mut system, ProcessesToUpdate::Some(&pids));
    system
}

fn refresh_process_snapshot(system: &mut ProcessSnapshotSystem, processes: ProcessesToUpdate<'_>) {
    system.refresh_processes_specifics(processes, true, process_refresh_kind());
}

fn process_refresh_kind() -> ProcessRefreshKind {
    ProcessRefreshKind::everything()
        .without_cmd()
        .without_cpu()
        .without_cwd()
        .without_disk_usage()
        .without_environ()
        .without_memory()
        .without_root()
        .without_user()
        .with_exe(UpdateKind::OnlyIfNotSet)
}

pub fn collect_process_snapshot(limit: usize) -> Vec<ProcessObservation> {
    let system = live_process_snapshot_system();
    snapshot_observations_from_system(&system, limit)
        .into_iter()
        .map(|snapshot| snapshot.observation)
        .collect()
}

pub fn process_snapshot_events_from_system(
    observed_at: &str,
    limit: usize,
    system: &ProcessSnapshotSystem,
) -> Vec<ActivityEvent> {
    snapshot_observations_from_system(system, usize::MAX)
        .into_iter()
        .filter(|snapshot| snapshot.start_time != 0)
        .take(limit)
        .enumerate()
        .map(|(index, snapshot)| {
            process_observation_event_with_generation(
                snapshot.observation,
                snapshot.start_time,
                observed_at,
                index,
            )
        })
        .collect()
}

pub fn process_snapshot_events(observed_at: &str, limit: usize) -> Vec<ActivityEvent> {
    let system = live_process_snapshot_system();
    process_snapshot_events_from_system(observed_at, limit, &system)
}

struct ProcessSnapshotObservation {
    observation: ProcessObservation,
    start_time: u64,
}

fn snapshot_observations_from_system(
    system: &ProcessSnapshotSystem,
    limit: usize,
) -> Vec<ProcessSnapshotObservation> {
    let mut observations = system
        .processes()
        .values()
        .map(|process| ProcessSnapshotObservation {
            observation: ProcessObservation {
                pid: process.pid().as_u32(),
                name: process.name().to_string_lossy().into_owned(),
                executable_path: process.exe().map(std::path::Path::to_path_buf),
            },
            start_time: process.start_time(),
        })
        .collect::<Vec<_>>();
    observations.sort_by(|left, right| {
        left.observation
            .pid
            .cmp(&right.observation.pid)
            .then_with(|| left.observation.name.cmp(&right.observation.name))
    });
    observations.truncate(limit);
    observations
}

pub fn process_observation_event(
    observation: ProcessObservation,
    observed_at: &str,
    sequence_index: usize,
) -> ActivityEvent {
    let event_id = process_event_id(&observation, observed_at, sequence_index);
    let subject_id = process_subject_id(observation.pid);
    process_observation_event_with_identity(observation, observed_at, event_id, subject_id)
}

fn process_observation_event_with_generation(
    observation: ProcessObservation,
    start_time: u64,
    observed_at: &str,
    sequence_index: usize,
) -> ActivityEvent {
    let event_id =
        generation_process_event_id(&observation, start_time, observed_at, sequence_index);
    let subject_id = generation_process_subject_id(observation.pid, start_time);
    process_observation_event_with_identity(observation, observed_at, event_id, subject_id)
}

fn process_observation_event_with_identity(
    observation: ProcessObservation,
    observed_at: &str,
    event_id: String,
    subject_id: String,
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
        event_id,
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
            subject_id,
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

fn generation_process_event_id(
    observation: &ProcessObservation,
    start_time: u64,
    observed_at: &str,
    sequence_index: usize,
) -> String {
    let mut event_id = String::from(constants::activity_capture::PROCESS_EVENT_ID_PREFIX);
    event_id.push_str(&observation.pid.to_string());
    event_id.push(constants::delimiter::HYPHEN);
    event_id.push_str(&start_time.to_string());
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

pub(crate) fn generation_process_subject_id(pid: u32, start_time: u64) -> String {
    let mut subject_id = String::from(constants::activity_capture::PROCESS_SUBJECT_ID_PREFIX);
    subject_id.push_str(&pid.to_string());
    subject_id.push(constants::delimiter::HYPHEN);
    subject_id.push_str(&start_time.to_string());
    subject_id
}
