use serde::Serialize;

use crate::activity_capture::ActivityCaptureCapabilityStatus;
use crate::screen_settings::ScreenAnalysisParentSetting;

#[path = "screen_child_disclosure_copy.rs"]
mod copy;

use copy::copy_for_state;

const CHILD_SURFACE_REQUIRED: bool = true;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum ActivityScreenChildDisclosureState {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "capture-active")]
    CaptureActive,
    #[serde(rename = "protected-surface")]
    ProtectedSurface,
    #[serde(rename = "summary-ready")]
    SummaryReady,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityScreenChildDisclosure {
    schema_version: u16,
    state: ActivityScreenChildDisclosureState,
    title: String,
    message: String,
    current_capability_status: Option<ActivityCaptureCapabilityStatus>,
    capture_active: bool,
    child_surface_required: bool,
    hidden_capture_claimed: bool,
    raw_screenshot_shown: bool,
    remote_viewer_claimed: bool,
    policy_authority_claimed: bool,
    child_agent_delivery_claimed: bool,
}

impl ActivityScreenChildDisclosure {
    pub fn unavailable(schema_version: u16) -> Self {
        Self::from_owner_state(
            schema_version,
            ActivityScreenChildDisclosureState::Unavailable,
            None,
        )
    }

    pub fn manual_required(schema_version: u16) -> Self {
        Self::from_owner_state(
            schema_version,
            ActivityScreenChildDisclosureState::ManualRequired,
            None,
        )
    }

    fn from_current_authority(
        schema_version: u16,
        setting: &ScreenAnalysisParentSetting,
        capability_status: ActivityCaptureCapabilityStatus,
    ) -> Self {
        let state = state_for_current_authority(setting, capability_status);
        Self::from_owner_state(schema_version, state, Some(capability_status))
    }

    fn from_owner_state(
        schema_version: u16,
        state: ActivityScreenChildDisclosureState,
        current_capability_status: Option<ActivityCaptureCapabilityStatus>,
    ) -> Self {
        let (title, message) = copy_for_state(state);
        Self {
            schema_version,
            state,
            title: title.to_string(),
            message: message.to_string(),
            current_capability_status,
            capture_active: state == ActivityScreenChildDisclosureState::CaptureActive,
            child_surface_required: CHILD_SURFACE_REQUIRED,
            hidden_capture_claimed: false,
            raw_screenshot_shown: false,
            remote_viewer_claimed: false,
            policy_authority_claimed: false,
            child_agent_delivery_claimed: false,
        }
    }

    pub fn schema_version(&self) -> u16 {
        self.schema_version
    }

    pub fn state(&self) -> ActivityScreenChildDisclosureState {
        self.state
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn current_capability_status(&self) -> Option<ActivityCaptureCapabilityStatus> {
        self.current_capability_status
    }

    pub fn capture_active(&self) -> bool {
        self.capture_active
    }

    pub fn child_surface_required(&self) -> bool {
        self.child_surface_required
    }

    pub fn hidden_capture_claimed(&self) -> bool {
        self.hidden_capture_claimed
    }

    pub fn raw_screenshot_shown(&self) -> bool {
        self.raw_screenshot_shown
    }

    pub fn remote_viewer_claimed(&self) -> bool {
        self.remote_viewer_claimed
    }

    pub fn policy_authority_claimed(&self) -> bool {
        self.policy_authority_claimed
    }

    pub fn child_agent_delivery_claimed(&self) -> bool {
        self.child_agent_delivery_claimed
    }
}

fn state_for_current_authority(
    setting: &ScreenAnalysisParentSetting,
    capability_status: ActivityCaptureCapabilityStatus,
) -> ActivityScreenChildDisclosureState {
    if !setting.screen_analysis_enabled {
        ActivityScreenChildDisclosureState::Disabled
    } else {
        state_for_capability(capability_status)
    }
}

fn state_for_capability(
    capability_status: ActivityCaptureCapabilityStatus,
) -> ActivityScreenChildDisclosureState {
    match capability_status {
        ActivityCaptureCapabilityStatus::Available => ActivityScreenChildDisclosureState::Enabled,
        ActivityCaptureCapabilityStatus::Unavailable
        | ActivityCaptureCapabilityStatus::AccessDenied
        | ActivityCaptureCapabilityStatus::NoActiveWindow
        | ActivityCaptureCapabilityStatus::NoNetworkObservations
        | ActivityCaptureCapabilityStatus::AdapterError => {
            ActivityScreenChildDisclosureState::Unavailable
        }
    }
}
