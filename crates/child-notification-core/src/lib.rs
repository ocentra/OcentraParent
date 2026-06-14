#![forbid(unsafe_code)]

mod child_domain_notification;
pub mod policy_control_notification;
mod tracking_notification;

pub const CRATE_NAME: &str = "ocentra-child-notification-core";

pub use child_domain_notification::request_child_domain_parent_notification;
pub use tracking_notification::request_parent_notification_from_policy_violation;
