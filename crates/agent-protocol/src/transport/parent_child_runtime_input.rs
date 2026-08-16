use serde::{Deserialize, Serialize};

use crate::parent_controller_events::{ParentControllerActionKind, ParentControllerSource};
use crate::{constants, ChildCommandKind};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentChildRuntimeInput {
    pub parent_intent_ref: String,
    pub parent_profile_ref: String,
    pub device_ref: String,
    pub observed_at: String,
    pub action_kind: ParentControllerActionKind,
    pub source: ParentControllerSource,
    pub child_command_kind: ChildCommandKind,
}

impl ParentChildRuntimeInput {
    pub fn validated_review_fixture() -> Self {
        Self {
            parent_intent_ref: constants::parent_controller::TEST_PARENT_INTENT_REF.to_string(),
            parent_profile_ref: constants::parent_controller::TEST_PARENT_PROFILE_REF.to_string(),
            device_ref: constants::parent_controller::TEST_DEVICE_REF.to_string(),
            observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
            action_kind: ParentControllerActionKind::Review,
            source: ParentControllerSource::PortalTypedIntent,
            child_command_kind: ChildCommandKind::ObserveNetwork,
        }
    }

    pub fn browser_action_intent_handoff_fixture() -> Self {
        Self {
            parent_intent_ref: constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_ID
                .to_string(),
            child_command_kind: ChildCommandKind::BrowserActionIntentHandoff,
            ..Self::validated_review_fixture()
        }
    }
}
