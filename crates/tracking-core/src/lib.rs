#![forbid(unsafe_code)]

mod ai_boundary;
mod alerting;
mod child_check_in;
mod expected_place;
mod geofence;
mod local_place;
mod location_validation;
mod missing_device;
mod nearby_place;
mod parent_acknowledgement;
mod read_model;
mod read_model_guard;
mod read_model_rows;
mod retention_settings;
mod runtime_flow;
mod status;
mod temporary_live;

pub const CRATE_NAME: &str = "ocentra-tracking-core";

pub fn evidence_crate_name() -> &'static str {
    ocentra_evidence::CRATE_NAME
}

pub use ai_boundary::{validate_tracking_ai_result_as_evidence, TrackingAiBoundaryDecision};
pub use alerting::{
    evaluate_tracking_alert, TrackingAlertDecision, TrackingParentNotificationDecisionState,
};
pub use child_check_in::record_child_check_in;
pub use expected_place::{
    evaluate_expected_place_state, expected_place_window_contains_minute,
    TrackingExpectedPlaceWindow,
};
pub use geofence::{
    detect_geofence_transition, TrackingGeofenceEvaluation, TrackingGeofenceInsideState,
};
pub use local_place::{
    evaluate_parent_defined_place, TrackingParentDefinedPlaceDecision,
    TrackingParentDefinedPlaceInput,
};
pub use location_validation::{
    validate_tracking_location_observation, TrackingLocationValidationDecision,
    TrackingLocationValidationResultState,
};
pub use missing_device::{
    evaluate_missing_device_mode, TrackingLastKnownVisibilityState, TrackingMissingDeviceDecision,
};
pub use nearby_place::{
    request_nearby_place_provider_analysis, TrackingNearbyPlaceProviderAvailabilityState,
    TrackingNearbyPlaceProviderDecision,
};
pub use parent_acknowledgement::record_parent_acknowledgement;
pub use read_model::tracking_read_model_for_connection;
pub use read_model_guard::{
    evaluate_tracking_read_model_differential, evaluate_tracking_read_model_schema,
    TrackingReadModelDifferentialDecision, TrackingReadModelDifferentialState,
    TrackingReadModelMigrationState, TrackingReadModelSchemaDecision,
};
pub use retention_settings::{
    apply_tracking_retention_settings_write, tracking_retention_settings_durable_store_path,
    TrackingRetentionSettingsWriteAppliedState,
};
pub use runtime_flow::{
    default_child_tracking_runtime_config, default_location_observed_event,
    observe_tracking_location, policy_eligible_child_tracking_runtime_config,
    record_tracking_evidence_from_location, tracking_ai_analysis_request_from_evidence,
    tracking_child_check_in_from_location, tracking_expected_place_state_from_evidence,
    tracking_geofence_transition_from_evidence,
    tracking_observation_portal_notification_candidate_state,
    tracking_parent_acknowledgement_from_notification, TrackingPortalNotificationCandidateState,
    TrackingRuntimeObservationReport,
};
pub use status::{
    evaluate_tracking_capability_status, evaluate_tracking_device_status,
    TrackingCapabilityAvailabilityState, TrackingCapabilityStatusDecision,
    TrackingCapabilityStatusInput, TrackingDeviceStatusDecision, TrackingDeviceStatusInput,
    TrackingLowPowerModeState, TrackingPermissionState, TrackingPlatformBackgroundState,
};
pub use temporary_live::{
    evaluate_temporary_live_tracking_session, TrackingHighCadenceState,
    TrackingTemporaryLiveSessionDecision, TrackingTemporaryLiveSessionInput,
};
