use serde::{Deserialize, Serialize};

use crate::app_game_notification_parent_surface_intent::AppGameNotificationParentSurfaceIntentReadModel;
use crate::notification_provider_status_boundary::V08NotificationProviderStatusBoundaryReadModel;

pub const APP_GAME_NOTIFICATION_PREFERENCE_STATUS_SCHEMA_VERSION: u16 = 1;
pub const APP_GAME_NOTIFICATION_PREFERENCE_DELIVERY_RESULT_NOT_SENT: &str = "not-sent";
pub const APP_GAME_NOTIFICATION_PREFERENCE_DELIVERY_RESULT_MANUAL_REQUIRED: &str =
    "manual-required";
pub const APP_GAME_NOTIFICATION_PREFERENCE_STATE_CHANNEL_DISABLED: &str = "channel-disabled";
pub const APP_GAME_NOTIFICATION_PREFERENCE_STATE_MANUAL_SETUP_REQUIRED: &str =
    "manual-setup-required";
pub const APP_GAME_NOTIFICATION_PREFERENCE_QUIET_HOURS_ALLOW: &str = "allow";
pub const APP_GAME_NOTIFICATION_PREFERENCE_QUIET_HOURS_MANUAL_REQUIRED: &str = "manual-required";
pub const APP_GAME_NOTIFICATION_PREFERENCE_CHANNEL_UNAVAILABLE: &str = "unavailable";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppGameNotificationPreferenceDeliveryResultState {
    #[serde(rename = "not-sent")]
    NotSent,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppGameNotificationParentPreferenceState {
    #[serde(rename = "channel-disabled")]
    ChannelDisabled,
    #[serde(rename = "manual-setup-required")]
    ManualSetupRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppGameNotificationQuietHoursDecision {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppGameNotificationProviderChannel {
    #[serde(rename = "unavailable")]
    Unavailable,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameNotificationPreferenceStatusEntry {
    #[serde(default)]
    pub readiness_ref: String,
    pub delivery_result_state: AppGameNotificationPreferenceDeliveryResultState,
    pub parent_preference_state: AppGameNotificationParentPreferenceState,
    pub quiet_hours_decision: AppGameNotificationQuietHoursDecision,
    pub provider_channel: AppGameNotificationProviderChannel,
    pub delivery_result_ref: String,
    pub audit_refs: Vec<String>,
    pub manual_proof_requirements: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameNotificationPreferenceStatusReadModel {
    pub schema_version: u16,
    pub read_model_id: String,
    pub generated_at: String,
    pub source_read_model_ids: Vec<String>,
    pub entries: Vec<AppGameNotificationPreferenceStatusEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameNotificationStatusReadModels {
    pub provider_status_boundary: V08NotificationProviderStatusBoundaryReadModel,
    pub preference_status: AppGameNotificationPreferenceStatusReadModel,
    #[serde(default)]
    pub parent_surface_intent: Option<AppGameNotificationParentSurfaceIntentReadModel>,
}
