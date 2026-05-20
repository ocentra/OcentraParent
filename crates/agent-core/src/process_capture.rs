use ocentra_parent_agent_protocol::{
    constants, ActivityEvent, ActivityEventKind, ActivityObserver, ActivitySource, ActivitySubject,
    ActivitySubjectKind, LogFieldValue, LogFields, ACTIVITY_SCHEMA_VERSION,
};
use sysinfo::{ProcessesToUpdate, System};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProcessObservation {
    pub pid: u32,
    pub name: String,
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
        LogFieldValue::String(constants::activity_capture::OBSERVATION_MODE_SNAPSHOT.to_string()),
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

#[cfg(test)]
mod tests {
    use ocentra_parent_agent_protocol::{
        constants, ActivityEventKind, ActivityObserver, ActivitySubjectKind, LogFieldValue,
    };

    use super::{collect_process_snapshot, process_observation_event, ProcessObservation};

    #[test]
    fn process_observation_event_maps_snapshot_to_activity_contract() {
        let event = process_observation_event(
            ProcessObservation {
                pid: 4242,
                name: constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string(),
            },
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            0,
        );

        assert_eq!(event.source.observer, ActivityObserver::WindowsProcess);
        assert_eq!(
            event.source.source_id,
            constants::activity_capture::WINDOWS_PROCESS_SOURCE_ID
        );
        assert_eq!(event.kind, ActivityEventKind::ProcessObserved);
        assert_eq!(event.subject.kind, ActivitySubjectKind::Process);
        assert_eq!(
            event.subject.subject_id,
            constants::activity_store::TEST_PROCESS_SUBJECT_ID
        );
        assert_eq!(
            event.fields.get(constants::field::PID),
            Some(&LogFieldValue::Number(4242.0))
        );
        assert_eq!(
            event.fields.get(constants::field::OBSERVATION_MODE),
            Some(&LogFieldValue::String(
                constants::activity_capture::OBSERVATION_MODE_SNAPSHOT.to_string()
            ))
        );
    }

    #[test]
    fn collect_process_snapshot_observes_current_process() {
        let observations = collect_process_snapshot(usize::MAX);
        let current_pid = std::process::id();

        assert!(observations
            .iter()
            .any(|observation| observation.pid == current_pid));
    }
}
