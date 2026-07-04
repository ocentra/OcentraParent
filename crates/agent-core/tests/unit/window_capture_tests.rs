use ocentra_parent_agent_protocol::activity::{
    ActivityEventKind, ActivityObserver, ActivitySubjectKind,
};
use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;

use crate::{foreground_window_observation_event, ForegroundWindowObservation};

#[test]
fn foreground_window_observation_event_maps_active_window_contract() {
    let event = foreground_window_observation_event(
        ForegroundWindowObservation::active(
            4242,
            constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string(),
            constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string(),
            constants::activity_store::TEST_WINDOW_TITLE.to_string(),
            constants::activity_store::TEST_WINDOW_ID.to_string(),
        ),
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    );

    assert_eq!(event.source.observer, ActivityObserver::WindowsWindow);
    assert_eq!(
        event.source.source_id,
        constants::activity_capture::WINDOWS_WINDOW_SOURCE_ID
    );
    assert_eq!(event.kind, ActivityEventKind::WindowFocused);
    assert_eq!(event.subject.kind, ActivitySubjectKind::Window);
    assert_eq!(
        event.subject.subject_id,
        constants::activity_store::TEST_WINDOW_SUBJECT_ID
    );
    assert_eq!(
        event.fields.get(constants::field::OBSERVATION_MODE),
        Some(&LogFieldValue::String(
            constants::activity_capture::OBSERVATION_MODE_ACTIVE_WINDOW.to_string()
        ))
    );
    assert_eq!(
        event.fields.get(constants::field::CAPABILITY_STATUS),
        Some(&LogFieldValue::String(
            constants::activity_capture::CAPABILITY_STATUS_AVAILABLE.to_string()
        ))
    );
    assert_eq!(
        event.fields.get(constants::field::FOREGROUND),
        Some(&LogFieldValue::Boolean(true))
    );
}

#[test]
fn foreground_window_observation_event_maps_degraded_status_contract() {
    let event = foreground_window_observation_event(
        ForegroundWindowObservation::degraded(ActivityCaptureCapabilityStatus::NoActiveWindow),
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    );

    assert_eq!(event.subject.kind, ActivitySubjectKind::Window);
    assert_eq!(
        event.fields.get(constants::field::CAPABILITY_STATUS),
        Some(&LogFieldValue::String(
            constants::activity_capture::CAPABILITY_STATUS_NO_ACTIVE_WINDOW.to_string()
        ))
    );
    assert_eq!(
        event.fields.get(constants::field::FOREGROUND),
        Some(&LogFieldValue::Boolean(false))
    );
    assert!(event.fields.get(constants::field::WINDOW_TITLE).is_none());
}
