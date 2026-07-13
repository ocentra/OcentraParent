use super::identifiers::{
    allowed_ocentra_hosted_metadata, data_custody_row, data_custody_source_of_truth_matrix_id,
    must_never_be_hosted_by_default, policy, source_derived, source_self, DataCustodyRowInput,
};
use super::*;

macro_rules! row {
    (
        $row_id:expr,
        $class_id:expr,
        $class_label:expr,
        $source_owner:expr,
        $source_of_truth:expr,
        $custody_authority:expr,
        $default_location:expr,
        $ocentra_hosted_by_default:expr,
        $must_never_be_hosted_by_default:expr,
        $encrypted_before_upload:expr,
        $may_appear_in_reports:expr,
        $may_appear_in_notifications:expr,
        $report_exposure:expr,
        $notification_exposure:expr,
        $raw_child_evidence_allowed:expr,
        $derived_use_only:expr,
        $sensitive:expr,
        $hosting_policy:expr,
        $notes:expr $(,)?
    ) => {
        data_custody_row(DataCustodyRowInput {
            row_id: $row_id,
            class_id: $class_id,
            class_label: $class_label,
            source_owner: $source_owner,
            source_of_truth: $source_of_truth,
            custody_authority: $custody_authority,
            default_location: $default_location,
            ocentra_hosted_by_default: $ocentra_hosted_by_default,
            must_never_be_hosted_by_default: $must_never_be_hosted_by_default,
            encrypted_before_upload: $encrypted_before_upload,
            may_appear_in_reports: $may_appear_in_reports,
            may_appear_in_notifications: $may_appear_in_notifications,
            report_exposure: $report_exposure,
            notification_exposure: $notification_exposure,
            raw_child_evidence_allowed: $raw_child_evidence_allowed,
            derived_use_only: $derived_use_only,
            sensitive: $sensitive,
            hosting_policy: $hosting_policy,
            notes: $notes,
        })
    };
}

pub(super) fn sample_data_custody_source_of_truth_contract_proof(
) -> DataCustodySourceOfTruthContractProof {
    DataCustodySourceOfTruthContractProof {
        schema_version: DATA_CUSTODY_SOURCE_OF_TRUTH_SCHEMA_VERSION.to_string(),
        contract_version: PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
        matrix_id: data_custody_source_of_truth_matrix_id(
            "data-custody-source-of-truth-wp01".to_string(),
        ),
        rows: data_custody_rows(),
        allowed_ocentra_hosted_metadata: allowed_ocentra_hosted_metadata(),
        must_never_be_hosted_by_default: must_never_be_hosted_by_default(),
        claim_safe_language: vec![
            format!("Ocentra-hosted infrastructure is not the default child-data store."),
            format!(
                "SQLite/read-model databases are rebuildable caches, not the evidence truth layer."
            ),
            format!(
                "Reports, notifications, and assistant context may reference allowed source data only."
            ),
            format!(
                "Provider payloads and support bundles require encryption and parent-initiated export before leaving the household boundary."
            ),
            format!(
                "Billing provider identity remains provider-owned even when entitlement metadata is mirrored in the control plane."
            ),
        ],
        non_claims: vec![
            DataCustodyNonClaim::NoDefaultOcentraChildActivityStore,
            DataCustodyNonClaim::NoSqliteTruthLayer,
            DataCustodyNonClaim::NoProviderAutoApply,
            DataCustodyNonClaim::NoSupportDecryptDefault,
            DataCustodyNonClaim::NoOcentraOwnedParentRules,
            DataCustodyNonClaim::NoRawChildEvidenceInNotifications,
            DataCustodyNonClaim::NoLongLivedHostedReports,
        ],
        account_control_plane_separated: true,
        provider_owned_billing_identity_separated: true,
        ocentra_is_default_child_data_store: false,
        provider_auto_apply_claimed: false,
        support_decrypt_by_default_claimed: false,
        sqlite_as_truth_layer_claimed: false,
        raw_child_activity_hosted_by_default_claimed: false,
        updated_at: "2026-06-28T18:44:00.000Z".to_string(),
    }
}

fn data_custody_rows() -> Vec<DataCustodySourceOfTruthRow> {
    let mut rows = data_custody_rows_01();

    rows.extend(data_custody_rows_02());

    rows.extend(data_custody_rows_03());

    rows.extend(data_custody_rows_04());

    rows.extend(data_custody_rows_05());

    rows.extend(data_custody_rows_06());

    rows.extend(data_custody_rows_07());

    rows.extend(data_custody_rows_08());

    rows.extend(data_custody_rows_09());

    rows.extend(data_custody_rows_10());

    rows.extend(data_custody_rows_11());

    rows.extend(data_custody_rows_12());

    rows.extend(data_custody_rows_13());

    rows.extend(data_custody_rows_14());

    rows
}

fn data_custody_rows_01() -> Vec<DataCustodySourceOfTruthRow> {
    vec![
        row!(
            format!("custody-row-account-identity-metadata"),
            DataCustodyClassId::AccountIdentityMetadata,
            format!("Account identity metadata"),
            format!("Control plane / account plan"),
            source_self(),
            DataCustodyAuthority::OcentraAccountControlPlane,
            DataCustodyDefaultLocation::OcentraAccountMetadataStore,
            true,
            false,
            true,
            true,
            true,
            DataCustodyExposure::RedactedMetadataOnly,
            DataCustodyExposure::RedactedMetadataOnly,
            false,
            false,
            true,
            policy(
                DataCustodyOcentraHostingMode::AllowedMetadataOnly,
                false,
                false,
                false,
            ),
            format!("Identity, household, and entitlement metadata only."),
        ),
        row!(
            format!("custody-row-subscription-entitlement-metadata"),
            DataCustodyClassId::SubscriptionEntitlementMetadata,
            format!("Subscription, billing, and entitlement metadata"),
            format!("Billing / payment plan"),
            source_self(),
            DataCustodyAuthority::PaymentControlPlane,
            DataCustodyDefaultLocation::OcentraBillingMetadataStore,
            true,
            false,
            true,
            true,
            false,
            DataCustodyExposure::RedactedMetadataOnly,
            DataCustodyExposure::None,
            false,
            false,
            true,
            policy(
                DataCustodyOcentraHostingMode::AllowedMetadataOnly,
                false,
                true,
                false,
            ),
            format!(
                "Billing and entitlement state may be mirrored, but child evidence stays separate."
            ),
        ),
    ]
}

fn data_custody_rows_02() -> Vec<DataCustodySourceOfTruthRow> {
    vec![
        row!(
            format!("custody-row-billing-provider-identity-reference"),
            DataCustodyClassId::BillingProviderIdentityReference,
            format!("Billing provider identity reference"),
            format!("Payment provider"),
            source_self(),
            DataCustodyAuthority::BillingProvider,
            DataCustodyDefaultLocation::BillingProviderCustomerRecord,
            false,
            false,
            true,
            true,
            false,
            DataCustodyExposure::RedactedMetadataOnly,
            DataCustodyExposure::None,
            false,
            false,
            true,
            policy(DataCustodyOcentraHostingMode::Forbidden, false, true, false),
            format!(
                "Provider-owned billing identity remains outside the parent evidence truth layer."
            ),
        ),
        row!(
            format!("custody-row-license-download-update-metadata"),
            DataCustodyClassId::LicenseDownloadUpdateMetadata,
            format!("License, download, and update metadata"),
            format!("Release process"),
            source_self(),
            DataCustodyAuthority::PublicReleasePipeline,
            DataCustodyDefaultLocation::OcentraLicenseUpdateStore,
            true,
            false,
            false,
            true,
            true,
            DataCustodyExposure::Public,
            DataCustodyExposure::Minimal,
            false,
            false,
            false,
            policy(
                DataCustodyOcentraHostingMode::AllowedMetadataOnly,
                false,
                false,
                false,
            ),
            format!("Public product and installer metadata only."),
        ),
    ]
}

fn data_custody_rows_03() -> Vec<DataCustodySourceOfTruthRow> {
    vec![
        row!(
            format!("custody-row-household-device-registry"),
            DataCustodyClassId::HouseholdDeviceRegistry,
            format!("Household device registry"),
            format!("Household control plane"),
            source_self(),
            DataCustodyAuthority::HouseholdControlPlane,
            DataCustodyDefaultLocation::HouseholdDeviceRegistry,
            false,
            false,
            true,
            true,
            false,
            DataCustodyExposure::AllowedReferencesOnly,
            DataCustodyExposure::None,
            false,
            false,
            true,
            policy(DataCustodyOcentraHostingMode::Forbidden, true, false, false),
            format!("Household-owned device list and role binding state."),
        ),
        row!(
            format!("custody-row-device-registration-pairing-route-metadata"),
            DataCustodyClassId::DeviceRegistrationPairingRouteMetadata,
            format!("Device registration and pairing route metadata"),
            format!("Household control plane"),
            source_self(),
            DataCustodyAuthority::OcentraRoutingService,
            DataCustodyDefaultLocation::OcentraHouseholdRouteStore,
            true,
            false,
            true,
            true,
            true,
            DataCustodyExposure::Minimal,
            DataCustodyExposure::Minimal,
            false,
            false,
            true,
            policy(
                DataCustodyOcentraHostingMode::AllowedMetadataOnly,
                false,
                false,
                false,
            ),
            format!("Route and pairing metadata only; no child activity payloads."),
        ),
    ]
}

fn data_custody_rows_04() -> Vec<DataCustodySourceOfTruthRow> {
    vec![
        row!(
            format!("custody-row-setup-state-and-pairing-draft"),
            DataCustodyClassId::SetupStateAndPairingDraft,
            format!("Setup state and pairing draft"),
            format!("Household setup flow"),
            source_self(),
            DataCustodyAuthority::HouseholdControlPlane,
            DataCustodyDefaultLocation::HouseholdSetupDraftStore,
            false,
            false,
            true,
            true,
            false,
            DataCustodyExposure::AllowedReferencesOnly,
            DataCustodyExposure::None,
            false,
            false,
            true,
            policy(DataCustodyOcentraHostingMode::Forbidden, true, false, false),
            format!(
                "Setup drafts stay household-owned until another workpack proves remote storage."
            ),
        ),
        row!(
            format!("custody-row-minimal-notification-routing-metadata"),
            DataCustodyClassId::MinimalNotificationRoutingMetadata,
            format!("Minimal notification routing metadata"),
            format!("Notification service"),
            source_self(),
            DataCustodyAuthority::OcentraRoutingService,
            DataCustodyDefaultLocation::OcentraNotificationRouteStore,
            true,
            false,
            true,
            true,
            true,
            DataCustodyExposure::Minimal,
            DataCustodyExposure::Minimal,
            false,
            false,
            true,
            policy(
                DataCustodyOcentraHostingMode::AllowedMetadataOnly,
                false,
                true,
                false,
            ),
            format!("Payload detail must stay redacted and drill-in must remain authenticated."),
        ),
    ]
}

fn data_custody_rows_05() -> Vec<DataCustodySourceOfTruthRow> {
    vec![
        row!(
            format!("custody-row-short-lived-report-compiler-status"),
            DataCustodyClassId::ShortLivedReportCompilerStatus,
            format!("Short-lived report compiler status"),
            format!("Report runtime / control plane"),
            source_self(),
            DataCustodyAuthority::OcentraReportStatusRuntime,
            DataCustodyDefaultLocation::OcentraShortLivedReportStatusStore,
            true,
            false,
            false,
            true,
            true,
            DataCustodyExposure::Minimal,
            DataCustodyExposure::Minimal,
            false,
            false,
            true,
            policy(
                DataCustodyOcentraHostingMode::ShortLivedStatusOnly,
                false,
                false,
                false,
            ),
            format!("Status only, not report content or source child evidence."),
        ),
        row!(
            format!("custody-row-support-case-metadata"),
            DataCustodyClassId::SupportCaseMetadata,
            format!("Support case metadata"),
            format!("Support system"),
            source_self(),
            DataCustodyAuthority::SupportSystem,
            DataCustodyDefaultLocation::OcentraSupportCaseStore,
            true,
            false,
            true,
            true,
            true,
            DataCustodyExposure::RedactedMetadataOnly,
            DataCustodyExposure::RedactedMetadataOnly,
            false,
            false,
            true,
            policy(
                DataCustodyOcentraHostingMode::AllowedMetadataOnly,
                false,
                false,
                true,
            ),
            format!("Support-safe metadata only; no raw child activity by default."),
        ),
    ]
}

fn data_custody_rows_06() -> Vec<DataCustodySourceOfTruthRow> {
    vec![
        row!(
            format!("custody-row-public-website-release-status"),
            DataCustodyClassId::PublicWebsiteReleaseStatus,
            format!("Public website and release status"),
            format!("Public site / release process"),
            source_self(),
            DataCustodyAuthority::PublicReleasePipeline,
            DataCustodyDefaultLocation::PublicReleaseSurface,
            true,
            false,
            false,
            true,
            true,
            DataCustodyExposure::Public,
            DataCustodyExposure::Public,
            false,
            false,
            false,
            policy(
                DataCustodyOcentraHostingMode::PublicReleaseOnly,
                false,
                false,
                false,
            ),
            format!("Public product metadata only."),
        ),
        row!(
            format!("custody-row-child-profile"),
            DataCustodyClassId::ChildProfile,
            format!("Child profile"),
            format!("Child device / household model"),
            source_self(),
            DataCustodyAuthority::HouseholdControlPlane,
            DataCustodyDefaultLocation::HouseholdProfileStore,
            false,
            false,
            true,
            true,
            true,
            DataCustodyExposure::AllowedReferencesOnly,
            DataCustodyExposure::AllowedReferencesOnly,
            false,
            false,
            true,
            policy(DataCustodyOcentraHostingMode::Forbidden, true, false, false),
            format!("Household-owned, role-bound profile state."),
        ),
    ]
}

fn data_custody_rows_07() -> Vec<DataCustodySourceOfTruthRow> {
    vec![
        row!(
            format!("custody-row-parent-rules-and-approval-history"),
            DataCustodyClassId::ParentRulesAndApprovalHistory,
            format!("Parent rules and approval history"),
            format!("Household control plane"),
            source_self(),
            DataCustodyAuthority::HouseholdControlPlane,
            DataCustodyDefaultLocation::HouseholdRuleStore,
            false,
            true,
            true,
            true,
            true,
            DataCustodyExposure::AllowedReferencesOnly,
            DataCustodyExposure::AllowedReferencesOnly,
            false,
            false,
            true,
            policy(DataCustodyOcentraHostingMode::Forbidden, true, false, false),
            format!("Parent rule and approval state must not become an Ocentra-owned truth layer."),
        ),
        row!(
            format!("custody-row-audit-log"),
            DataCustodyClassId::AuditLog,
            format!("Audit log"),
            format!("Household control plane"),
            source_self(),
            DataCustodyAuthority::HouseholdControlPlane,
            DataCustodyDefaultLocation::HouseholdAuditStore,
            false,
            false,
            true,
            true,
            false,
            DataCustodyExposure::AllowedReferencesOnly,
            DataCustodyExposure::None,
            false,
            false,
            true,
            policy(DataCustodyOcentraHostingMode::Forbidden, true, false, false),
            format!(
                "Audit history may prove action lineage but should not carry raw child payloads."
            ),
        ),
    ]
}

fn data_custody_rows_08() -> Vec<DataCustodySourceOfTruthRow> {
    vec![
        row!(
            format!("custody-row-evidence-journal-segments"),
            DataCustodyClassId::EvidenceJournalSegments,
            format!("Evidence journal segments"),
            format!("Child device local journal"),
            source_self(),
            DataCustodyAuthority::ChildDevice,
            DataCustodyDefaultLocation::ChildDeviceEncryptedJournal,
            false,
            true,
            true,
            true,
            false,
            DataCustodyExposure::AllowedReferencesOnly,
            DataCustodyExposure::None,
            true,
            false,
            true,
            policy(DataCustodyOcentraHostingMode::Forbidden, true, false, false),
            format!("Encrypted append-only journal is the evidence source of truth."),
        ),
        row!(
            format!("custody-row-sqlite-evidence-read-model-database"),
            DataCustodyClassId::SqliteEvidenceReadModelDatabase,
            format!("SQLite evidence/read-model database"),
            format!("Child device local cache"),
            source_derived(vec![DataCustodyClassId::EvidenceJournalSegments]),
            DataCustodyAuthority::ChildDevice,
            DataCustodyDefaultLocation::ChildDeviceLocalQueryStore,
            false,
            true,
            true,
            false,
            false,
            DataCustodyExposure::None,
            DataCustodyExposure::None,
            false,
            true,
            true,
            policy(DataCustodyOcentraHostingMode::Forbidden, true, false, false),
            format!("SQLite is rebuildable local cache, not the truth layer."),
        ),
    ]
}

fn data_custody_rows_09() -> Vec<DataCustodySourceOfTruthRow> {
    vec![
        row!(
            format!("custody-row-screenshots-and-screen-analysis-images"),
            DataCustodyClassId::ScreenshotsAndScreenAnalysisImages,
            format!("Screenshots and screen-analysis images"),
            format!("Child device local evidence"),
            source_self(),
            DataCustodyAuthority::ChildDevice,
            DataCustodyDefaultLocation::ChildDeviceSensitiveEvidenceStore,
            false,
            true,
            true,
            false,
            false,
            DataCustodyExposure::None,
            DataCustodyExposure::None,
            true,
            false,
            true,
            policy(DataCustodyOcentraHostingMode::Forbidden, true, false, false),
            format!("Sensitive visual evidence stays local unless explicitly exported."),
        ),
        row!(
            format!("custody-row-browser-url-history"),
            DataCustodyClassId::BrowserUrlHistory,
            format!("Browser URL history"),
            format!("Child device local evidence"),
            source_self(),
            DataCustodyAuthority::ChildDevice,
            DataCustodyDefaultLocation::ChildDeviceSensitiveEvidenceStore,
            false,
            true,
            true,
            false,
            false,
            DataCustodyExposure::None,
            DataCustodyExposure::None,
            true,
            false,
            true,
            policy(DataCustodyOcentraHostingMode::Forbidden, true, false, false),
            format!("URL history never becomes generic telemetry."),
        ),
    ]
}

fn data_custody_rows_10() -> Vec<DataCustodySourceOfTruthRow> {
    vec![
        row!(
            format!("custody-row-network-app-game-evidence"),
            DataCustodyClassId::NetworkAppGameEvidence,
            format!("Network, app, and game evidence"),
            format!("Child device local evidence"),
            source_self(),
            DataCustodyAuthority::ChildDevice,
            DataCustodyDefaultLocation::ChildDeviceSensitiveEvidenceStore,
            false,
            true,
            true,
            false,
            false,
            DataCustodyExposure::None,
            DataCustodyExposure::None,
            true,
            false,
            true,
            policy(DataCustodyOcentraHostingMode::Forbidden, true, false, false),
            format!(
                "Network, app, and game evidence stays under the same local-first custody rule."
            ),
        ),
        row!(
            format!("custody-row-location-tracking-evidence"),
            DataCustodyClassId::LocationTrackingEvidence,
            format!("Location and tracking evidence"),
            format!("Child device local evidence"),
            source_self(),
            DataCustodyAuthority::ChildDevice,
            DataCustodyDefaultLocation::ChildDeviceLocationStore,
            false,
            true,
            true,
            false,
            false,
            DataCustodyExposure::None,
            DataCustodyExposure::None,
            true,
            false,
            true,
            policy(DataCustodyOcentraHostingMode::Forbidden, true, false, false),
            format!(
                "Tracking evidence remains child-device local unless a later plan proves otherwise."
            ),
        ),
    ]
}

fn data_custody_rows_11() -> Vec<DataCustodySourceOfTruthRow> {
    vec![
        row!(
            format!("custody-row-local-ai-and-policy-decisions"),
            DataCustodyClassId::LocalAiAndPolicyDecisions,
            format!("Local AI and policy decisions"),
            format!("Child device local runtime"),
            source_derived(vec![
                DataCustodyClassId::SqliteEvidenceReadModelDatabase,
                DataCustodyClassId::ParentRulesAndApprovalHistory,
            ]),
            DataCustodyAuthority::ChildDevice,
            DataCustodyDefaultLocation::ChildDeviceLocalAiStore,
            false,
            false,
            true,
            true,
            true,
            DataCustodyExposure::DerivedOutputOnly,
            DataCustodyExposure::Minimal,
            false,
            true,
            true,
            policy(DataCustodyOcentraHostingMode::Forbidden, true, false, false),
            format!(
                "Derived decisions may inform reports or alerts without replacing cited evidence."
            ),
        ),
        row!(
            format!("custody-row-generated-long-term-reports"),
            DataCustodyClassId::GeneratedLongTermReports,
            format!("Generated long-term reports"),
            format!("Parent-owned output"),
            source_derived(vec![
                DataCustodyClassId::SqliteEvidenceReadModelDatabase,
                DataCustodyClassId::LocalAiAndPolicyDecisions,
                DataCustodyClassId::ChildProfile,
            ]),
            DataCustodyAuthority::ParentDevice,
            DataCustodyDefaultLocation::ParentDeviceReportCache,
            false,
            true,
            true,
            true,
            true,
            DataCustodyExposure::DerivedOutputOnly,
            DataCustodyExposure::Minimal,
            false,
            true,
            true,
            policy(DataCustodyOcentraHostingMode::Forbidden, true, false, false),
            format!("Reports are derived output, not an Ocentra-owned truth layer."),
        ),
    ]
}

fn data_custody_rows_12() -> Vec<DataCustodySourceOfTruthRow> {
    vec![
        row!(
            format!("custody-row-parent-notification-history-cache"),
            DataCustodyClassId::ParentNotificationHistoryCache,
            format!("Parent notification history cache"),
            format!("Parent device notification history/cache"),
            source_derived(vec![DataCustodyClassId::MinimalNotificationRoutingMetadata]),
            DataCustodyAuthority::ParentDevice,
            DataCustodyDefaultLocation::ParentDeviceNotificationHistoryCache,
            false,
            false,
            true,
            true,
            true,
            DataCustodyExposure::RedactedMetadataOnly,
            DataCustodyExposure::Minimal,
            false,
            true,
            true,
            policy(DataCustodyOcentraHostingMode::Forbidden, true, false, false),
            format!(
                "Parent device caches notification status without making cloud routing the family-data store."
            ),
        ),
        row!(
            format!("custody-row-assistant-child-evidence-context"),
            DataCustodyClassId::AssistantChildEvidenceContext,
            format!("Assistant child-evidence context"),
            format!("Parent assistant runtime"),
            source_derived(vec![
                DataCustodyClassId::GeneratedLongTermReports,
                DataCustodyClassId::AuditLog,
            ]),
            DataCustodyAuthority::ParentDevice,
            DataCustodyDefaultLocation::ParentAssistantEphemeralContext,
            false,
            true,
            true,
            false,
            false,
            DataCustodyExposure::None,
            DataCustodyExposure::None,
            false,
            true,
            true,
            policy(DataCustodyOcentraHostingMode::Forbidden, true, false, false),
            format!("Assistant context stays citation-only and excludes raw child content."),
        ),
    ]
}

fn data_custody_rows_13() -> Vec<DataCustodySourceOfTruthRow> {
    vec![
        row!(
            format!("custody-row-parent-owned-storage-contents"),
            DataCustodyClassId::ParentOwnedStorageContents,
            format!("Parent-owned storage contents"),
            format!("Parent-selected provider"),
            source_derived(vec![
                DataCustodyClassId::ChildProfile,
                DataCustodyClassId::ParentRulesAndApprovalHistory,
                DataCustodyClassId::EvidenceJournalSegments,
                DataCustodyClassId::GeneratedLongTermReports,
                DataCustodyClassId::LocalAiAndPolicyDecisions,
            ]),
            DataCustodyAuthority::ParentOwnedStorage,
            DataCustodyDefaultLocation::ParentOwnedEncryptedStorage,
            false,
            true,
            true,
            true,
            false,
            DataCustodyExposure::RedactedMetadataOnly,
            DataCustodyExposure::None,
            false,
            true,
            true,
            policy(DataCustodyOcentraHostingMode::Forbidden, true, false, false),
            format!("Parent-owned storage is an encrypted destination, not an Ocentra default."),
        ),
        row!(
            format!("custody-row-provider-sync-payloads"),
            DataCustodyClassId::ProviderSyncPayloads,
            format!("Provider sync payloads"),
            format!("Provider bundle"),
            source_derived(vec![DataCustodyClassId::ParentOwnedStorageContents]),
            DataCustodyAuthority::ParentOwnedStorage,
            DataCustodyDefaultLocation::ProviderEnvelopeMetadata,
            false,
            true,
            true,
            false,
            false,
            DataCustodyExposure::None,
            DataCustodyExposure::None,
            false,
            true,
            true,
            policy(DataCustodyOcentraHostingMode::Forbidden, false, true, false),
            format!(
                "Provider-visible envelope metadata is limited to unavoidable connector behavior."
            ),
        ),
    ]
}

fn data_custody_rows_14() -> Vec<DataCustodySourceOfTruthRow> {
    vec![
        row!(
            format!("custody-row-support-bundles-containing-raw-child-activity"),
            DataCustodyClassId::SupportBundlesContainingRawChildActivity,
            format!("Support bundles containing raw child activity"),
            format!("Support flow"),
            source_derived(vec![
                DataCustodyClassId::ScreenshotsAndScreenAnalysisImages,
                DataCustodyClassId::BrowserUrlHistory,
                DataCustodyClassId::NetworkAppGameEvidence,
                DataCustodyClassId::LocationTrackingEvidence,
                DataCustodyClassId::GeneratedLongTermReports,
            ]),
            DataCustodyAuthority::SupportExportBoundary,
            DataCustodyDefaultLocation::SupportExportArtifact,
            false,
            true,
            true,
            false,
            false,
            DataCustodyExposure::None,
            DataCustodyExposure::None,
            true,
            true,
            true,
            policy(DataCustodyOcentraHostingMode::Forbidden, true, false, true),
            format!(
                "Support raw-activity export requires explicit parent initiation and redaction review."
            ),
        ),
        row!(
            format!("custody-row-universal-decrypt-keys"),
            DataCustodyClassId::UniversalDecryptKeys,
            format!("Universal decrypt keys"),
            format!("Household key model"),
            source_self(),
            DataCustodyAuthority::HouseholdControlPlane,
            DataCustodyDefaultLocation::HouseholdKeyStore,
            false,
            true,
            false,
            false,
            false,
            DataCustodyExposure::None,
            DataCustodyExposure::None,
            false,
            false,
            true,
            policy(
                DataCustodyOcentraHostingMode::Forbidden,
                false,
                false,
                false
            ),
            format!(
                "Universal decrypt keys never host by default and remain outside this workpack's runtime claims."
            ),
        ),
    ]
}
