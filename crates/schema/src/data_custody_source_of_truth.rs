use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

mod identifiers;
mod sample;

macro_rules! data_custody_identifier {
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

data_custody_identifier!(DataCustodySourceOfTruthRowId);
data_custody_identifier!(DataCustodySourceOfTruthMatrixId);

pub const DATA_CUSTODY_SOURCE_OF_TRUTH_SCHEMA_VERSION: &str = "data-custody-source-of-truth-proof";
pub const PARENT_CONTRACT_SCHEMA_VERSION: &str = "v0.6";
const DATA_CUSTODY_KNOWN_GAP_SUPPORT_DECRYPT_BY_DEFAULT: &str =
    "Support decrypt-by-default remains false until product and key-custody decisions exist.";
const DATA_CUSTODY_KNOWN_GAP_PROVIDER_MODE_DEFAULTS: &str =
    "Provider mode defaults stay explicit and are not implied by the custody matrix.";
const DATA_CUSTODY_KNOWN_GAP_MOBILE_RESTORE_AND_KEY_CUSTODY: &str =
    "Mobile restore and key-custody proof stays manual-required outside this workpack.";
const DATA_CUSTODY_KNOWN_GAP_DELETE_ERGONOMICS: &str =
    "Delete ergonomics and tombstone propagation are owned by later data-custody workpacks.";
const DATA_CUSTODY_KNOWN_GAP_TRANSFER_RUNTIME_SCOPE: &str = "The matrix records source truth and no-hosting boundaries; it does not claim transfer runtime.";
const DATA_CUSTODY_ROW_ID_EXPECTATION: &str = "data custody source-of-truth row id";
const DATA_CUSTODY_MATRIX_ID_EXPECTATION: &str = "data custody source-of-truth matrix id";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataCustodySourceOfTruthKind {
    #[serde(rename = "self")]
    Self_,
    #[serde(rename = "derived-from-data-classes")]
    DerivedFromDataClasses,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataCustodySourceOfTruth {
    pub kind: DataCustodySourceOfTruthKind,
    pub source_class_ids: Vec<DataCustodyClassId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DataCustodyAuthority {
    OcentraAccountControlPlane,
    PaymentControlPlane,
    BillingProvider,
    HouseholdControlPlane,
    OcentraRoutingService,
    OcentraReportStatusRuntime,
    SupportSystem,
    PublicReleasePipeline,
    ChildDevice,
    ParentDevice,
    ParentOwnedStorage,
    SupportExportBoundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DataCustodyDefaultLocation {
    OcentraAccountMetadataStore,
    OcentraBillingMetadataStore,
    BillingProviderCustomerRecord,
    OcentraLicenseUpdateStore,
    HouseholdDeviceRegistry,
    OcentraHouseholdRouteStore,
    HouseholdSetupDraftStore,
    OcentraNotificationRouteStore,
    OcentraShortLivedReportStatusStore,
    OcentraSupportCaseStore,
    PublicReleaseSurface,
    HouseholdProfileStore,
    HouseholdRuleStore,
    HouseholdAuditStore,
    ChildDeviceEncryptedJournal,
    ChildDeviceLocalQueryStore,
    ChildDeviceSensitiveEvidenceStore,
    ChildDeviceLocationStore,
    ChildDeviceLocalAiStore,
    ParentDeviceReportCache,
    ParentDeviceNotificationHistoryCache,
    ParentAssistantEphemeralContext,
    ParentOwnedEncryptedStorage,
    ProviderEnvelopeMetadata,
    SupportExportArtifact,
    HouseholdKeyStore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DataCustodyExposure {
    None,
    AllowedReferencesOnly,
    RedactedMetadataOnly,
    Minimal,
    DerivedOutputOnly,
    Public,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DataCustodyOcentraHostingMode {
    Forbidden,
    AllowedMetadataOnly,
    ShortLivedStatusOnly,
    PublicReleaseOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DataCustodyNonClaim {
    NoDefaultOcentraChildActivityStore,
    NoSqliteTruthLayer,
    NoProviderAutoApply,
    NoSupportDecryptDefault,
    NoOcentraOwnedParentRules,
    NoRawChildEvidenceInNotifications,
    NoLongLivedHostedReports,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DataCustodyClassId {
    AccountIdentityMetadata,
    SubscriptionEntitlementMetadata,
    BillingProviderIdentityReference,
    LicenseDownloadUpdateMetadata,
    HouseholdDeviceRegistry,
    DeviceRegistrationPairingRouteMetadata,
    SetupStateAndPairingDraft,
    MinimalNotificationRoutingMetadata,
    ShortLivedReportCompilerStatus,
    SupportCaseMetadata,
    PublicWebsiteReleaseStatus,
    ChildProfile,
    ParentRulesAndApprovalHistory,
    AuditLog,
    EvidenceJournalSegments,
    SqliteEvidenceReadModelDatabase,
    ScreenshotsAndScreenAnalysisImages,
    BrowserUrlHistory,
    NetworkAppGameEvidence,
    LocationTrackingEvidence,
    LocalAiAndPolicyDecisions,
    GeneratedLongTermReports,
    ParentNotificationHistoryCache,
    AssistantChildEvidenceContext,
    ParentOwnedStorageContents,
    ProviderSyncPayloads,
    SupportBundlesContainingRawChildActivity,
    UniversalDecryptKeys,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataCustodyHostingPolicy {
    pub ocentra_hosting_mode: DataCustodyOcentraHostingMode,
    pub parent_owned_storage_allowed: bool,
    pub provider_metadata_allowed: bool,
    pub support_export_parent_initiated_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataCustodySourceOfTruthRow {
    pub row_id: DataCustodySourceOfTruthRowId,
    pub class_id: DataCustodyClassId,
    pub class_label: String,
    pub source_owner: String,
    pub source_of_truth: DataCustodySourceOfTruth,
    pub custody_authority: DataCustodyAuthority,
    pub default_location: DataCustodyDefaultLocation,
    pub ocentra_hosted_by_default: bool,
    pub must_never_be_hosted_by_default: bool,
    pub encrypted_before_upload: bool,
    pub may_appear_in_reports: bool,
    pub may_appear_in_notifications: bool,
    pub report_exposure: DataCustodyExposure,
    pub notification_exposure: DataCustodyExposure,
    pub raw_child_evidence_allowed: bool,
    pub derived_use_only: bool,
    pub sensitive: bool,
    pub hosting_policy: DataCustodyHostingPolicy,
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataCustodySourceOfTruthContractProof {
    pub schema_version: String,
    pub contract_version: String,
    pub matrix_id: DataCustodySourceOfTruthMatrixId,
    pub rows: Vec<DataCustodySourceOfTruthRow>,
    pub allowed_ocentra_hosted_metadata: Vec<DataCustodyClassId>,
    pub must_never_be_hosted_by_default: Vec<DataCustodyClassId>,
    pub claim_safe_language: Vec<String>,
    pub non_claims: Vec<DataCustodyNonClaim>,
    pub account_control_plane_separated: bool,
    pub provider_owned_billing_identity_separated: bool,
    pub ocentra_is_default_child_data_store: bool,
    pub provider_auto_apply_claimed: bool,
    pub support_decrypt_by_default_claimed: bool,
    pub sqlite_as_truth_layer_claimed: bool,
    pub raw_child_activity_hosted_by_default_claimed: bool,
    pub updated_at: String,
}

pub fn data_custody_source_of_truth_known_gaps() -> Vec<&'static str> {
    vec![
        DATA_CUSTODY_KNOWN_GAP_SUPPORT_DECRYPT_BY_DEFAULT,
        DATA_CUSTODY_KNOWN_GAP_PROVIDER_MODE_DEFAULTS,
        DATA_CUSTODY_KNOWN_GAP_MOBILE_RESTORE_AND_KEY_CUSTODY,
        DATA_CUSTODY_KNOWN_GAP_DELETE_ERGONOMICS,
        DATA_CUSTODY_KNOWN_GAP_TRANSFER_RUNTIME_SCOPE,
    ]
}

pub fn sample_data_custody_source_of_truth_contract_proof() -> DataCustodySourceOfTruthContractProof
{
    sample::sample_data_custody_source_of_truth_contract_proof()
}
