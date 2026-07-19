type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

#[macro_use]
#[path = "../support/mod.rs"]
mod test_support;

#[path = "policy_source.rs"]
mod policy_source;

#[path = "policy_source_migration.rs"]
mod policy_source_migration;

#[path = "policy_compiler.rs"]
mod policy_compiler;

#[path = "policy_delivery.rs"]
mod policy_delivery;

#[path = "policy_delivery_hydration_boundaries.rs"]
mod policy_delivery_hydration_boundaries;

#[path = "policy_event.rs"]
mod policy_event;

#[path = "policy_preview.rs"]
mod policy_preview;

#[path = "policy_request.rs"]
mod policy_request;
