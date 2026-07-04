use std::{env, fs::write, path::PathBuf};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_screen_capture_adapter::{
    trigger_scheduler::{
        evaluate_screen_capture_schedule, ScreenCaptureScheduleDecision,
        ScreenCaptureScheduleTrigger, ScreenCaptureSchedulerSettings, ScreenCaptureSchedulerState,
    },
    ScreenCaptureScope,
};
use serde_json::json;

#[derive(Debug)]
pub struct ScheduleDecisionError(String);

impl std::fmt::Display for ScheduleDecisionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for ScheduleDecisionError {}

pub type MainResult<T = ()> = Result<T, ScheduleDecisionError>;

pub fn main() -> MainResult<()> {
    let output_dir = env::args().nth(1).map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from("output/screen-plan-proof/real-capture/scheduler-decision")
    });
    std::fs::create_dir_all(&output_dir).map_err(|error| {
        ScheduleDecisionError(format!("{}: {error:?}", constants::error::JOURNAL_OPENS))
    })?;

    let trigger = env::var("OCENTRA_SCREEN_CAPTURE_TRIGGER")
        .ok()
        .as_deref()
        .and_then(ScreenCaptureScheduleTrigger::from_proof_label)
        .unwrap_or(ScreenCaptureScheduleTrigger::ParentManualTestCapture);
    let settings = ScreenCaptureSchedulerSettings {
        screen_analysis_enabled: env_bool("OCENTRA_SCREEN_ANALYSIS_ENABLED", true),
        trigger_capture_enabled: env_bool("OCENTRA_SCREEN_TRIGGER_CAPTURE_ENABLED", true),
        cadence_capture_enabled: env_bool("OCENTRA_SCREEN_CADENCE_CAPTURE_ENABLED", true),
        allowed_scope: env_scope(
            "OCENTRA_SCREEN_CAPTURE_ALLOWED_SCOPE",
            ScreenCaptureScope::SelectedWindow,
        ),
        cadence_seconds: env_u64("OCENTRA_SCREEN_CADENCE_SECONDS", 60),
        min_trigger_gap_seconds: env_u64("OCENTRA_SCREEN_MIN_TRIGGER_GAP_SECONDS", 10),
        enabled_triggers: enabled_triggers(),
    };
    let observed_at = env_u64("OCENTRA_SCREEN_CAPTURE_OBSERVED_AT", 1_780_000_000);
    let state = ScreenCaptureSchedulerState {
        last_capture_at_epoch_seconds: env_optional_u64("OCENTRA_SCREEN_LAST_CAPTURE_AT"),
    };
    let requested_scope = env::var("OCENTRA_SCREEN_CAPTURE_REQUESTED_SCOPE")
        .ok()
        .map(|_| {
            env_scope(
                "OCENTRA_SCREEN_CAPTURE_REQUESTED_SCOPE",
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

    let decision_json = decision_json(
        decision,
        settings,
        trigger,
        observed_at,
        state.last_capture_at_epoch_seconds,
    );
    let bytes = serde_json::to_vec_pretty(&decision_json).map_err(|error| {
        ScheduleDecisionError(format!(
            "{}: {error:?}",
            constants::error::AGENT_EVENT_SERIALIZES
        ))
    })?;

    write(output_dir.join("00-scheduler-decision.json"), bytes).map_err(|error| {
        ScheduleDecisionError(format!("{}: {error:?}", constants::error::JOURNAL_APPENDS))
    })?;

    Ok(())
}

fn decision_json(
    decision: ScreenCaptureScheduleDecision,
    settings: ScreenCaptureSchedulerSettings,
    trigger: ScreenCaptureScheduleTrigger,
    observed_at: u64,
    last_capture_at: Option<u64>,
) -> serde_json::Value {
    match decision {
        ScreenCaptureScheduleDecision::EnqueueCapture { reason, scope } => json!({
            "productSchedulerImplemented": true,
            "decision": "enqueueCapture",
            "trigger": trigger.as_proof_label(),
            "reason": reason.as_proof_label(),
            "scope": scope_label(scope),
            "suppression": null,
            "observedAtEpochSeconds": observed_at,
            "lastCaptureAtEpochSeconds": last_capture_at,
            "parentSetting": parent_setting_json(settings),
        }),
        ScreenCaptureScheduleDecision::SuppressCapture { reason } => json!({
            "productSchedulerImplemented": true,
            "decision": "suppressCapture",
            "trigger": trigger.as_proof_label(),
            "reason": null,
            "scope": null,
            "suppression": reason.as_proof_label(),
            "observedAtEpochSeconds": observed_at,
            "lastCaptureAtEpochSeconds": last_capture_at,
            "parentSetting": parent_setting_json(settings),
        }),
    }
}

fn parent_setting_json(settings: ScreenCaptureSchedulerSettings) -> serde_json::Value {
    json!({
        "screenAnalysisEnabled": settings.screen_analysis_enabled,
        "triggerCaptureEnabled": settings.trigger_capture_enabled,
        "cadenceCaptureEnabled": settings.cadence_capture_enabled,
        "allowedScope": scope_label(settings.allowed_scope),
        "cadenceSeconds": settings.cadence_seconds,
        "minTriggerGapSeconds": settings.min_trigger_gap_seconds,
        "enabledTriggers": settings
            .enabled_triggers
            .iter()
            .map(|trigger| trigger.as_proof_label())
            .collect::<Vec<_>>(),
    })
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

fn env_scope(name: &str, fallback: ScreenCaptureScope) -> ScreenCaptureScope {
    match env::var(name).ok().as_deref() {
        Some("active-window" | "activeWindow") => ScreenCaptureScope::ActiveWindow,
        Some("selected-window" | "selectedWindow") => ScreenCaptureScope::SelectedWindow,
        Some("primary-display" | "primaryDisplay") => ScreenCaptureScope::PrimaryDisplay,
        _ => fallback,
    }
}

fn scope_label(scope: ScreenCaptureScope) -> &'static str {
    match scope {
        ScreenCaptureScope::ActiveWindow => "activeWindow",
        ScreenCaptureScope::SelectedWindow => "selectedWindow",
        ScreenCaptureScope::PrimaryDisplay => "primaryDisplay",
    }
}

fn env_bool(name: &str, fallback: bool) -> bool {
    env::var(name)
        .ok()
        .map(|value| value == "1" || value == "true")
        .unwrap_or(fallback)
}

fn env_optional_u64(name: &str) -> Option<u64> {
    env::var(name)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
}

fn env_u64(name: &str, fallback: u64) -> u64 {
    env_optional_u64(name).unwrap_or(fallback)
}
