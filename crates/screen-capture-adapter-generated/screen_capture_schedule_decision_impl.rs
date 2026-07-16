use std::{env, fs::write, path::PathBuf};

use ocentra_parent_agent_protocol::constants as protocol_constants;
use ocentra_parent_screen_capture_adapter::{
    trigger_scheduler::{
        evaluate_screen_capture_schedule, ScreenCaptureScheduleTrigger,
        ScreenCaptureSchedulerSettings, ScreenCaptureSchedulerState,
    },
    ScreenCaptureScope,
};

mod constants {
    pub const DEFAULT_OUTPUT_DIR: &str = "output/screen-plan-proof/real-capture/scheduler-decision";
    pub const DECISION_FILE: &str = "00-scheduler-decision.json";
    pub const ERROR_SEPARATOR: &str = ": ";
    pub const ENV_TRIGGER: &str = "OCENTRA_SCREEN_CAPTURE_TRIGGER";
    pub const ENV_ANALYSIS_ENABLED: &str = "OCENTRA_SCREEN_ANALYSIS_ENABLED";
    pub const ENV_TRIGGER_CAPTURE_ENABLED: &str = "OCENTRA_SCREEN_TRIGGER_CAPTURE_ENABLED";
    pub const ENV_CADENCE_CAPTURE_ENABLED: &str = "OCENTRA_SCREEN_CADENCE_CAPTURE_ENABLED";
    pub const ENV_ALLOWED_SCOPE: &str = "OCENTRA_SCREEN_CAPTURE_ALLOWED_SCOPE";
    pub const ENV_CADENCE_SECONDS: &str = "OCENTRA_SCREEN_CADENCE_SECONDS";
    pub const ENV_MIN_TRIGGER_GAP_SECONDS: &str = "OCENTRA_SCREEN_MIN_TRIGGER_GAP_SECONDS";
    pub const ENV_OBSERVED_AT: &str = "OCENTRA_SCREEN_CAPTURE_OBSERVED_AT";
    pub const ENV_LAST_CAPTURE_AT: &str = "OCENTRA_SCREEN_LAST_CAPTURE_AT";
    pub const ENV_REQUESTED_SCOPE: &str = "OCENTRA_SCREEN_CAPTURE_REQUESTED_SCOPE";
    pub const VALUE_ONE: &str = "1";
    pub const VALUE_TRUE: &str = "true";
    pub const SCOPE_ACTIVE_WINDOW: &str = "activeWindow";
    pub const SCOPE_SELECTED_WINDOW: &str = "selectedWindow";
    pub const SCOPE_PRIMARY_DISPLAY: &str = "primaryDisplay";
    pub const SCOPE_ACTIVE_WINDOW_INPUT: &str = "active-window";
    pub const SCOPE_SELECTED_WINDOW_INPUT: &str = "selected-window";
    pub const SCOPE_PRIMARY_DISPLAY_INPUT: &str = "primary-display";
    pub const JSON_PRODUCT_SCHEDULER_IMPLEMENTED: &str = "productSchedulerImplemented";
    pub const JSON_DECISION: &str = "decision";
    pub const JSON_TRIGGER: &str = "trigger";
    pub const JSON_REASON: &str = "reason";
    pub const JSON_SCOPE: &str = "scope";
    pub const JSON_SUPPRESSION: &str = "suppression";
    pub const JSON_OBSERVED_AT: &str = "observedAtEpochSeconds";
    pub const JSON_LAST_CAPTURE_AT: &str = "lastCaptureAtEpochSeconds";
    pub const JSON_PARENT_SETTING: &str = "parentSetting";
    pub const JSON_SCREEN_ANALYSIS_ENABLED: &str = "screenAnalysisEnabled";
    pub const JSON_TRIGGER_CAPTURE_ENABLED: &str = "triggerCaptureEnabled";
    pub const JSON_CADENCE_CAPTURE_ENABLED: &str = "cadenceCaptureEnabled";
    pub const JSON_ALLOWED_SCOPE: &str = "allowedScope";
    pub const JSON_CADENCE_SECONDS: &str = "cadenceSeconds";
    pub const JSON_MIN_TRIGGER_GAP_SECONDS: &str = "minTriggerGapSeconds";
    pub const JSON_ENABLED_TRIGGERS: &str = "enabledTriggers";
    pub const DECISION_ENQUEUE_CAPTURE: &str = "enqueueCapture";
    pub const DECISION_SUPPRESS_CAPTURE: &str = "suppressCapture";
}

#[path = "screen_capture_schedule_decision_helpers.rs"]
mod helpers;

#[derive(Clone, Copy)]
enum ErrorContext {
    JournalOpens,
    EventSerializes,
    JournalAppends,
}

const ERROR_CONTEXT_LABELS: &[&str] = &[
    protocol_constants::error::JOURNAL_OPENS,
    protocol_constants::error::AGENT_EVENT_SERIALIZES,
    protocol_constants::error::JOURNAL_APPENDS,
];

#[derive(Debug)]
pub struct ScheduleDecisionError(String);

impl ScheduleDecisionError {
    fn from_context(context: ErrorContext, error: impl std::fmt::Display) -> Self {
        let mut message = ERROR_CONTEXT_LABELS[context as usize].to_owned();
        message.push_str(constants::ERROR_SEPARATOR);
        message.push_str(&error.to_string());
        Self(message)
    }
}

impl std::fmt::Display for ScheduleDecisionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for ScheduleDecisionError {}

pub type MainResult<T = ()> = Result<T, ScheduleDecisionError>;

pub fn main() -> MainResult<()> {
    let output_dir = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(constants::DEFAULT_OUTPUT_DIR));
    std::fs::create_dir_all(&output_dir)
        .map_err(|error| ScheduleDecisionError::from_context(ErrorContext::JournalOpens, error))?;

    let trigger = env::var(constants::ENV_TRIGGER)
        .ok()
        .as_deref()
        .and_then(ScreenCaptureScheduleTrigger::from_proof_label)
        .unwrap_or(ScreenCaptureScheduleTrigger::ParentManualTestCapture);
    let settings = ScreenCaptureSchedulerSettings {
        screen_analysis_enabled: helpers::env_bool(
            helpers::EnvironmentVariable::AnalysisEnabled,
            true,
        ),
        trigger_capture_enabled: helpers::env_bool(
            helpers::EnvironmentVariable::TriggerCaptureEnabled,
            true,
        ),
        cadence_capture_enabled: helpers::env_bool(
            helpers::EnvironmentVariable::CadenceCaptureEnabled,
            true,
        ),
        allowed_scope: helpers::env_scope(
            helpers::EnvironmentVariable::AllowedScope,
            ScreenCaptureScope::SelectedWindow,
        ),
        cadence_seconds: helpers::env_u64(helpers::EnvironmentVariable::CadenceSeconds, 60),
        min_trigger_gap_seconds: helpers::env_u64(
            helpers::EnvironmentVariable::MinTriggerGapSeconds,
            10,
        ),
        enabled_triggers: enabled_triggers(),
    };
    let observed_at = helpers::env_u64(helpers::EnvironmentVariable::ObservedAt, 1_780_000_000);
    let state = ScreenCaptureSchedulerState {
        last_capture_at_epoch_seconds: helpers::env_optional_u64(
            helpers::EnvironmentVariable::LastCaptureAt,
        ),
    };
    let requested_scope = env::var(constants::ENV_REQUESTED_SCOPE).ok().map(|_| {
        helpers::env_scope(
            helpers::EnvironmentVariable::RequestedScope,
            settings.allowed_scope,
        )
    });

    let decision = evaluate_screen_capture_schedule(
        &settings,
        state,
        ocentra_parent_screen_capture_adapter::trigger_scheduler::ScreenCaptureTriggerInput {
            observed_at_epoch_seconds: observed_at,
            trigger,
            requested_scope,
        },
    );

    let decision_json = helpers::decision_json(
        decision,
        settings,
        trigger,
        observed_at,
        state.last_capture_at_epoch_seconds,
    );
    let bytes = serde_json::to_vec_pretty(&decision_json).map_err(|error| {
        ScheduleDecisionError::from_context(ErrorContext::EventSerializes, error)
    })?;

    write(output_dir.join(constants::DECISION_FILE), bytes).map_err(|error| {
        ScheduleDecisionError::from_context(ErrorContext::JournalAppends, error)
    })?;

    Ok(())
}

fn enabled_triggers() -> &'static [ScreenCaptureScheduleTrigger] {
    &[
        ScreenCaptureScheduleTrigger::ManagedBrowserUrlChange,
        ScreenCaptureScheduleTrigger::BrowserGameDetected,
        ScreenCaptureScheduleTrigger::NativeAppForegroundStart,
        ScreenCaptureScheduleTrigger::NativeGameForegroundStart,
        ScreenCaptureScheduleTrigger::LauncherForegroundStart,
        ScreenCaptureScheduleTrigger::UnknownProcessForegroundStart,
        ScreenCaptureScheduleTrigger::UnusualNetworkChange,
        ScreenCaptureScheduleTrigger::PolicyAmbiguity,
        ScreenCaptureScheduleTrigger::ParentManualTestCapture,
    ]
}
