use serde::{Deserialize, Serialize};

use crate::LanPairingDiscoverySource;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanBrowserDiscoveryScanRequest {
    pub schema_version: u16,
    pub requested_discovery_source: LanPairingDiscoverySource,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanBrowserAddDeviceRequest {
    pub schema_version: u16,
    pub child_device_id: String,
    pub parent_device_id: String,
    pub route_id: String,
    pub origin: String,
    pub issued_at: String,
    pub expires_at: String,
}
