use serde::{
    de::{self, Deserializer},
    Deserialize, Serialize,
};
use std::fmt::{Display, Formatter};

use crate::constants;

pub mod signed_household_mesh_ingress;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LanPairingText(pub String);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LanPairingOptionalText(pub Option<String>);

impl Display for LanPairingText {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Display for LanPairingOptionalText {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(value) => formatter.write_str(value),
            None => Ok(()),
        }
    }
}

#[path = "lan_pairing/device_roles.rs"]
mod device_roles;
pub type DeviceRuntimeRole = device_roles::DeviceRuntimeRole;
pub type DeviceRuntimeRoleState = device_roles::DeviceRuntimeRoleState;
pub type DeviceRuntimeSurface = device_roles::DeviceRuntimeSurface;
pub type DeviceRuntimeRouteState = device_roles::DeviceRuntimeRouteState;
pub type DeviceRuntimeAiProviderState = device_roles::DeviceRuntimeAiProviderState;
pub type DeviceRuntimeLocalAiClaim = device_roles::DeviceRuntimeLocalAiClaim;
pub type DeviceRuntimeRoleEntry = device_roles::DeviceRuntimeRoleEntry;
pub type DeviceRoleRuntimeReadModel = device_roles::DeviceRoleRuntimeReadModel;

#[path = "lan_pairing/discovery_states.rs"]
mod discovery_states;
pub type LanPairingProductionDiscoveryState = discovery_states::LanPairingProductionDiscoveryState;
pub type LanAiProviderRoutingState = discovery_states::LanAiProviderRoutingState;

#[path = "lan_pairing/device_hardware.rs"]
mod device_hardware;
pub type LanPairingDeviceHardwareProfile = device_hardware::LanPairingDeviceHardwareProfile;

#[path = "lan_pairing/household_proof.rs"]
mod household_proof;
pub type V09ProductionDiscoveryHouseholdProofBoundary =
    household_proof::V09ProductionDiscoveryHouseholdProofBoundary;
pub type V09ProductionDiscoveryHouseholdRuntimeOwner =
    household_proof::V09ProductionDiscoveryHouseholdRuntimeOwner;
pub type V09ProductionDiscoveryHouseholdCheck =
    household_proof::V09ProductionDiscoveryHouseholdCheck;
pub type V09ProductionDiscoveryHouseholdSourceState =
    household_proof::V09ProductionDiscoveryHouseholdSourceState;
pub type V09ProductionDiscoveryHouseholdRouteRecoveryState =
    household_proof::V09ProductionDiscoveryHouseholdRouteRecoveryState;
pub type V09ProductionDiscoveryHouseholdProofState =
    household_proof::V09ProductionDiscoveryHouseholdProofState;
pub type V09ProductionDiscoveryHouseholdReadinessDecision =
    household_proof::V09ProductionDiscoveryHouseholdReadinessDecision;
pub type V09ProductionDiscoveryHouseholdManualProofGate =
    household_proof::V09ProductionDiscoveryHouseholdManualProofGate;
pub type V09ProductionDiscoveryHouseholdStateEvidence =
    household_proof::V09ProductionDiscoveryHouseholdStateEvidence;
pub type V09ProductionDiscoveryHouseholdManualChecklistItem =
    household_proof::V09ProductionDiscoveryHouseholdManualChecklistItem;
pub type V09ProductionDiscoveryHouseholdProofReadModel =
    household_proof::V09ProductionDiscoveryHouseholdProofReadModel;

#[path = "lan_pairing/core.rs"]
mod core;
pub type LanPairingNetworkMode = core::LanPairingNetworkMode;
pub type LanPairingTrustState = core::LanPairingTrustState;
pub type LanPairingAuthenticationState = core::LanPairingAuthenticationState;
pub type LanPairingDeviceReachability = core::LanPairingDeviceReachability;
pub type LanPairingDiscoveryRuntimeStatus = core::LanPairingDiscoveryRuntimeStatus;
pub type LanPairingIntentKind = core::LanPairingIntentKind;
pub type LanPairingResponseState = core::LanPairingResponseState;
pub type LanSignedChildAgentMessageKind = core::LanSignedChildAgentMessageKind;
pub type LanPairingRejectionReason = core::LanPairingRejectionReason;
pub type LanPairingAuditEventType = core::LanPairingAuditEventType;
pub type LanPairingDeviceRef = core::LanPairingDeviceRef;
pub type LanPairingEnablement = core::LanPairingEnablement;
pub type LanPairingDiscoveryDevice = core::LanPairingDiscoveryDevice;

#[path = "lan_pairing/mdns.rs"]
mod mdns;
pub type LanMdnsAdvertisementLifecycleState = mdns::LanMdnsAdvertisementLifecycleState;
pub type LanMdnsAdvertisementSupportState = mdns::LanMdnsAdvertisementSupportState;
pub type LanMdnsAdvertisementConfirmationState = mdns::LanMdnsAdvertisementConfirmationState;
pub type LanMdnsTxtRecord = mdns::LanMdnsTxtRecord;
pub type LanParentMdnsAdvertisement = mdns::LanParentMdnsAdvertisement;
pub type LanChildMdnsAdvertisement = mdns::LanChildMdnsAdvertisement;
pub type LanChildMdnsAdvertisementInput = mdns::LanChildMdnsAdvertisementInput;

#[path = "lan_pairing/pairing_contracts.rs"]
mod pairing_contracts;
pub type LanPairingChallenge = pairing_contracts::LanPairingChallenge;
pub type LanPairingChallengeRequest = pairing_contracts::LanPairingChallengeRequest;
pub type LanPairingProofPreview = pairing_contracts::LanPairingProofPreview;
pub type LanPairingProof = pairing_contracts::LanPairingProof;
pub type LanSignedChildAgentClaim = pairing_contracts::LanSignedChildAgentClaim;
pub type LanSignedChildAgentEnvelope = pairing_contracts::LanSignedChildAgentEnvelope;
pub type LanTrustedDeviceRegistryEntry = pairing_contracts::LanTrustedDeviceRegistryEntry;
pub type LanSelectedRouteTarget = pairing_contracts::LanSelectedRouteTarget;
pub type LanTrustedDeviceRegistrySnapshot = pairing_contracts::LanTrustedDeviceRegistrySnapshot;
pub type LanPairingRouteSelectionRequest = pairing_contracts::LanPairingRouteSelectionRequest;
pub type LanPairingRoutingDecision = pairing_contracts::LanPairingRoutingDecision;
pub type LanParentIntentEnvelope = pairing_contracts::LanParentIntentEnvelope;
pub type LanChildAgentResponse = pairing_contracts::LanChildAgentResponse;
pub type LanPairingAuditEvent = pairing_contracts::LanPairingAuditEvent;

fn deserialize_lan_schema_version<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    let version = u16::deserialize(deserializer)?;
    (version == constants::lan_pairing::SCHEMA_VERSION)
        .then_some(version)
        .ok_or_else(|| {
            de::Error::custom(format!(
                "unsupported LAN schema version {version}; expected {}",
                constants::lan_pairing::SCHEMA_VERSION
            ))
        })
}

fn deserialize_lan_schema_version_text<'de, D>(deserializer: D) -> Result<LanPairingText, D::Error>
where
    D: Deserializer<'de>,
{
    let version = String::deserialize(deserializer)?;
    if version == constants::lan_pairing::SCHEMA_VERSION_TEXT {
        Ok(LanPairingText(version))
    } else {
        Err(de::Error::custom(format!(
            "unsupported LAN schema version {version}; expected {}",
            constants::lan_pairing::SCHEMA_VERSION_TEXT
        )))
    }
}
