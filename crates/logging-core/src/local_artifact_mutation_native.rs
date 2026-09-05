//! Windows-only conversion at the logging-core/native boundary.

#[cfg(windows)]
#[path = "local_artifact_mutation_native_error.rs"]
pub(super) mod error;
#[cfg(windows)]
#[path = "local_artifact_mutation_native_mutation.rs"]
pub(super) mod mutation;
#[cfg(windows)]
#[path = "local_artifact_mutation_native_outcome.rs"]
pub(super) mod outcome;
