#![forbid(unsafe_code)]

mod child_domain_analysis;
mod tracking_boundary;

pub const CRATE_NAME: &str = "ocentra-child-ai-core";

pub use child_domain_analysis::complete_child_domain_ai_analysis;
pub use tracking_boundary::classify_tracking_nearby_place;
