use crate::ScreenCaptureScope;

pub const TRIGGER_MANAGED_BROWSER_URL_CHANGE: &str = "managedBrowserUrlChange";
pub const TRIGGER_BROWSER_GAME_DETECTED: &str = "browserGameDetected";
pub const TRIGGER_NATIVE_APP_FOREGROUND_START: &str = "nativeAppForegroundStart";
pub const TRIGGER_NATIVE_GAME_FOREGROUND_START: &str = "nativeGameForegroundStart";
pub const TRIGGER_LAUNCHER_FOREGROUND_START: &str = "launcherForegroundStart";
pub const TRIGGER_UNKNOWN_PROCESS_FOREGROUND_START: &str = "unknownProcessForegroundStart";
pub const TRIGGER_UNUSUAL_NETWORK_CHANGE: &str = "unusualNetworkChange";
pub const TRIGGER_POLICY_AMBIGUITY: &str = "policyAmbiguity";
pub const TRIGGER_PARENT_MANUAL_TEST_CAPTURE: &str = "parentManualTestCapture";
pub const TRIGGER_TIMED_CADENCE: &str = "timedCadence";

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
        match value {
            TRIGGER_MANAGED_BROWSER_URL_CHANGE => Some(Self::ManagedBrowserUrlChange),
            TRIGGER_BROWSER_GAME_DETECTED => Some(Self::BrowserGameDetected),
            TRIGGER_NATIVE_APP_FOREGROUND_START => Some(Self::NativeAppForegroundStart),
            TRIGGER_NATIVE_GAME_FOREGROUND_START => Some(Self::NativeGameForegroundStart),
            TRIGGER_LAUNCHER_FOREGROUND_START => Some(Self::LauncherForegroundStart),
            TRIGGER_UNKNOWN_PROCESS_FOREGROUND_START => Some(Self::UnknownProcessForegroundStart),
            TRIGGER_UNUSUAL_NETWORK_CHANGE => Some(Self::UnusualNetworkChange),
            TRIGGER_POLICY_AMBIGUITY => Some(Self::PolicyAmbiguity),
            TRIGGER_PARENT_MANUAL_TEST_CAPTURE => Some(Self::ParentManualTestCapture),
            TRIGGER_TIMED_CADENCE => Some(Self::TimedCadence),
            _ => None,
        }
    }

    pub fn as_proof_label(self) -> &'static str {
        match self {
            Self::ManagedBrowserUrlChange => TRIGGER_MANAGED_BROWSER_URL_CHANGE,
            Self::BrowserGameDetected => TRIGGER_BROWSER_GAME_DETECTED,
            Self::NativeAppForegroundStart => TRIGGER_NATIVE_APP_FOREGROUND_START,
            Self::NativeGameForegroundStart => TRIGGER_NATIVE_GAME_FOREGROUND_START,
            Self::LauncherForegroundStart => TRIGGER_LAUNCHER_FOREGROUND_START,
            Self::UnknownProcessForegroundStart => TRIGGER_UNKNOWN_PROCESS_FOREGROUND_START,
            Self::UnusualNetworkChange => TRIGGER_UNUSUAL_NETWORK_CHANGE,
            Self::PolicyAmbiguity => TRIGGER_POLICY_AMBIGUITY,
            Self::ParentManualTestCapture => TRIGGER_PARENT_MANUAL_TEST_CAPTURE,
            Self::TimedCadence => TRIGGER_TIMED_CADENCE,
        }
    }

    fn requires_cadence(self) -> bool {
        matches!(self, Self::TimedCadence)
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
        match self {
            Self::DisabledByParent => "disabledByParent",
            Self::TriggerCaptureDisabled => "triggerCaptureDisabled",
            Self::TriggerNotEnabled => "triggerNotEnabled",
            Self::CadenceCaptureDisabled => "cadenceCaptureDisabled",
            Self::CadenceNotDue => "cadenceNotDue",
            Self::TriggerDebounced => "triggerDebounced",
            Self::UnsupportedScope => "unsupportedScope",
        }
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
    matches!(
        scope,
        ScreenCaptureScope::ActiveWindow
            | ScreenCaptureScope::SelectedWindow
            | ScreenCaptureScope::PrimaryDisplay
    )
}

fn suppressed(reason: ScreenCaptureSuppressionReason) -> ScreenCaptureScheduleDecision {
    ScreenCaptureScheduleDecision::SuppressCapture { reason }
}
