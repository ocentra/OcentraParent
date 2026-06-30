use std::fmt::{Display, Formatter};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub const ENCRYPTION_KEY_CUSTODY_SCHEMA_VERSION: &str = "encryption-key-custody-proof";

const ENCRYPTION_KEY_CLASS_CHILD_DEVICE_LOCAL_KEY: &str = "child-device-local-key";
const ENCRYPTION_KEY_CLASS_PARENT_DESKTOP_KEY: &str = "parent-desktop-key";
const ENCRYPTION_KEY_CLASS_PARENT_MOBILE_KEY: &str = "parent-mobile-key";
const ENCRYPTION_KEY_CLASS_HOUSEHOLD_RECOVERY_KEY: &str = "household-recovery-key";
const ENCRYPTION_KEY_CLASS_PROVIDER_AUTH_TOKEN: &str = "provider-auth-token";
const ENCRYPTION_KEY_CLASS_SUPPORT_DIAGNOSTIC_TOKEN: &str = "support-diagnostic-token";
const ENCRYPTION_KEY_HOLDER_CHILD_DEVICE: &str = "child-device";
const ENCRYPTION_KEY_HOLDER_PARENT_DESKTOP: &str = "parent-desktop";
const ENCRYPTION_KEY_HOLDER_PARENT_MOBILE: &str = "parent-mobile";
const ENCRYPTION_KEY_HOLDER_HOUSEHOLD_RECOVERY_PATH: &str = "household-recovery-path";
const ENCRYPTION_KEY_HOLDER_PROVIDER_CONNECTION: &str = "provider-connection";
const ENCRYPTION_KEY_HOLDER_SUPPORT_FLOW: &str = "support-flow";
const ENCRYPTION_KEY_HOLDER_HOSTED_PORTAL: &str = "hosted-portal";
const ENCRYPTION_UNLOCK_SCOPE_CHILD_EVIDENCE_LOCAL: &str = "child-evidence-local";
const ENCRYPTION_UNLOCK_SCOPE_PARENT_OWNED_BUNDLE: &str = "parent-owned-bundle";
const ENCRYPTION_UNLOCK_SCOPE_PARENT_CACHE_REPORTS: &str = "parent-cache-reports";
const ENCRYPTION_UNLOCK_SCOPE_HOUSEHOLD_RECOVERY_BUNDLE: &str = "household-recovery-bundle";
const ENCRYPTION_UNLOCK_SCOPE_PROVIDER_API_ONLY: &str = "provider-api-only";
const ENCRYPTION_UNLOCK_SCOPE_DIAGNOSTICS_METADATA_ONLY: &str = "diagnostics-metadata-only";
const ENCRYPTION_UNLOCK_SCOPE_STATUS_ONLY: &str = "status-only";
const PLATFORM_KEY_CUSTODY_SURFACE_WINDOWS: &str = "windows";
const PLATFORM_KEY_CUSTODY_SURFACE_MACOS: &str = "macos";
const PLATFORM_KEY_CUSTODY_SURFACE_LINUX: &str = "linux";
const PLATFORM_KEY_CUSTODY_SURFACE_ANDROID: &str = "android";
const PLATFORM_KEY_CUSTODY_SURFACE_IOS: &str = "ios";
const PLATFORM_KEY_CUSTODY_SURFACE_WEB_PORTAL: &str = "web-portal";
const PLATFORM_KEY_CUSTODY_SURFACE_PARENT_DESKTOP: &str = "parent-desktop";
const PLATFORM_KEY_CUSTODY_SURFACE_CHILD_SERVICE: &str = "child-service";
const PLATFORM_KEY_CUSTODY_SURFACE_PARENT_MOBILE: &str = "parent-mobile";
const PLATFORM_KEY_CUSTODY_SURFACE_CHILD_MOBILE: &str = "child-mobile";
const PLATFORM_KEY_STORE_KIND_WINDOWS_DPAPI_USER: &str = "windows-dpapi-user";
const PLATFORM_KEY_STORE_KIND_WINDOWS_DPAPI_MACHINE: &str = "windows-dpapi-machine";
const PLATFORM_KEY_STORE_KIND_MACOS_KEYCHAIN: &str = "macos-keychain";
const PLATFORM_KEY_STORE_KIND_SECURE_ENCLAVE_BACKED: &str = "secure-enclave-backed";
const PLATFORM_KEY_STORE_KIND_LINUX_SECRET_STORE_UNDECIDED: &str = "linux-secret-store-undecided";
const PLATFORM_KEY_STORE_KIND_ANDROID_KEYSTORE: &str = "android-keystore";
const PLATFORM_KEY_STORE_KIND_IOS_KEYCHAIN: &str = "ios-keychain";
const PLATFORM_KEY_STORE_KIND_NO_DECRYPT_ROOT: &str = "no-decrypt-root";
const PLATFORM_KEY_STORE_KIND_PARENT_DESKTOP_LOCAL_KEY_PATH: &str = "parent-desktop-local-key-path";
const PLATFORM_KEY_STORE_KIND_CHILD_SERVICE_LOCAL_KEY_PATH: &str = "child-service-local-key-path";
const PLATFORM_KEY_STORE_KIND_PARENT_MOBILE_APPROVAL_PATH: &str = "parent-mobile-approval-path";
const PLATFORM_KEY_STORE_KIND_CHILD_MOBILE_PLATFORM_KEY_PATH: &str =
    "child-mobile-platform-key-path";
const PLATFORM_DECRYPT_AUTHORITY_CHILD_LOCAL_EVIDENCE_ONLY: &str = "child-local-evidence-only";
const PLATFORM_DECRYPT_AUTHORITY_PARENT_OWNED_BUNDLES_ONLY: &str = "parent-owned-bundles-only";
const PLATFORM_DECRYPT_AUTHORITY_PARENT_CACHE_REPORTS_AND_BUNDLES: &str =
    "parent-cache-reports-and-bundles";
const PLATFORM_DECRYPT_AUTHORITY_HOUSEHOLD_RECOVERY_BUNDLES_ONLY: &str =
    "household-recovery-bundles-only";
const PLATFORM_DECRYPT_AUTHORITY_NOT_DECRYPT_ROOT: &str = "not-decrypt-root";
const PLATFORM_DECRYPT_AUTHORITY_MANUAL_REQUIRED: &str = "manual-required";
const KEY_CUSTODY_STATE_KEY_AVAILABLE: &str = "keyAvailable";
const KEY_CUSTODY_STATE_KEY_UNAVAILABLE: &str = "keyUnavailable";
const KEY_CUSTODY_STATE_KEY_REVOKED: &str = "keyRevoked";
const KEY_CUSTODY_STATE_WRONG_HOUSEHOLD: &str = "wrongHousehold";
const KEY_CUSTODY_STATE_WRONG_DEVICE: &str = "wrongDevice";
const KEY_CUSTODY_STATE_REINSTALL_REQUIRED: &str = "reinstallRequired";
const KEY_CUSTODY_STATE_RECOVERY_AVAILABLE: &str = "recoveryAvailable";
const KEY_CUSTODY_STATE_RECOVERY_NOT_SUPPORTED: &str = "recoveryNotSupported";
const RECOVERY_MODE_MANUAL_REQUIRED: &str = "manualRequired";
const RECOVERY_MODE_PARENT_OWNED_RECOVERY: &str = "parent-owned-recovery";
const RECOVERY_MODE_NOT_SUPPORTED: &str = "notSupported";
const DECRYPT_DECISION_STATE_ALLOWED: &str = "allowed";
const DECRYPT_DECISION_STATE_WRONG_HOUSEHOLD_DENIED: &str = "wrongHouseholdDenied";
const DECRYPT_DECISION_STATE_WRONG_DEVICE_DENIED: &str = "wrongDeviceDenied";
const DECRYPT_DECISION_STATE_REVOKED_KEY_DENIED: &str = "revokedKeyDenied";
const DECRYPT_DECISION_STATE_LOST_KEY_MANUAL_REQUIRED: &str = "lostKeyManualRequired";
const DECRYPT_DECISION_STATE_RECOVERY_AVAILABLE_MANUAL_REQUIRED: &str =
    "recoveryAvailableManualRequired";
const DECRYPT_DECISION_STATE_LIMITED_UNTIL_DEVICE_PROOF: &str = "limitedUntilDeviceProof";
const DECRYPT_DECISION_STATE_NOT_DECRYPT_ROOT_DENIED: &str = "notDecryptRootDenied";
const DECRYPT_DECISION_STATE_PLATFORM_MANUAL_REQUIRED: &str = "platformManualRequired";
const ENCRYPTION_KEY_NON_CLAIM_NO_UNIVERSAL_OCENTRA_KEY: &str = "no-universal-ocentra-key";
const ENCRYPTION_KEY_NON_CLAIM_NO_HOSTED_DECRYPT_ROOT: &str = "no-hosted-decrypt-root";
const ENCRYPTION_KEY_NON_CLAIM_NO_PLAINTEXT_FALLBACK: &str = "no-plaintext-fallback";
const ENCRYPTION_KEY_NON_CLAIM_NO_TS_BUSINESS_OWNER: &str = "no-ts-business-owner";
const ENCRYPTION_KEY_NON_CLAIM_NO_LAN_OWNERSHIP: &str = "no-lan-ownership";
const ENCRYPTION_KEY_NON_CLAIM_NO_MOBILE_BROAD_CLAIM: &str = "no-mobile-broad-claim";
const ENCRYPTION_KEY_CONTRACT_VERSION_V0_2: &str = "v0.2";
const ENCRYPTION_ATTEMPT_WINDOWS_PARENT: &str = "attempt-windows-parent";
const ENCRYPTION_ATTEMPT_WRONG_HOUSEHOLD: &str = "attempt-wrong-household";
const ENCRYPTION_ATTEMPT_REVOKED_CHILD: &str = "attempt-revoked-child";
const ENCRYPTION_ATTEMPT_LINUX_LOST_KEY: &str = "attempt-linux-lost-key";
const ENCRYPTION_ATTEMPT_IOS_LIMITED: &str = "attempt-ios-limited";
const ENCRYPTION_KEY_CUSTODY_UPDATED_AT: &str = "2026-06-28T18:50:00.000Z";
const ENCRYPTION_EXPECT_CONTRACT_VERSION: &str = "contract version";
const ENCRYPTION_EXPECT_ATTEMPT_ID: &str = "attempt id";
const ENCRYPTION_EXPECT_TIMESTAMP: &str = "timestamp";
const ENCRYPTION_KEY_HIERARCHY_NOTE_CHILD_DEVICE_LOCAL: &str =
    "Device-bound child key unlocks only local evidence on the provisioned child device.";
const ENCRYPTION_KEY_HIERARCHY_NOTE_PARENT_DESKTOP: &str =
    "Parent desktop is the primary near-term parent decrypt surface for parent-owned bundles.";
const ENCRYPTION_KEY_HIERARCHY_NOTE_PARENT_MOBILE: &str =
    "Parent mobile remains a limited approval and decrypt path until device proof exists.";
const ENCRYPTION_KEY_HIERARCHY_NOTE_HOUSEHOLD_RECOVERY: &str =
    "Recovery is deliberate and parent-owned, not a default support path.";
const ENCRYPTION_KEY_HIERARCHY_NOTE_PROVIDER_AUTH_TOKEN: &str =
    "Provider tokens access APIs only and never become decrypt authority.";
const ENCRYPTION_KEY_HIERARCHY_NOTE_SUPPORT_DIAGNOSTIC_TOKEN: &str =
    "Support diagnostics never become a universal decrypt path.";
const PLATFORM_KEY_CUSTODY_NOTE_WINDOWS: &str =
    "First proof target with explicit user or machine scope.";
const PLATFORM_KEY_CUSTODY_NOTE_MACOS: &str =
    "macOS uses explicit Keychain-backed custody only when provisioned.";
const PLATFORM_KEY_CUSTODY_NOTE_LINUX: &str =
    "Linux remains manual-required until a real secret-store decision exists.";
const PLATFORM_KEY_CUSTODY_NOTE_ANDROID: &str =
    "Android remains limited until real device proof exists.";
const PLATFORM_KEY_CUSTODY_NOTE_IOS: &str = "iOS remains limited until real device proof exists.";
const PLATFORM_KEY_CUSTODY_NOTE_WEB_PORTAL: &str =
    "Hosted portal orchestrates status only and is never the decrypt root.";
const PLATFORM_KEY_CUSTODY_NOTE_PARENT_DESKTOP: &str =
    "Parent desktop is the primary parent-owned decrypt authority surface.";
const PLATFORM_KEY_CUSTODY_NOTE_CHILD_SERVICE: &str =
    "Child service owns child-device local evidence execution only.";
const PLATFORM_KEY_CUSTODY_NOTE_PARENT_MOBILE: &str =
    "Parent mobile remains limited/manual-required until device proof exists.";
const PLATFORM_KEY_CUSTODY_NOTE_CHILD_MOBILE: &str =
    "Child mobile has no broad custody claim without device proof.";
const DECRYPT_ATTEMPT_NOTE_PARENT_DESKTOP: &str =
    "Parent desktop may decrypt parent-owned bundles when provisioned for the right household.";
const DECRYPT_ATTEMPT_NOTE_WRONG_HOUSEHOLD: &str = "Wrong-household bundles fail closed.";
const DECRYPT_ATTEMPT_NOTE_REVOKED_CHILD: &str =
    "Revoked device keys fail closed for child-local evidence.";
const DECRYPT_ATTEMPT_NOTE_LINUX: &str =
    "Linux remains manual-required until a concrete secret-store decision exists.";
const DECRYPT_ATTEMPT_NOTE_IOS: &str = "iOS remains limited until real device proof exists.";

macro_rules! key_custody_text_identifier {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Option<Self> {
                let value = value.into();
                if value.trim().is_empty() {
                    None
                } else {
                    Some(Self(value))
                }
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

macro_rules! key_custody_string_enum {
    ($name:ident { $($variant:ident => $value:ident),+ $(,)? }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum $name {
            $(
                $variant,
            )+
        }

        impl $name {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$variant => $value,)+
                }
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;
                match value.as_str() {
                    $(candidate if candidate == $value => Ok(Self::$variant),)+
                    _ => Err(serde::de::Error::custom(format!("invalid {}: {}", stringify!($name), value))),
                }
            }
        }
    };
}

key_custody_string_enum!(EncryptionKeyClass {
    ChildDeviceLocalKey => ENCRYPTION_KEY_CLASS_CHILD_DEVICE_LOCAL_KEY,
    ParentDesktopKey => ENCRYPTION_KEY_CLASS_PARENT_DESKTOP_KEY,
    ParentMobileKey => ENCRYPTION_KEY_CLASS_PARENT_MOBILE_KEY,
    HouseholdRecoveryKey => ENCRYPTION_KEY_CLASS_HOUSEHOLD_RECOVERY_KEY,
    ProviderAuthToken => ENCRYPTION_KEY_CLASS_PROVIDER_AUTH_TOKEN,
    SupportDiagnosticToken => ENCRYPTION_KEY_CLASS_SUPPORT_DIAGNOSTIC_TOKEN,
});

key_custody_string_enum!(EncryptionKeyHolder {
    ChildDevice => ENCRYPTION_KEY_HOLDER_CHILD_DEVICE,
    ParentDesktop => ENCRYPTION_KEY_HOLDER_PARENT_DESKTOP,
    ParentMobile => ENCRYPTION_KEY_HOLDER_PARENT_MOBILE,
    HouseholdRecoveryPath => ENCRYPTION_KEY_HOLDER_HOUSEHOLD_RECOVERY_PATH,
    ProviderConnection => ENCRYPTION_KEY_HOLDER_PROVIDER_CONNECTION,
    SupportFlow => ENCRYPTION_KEY_HOLDER_SUPPORT_FLOW,
    HostedPortal => ENCRYPTION_KEY_HOLDER_HOSTED_PORTAL,
});

key_custody_string_enum!(EncryptionUnlockScope {
    ChildEvidenceLocal => ENCRYPTION_UNLOCK_SCOPE_CHILD_EVIDENCE_LOCAL,
    ParentOwnedBundle => ENCRYPTION_UNLOCK_SCOPE_PARENT_OWNED_BUNDLE,
    ParentCacheReports => ENCRYPTION_UNLOCK_SCOPE_PARENT_CACHE_REPORTS,
    HouseholdRecoveryBundle => ENCRYPTION_UNLOCK_SCOPE_HOUSEHOLD_RECOVERY_BUNDLE,
    ProviderApiOnly => ENCRYPTION_UNLOCK_SCOPE_PROVIDER_API_ONLY,
    DiagnosticsMetadataOnly => ENCRYPTION_UNLOCK_SCOPE_DIAGNOSTICS_METADATA_ONLY,
    StatusOnly => ENCRYPTION_UNLOCK_SCOPE_STATUS_ONLY,
});

key_custody_string_enum!(PlatformKeyCustodySurface {
    Windows => PLATFORM_KEY_CUSTODY_SURFACE_WINDOWS,
    MacOs => PLATFORM_KEY_CUSTODY_SURFACE_MACOS,
    Linux => PLATFORM_KEY_CUSTODY_SURFACE_LINUX,
    Android => PLATFORM_KEY_CUSTODY_SURFACE_ANDROID,
    IOS => PLATFORM_KEY_CUSTODY_SURFACE_IOS,
    WebPortal => PLATFORM_KEY_CUSTODY_SURFACE_WEB_PORTAL,
    ParentDesktop => PLATFORM_KEY_CUSTODY_SURFACE_PARENT_DESKTOP,
    ChildService => PLATFORM_KEY_CUSTODY_SURFACE_CHILD_SERVICE,
    ParentMobile => PLATFORM_KEY_CUSTODY_SURFACE_PARENT_MOBILE,
    ChildMobile => PLATFORM_KEY_CUSTODY_SURFACE_CHILD_MOBILE,
});

key_custody_string_enum!(PlatformKeyStoreKind {
    WindowsDpapiUser => PLATFORM_KEY_STORE_KIND_WINDOWS_DPAPI_USER,
    WindowsDpapiMachine => PLATFORM_KEY_STORE_KIND_WINDOWS_DPAPI_MACHINE,
    MacOsKeychain => PLATFORM_KEY_STORE_KIND_MACOS_KEYCHAIN,
    SecureEnclaveBacked => PLATFORM_KEY_STORE_KIND_SECURE_ENCLAVE_BACKED,
    LinuxSecretStoreUndecided => PLATFORM_KEY_STORE_KIND_LINUX_SECRET_STORE_UNDECIDED,
    AndroidKeystore => PLATFORM_KEY_STORE_KIND_ANDROID_KEYSTORE,
    IOSKeychain => PLATFORM_KEY_STORE_KIND_IOS_KEYCHAIN,
    NoDecryptRoot => PLATFORM_KEY_STORE_KIND_NO_DECRYPT_ROOT,
    ParentDesktopLocalKeyPath => PLATFORM_KEY_STORE_KIND_PARENT_DESKTOP_LOCAL_KEY_PATH,
    ChildServiceLocalKeyPath => PLATFORM_KEY_STORE_KIND_CHILD_SERVICE_LOCAL_KEY_PATH,
    ParentMobileApprovalPath => PLATFORM_KEY_STORE_KIND_PARENT_MOBILE_APPROVAL_PATH,
    ChildMobilePlatformKeyPath => PLATFORM_KEY_STORE_KIND_CHILD_MOBILE_PLATFORM_KEY_PATH,
});

key_custody_string_enum!(PlatformDecryptAuthority {
    ChildLocalEvidenceOnly => PLATFORM_DECRYPT_AUTHORITY_CHILD_LOCAL_EVIDENCE_ONLY,
    ParentOwnedBundlesOnly => PLATFORM_DECRYPT_AUTHORITY_PARENT_OWNED_BUNDLES_ONLY,
    ParentCacheReportsAndBundles => PLATFORM_DECRYPT_AUTHORITY_PARENT_CACHE_REPORTS_AND_BUNDLES,
    HouseholdRecoveryBundlesOnly => PLATFORM_DECRYPT_AUTHORITY_HOUSEHOLD_RECOVERY_BUNDLES_ONLY,
    NotDecryptRoot => PLATFORM_DECRYPT_AUTHORITY_NOT_DECRYPT_ROOT,
    ManualRequired => PLATFORM_DECRYPT_AUTHORITY_MANUAL_REQUIRED,
});

key_custody_string_enum!(KeyCustodyState {
    KeyAvailable => KEY_CUSTODY_STATE_KEY_AVAILABLE,
    KeyUnavailable => KEY_CUSTODY_STATE_KEY_UNAVAILABLE,
    KeyRevoked => KEY_CUSTODY_STATE_KEY_REVOKED,
    WrongHousehold => KEY_CUSTODY_STATE_WRONG_HOUSEHOLD,
    WrongDevice => KEY_CUSTODY_STATE_WRONG_DEVICE,
    ReinstallRequired => KEY_CUSTODY_STATE_REINSTALL_REQUIRED,
    RecoveryAvailable => KEY_CUSTODY_STATE_RECOVERY_AVAILABLE,
    RecoveryNotSupported => KEY_CUSTODY_STATE_RECOVERY_NOT_SUPPORTED,
});

key_custody_string_enum!(RecoveryMode {
    ManualRequired => RECOVERY_MODE_MANUAL_REQUIRED,
    ParentOwnedRecovery => RECOVERY_MODE_PARENT_OWNED_RECOVERY,
    NotSupported => RECOVERY_MODE_NOT_SUPPORTED,
});

key_custody_string_enum!(DecryptDecisionState {
    Allowed => DECRYPT_DECISION_STATE_ALLOWED,
    WrongHouseholdDenied => DECRYPT_DECISION_STATE_WRONG_HOUSEHOLD_DENIED,
    WrongDeviceDenied => DECRYPT_DECISION_STATE_WRONG_DEVICE_DENIED,
    RevokedKeyDenied => DECRYPT_DECISION_STATE_REVOKED_KEY_DENIED,
    LostKeyManualRequired => DECRYPT_DECISION_STATE_LOST_KEY_MANUAL_REQUIRED,
    RecoveryAvailableManualRequired => DECRYPT_DECISION_STATE_RECOVERY_AVAILABLE_MANUAL_REQUIRED,
    LimitedUntilDeviceProof => DECRYPT_DECISION_STATE_LIMITED_UNTIL_DEVICE_PROOF,
    NotDecryptRootDenied => DECRYPT_DECISION_STATE_NOT_DECRYPT_ROOT_DENIED,
    PlatformManualRequired => DECRYPT_DECISION_STATE_PLATFORM_MANUAL_REQUIRED,
});

key_custody_string_enum!(EncryptionKeyNonClaim {
    NoUniversalOcentraKey => ENCRYPTION_KEY_NON_CLAIM_NO_UNIVERSAL_OCENTRA_KEY,
    NoHostedDecryptRoot => ENCRYPTION_KEY_NON_CLAIM_NO_HOSTED_DECRYPT_ROOT,
    NoPlaintextFallback => ENCRYPTION_KEY_NON_CLAIM_NO_PLAINTEXT_FALLBACK,
    NoTsBusinessOwner => ENCRYPTION_KEY_NON_CLAIM_NO_TS_BUSINESS_OWNER,
    NoLanOwnership => ENCRYPTION_KEY_NON_CLAIM_NO_LAN_OWNERSHIP,
    NoMobileBroadClaim => ENCRYPTION_KEY_NON_CLAIM_NO_MOBILE_BROAD_CLAIM,
});

key_custody_text_identifier!(EncryptionKeyContractVersion);
key_custody_text_identifier!(EncryptionHouseholdId);
key_custody_text_identifier!(EncryptionDeviceId);
key_custody_text_identifier!(EncryptionAttemptId);
key_custody_text_identifier!(EncryptionTimestamp);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncryptionKeyHierarchyRow {
    pub key_class: EncryptionKeyClass,
    pub default_holder: EncryptionKeyHolder,
    pub unlock_scope: EncryptionUnlockScope,
    pub may_decrypt_child_evidence: bool,
    pub may_decrypt_parent_exports: bool,
    pub default_by_product: bool,
    pub notes: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformKeyCustodyRow {
    pub surface: PlatformKeyCustodySurface,
    pub key_store: PlatformKeyStoreKind,
    pub decrypt_authority: PlatformDecryptAuthority,
    pub manual_required: bool,
    pub device_proof_required: bool,
    pub wrong_household_fails_closed: bool,
    pub wrong_device_fails_closed: bool,
    pub revoked_key_fails_closed: bool,
    pub notes: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecryptAttemptResult {
    pub attempt_id: EncryptionAttemptId,
    pub surface: PlatformKeyCustodySurface,
    pub requested_scope: EncryptionUnlockScope,
    pub state: DecryptDecisionState,
    pub decrypt_allowed: bool,
    pub fail_closed: bool,
    pub manual_required: bool,
    pub used_recovery_path: bool,
    pub notes: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncryptionKeyCustodyContractProof {
    pub schema_version: String,
    pub contract_version: EncryptionKeyContractVersion,
    pub key_hierarchy: Vec<EncryptionKeyHierarchyRow>,
    pub platform_matrix: Vec<PlatformKeyCustodyRow>,
    pub attempts: Vec<DecryptAttemptResult>,
    pub non_claims: Vec<EncryptionKeyNonClaim>,
    pub universal_ocentra_key_present: bool,
    pub hosted_portal_decrypt_root: bool,
    pub updated_at: EncryptionTimestamp,
}

#[derive(Clone, Copy)]
struct HierarchySeed {
    key_class: EncryptionKeyClass,
    default_holder: EncryptionKeyHolder,
    unlock_scope: EncryptionUnlockScope,
    may_decrypt_child_evidence: bool,
    may_decrypt_parent_exports: bool,
    default_by_product: bool,
    notes: &'static str,
}

#[derive(Clone, Copy)]
struct PlatformSeed {
    surface: PlatformKeyCustodySurface,
    key_store: PlatformKeyStoreKind,
    decrypt_authority: PlatformDecryptAuthority,
    manual_required: bool,
    device_proof_required: bool,
    notes: &'static str,
}

#[derive(Clone, Copy)]
struct AttemptSeed {
    attempt_id: &'static str,
    surface: PlatformKeyCustodySurface,
    requested_scope: EncryptionUnlockScope,
    state: DecryptDecisionState,
    decrypt_allowed: bool,
    fail_closed: bool,
    manual_required: bool,
    notes: &'static str,
}

const ENCRYPTION_KEY_HIERARCHY_SEEDS: &[HierarchySeed] = &[
    HierarchySeed {
        key_class: EncryptionKeyClass::ChildDeviceLocalKey,
        default_holder: EncryptionKeyHolder::ChildDevice,
        unlock_scope: EncryptionUnlockScope::ChildEvidenceLocal,
        may_decrypt_child_evidence: true,
        may_decrypt_parent_exports: false,
        default_by_product: true,
        notes: ENCRYPTION_KEY_HIERARCHY_NOTE_CHILD_DEVICE_LOCAL,
    },
    HierarchySeed {
        key_class: EncryptionKeyClass::ParentDesktopKey,
        default_holder: EncryptionKeyHolder::ParentDesktop,
        unlock_scope: EncryptionUnlockScope::ParentOwnedBundle,
        may_decrypt_child_evidence: false,
        may_decrypt_parent_exports: true,
        default_by_product: true,
        notes: ENCRYPTION_KEY_HIERARCHY_NOTE_PARENT_DESKTOP,
    },
    HierarchySeed {
        key_class: EncryptionKeyClass::ParentMobileKey,
        default_holder: EncryptionKeyHolder::ParentMobile,
        unlock_scope: EncryptionUnlockScope::ParentOwnedBundle,
        may_decrypt_child_evidence: false,
        may_decrypt_parent_exports: true,
        default_by_product: false,
        notes: ENCRYPTION_KEY_HIERARCHY_NOTE_PARENT_MOBILE,
    },
    HierarchySeed {
        key_class: EncryptionKeyClass::HouseholdRecoveryKey,
        default_holder: EncryptionKeyHolder::HouseholdRecoveryPath,
        unlock_scope: EncryptionUnlockScope::HouseholdRecoveryBundle,
        may_decrypt_child_evidence: false,
        may_decrypt_parent_exports: true,
        default_by_product: false,
        notes: ENCRYPTION_KEY_HIERARCHY_NOTE_HOUSEHOLD_RECOVERY,
    },
    HierarchySeed {
        key_class: EncryptionKeyClass::ProviderAuthToken,
        default_holder: EncryptionKeyHolder::ProviderConnection,
        unlock_scope: EncryptionUnlockScope::ProviderApiOnly,
        may_decrypt_child_evidence: false,
        may_decrypt_parent_exports: false,
        default_by_product: false,
        notes: ENCRYPTION_KEY_HIERARCHY_NOTE_PROVIDER_AUTH_TOKEN,
    },
    HierarchySeed {
        key_class: EncryptionKeyClass::SupportDiagnosticToken,
        default_holder: EncryptionKeyHolder::SupportFlow,
        unlock_scope: EncryptionUnlockScope::DiagnosticsMetadataOnly,
        may_decrypt_child_evidence: false,
        may_decrypt_parent_exports: false,
        default_by_product: false,
        notes: ENCRYPTION_KEY_HIERARCHY_NOTE_SUPPORT_DIAGNOSTIC_TOKEN,
    },
];

const PLATFORM_KEY_CUSTODY_SEEDS: &[PlatformSeed] = &[
    PlatformSeed {
        surface: PlatformKeyCustodySurface::Windows,
        key_store: PlatformKeyStoreKind::WindowsDpapiUser,
        decrypt_authority: PlatformDecryptAuthority::ParentCacheReportsAndBundles,
        manual_required: false,
        device_proof_required: false,
        notes: PLATFORM_KEY_CUSTODY_NOTE_WINDOWS,
    },
    PlatformSeed {
        surface: PlatformKeyCustodySurface::MacOs,
        key_store: PlatformKeyStoreKind::MacOsKeychain,
        decrypt_authority: PlatformDecryptAuthority::ParentOwnedBundlesOnly,
        manual_required: false,
        device_proof_required: false,
        notes: PLATFORM_KEY_CUSTODY_NOTE_MACOS,
    },
    PlatformSeed {
        surface: PlatformKeyCustodySurface::Linux,
        key_store: PlatformKeyStoreKind::LinuxSecretStoreUndecided,
        decrypt_authority: PlatformDecryptAuthority::ManualRequired,
        manual_required: true,
        device_proof_required: false,
        notes: PLATFORM_KEY_CUSTODY_NOTE_LINUX,
    },
    PlatformSeed {
        surface: PlatformKeyCustodySurface::Android,
        key_store: PlatformKeyStoreKind::AndroidKeystore,
        decrypt_authority: PlatformDecryptAuthority::ManualRequired,
        manual_required: true,
        device_proof_required: true,
        notes: PLATFORM_KEY_CUSTODY_NOTE_ANDROID,
    },
    PlatformSeed {
        surface: PlatformKeyCustodySurface::IOS,
        key_store: PlatformKeyStoreKind::IOSKeychain,
        decrypt_authority: PlatformDecryptAuthority::ManualRequired,
        manual_required: true,
        device_proof_required: true,
        notes: PLATFORM_KEY_CUSTODY_NOTE_IOS,
    },
    PlatformSeed {
        surface: PlatformKeyCustodySurface::WebPortal,
        key_store: PlatformKeyStoreKind::NoDecryptRoot,
        decrypt_authority: PlatformDecryptAuthority::NotDecryptRoot,
        manual_required: true,
        device_proof_required: false,
        notes: PLATFORM_KEY_CUSTODY_NOTE_WEB_PORTAL,
    },
    PlatformSeed {
        surface: PlatformKeyCustodySurface::ParentDesktop,
        key_store: PlatformKeyStoreKind::ParentDesktopLocalKeyPath,
        decrypt_authority: PlatformDecryptAuthority::ParentOwnedBundlesOnly,
        manual_required: false,
        device_proof_required: false,
        notes: PLATFORM_KEY_CUSTODY_NOTE_PARENT_DESKTOP,
    },
    PlatformSeed {
        surface: PlatformKeyCustodySurface::ChildService,
        key_store: PlatformKeyStoreKind::ChildServiceLocalKeyPath,
        decrypt_authority: PlatformDecryptAuthority::ChildLocalEvidenceOnly,
        manual_required: false,
        device_proof_required: false,
        notes: PLATFORM_KEY_CUSTODY_NOTE_CHILD_SERVICE,
    },
    PlatformSeed {
        surface: PlatformKeyCustodySurface::ParentMobile,
        key_store: PlatformKeyStoreKind::ParentMobileApprovalPath,
        decrypt_authority: PlatformDecryptAuthority::ManualRequired,
        manual_required: true,
        device_proof_required: true,
        notes: PLATFORM_KEY_CUSTODY_NOTE_PARENT_MOBILE,
    },
    PlatformSeed {
        surface: PlatformKeyCustodySurface::ChildMobile,
        key_store: PlatformKeyStoreKind::ChildMobilePlatformKeyPath,
        decrypt_authority: PlatformDecryptAuthority::ManualRequired,
        manual_required: true,
        device_proof_required: true,
        notes: PLATFORM_KEY_CUSTODY_NOTE_CHILD_MOBILE,
    },
];

const DECRYPT_ATTEMPT_SEEDS: &[AttemptSeed] = &[
    AttemptSeed {
        attempt_id: ENCRYPTION_ATTEMPT_WINDOWS_PARENT,
        surface: PlatformKeyCustodySurface::ParentDesktop,
        requested_scope: EncryptionUnlockScope::ParentOwnedBundle,
        state: DecryptDecisionState::Allowed,
        decrypt_allowed: true,
        fail_closed: false,
        manual_required: false,
        notes: DECRYPT_ATTEMPT_NOTE_PARENT_DESKTOP,
    },
    AttemptSeed {
        attempt_id: ENCRYPTION_ATTEMPT_WRONG_HOUSEHOLD,
        surface: PlatformKeyCustodySurface::ParentDesktop,
        requested_scope: EncryptionUnlockScope::ParentOwnedBundle,
        state: DecryptDecisionState::WrongHouseholdDenied,
        decrypt_allowed: false,
        fail_closed: true,
        manual_required: false,
        notes: DECRYPT_ATTEMPT_NOTE_WRONG_HOUSEHOLD,
    },
    AttemptSeed {
        attempt_id: ENCRYPTION_ATTEMPT_REVOKED_CHILD,
        surface: PlatformKeyCustodySurface::ChildService,
        requested_scope: EncryptionUnlockScope::ChildEvidenceLocal,
        state: DecryptDecisionState::RevokedKeyDenied,
        decrypt_allowed: false,
        fail_closed: true,
        manual_required: false,
        notes: DECRYPT_ATTEMPT_NOTE_REVOKED_CHILD,
    },
    AttemptSeed {
        attempt_id: ENCRYPTION_ATTEMPT_LINUX_LOST_KEY,
        surface: PlatformKeyCustodySurface::Linux,
        requested_scope: EncryptionUnlockScope::ParentOwnedBundle,
        state: DecryptDecisionState::PlatformManualRequired,
        decrypt_allowed: false,
        fail_closed: true,
        manual_required: true,
        notes: DECRYPT_ATTEMPT_NOTE_LINUX,
    },
    AttemptSeed {
        attempt_id: ENCRYPTION_ATTEMPT_IOS_LIMITED,
        surface: PlatformKeyCustodySurface::IOS,
        requested_scope: EncryptionUnlockScope::ParentOwnedBundle,
        state: DecryptDecisionState::LimitedUntilDeviceProof,
        decrypt_allowed: false,
        fail_closed: true,
        manual_required: true,
        notes: DECRYPT_ATTEMPT_NOTE_IOS,
    },
];

pub fn required_encryption_key_classes() -> Vec<EncryptionKeyClass> {
    vec![
        EncryptionKeyClass::ChildDeviceLocalKey,
        EncryptionKeyClass::ParentDesktopKey,
        EncryptionKeyClass::ParentMobileKey,
        EncryptionKeyClass::HouseholdRecoveryKey,
        EncryptionKeyClass::ProviderAuthToken,
        EncryptionKeyClass::SupportDiagnosticToken,
    ]
}

pub fn required_platform_key_surfaces() -> Vec<PlatformKeyCustodySurface> {
    vec![
        PlatformKeyCustodySurface::Windows,
        PlatformKeyCustodySurface::MacOs,
        PlatformKeyCustodySurface::Linux,
        PlatformKeyCustodySurface::Android,
        PlatformKeyCustodySurface::IOS,
        PlatformKeyCustodySurface::WebPortal,
        PlatformKeyCustodySurface::ParentDesktop,
        PlatformKeyCustodySurface::ChildService,
        PlatformKeyCustodySurface::ParentMobile,
        PlatformKeyCustodySurface::ChildMobile,
    ]
}

pub fn required_key_custody_non_claims() -> Vec<EncryptionKeyNonClaim> {
    vec![
        EncryptionKeyNonClaim::NoUniversalOcentraKey,
        EncryptionKeyNonClaim::NoHostedDecryptRoot,
        EncryptionKeyNonClaim::NoPlaintextFallback,
        EncryptionKeyNonClaim::NoTsBusinessOwner,
        EncryptionKeyNonClaim::NoLanOwnership,
        EncryptionKeyNonClaim::NoMobileBroadClaim,
    ]
}

pub fn sample_encryption_key_custody_contract_proof() -> EncryptionKeyCustodyContractProof {
    EncryptionKeyCustodyContractProof {
        schema_version: ENCRYPTION_KEY_CUSTODY_SCHEMA_VERSION.to_string(),
        contract_version: contract_version(ENCRYPTION_KEY_CONTRACT_VERSION_V0_2),
        key_hierarchy: ENCRYPTION_KEY_HIERARCHY_SEEDS
            .iter()
            .copied()
            .map(hierarchy_row)
            .collect(),
        platform_matrix: PLATFORM_KEY_CUSTODY_SEEDS
            .iter()
            .copied()
            .map(platform_row)
            .collect(),
        attempts: DECRYPT_ATTEMPT_SEEDS
            .iter()
            .copied()
            .map(attempt_result)
            .collect(),
        non_claims: required_key_custody_non_claims(),
        universal_ocentra_key_present: false,
        hosted_portal_decrypt_root: false,
        updated_at: timestamp(ENCRYPTION_KEY_CUSTODY_UPDATED_AT),
    }
}

fn hierarchy_row(seed: HierarchySeed) -> EncryptionKeyHierarchyRow {
    EncryptionKeyHierarchyRow {
        key_class: seed.key_class,
        default_holder: seed.default_holder,
        unlock_scope: seed.unlock_scope,
        may_decrypt_child_evidence: seed.may_decrypt_child_evidence,
        may_decrypt_parent_exports: seed.may_decrypt_parent_exports,
        default_by_product: seed.default_by_product,
        notes: seed.notes.to_string(),
    }
}

fn platform_row(seed: PlatformSeed) -> PlatformKeyCustodyRow {
    PlatformKeyCustodyRow {
        surface: seed.surface,
        key_store: seed.key_store,
        decrypt_authority: seed.decrypt_authority,
        manual_required: seed.manual_required,
        device_proof_required: seed.device_proof_required,
        wrong_household_fails_closed: true,
        wrong_device_fails_closed: true,
        revoked_key_fails_closed: true,
        notes: seed.notes.to_string(),
    }
}

fn attempt_result(seed: AttemptSeed) -> DecryptAttemptResult {
    DecryptAttemptResult {
        attempt_id: attempt_id(seed.attempt_id),
        surface: seed.surface,
        requested_scope: seed.requested_scope,
        state: seed.state,
        decrypt_allowed: seed.decrypt_allowed,
        fail_closed: seed.fail_closed,
        manual_required: seed.manual_required,
        used_recovery_path: seed.state == DecryptDecisionState::RecoveryAvailableManualRequired,
        notes: seed.notes.to_string(),
    }
}

fn contract_version(value: &str) -> EncryptionKeyContractVersion {
    crate::schema_option_or_unreachable(
        EncryptionKeyContractVersion::parse(value),
        ENCRYPTION_EXPECT_CONTRACT_VERSION,
    )
}

fn attempt_id(value: &str) -> EncryptionAttemptId {
    crate::schema_option_or_unreachable(
        EncryptionAttemptId::parse(value),
        ENCRYPTION_EXPECT_ATTEMPT_ID,
    )
}

fn timestamp(value: &str) -> EncryptionTimestamp {
    crate::schema_option_or_unreachable(
        EncryptionTimestamp::parse(value),
        ENCRYPTION_EXPECT_TIMESTAMP,
    )
}
