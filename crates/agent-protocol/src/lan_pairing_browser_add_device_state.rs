use serde::{
    de::{self, Deserializer},
    Deserialize, Serialize,
};

use crate::{
    LanPairingDeviceReachability, LanPairingDeviceRef, LanPairingDiscoveryRuntimeStatus,
    LanPairingNetworkMode, LanPairingProductionDiscoveryState, LanPairingRejectionReason,
    LanPairingTrustState,
};

pub mod production_household_proof;
pub mod signed_discovery_relay_spine;
pub mod source_matrix;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanCanonicalHouseholdDeviceClassification {
    ChildAgent,
    Phone,
    Tablet,
    Laptop,
    Desktop,
    Printer,
    Television,
    GameConsole,
    Camera,
    NetworkAttachedStorage,
    InternetOfThings,
    NetworkInfrastructure,
    UnsupportedLanDevice,
    UnknownLanDevice,
}

#[path = "lan_pairing_browser_add_device_state/discovery_contracts.rs"]
mod discovery_contracts;
pub type LanPairingDiscoverySource = discovery_contracts::LanPairingDiscoverySource;
pub type LanBrowserAddDeviceDiscoveryDevice =
    discovery_contracts::LanBrowserAddDeviceDiscoveryDevice;
pub type LanServiceIdentityProbeEvidenceKind =
    discovery_contracts::LanServiceIdentityProbeEvidenceKind;
pub type LanServiceIdentityProbeEvidence = discovery_contracts::LanServiceIdentityProbeEvidence;
pub type LanBrowserAddDevicePairingRequest = discovery_contracts::LanBrowserAddDevicePairingRequest;
pub type LanBrowserAddDeviceScanSummary = discovery_contracts::LanBrowserAddDeviceScanSummary;
pub type LanPassiveDiscoveryLocalNeighborCollectionSummary =
    discovery_contracts::LanPassiveDiscoveryLocalNeighborCollectionSummary;
pub type LanSelectedDeviceReadiness = discovery_contracts::LanSelectedDeviceReadiness;
pub type LanDiscoveryEventHistoryState = discovery_contracts::LanDiscoveryEventHistoryState;
pub type LanDiscoveryEventKind = discovery_contracts::LanDiscoveryEventKind;
pub type LanDiscoveryEventRow = discovery_contracts::LanDiscoveryEventRow;
pub type LanDiscoveryEventHistory = discovery_contracts::LanDiscoveryEventHistory;
pub type LanCanonicalHouseholdDeviceRole = discovery_contracts::LanCanonicalHouseholdDeviceRole;
pub type LanCanonicalHouseholdDeviceSource = discovery_contracts::LanCanonicalHouseholdDeviceSource;
pub type LanCanonicalHouseholdDeviceConfidence =
    discovery_contracts::LanCanonicalHouseholdDeviceConfidence;
pub type LanDiscoveryEvidenceSource = discovery_contracts::LanDiscoveryEvidenceSource;
pub type LanDiscoveryEvidenceKind = discovery_contracts::LanDiscoveryEvidenceKind;
pub type LanDiscoveryEvidenceConfidence = discovery_contracts::LanDiscoveryEvidenceConfidence;
pub type LanDiscoveryEvidenceRecord = discovery_contracts::LanDiscoveryEvidenceRecord;

#[path = "lan_pairing_browser_add_device_state/household_device_contracts.rs"]
mod household_device_contracts;
pub type LanHouseholdDeviceActionKind = household_device_contracts::LanHouseholdDeviceActionKind;
pub type LanHouseholdDeviceDecision = household_device_contracts::LanHouseholdDeviceDecision;
pub type LanCanonicalHouseholdRouteState =
    household_device_contracts::LanCanonicalHouseholdRouteState;
pub type LanCanonicalHouseholdRoleState =
    household_device_contracts::LanCanonicalHouseholdRoleState;
pub type LanCanonicalHouseholdSurface = household_device_contracts::LanCanonicalHouseholdSurface;
pub type LanCanonicalHouseholdNetworkIdentity =
    household_device_contracts::LanCanonicalHouseholdNetworkIdentity;
pub type LanChildAgentInventoryPacket = household_device_contracts::LanChildAgentInventoryPacket;
pub type LanCanonicalHouseholdDevice = household_device_contracts::LanCanonicalHouseholdDevice;
pub type LanBrowserAddDeviceReadModel = household_device_contracts::LanBrowserAddDeviceReadModel;

fn deserialize_lan_schema_version<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    let version = u16::deserialize(deserializer)?;
    if version == crate::constants::lan_pairing::SCHEMA_VERSION {
        Ok(version)
    } else {
        Err(de::Error::custom(format!(
            "unsupported LAN schema version {version}; expected {}",
            crate::constants::lan_pairing::SCHEMA_VERSION
        )))
    }
}
