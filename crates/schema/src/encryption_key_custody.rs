use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

pub const ENCRYPTION_KEY_CUSTODY_SCHEMA_VERSION: &str = "encryption-key-custody-proof";

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
                (!value.trim().is_empty()).then_some(Self(value))
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

macro_rules! key_custody_string_enums {
    ($(
        $name:ident {
            variants: [$($variant:ident),+ $(,)?],
            values: [$($value:literal),+ $(,)?]
        }
    ),+ $(,)?) => {
        $(
            #[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
            pub enum $name {
                $(
                    #[serde(rename = $value)]
                    $variant,
                )+
            }

            impl $name {
                pub fn as_str(&self) -> &'static str {
                    const VALUES: &[&str] = &[$($value),+];
                    VALUES[*self as usize]
                }
            }
        )+
    };
}

key_custody_string_enums!(
    EncryptionKeyClass {
        variants: [
            ChildDeviceLocalKey,
            ParentDesktopKey,
            ParentMobileKey,
            HouseholdRecoveryKey,
            ProviderAuthToken,
            SupportDiagnosticToken,
        ],
        values: [
            "child-device-local-key",
            "parent-desktop-key",
            "parent-mobile-key",
            "household-recovery-key",
            "provider-auth-token",
            "support-diagnostic-token",
        ]
    },
    EncryptionKeyHolder {
        variants: [
            ChildDevice,
            ParentDesktop,
            ParentMobile,
            HouseholdRecoveryPath,
            ProviderConnection,
            SupportFlow,
            HostedPortal,
        ],
        values: [
            "child-device",
            "parent-desktop",
            "parent-mobile",
            "household-recovery-path",
            "provider-connection",
            "support-flow",
            "hosted-portal",
        ]
    },
    EncryptionUnlockScope {
        variants: [
            ChildEvidenceLocal,
            ParentOwnedBundle,
            ParentCacheReports,
            HouseholdRecoveryBundle,
            ProviderApiOnly,
            DiagnosticsMetadataOnly,
            StatusOnly,
        ],
        values: [
            "child-evidence-local",
            "parent-owned-bundle",
            "parent-cache-reports",
            "household-recovery-bundle",
            "provider-api-only",
            "diagnostics-metadata-only",
            "status-only",
        ]
    },
    PlatformKeyCustodySurface {
        variants: [
            Windows,
            MacOs,
            Linux,
            Android,
            IOS,
            WebPortal,
            ParentDesktop,
            ChildService,
            ParentMobile,
            ChildMobile,
        ],
        values: [
            "windows",
            "macos",
            "linux",
            "android",
            "ios",
            "web-portal",
            "parent-desktop",
            "child-service",
            "parent-mobile",
            "child-mobile",
        ]
    },
    PlatformKeyStoreKind {
        variants: [
            WindowsDpapiUser,
            WindowsDpapiMachine,
            MacOsKeychain,
            SecureEnclaveBacked,
            LinuxSecretStoreUndecided,
            AndroidKeystore,
            IOSKeychain,
            NoDecryptRoot,
            ParentDesktopLocalKeyPath,
            ChildServiceLocalKeyPath,
            ParentMobileApprovalPath,
            ChildMobilePlatformKeyPath,
        ],
        values: [
            "windows-dpapi-user",
            "windows-dpapi-machine",
            "macos-keychain",
            "secure-enclave-backed",
            "linux-secret-store-undecided",
            "android-keystore",
            "ios-keychain",
            "no-decrypt-root",
            "parent-desktop-local-key-path",
            "child-service-local-key-path",
            "parent-mobile-approval-path",
            "child-mobile-platform-key-path",
        ]
    },
    PlatformDecryptAuthority {
        variants: [
            ChildLocalEvidenceOnly,
            ParentOwnedBundlesOnly,
            ParentCacheReportsAndBundles,
            HouseholdRecoveryBundlesOnly,
            NotDecryptRoot,
            ManualRequired,
        ],
        values: [
            "child-local-evidence-only",
            "parent-owned-bundles-only",
            "parent-cache-reports-and-bundles",
            "household-recovery-bundles-only",
            "not-decrypt-root",
            "manual-required",
        ]
    },
    KeyCustodyState {
        variants: [
            KeyAvailable,
            KeyUnavailable,
            KeyRevoked,
            WrongHousehold,
            WrongDevice,
            ReinstallRequired,
            RecoveryAvailable,
            RecoveryNotSupported,
        ],
        values: [
            "keyAvailable",
            "keyUnavailable",
            "keyRevoked",
            "wrongHousehold",
            "wrongDevice",
            "reinstallRequired",
            "recoveryAvailable",
            "recoveryNotSupported",
        ]
    },
    RecoveryMode {
        variants: [ManualRequired, ParentOwnedRecovery, NotSupported,],
        values: ["manualRequired", "parent-owned-recovery", "notSupported",]
    },
    DecryptDecisionState {
        variants: [
            Allowed,
            WrongHouseholdDenied,
            WrongDeviceDenied,
            RevokedKeyDenied,
            LostKeyManualRequired,
            RecoveryAvailableManualRequired,
            LimitedUntilDeviceProof,
            NotDecryptRootDenied,
            PlatformManualRequired,
        ],
        values: [
            "allowed",
            "wrongHouseholdDenied",
            "wrongDeviceDenied",
            "revokedKeyDenied",
            "lostKeyManualRequired",
            "recoveryAvailableManualRequired",
            "limitedUntilDeviceProof",
            "notDecryptRootDenied",
            "platformManualRequired",
        ]
    },
    EncryptionKeyNonClaim {
        variants: [
            NoUniversalOcentraKey,
            NoHostedDecryptRoot,
            NoPlaintextFallback,
            NoTsBusinessOwner,
            NoLanOwnership,
            NoMobileBroadClaim,
        ],
        values: [
            "no-universal-ocentra-key",
            "no-hosted-decrypt-root",
            "no-plaintext-fallback",
            "no-ts-business-owner",
            "no-lan-ownership",
            "no-mobile-broad-claim",
        ]
    }
);

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
