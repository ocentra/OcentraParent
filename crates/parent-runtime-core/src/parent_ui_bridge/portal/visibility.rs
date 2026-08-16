use super::*;

pub(super) fn lan_visible_device_count(read_model: &LanBrowserAddDeviceReadModel) -> usize {
    if !read_model.canonical_household_devices.is_empty() {
        read_model.canonical_household_devices.len()
    } else if !read_model.discovered_devices.is_empty() {
        read_model.discovered_devices.len()
    } else if !read_model.trusted_device_registry.is_empty() {
        read_model.trusted_device_registry.len()
    } else {
        read_model.pairing_requests.len()
    }
}
