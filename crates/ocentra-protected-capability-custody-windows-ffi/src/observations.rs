//! Accessors for copied operating-system observations.

#[path = "observation_image.rs"]
mod image;
#[path = "observation_image_ancestor.rs"]
mod image_ancestor;
#[path = "observation_process.rs"]
mod process;
#[path = "observation_registry.rs"]
mod registry;
#[path = "observation_registry_value.rs"]
mod registry_value;
#[path = "observation_security.rs"]
mod security;
#[path = "observation_service.rs"]
mod service;
#[path = "observation_service_failure.rs"]
mod service_failure;
#[path = "observation_token.rs"]
mod token;
