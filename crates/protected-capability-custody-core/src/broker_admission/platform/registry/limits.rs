#[cfg(windows)]
use crate::platform::PlatformError;

#[cfg(windows)]
pub(super) fn count_values_with_prefix(
    registry_id: &str,
    prefix: &str,
    limit: usize,
) -> Result<usize, PlatformError> {
    let key = super::open_key(registry_id)?;
    let mut count = 0_usize;
    for item in key.enum_values() {
        let (name, _value) = item.map_err(super::map_io_error)?;
        if name.starts_with(prefix) {
            count = count.checked_add(1).ok_or(PlatformError::Tampered)?;
            if count >= limit {
                return Ok(count);
            }
        }
    }
    Ok(count)
}
