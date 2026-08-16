pub const DEFAULT_MANIFEST_URL: &str =
    "https://github.com/ocentra/OcentraParent/releases/latest/download/latest-windows.json";
pub const MANIFEST_URL_ENV: &str = "OCENTRA_CHILD_UPDATE_MANIFEST_URL";
pub const INTERVAL_SECONDS_ENV: &str = "OCENTRA_CHILD_UPDATE_INTERVAL_SECONDS";
pub const INITIAL_DELAY_SECONDS_ENV: &str = "OCENTRA_CHILD_UPDATE_INITIAL_DELAY_SECONDS";
pub const PUBLIC_KEY_ENV: &str = "OCENTRA_CHILD_UPDATE_PUBLIC_KEY_BASE64";
pub const SIGNING_KEY_ENV: &str = "OCENTRA_CHILD_UPDATE_SIGNING_KEY_BASE64";
pub const CHILD_PRODUCT: &str = "Ocentra Child Agent";
pub const CHILD_PACKAGE: &str = "ocentra-child-agent";
pub const CHILD_SERVICE_ID: &str = "OcentraChildAgent";
pub const CHILD_UPDATER_ID: &str = "OcentraChildUpdater";
pub const ED25519_ALGORITHM: &str = "Ed25519";
pub const WINDOWS_X64_TARGET: &str = "windows-x64";
pub const MSI_INSTALLER_TYPE: &str = "msi";
pub const DEFAULT_INTERVAL_SECONDS: u64 = 3600;
pub const DEFAULT_INITIAL_DELAY_SECONDS: u64 = 120;

pub fn built_in_public_key_base64() -> &'static str {
    option_env!("OCENTRA_CHILD_UPDATE_PUBLIC_KEY_BASE64").unwrap_or("")
}
