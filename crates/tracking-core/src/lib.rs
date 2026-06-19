#![forbid(unsafe_code)]
#![allow(
    clippy::clone_on_copy,
    clippy::enum_variant_names,
    clippy::expect_used,
    clippy::needless_pass_by_value
)]

pub mod ai_boundary;
pub mod alerting;
pub mod child_check_in;
mod expected_place;
mod geofence;
mod local_place;
mod location_validation;
mod missing_device;
mod nearby_place;
pub mod parent_acknowledgement;
pub mod read_model;
mod read_model_guard;
mod read_model_rows;
mod retention_settings;
mod runtime_flow;
mod status;
pub mod temporary_live;

pub const CRATE_NAME: &str = "ocentra-tracking-core";

pub fn evidence_crate_name() -> &'static str {
    ocentra_evidence::CRATE_NAME
}

pub use expected_place::{
    default_expected_place_evaluation, evaluate_expected_place_state,
    expected_place_window_contains_minute, TrackingExpectedPlaceEvaluation,
    TrackingExpectedPlaceException, TrackingExpectedPlaceWindow,
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
    classify_tracking_nearby_place_request, request_nearby_place_provider_analysis,
    TrackingNearbyPlaceProviderAvailabilityState, TrackingNearbyPlaceProviderDecision,
};
pub use read_model_guard::{
    evaluate_tracking_read_model_differential, evaluate_tracking_read_model_schema,
    TrackingReadModelDifferentialDecision, TrackingReadModelDifferentialState,
    TrackingReadModelMigrationState, TrackingReadModelSchemaDecision,
};
pub use retention_settings::{
    apply_tracking_config_update, apply_tracking_retention_settings_write,
    tracking_retention_settings_durable_store_path, TrackingConfigUpdateAppliedState,
    TrackingRetentionSettingsWriteAppliedState,
};
pub use runtime_flow::{
    default_at_expected_place_location_observed_event,
    default_away_from_expected_place_location_observed_event,
    default_child_tracking_runtime_config, default_location_observed_event,
    default_uncertain_location_observed_event, observe_tracking_location,
    policy_eligible_child_tracking_runtime_config, record_tracking_evidence_from_location,
    tracking_ai_analysis_request_from_evidence, tracking_child_check_in_from_location,
    tracking_expected_place_state_from_evidence, tracking_geofence_transition_from_evidence,
    tracking_observation_portal_notification_candidate_state,
    tracking_parent_acknowledgement_from_notification, TrackingPortalNotificationCandidateState,
    TrackingRuntimeObservationReport,
};
pub use status::{
    evaluate_tracking_capability_status, evaluate_tracking_device_status,
    TrackingBackgroundCapabilityState, TrackingCapabilityAvailabilityState,
    TrackingCapabilityStatusDecision, TrackingCapabilityStatusInput, TrackingChargingState,
    TrackingConnectivityState, TrackingDeviceStatusDecision, TrackingDeviceStatusInput,
    TrackingLowPowerModeState, TrackingPermissionState, TrackingPlatformState, TrackingRadioState,
    TrackingRuntimeServiceState,
};
