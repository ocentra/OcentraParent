use ocentra_protected_capability_custody_protocol::constants as protocol;
use ocentra_protected_capability_custody_windows_ffi::{RegistryPath, Result, WindowsText};

pub const REGISTRY_ROOT: &str = "Software\\Ocentra\\ProtectedCapabilityCustody";
pub const ENROLLMENT_SUBKEY: &str = "Enrollment";
pub const ENROLLMENT_VALUE_NAME: &str = "authority-v1";

// This is the registry identity of the fixed broker database path.  It is
// derived from the same domain/UTF-16 path rule used by the private core; no
// installer property or command-line input may replace it.
const FIXED_REGISTRY_ID_PREFIX: &str = "2cc753a30323ee51ee0301439996c5e4";
const FIXED_REGISTRY_ID_SUFFIX: &str = "077fe49d3a31250ee75b32b6ecd1baf7";

pub(super) const FIXED_SERVICE_NAME: &str = protocol::BROKER_SERVICE_NAME;
const FIXED_INSTALL_ROOT: &str = "C:\\Program Files\\Ocentra\\OcentraParent";
const FIXED_BROKER_EXE: &str = "ocentra-protected-capability-custody-broker.exe";

pub(super) const REG_BINARY: u32 = 3;
pub(super) const TPM_NV_INDEX: u32 = 0x0180_f001;
pub(super) const TPM_ALG_SHA256: u16 = 0x000b;
pub(super) const TPM_COUNTER_ATTRIBUTES: u32 = 0x6208_0018;
pub(super) const TPM_COUNTER_BYTES: u16 = 8;

pub(super) const SYSTEM_SID: &[u8] = &[1, 1, 0, 0, 0, 0, 0, 5, 18, 0, 0, 0];
pub(super) const TRUSTED_INSTALLER_SID: &[u8] = &[
    1, 6, 0, 0, 0, 0, 0, 5, 80, 0, 0, 0, 181, 137, 251, 56, 25, 132, 194, 203, 92, 108, 35, 109,
    87, 0, 119, 110, 192, 2, 100, 135,
];
pub(super) const ACCESS_ALLOWED_ACE_TYPE: u8 = 0;
pub(super) const KEY_READ: u32 = 0x0002_0019;
pub(super) const KEY_ALL_ACCESS: u32 = 0x000f_003f;

pub(super) fn enrollment_path() -> Result<RegistryPath> {
    let registry_id = fixed_registry_id()?;
    let mut value = String::from(REGISTRY_ROOT);
    value.push('\\');
    value.push_str(registry_id.as_str());
    value.push('\\');
    value.push_str(ENROLLMENT_SUBKEY);
    RegistryPath::try_from_str(&value)
}

pub(super) fn expected_service_binary_path() -> Result<WindowsText> {
    let value = format!("\"{FIXED_INSTALL_ROOT}\\{FIXED_BROKER_EXE}\"");
    WindowsText::try_from_str(&value)
}

pub(super) fn fixed_registry_id() -> Result<WindowsText> {
    let value = format!("{FIXED_REGISTRY_ID_PREFIX}{FIXED_REGISTRY_ID_SUFFIX}");
    WindowsText::try_from_str(&value)
}
