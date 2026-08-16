use crate::trigger_scheduler::{ScreenCaptureScheduleTrigger, ScreenCaptureSuppressionReason};
use ocentra_parent_agent_protocol::constants::activity_capture as protocol_constants;

const TRIGGER_LABELS: &[(ScreenCaptureScheduleTrigger, &str)] = &[
    (
        ScreenCaptureScheduleTrigger::ManagedBrowserUrlChange,
        protocol_constants::SCREEN_TRIGGER_MANAGED_BROWSER_URL_CHANGE,
    ),
    (
        ScreenCaptureScheduleTrigger::BrowserGameDetected,
        protocol_constants::SCREEN_TRIGGER_BROWSER_GAME_DETECTED,
    ),
    (
        ScreenCaptureScheduleTrigger::NativeAppForegroundStart,
        protocol_constants::SCREEN_TRIGGER_NATIVE_APP_FOREGROUND_START,
    ),
    (
        ScreenCaptureScheduleTrigger::NativeGameForegroundStart,
        protocol_constants::SCREEN_TRIGGER_NATIVE_GAME_FOREGROUND_START,
    ),
    (
        ScreenCaptureScheduleTrigger::LauncherForegroundStart,
        protocol_constants::SCREEN_TRIGGER_LAUNCHER_FOREGROUND_START,
    ),
    (
        ScreenCaptureScheduleTrigger::UnknownProcessForegroundStart,
        protocol_constants::SCREEN_TRIGGER_UNKNOWN_PROCESS_FOREGROUND_START,
    ),
    (
        ScreenCaptureScheduleTrigger::UnusualNetworkChange,
        protocol_constants::SCREEN_TRIGGER_UNUSUAL_NETWORK_CHANGE,
    ),
    (
        ScreenCaptureScheduleTrigger::PolicyAmbiguity,
        protocol_constants::SCREEN_TRIGGER_POLICY_AMBIGUITY,
    ),
    (
        ScreenCaptureScheduleTrigger::ParentManualTestCapture,
        protocol_constants::SCREEN_TRIGGER_PARENT_MANUAL_TEST_CAPTURE,
    ),
    (
        ScreenCaptureScheduleTrigger::TimedCadence,
        protocol_constants::SCREEN_TRIGGER_TIMED_CADENCE,
    ),
];

const SUPPRESSION_LABELS: &[(ScreenCaptureSuppressionReason, &str)] = &[
    (
        ScreenCaptureSuppressionReason::DisabledByParent,
        protocol_constants::SCREEN_SUPPRESSION_DISABLED_BY_PARENT,
    ),
    (
        ScreenCaptureSuppressionReason::TriggerCaptureDisabled,
        protocol_constants::SCREEN_SUPPRESSION_TRIGGER_CAPTURE_DISABLED,
    ),
    (
        ScreenCaptureSuppressionReason::TriggerNotEnabled,
        protocol_constants::SCREEN_SUPPRESSION_TRIGGER_NOT_ENABLED,
    ),
    (
        ScreenCaptureSuppressionReason::CadenceCaptureDisabled,
        protocol_constants::SCREEN_SUPPRESSION_CADENCE_CAPTURE_DISABLED,
    ),
    (
        ScreenCaptureSuppressionReason::CadenceNotDue,
        protocol_constants::SCREEN_SUPPRESSION_CADENCE_NOT_DUE,
    ),
    (
        ScreenCaptureSuppressionReason::TriggerDebounced,
        protocol_constants::SCREEN_SUPPRESSION_TRIGGER_DEBOUNCED,
    ),
    (
        ScreenCaptureSuppressionReason::UnsupportedScope,
        protocol_constants::SCREEN_SUPPRESSION_UNSUPPORTED_SCOPE,
    ),
];

pub(super) fn trigger_from_proof_label(value: &str) -> Option<ScreenCaptureScheduleTrigger> {
    TRIGGER_LABELS
        .iter()
        .find_map(|(trigger, label)| (*label == value).then_some(*trigger))
}

pub(super) fn trigger_proof_label(trigger: ScreenCaptureScheduleTrigger) -> &'static str {
    TRIGGER_LABELS
        .iter()
        .find_map(|(candidate, label)| (*candidate == trigger).then_some(*label))
        .unwrap_or(protocol_constants::SCREEN_TRIGGER_UNKNOWN_PROCESS_FOREGROUND_START)
}

pub(super) fn suppression_proof_label(reason: ScreenCaptureSuppressionReason) -> &'static str {
    SUPPRESSION_LABELS
        .iter()
        .find_map(|(candidate, label)| (*candidate == reason).then_some(*label))
        .unwrap_or(protocol_constants::SCREEN_SUPPRESSION_UNSUPPORTED_SCOPE)
}
