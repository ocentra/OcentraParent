use ocentra_parent_screen_capture_adapter::{
    trigger_scheduler::{
        evaluate_screen_capture_schedule, ScreenCaptureScheduleDecision,
        ScreenCaptureScheduleTrigger, ScreenCaptureSchedulerSettings, ScreenCaptureSchedulerState,
        ScreenCaptureSuppressionReason, ScreenCaptureTriggerInput,
    },
    ScreenCaptureScope,
};

const ENABLED_TRIGGERS: &[ScreenCaptureScheduleTrigger] = &[
    ScreenCaptureScheduleTrigger::ManagedBrowserUrlChange,
    ScreenCaptureScheduleTrigger::NativeAppForegroundStart,
    ScreenCaptureScheduleTrigger::NativeGameForegroundStart,
    ScreenCaptureScheduleTrigger::BrowserGameDetected,
    ScreenCaptureScheduleTrigger::UnknownProcessForegroundStart,
    ScreenCaptureScheduleTrigger::ParentManualTestCapture,
];

fn enabled_settings() -> ScreenCaptureSchedulerSettings {
    ScreenCaptureSchedulerSettings {
        screen_analysis_enabled: true,
        trigger_capture_enabled: true,
        cadence_capture_enabled: true,
        allowed_scope: ScreenCaptureScope::SelectedWindow,
        cadence_seconds: 60,
        min_trigger_gap_seconds: 10,
        enabled_triggers: ENABLED_TRIGGERS,
    }
}

fn input(trigger: ScreenCaptureScheduleTrigger) -> ScreenCaptureTriggerInput {
    ScreenCaptureTriggerInput {
        observed_at_epoch_seconds: 1_780_000_000,
        trigger,
        requested_scope: None,
    }
}

#[test]
fn managed_browser_trigger_enqueues_selected_window_capture_when_parent_enabled() {
    let decision = evaluate_screen_capture_schedule(
        &enabled_settings(),
        ScreenCaptureSchedulerState {
            last_capture_at_epoch_seconds: None,
        },
        input(ScreenCaptureScheduleTrigger::ManagedBrowserUrlChange),
    );

    assert_eq!(
        decision,
        ScreenCaptureScheduleDecision::EnqueueCapture {
            reason: ScreenCaptureScheduleTrigger::ManagedBrowserUrlChange,
            scope: ScreenCaptureScope::SelectedWindow,
        }
    );
}

#[test]
fn disabled_parent_setting_suppresses_every_trigger() {
    let decision = evaluate_screen_capture_schedule(
        &ScreenCaptureSchedulerSettings {
            screen_analysis_enabled: false,
            ..enabled_settings()
        },
        ScreenCaptureSchedulerState {
            last_capture_at_epoch_seconds: None,
        },
        input(ScreenCaptureScheduleTrigger::NativeAppForegroundStart),
    );

    assert_eq!(
        decision,
        ScreenCaptureScheduleDecision::SuppressCapture {
            reason: ScreenCaptureSuppressionReason::DisabledByParent,
        }
    );
}

#[test]
fn unknown_process_trigger_requires_parent_enabled_trigger_list() {
    let decision = evaluate_screen_capture_schedule(
        &ScreenCaptureSchedulerSettings {
            enabled_triggers: &[ScreenCaptureScheduleTrigger::ManagedBrowserUrlChange],
            ..enabled_settings()
        },
        ScreenCaptureSchedulerState {
            last_capture_at_epoch_seconds: None,
        },
        input(ScreenCaptureScheduleTrigger::UnknownProcessForegroundStart),
    );

    assert_eq!(
        decision,
        ScreenCaptureScheduleDecision::SuppressCapture {
            reason: ScreenCaptureSuppressionReason::TriggerNotEnabled,
        }
    );
}

#[test]
fn recent_trigger_is_debounced_without_losing_future_triggers() {
    let decision = evaluate_screen_capture_schedule(
        &enabled_settings(),
        ScreenCaptureSchedulerState {
            last_capture_at_epoch_seconds: Some(1_779_999_995),
        },
        input(ScreenCaptureScheduleTrigger::NativeGameForegroundStart),
    );

    assert_eq!(
        decision,
        ScreenCaptureScheduleDecision::SuppressCapture {
            reason: ScreenCaptureSuppressionReason::TriggerDebounced,
        }
    );
}

#[test]
fn cadence_tick_waits_until_configured_interval_is_due() {
    let settings = enabled_settings();
    let not_due = evaluate_screen_capture_schedule(
        &settings,
        ScreenCaptureSchedulerState {
            last_capture_at_epoch_seconds: Some(1_779_999_950),
        },
        input(ScreenCaptureScheduleTrigger::TimedCadence),
    );
    let due = evaluate_screen_capture_schedule(
        &settings,
        ScreenCaptureSchedulerState {
            last_capture_at_epoch_seconds: Some(1_779_999_900),
        },
        input(ScreenCaptureScheduleTrigger::TimedCadence),
    );

    assert_eq!(
        not_due,
        ScreenCaptureScheduleDecision::SuppressCapture {
            reason: ScreenCaptureSuppressionReason::CadenceNotDue,
        }
    );
    assert_eq!(
        due,
        ScreenCaptureScheduleDecision::EnqueueCapture {
            reason: ScreenCaptureScheduleTrigger::TimedCadence,
            scope: ScreenCaptureScope::SelectedWindow,
        }
    );
}

#[test]
fn parent_opt_in_can_request_primary_display_scope() {
    let decision = evaluate_screen_capture_schedule(
        &ScreenCaptureSchedulerSettings {
            allowed_scope: ScreenCaptureScope::PrimaryDisplay,
            ..enabled_settings()
        },
        ScreenCaptureSchedulerState {
            last_capture_at_epoch_seconds: None,
        },
        ScreenCaptureTriggerInput {
            requested_scope: Some(ScreenCaptureScope::PrimaryDisplay),
            ..input(ScreenCaptureScheduleTrigger::ParentManualTestCapture)
        },
    );

    assert_eq!(
        decision,
        ScreenCaptureScheduleDecision::EnqueueCapture {
            reason: ScreenCaptureScheduleTrigger::ParentManualTestCapture,
            scope: ScreenCaptureScope::PrimaryDisplay,
        }
    );
}
