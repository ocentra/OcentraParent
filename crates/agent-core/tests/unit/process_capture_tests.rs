use ocentra_parent_agent_protocol::activity::{
    ActivityEventKind, ActivityObserver, ActivitySubjectKind,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;

use crate::{collect_process_snapshot, process_observation_event, ProcessObservation};

#[test]
fn process_observation_event_maps_snapshot_to_activity_contract() {
    let event = process_observation_event(
        ProcessObservation {
            pid: 4242,
            name: constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string(),
            executable_path: None,
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
    assert_eq!(
        event.fields.get(constants::field::CAPABILITY_STATUS),
        Some(&LogFieldValue::String(
            constants::activity_capture::CAPABILITY_STATUS_AVAILABLE.to_string()
        ))
    );
    assert_eq!(
        event.fields.get(constants::field::ADAPTER_ID),
        Some(&LogFieldValue::String(
            constants::activity_capture::PROCESS_ADAPTER_ID.to_string()
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
