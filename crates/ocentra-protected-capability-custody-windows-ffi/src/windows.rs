//! Windows ABI module split by ownership boundary.

pub(crate) mod cng;
pub(crate) mod cng_handles;
pub(crate) mod cng_lifecycle;
pub(crate) mod cng_observation;
pub(crate) mod cng_sign;
pub(crate) mod handles;
#[path = "windows/process.rs"]
pub(crate) mod process;
pub(crate) mod registry;
#[path = "windows/service_sid.rs"]
pub(crate) mod service_sid;
pub(crate) mod tpm;
