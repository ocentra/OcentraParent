#[cfg(windows)]
#[path = "read.rs"]
mod registry_read;
#[cfg(windows)]
#[path = "write.rs"]
mod registry_write;

#[cfg(windows)]
use crate::platform::PlatformError;

#[cfg(windows)]
pub(super) fn read(registry_id: &str, name: &str) -> Result<Option<Vec<u8>>, PlatformError> {
    registry_read::runtime(registry_id, name)
}

#[cfg(windows)]
pub(super) fn read_enrollment(
    registry_id: &str,
    name: &str,
) -> Result<Option<Vec<u8>>, PlatformError> {
    registry_read::enrollment(registry_id, name)
}

#[cfg(windows)]
pub(super) fn write(registry_id: &str, name: &str, value: &[u8]) -> Result<(), PlatformError> {
    registry_write::one(registry_id, name, value)
}

#[cfg(windows)]
pub(super) fn delete(registry_id: &str, name: &str) -> Result<(), PlatformError> {
    registry_write::delete(registry_id, name)
}

#[cfg(windows)]
pub(super) fn write_batch(
    registry_id: &str,
    mutations: &[super::RuntimeMutation<'_>],
) -> Result<(), super::RuntimeBatchFailure> {
    registry_write::batch(registry_id, mutations)
}
