use crate::ScreenCaptureScope;

#[path = "trigger_scheduler_labels.rs"]
mod labels;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScreenCaptureScheduleTrigger {
    ManagedBrowserUrlChange,
    BrowserGameDetected,
    NativeAppForegroundStart,
    NativeGameForegroundStart,
    LauncherForegroundStart,
    UnknownProcessForegroundStart,
    UnusualNetworkChange,
    PolicyAmbiguity,
    ParentManualTestCapture,
    TimedCadence,
}

impl ScreenCaptureScheduleTrigger {
    pub fn from_proof_label(value: &str) -> Option<Self> {
        labels::trigger_from_proof_label(value)
    }

    pub fn as_proof_label(self) -> &'static str {
        labels::trigger_proof_label(self)
    }

    fn requires_cadence(self) -> bool {
        self == Self::TimedCadence
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScreenCaptureSchedulerSettings {
    pub screen_analysis_enabled: bool,
    pub trigger_capture_enabled: bool,
    pub cadence_capture_enabled: bool,
    pub allowed_scope: ScreenCaptureScope,
    pub cadence_seconds: u64,
    pub min_trigger_gap_seconds: u64,
    pub enabled_triggers: &'static [ScreenCaptureScheduleTrigger],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScreenCaptureSchedulerState {
    pub last_capture_at_epoch_seconds: Option<u64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScreenCaptureTriggerInput {
    pub observed_at_epoch_seconds: u64,
    pub trigger: ScreenCaptureScheduleTrigger,
    pub requested_scope: Option<ScreenCaptureScope>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScreenCaptureSuppressionReason {
    DisabledByParent,
    TriggerCaptureDisabled,
    TriggerNotEnabled,
    CadenceCaptureDisabled,
    CadenceNotDue,
    TriggerDebounced,
    UnsupportedScope,
}

impl ScreenCaptureSuppressionReason {
    pub fn as_proof_label(self) -> &'static str {
        labels::suppression_proof_label(self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScreenCaptureScheduleDecision {
    EnqueueCapture {
        reason: ScreenCaptureScheduleTrigger,
        scope: ScreenCaptureScope,
    },
    SuppressCapture {
        reason: ScreenCaptureSuppressionReason,
    },
}

impl ScreenCaptureScheduleDecision {
    pub fn enqueued_scope(self) -> Option<ScreenCaptureScope> {
        match self {
            Self::EnqueueCapture { scope, .. } => Some(scope),
            Self::SuppressCapture { .. } => None,
        }
    }
}

pub fn evaluate_screen_capture_schedule(
    settings: &ScreenCaptureSchedulerSettings,
    state: ScreenCaptureSchedulerState,
    input: ScreenCaptureTriggerInput,
) -> ScreenCaptureScheduleDecision {
    if !settings.screen_analysis_enabled {
        return suppressed(ScreenCaptureSuppressionReason::DisabledByParent);
    }

    let scope = input.requested_scope.unwrap_or(settings.allowed_scope);
    if !scope_supported(scope) {
        return suppressed(ScreenCaptureSuppressionReason::UnsupportedScope);
    }

    if input.trigger.requires_cadence() {
        return evaluate_cadence(settings, state, input, scope);
    }

    if !settings.trigger_capture_enabled {
        return suppressed(ScreenCaptureSuppressionReason::TriggerCaptureDisabled);
    }
    if !settings.enabled_triggers.contains(&input.trigger) {
        return suppressed(ScreenCaptureSuppressionReason::TriggerNotEnabled);
    }
    if captured_too_recently(
        state.last_capture_at_epoch_seconds,
        input.observed_at_epoch_seconds,
        settings.min_trigger_gap_seconds,
    ) {
        return suppressed(ScreenCaptureSuppressionReason::TriggerDebounced);
    }

    ScreenCaptureScheduleDecision::EnqueueCapture {
        reason: input.trigger,
        scope,
    }
}

fn evaluate_cadence(
    settings: &ScreenCaptureSchedulerSettings,
    state: ScreenCaptureSchedulerState,
    input: ScreenCaptureTriggerInput,
    scope: ScreenCaptureScope,
) -> ScreenCaptureScheduleDecision {
    if !settings.cadence_capture_enabled {
        return suppressed(ScreenCaptureSuppressionReason::CadenceCaptureDisabled);
    }
    if captured_too_recently(
        state.last_capture_at_epoch_seconds,
        input.observed_at_epoch_seconds,
        settings.cadence_seconds,
    ) {
        return suppressed(ScreenCaptureSuppressionReason::CadenceNotDue);
    }
    ScreenCaptureScheduleDecision::EnqueueCapture {
        reason: input.trigger,
        scope,
    }
}

fn captured_too_recently(last_capture_at: Option<u64>, observed_at: u64, minimum_gap: u64) -> bool {
    last_capture_at.is_some_and(|last| observed_at.saturating_sub(last) < minimum_gap)
}

fn scope_supported(scope: ScreenCaptureScope) -> bool {
    const SUPPORTED_SCOPES: &[ScreenCaptureScope] = &[
        ScreenCaptureScope::ActiveWindow,
        ScreenCaptureScope::SelectedWindow,
        ScreenCaptureScope::PrimaryDisplay,
    ];

    SUPPORTED_SCOPES.contains(&scope)
}

fn suppressed(reason: ScreenCaptureSuppressionReason) -> ScreenCaptureScheduleDecision {
    ScreenCaptureScheduleDecision::SuppressCapture { reason }
}
