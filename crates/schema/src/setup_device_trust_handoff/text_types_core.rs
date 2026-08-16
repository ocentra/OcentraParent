#[path = "identity.rs"]
mod identity;
#[path = "session.rs"]
mod session;

pub type SetupDeviceTrustHandoffId = identity::SetupDeviceTrustHandoffId;
pub type SetupDeviceTrustHandoffHouseholdRef = identity::SetupDeviceTrustHandoffHouseholdRef;
pub type SetupDeviceTrustHandoffChildProfileRef = identity::SetupDeviceTrustHandoffChildProfileRef;
pub type SetupDeviceTrustHandoffTargetDeviceRef = session::SetupDeviceTrustHandoffTargetDeviceRef;
pub type SetupDeviceTrustHandoffSetupSessionRef = session::SetupDeviceTrustHandoffSetupSessionRef;
pub type SetupDeviceTrustHandoffTrustBootstrapRef =
    session::SetupDeviceTrustHandoffTrustBootstrapRef;
