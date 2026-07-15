use std::env;

use super::{constants, ScreenCaptureScope};
use ocentra_parent_screen_capture_adapter::trigger_scheduler::{
    ScreenCaptureScheduleDecision, ScreenCaptureScheduleTrigger, ScreenCaptureSchedulerSettings,
};
use serde_json::json;

#[derive(Clone, Copy)]
pub(crate) enum EnvironmentVariable {
    AnalysisEnabled,
    TriggerCaptureEnabled,
    CadenceCaptureEnabled,
    AllowedScope,
    RequestedScope,
    CadenceSeconds,
    MinTriggerGapSeconds,
    ObservedAt,
    LastCaptureAt,
}

const ENVIRONMENT_VARIABLE_NAMES: &[&str] = &[
    constants::ENV_ANALYSIS_ENABLED,
    constants::ENV_TRIGGER_CAPTURE_ENABLED,
    constants::ENV_CADENCE_CAPTURE_ENABLED,
    constants::ENV_ALLOWED_SCOPE,
    constants::ENV_REQUESTED_SCOPE,
    constants::ENV_CADENCE_SECONDS,
    constants::ENV_MIN_TRIGGER_GAP_SECONDS,
    constants::ENV_OBSERVED_AT,
    constants::ENV_LAST_CAPTURE_AT,
];

const SCOPE_INPUTS: &[(&str, ScreenCaptureScope)] = &[
    (
        constants::SCOPE_ACTIVE_WINDOW_INPUT,
        ScreenCaptureScope::ActiveWindow,
    ),
    (
        constants::SCOPE_ACTIVE_WINDOW,
        ScreenCaptureScope::ActiveWindow,
    ),
    (
        constants::SCOPE_SELECTED_WINDOW_INPUT,
        ScreenCaptureScope::SelectedWindow,
    ),
    (
        constants::SCOPE_SELECTED_WINDOW,
        ScreenCaptureScope::SelectedWindow,
    ),
    (
        constants::SCOPE_PRIMARY_DISPLAY_INPUT,
        ScreenCaptureScope::PrimaryDisplay,
    ),
    (
        constants::SCOPE_PRIMARY_DISPLAY,
        ScreenCaptureScope::PrimaryDisplay,
    ),
];

const SCOPE_LABELS: &[(ScreenCaptureScope, &str)] = &[
    (
        ScreenCaptureScope::ActiveWindow,
        constants::SCOPE_ACTIVE_WINDOW,
    ),
    (
        ScreenCaptureScope::SelectedWindow,
        constants::SCOPE_SELECTED_WINDOW,
    ),
    (
        ScreenCaptureScope::PrimaryDisplay,
        constants::SCOPE_PRIMARY_DISPLAY,
    ),
];

pub(crate) fn decision_json(
    decision: ScreenCaptureScheduleDecision,
    settings: ScreenCaptureSchedulerSettings,
    trigger: ScreenCaptureScheduleTrigger,
    observed_at: u64,
    last_capture_at: Option<u64>,
) -> serde_json::Value {
    match decision {
        ScreenCaptureScheduleDecision::EnqueueCapture { reason, scope } => {
            let mut object = serde_json::Map::new();
            object.insert(
                constants::JSON_PRODUCT_SCHEDULER_IMPLEMENTED.to_owned(),
                json!(true),
            );
            object.insert(
                constants::JSON_DECISION.to_owned(),
                json!(constants::DECISION_ENQUEUE_CAPTURE),
            );
            object.insert(
                constants::JSON_TRIGGER.to_owned(),
                json!(trigger.as_proof_label()),
            );
            object.insert(
                constants::JSON_REASON.to_owned(),
                json!(reason.as_proof_label()),
            );
            object.insert(constants::JSON_SCOPE.to_owned(), scope_label(scope));
            object.insert(constants::JSON_SUPPRESSION.to_owned(), json!(null));
            object.insert(constants::JSON_OBSERVED_AT.to_owned(), json!(observed_at));
            object.insert(
                constants::JSON_LAST_CAPTURE_AT.to_owned(),
                json!(last_capture_at),
            );
            object.insert(
                constants::JSON_PARENT_SETTING.to_owned(),
                parent_setting_json(settings),
            );
            serde_json::Value::Object(object)
        }
        ScreenCaptureScheduleDecision::SuppressCapture { reason } => {
            let mut object = serde_json::Map::new();
            object.insert(
                constants::JSON_PRODUCT_SCHEDULER_IMPLEMENTED.to_owned(),
                json!(true),
            );
            object.insert(
                constants::JSON_DECISION.to_owned(),
                json!(constants::DECISION_SUPPRESS_CAPTURE),
            );
            object.insert(
                constants::JSON_TRIGGER.to_owned(),
                json!(trigger.as_proof_label()),
            );
            object.insert(constants::JSON_REASON.to_owned(), json!(null));
            object.insert(constants::JSON_SCOPE.to_owned(), json!(null));
            object.insert(
                constants::JSON_SUPPRESSION.to_owned(),
                json!(reason.as_proof_label()),
            );
            object.insert(constants::JSON_OBSERVED_AT.to_owned(), json!(observed_at));
            object.insert(
                constants::JSON_LAST_CAPTURE_AT.to_owned(),
                json!(last_capture_at),
            );
            object.insert(
                constants::JSON_PARENT_SETTING.to_owned(),
                parent_setting_json(settings),
            );
            serde_json::Value::Object(object)
        }
    }
}

fn parent_setting_json(settings: ScreenCaptureSchedulerSettings) -> serde_json::Value {
    let mut object = serde_json::Map::new();
    object.insert(
        constants::JSON_SCREEN_ANALYSIS_ENABLED.to_owned(),
        json!(settings.screen_analysis_enabled),
    );
    object.insert(
        constants::JSON_TRIGGER_CAPTURE_ENABLED.to_owned(),
        json!(settings.trigger_capture_enabled),
    );
    object.insert(
        constants::JSON_CADENCE_CAPTURE_ENABLED.to_owned(),
        json!(settings.cadence_capture_enabled),
    );
    object.insert(
        constants::JSON_ALLOWED_SCOPE.to_owned(),
        scope_label(settings.allowed_scope),
    );
    object.insert(
        constants::JSON_CADENCE_SECONDS.to_owned(),
        json!(settings.cadence_seconds),
    );
    object.insert(
        constants::JSON_MIN_TRIGGER_GAP_SECONDS.to_owned(),
        json!(settings.min_trigger_gap_seconds),
    );
    object.insert(
        constants::JSON_ENABLED_TRIGGERS.to_owned(),
        json!(settings
            .enabled_triggers
            .iter()
            .map(|trigger| trigger.as_proof_label())
            .collect::<Vec<_>>()),
    );
    serde_json::Value::Object(object)
}

fn scope_label(scope: ScreenCaptureScope) -> serde_json::Value {
    let label = SCOPE_LABELS
        .iter()
        .find_map(|(candidate, label)| (*candidate == scope).then_some(*label))
        .unwrap_or(constants::SCOPE_ACTIVE_WINDOW);
    json!(label)
}

pub(crate) fn env_scope(
    variable: EnvironmentVariable,
    fallback: ScreenCaptureScope,
) -> ScreenCaptureScope {
    env::var(ENVIRONMENT_VARIABLE_NAMES[variable as usize])
        .ok()
        .as_deref()
        .and_then(|value| {
            SCOPE_INPUTS
                .iter()
                .find_map(|(label, scope)| (*label == value).then_some(*scope))
        })
        .unwrap_or(fallback)
}

pub(crate) fn env_bool(variable: EnvironmentVariable, fallback: bool) -> bool {
    env::var(ENVIRONMENT_VARIABLE_NAMES[variable as usize])
        .ok()
        .map(|value| value == constants::VALUE_ONE || value == constants::VALUE_TRUE)
        .unwrap_or(fallback)
}

pub(crate) fn env_optional_u64(variable: EnvironmentVariable) -> Option<u64> {
    env::var(ENVIRONMENT_VARIABLE_NAMES[variable as usize])
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
}

pub(crate) fn env_u64(variable: EnvironmentVariable, fallback: u64) -> u64 {
    env_optional_u64(variable).unwrap_or(fallback)
}
