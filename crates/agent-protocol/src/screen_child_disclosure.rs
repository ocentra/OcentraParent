use serde::{Deserialize, Serialize};

use crate::screen_evidence::{
    SCREEN_CAPABILITY_DISABLED_BY_PARENT, SCREEN_CAPABILITY_READY, SCREEN_DELETION_DELETED,
    SCREEN_DELETION_EXPIRED_DELETED, SCREEN_DELETION_REQUIRED,
};

#[path = "screen_child_disclosure_copy.rs"]
mod copy;
use copy::copy_for_state;

pub const SCREEN_CHILD_DISCLOSURE_CAPABILITY_PAUSED: &str = "pausedByParent";
pub const SCREEN_CHILD_DISCLOSURE_CAPABILITY_PERMISSION_FRAGMENT: &str = "permission";
pub const SCREEN_CHILD_DISCLOSURE_CAPABILITY_MANUAL_FRAGMENT: &str = "manual";
pub const SCREEN_CHILD_DISCLOSURE_CAPABILITY_PROTECTED_FRAGMENT: &str = "protected";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityScreenChildDisclosure {
    pub schema_version: u16,
    pub state: ActivityScreenChildDisclosureState,
    pub title: String,
    pub message: String,
    pub source_result_id: Option<String>,
    pub capture_active: bool,
    pub child_visible_required: bool,
    pub hidden_capture_claimed: bool,
    pub raw_screenshot_shown: bool,
    pub remote_viewer_claimed: bool,
    pub policy_authority_claimed: bool,
    pub child_agent_delivery_claimed: bool,
}

impl ActivityScreenChildDisclosure {
    pub fn unavailable(schema_version: u16) -> Self {
        Self::for_state(
            schema_version,
            ActivityScreenChildDisclosureState::Unavailable,
            None,
        )
    }

    pub fn for_state(
        schema_version: u16,
        state: ActivityScreenChildDisclosureState,
        source_result_id: Option<String>,
    ) -> Self {
        let (title, message) = copy_for_state(state);
        Self {
            schema_version,
            state,
            title: title.to_string(),
            message: message.to_string(),
            source_result_id,
            capture_active: state == ActivityScreenChildDisclosureState::CaptureActive,
            child_visible_required: true,
            hidden_capture_claimed: false,
            raw_screenshot_shown: false,
            remote_viewer_claimed: false,
            policy_authority_claimed: false,
            child_agent_delivery_claimed: false,
        }
    }

    pub fn from_observation(
        schema_version: u16,
        source_result_id: String,
        capability_status: &str,
        deletion_state: &str,
    ) -> Self {
        let state = if deletion_state == SCREEN_DELETION_REQUIRED {
            ActivityScreenChildDisclosureState::CaptureActive
        } else if capability_status == SCREEN_CAPABILITY_DISABLED_BY_PARENT {
            ActivityScreenChildDisclosureState::Disabled
        } else if capability_status == SCREEN_CHILD_DISCLOSURE_CAPABILITY_PAUSED {
            ActivityScreenChildDisclosureState::Paused
        } else if capability_status.contains(SCREEN_CHILD_DISCLOSURE_CAPABILITY_PERMISSION_FRAGMENT)
            || capability_status.contains(SCREEN_CHILD_DISCLOSURE_CAPABILITY_MANUAL_FRAGMENT)
        {
            ActivityScreenChildDisclosureState::ManualRequired
        } else if capability_status.contains(SCREEN_CHILD_DISCLOSURE_CAPABILITY_PROTECTED_FRAGMENT)
        {
            ActivityScreenChildDisclosureState::ProtectedSurface
        } else if deletion_state == SCREEN_DELETION_DELETED
            || deletion_state == SCREEN_DELETION_EXPIRED_DELETED
        {
            ActivityScreenChildDisclosureState::SummaryReady
        } else if capability_status == SCREEN_CAPABILITY_READY {
            ActivityScreenChildDisclosureState::Enabled
        } else {
            ActivityScreenChildDisclosureState::Unavailable
        };
        Self::for_state(schema_version, state, Some(source_result_id))
    }
}
