#![forbid(unsafe_code)]

mod child_domain_policy;
mod tracking_policy;

pub const CRATE_NAME: &str = "ocentra-child-policy-core";

pub use child_domain_policy::evaluate_child_domain_policy;
pub use tracking_policy::{
    evaluate_tracking_expected_place_policy, evaluate_tracking_nearby_place_policy,
    TrackingExpectedPlacePolicyDecision, TrackingNearbyPlacePolicyDecision,
    TrackingPolicyViolationState,
};
