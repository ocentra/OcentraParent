use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAndroidPhysicalTargetState {
    PhysicalDeviceObserved,
    ManualRequired,
    Unavailable,
    Mismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAndroidPhysicalTargetBoundaryReason {
    AdbUnavailable,
    TargetNotConnected,
    ObservationMissing,
    IdentityMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAndroidPhysicalTargetField {
    TargetRef,
    Serial,
    Product,
    Model,
    Device,
    AndroidRelease,
    Abi,
    AdbConnectCommandRef,
    AdbDevicesCommandRef,
    AdbGetpropCommandRef,
    EvidenceRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkAndroidPhysicalTargetExpected {
    pub target_ref: String,
    pub serial: String,
    pub product: String,
    pub model: String,
    pub device: String,
    pub android_release: String,
    pub abi: String,
    pub adb_connect_command_ref: String,
    pub adb_devices_command_ref: String,
    pub adb_getprop_command_ref: String,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkAndroidPhysicalTargetObserved {
    pub serial: String,
    pub product: String,
    pub model: String,
    pub device: String,
    pub android_release: String,
    pub abi: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkAndroidPhysicalTargetUnsupportedClaims {
    pub exact_url_claimed: bool,
    pub decrypted_payload_claimed: bool,
    pub page_content_claimed: bool,
    pub emulator_only_product_support_claimed: bool,
    pub live_vpn_service_execution_claimed: bool,
    pub packet_capture_claimed: bool,
    pub packet_block_claimed: bool,
    pub app_package_correlation_claimed: bool,
    pub adapter_authority_claimed: bool,
    pub enforcement_command_claimed: bool,
    pub production_android_support_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkAndroidPhysicalTargetInput {
    pub proof_ref: String,
    pub adb_available: bool,
    pub target_connected: bool,
    pub expected: NetworkAndroidPhysicalTargetExpected,
    pub observed: Option<NetworkAndroidPhysicalTargetObserved>,
    pub unsupported_claims: NetworkAndroidPhysicalTargetUnsupportedClaims,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkAndroidPhysicalTargetMismatch {
    pub field: NetworkAndroidPhysicalTargetField,
    pub expected: String,
    pub observed: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkAndroidPhysicalTargetProof {
    pub proof_ref: String,
    pub expected: NetworkAndroidPhysicalTargetExpected,
    pub observed: Option<NetworkAndroidPhysicalTargetObserved>,
    pub state: NetworkAndroidPhysicalTargetState,
    pub boundary_reasons: Vec<NetworkAndroidPhysicalTargetBoundaryReason>,
    pub mismatches: Vec<NetworkAndroidPhysicalTargetMismatch>,
    pub evidence_refs: Vec<String>,
    pub adb_available: bool,
    pub target_connected: bool,
    pub read_only_adb_probe_executed: bool,
    pub physical_device_identity_proved: bool,
    pub live_vpn_service_executed: bool,
    pub packet_capture_executed: bool,
    pub packet_blocked: bool,
    pub app_package_correlation_claimed: bool,
    pub adapter_authority_claimed: bool,
    pub enforcement_command_published: bool,
    pub production_android_support_claimed: bool,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub page_content_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkAndroidPhysicalTargetError {
    EmptyProofRef,
    EmptyExpectedField(NetworkAndroidPhysicalTargetField),
    EmptyObservedField(NetworkAndroidPhysicalTargetField),
    EmptyEvidenceRef,
    ExactUrlClaimRejected,
    DecryptedPayloadClaimRejected,
    PageContentClaimRejected,
    EmulatorOnlyProductSupportClaimRejected,
    LiveVpnServiceExecutionClaimRejected,
    PacketCaptureClaimRejected,
    PacketBlockClaimRejected,
    AppPackageCorrelationClaimRejected,
    AdapterAuthorityClaimRejected,
    EnforcementCommandClaimRejected,
    ProductionAndroidSupportClaimRejected,
}

pub(crate) fn normalize_text(value: &str) -> Option<String> {
    let trimmed = value.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_owned())
}
