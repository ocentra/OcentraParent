use super::*;

pub(super) fn source_self() -> DataCustodySourceOfTruth {
    DataCustodySourceOfTruth {
        kind: DataCustodySourceOfTruthKind::Self_,
        source_class_ids: Vec::new(),
    }
}

pub(super) fn source_derived(
    source_class_ids: Vec<DataCustodyClassId>,
) -> DataCustodySourceOfTruth {
    DataCustodySourceOfTruth {
        kind: DataCustodySourceOfTruthKind::DerivedFromDataClasses,
        source_class_ids,
    }
}

pub(super) fn policy(
    ocentra_hosting_mode: DataCustodyOcentraHostingMode,
    parent_owned_storage_allowed: bool,
    provider_metadata_allowed: bool,
    support_export_parent_initiated_only: bool,
) -> DataCustodyHostingPolicy {
    DataCustodyHostingPolicy {
        ocentra_hosting_mode,
        parent_owned_storage_allowed,
        provider_metadata_allowed,
        support_export_parent_initiated_only,
    }
}

pub(super) struct DataCustodyRowInput {
    pub(super) row_id: String,
    pub(super) class_id: DataCustodyClassId,
    pub(super) class_label: String,
    pub(super) source_owner: String,
    pub(super) source_of_truth: DataCustodySourceOfTruth,
    pub(super) custody_authority: DataCustodyAuthority,
    pub(super) default_location: DataCustodyDefaultLocation,
    pub(super) ocentra_hosted_by_default: bool,
    pub(super) must_never_be_hosted_by_default: bool,
    pub(super) encrypted_before_upload: bool,
    pub(super) may_appear_in_reports: bool,
    pub(super) may_appear_in_notifications: bool,
    pub(super) report_exposure: DataCustodyExposure,
    pub(super) notification_exposure: DataCustodyExposure,
    pub(super) raw_child_evidence_allowed: bool,
    pub(super) derived_use_only: bool,
    pub(super) sensitive: bool,
    pub(super) hosting_policy: DataCustodyHostingPolicy,
    pub(super) notes: String,
}

pub(super) fn data_custody_row(input: DataCustodyRowInput) -> DataCustodySourceOfTruthRow {
    let DataCustodyRowInput {
        row_id,
        class_id,
        class_label,
        source_owner,
        source_of_truth,
        custody_authority,
        default_location,
        ocentra_hosted_by_default,
        must_never_be_hosted_by_default,
        encrypted_before_upload,
        may_appear_in_reports,
        may_appear_in_notifications,
        report_exposure,
        notification_exposure,
        raw_child_evidence_allowed,
        derived_use_only,
        sensitive,
        hosting_policy,
        notes,
    } = input;

    DataCustodySourceOfTruthRow {
        row_id: data_custody_source_of_truth_row_id(row_id),
        class_id,
        class_label,
        source_owner,
        source_of_truth,
        custody_authority,
        default_location,
        ocentra_hosted_by_default,
        must_never_be_hosted_by_default,
        encrypted_before_upload,
        may_appear_in_reports,
        may_appear_in_notifications,
        report_exposure,
        notification_exposure,
        raw_child_evidence_allowed,
        derived_use_only,
        sensitive,
        hosting_policy,
        notes,
    }
}

pub(super) fn data_custody_source_of_truth_row_id(
    value: impl Into<String>,
) -> DataCustodySourceOfTruthRowId {
    crate::schema_option_or_unreachable(
        DataCustodySourceOfTruthRowId::parse(value),
        DATA_CUSTODY_ROW_ID_EXPECTATION,
    )
}

pub(super) fn data_custody_source_of_truth_matrix_id(
    value: impl Into<String>,
) -> DataCustodySourceOfTruthMatrixId {
    crate::schema_option_or_unreachable(
        DataCustodySourceOfTruthMatrixId::parse(value),
        DATA_CUSTODY_MATRIX_ID_EXPECTATION,
    )
}

pub(super) fn allowed_ocentra_hosted_metadata() -> Vec<DataCustodyClassId> {
    vec![
        DataCustodyClassId::AccountIdentityMetadata,
        DataCustodyClassId::SubscriptionEntitlementMetadata,
        DataCustodyClassId::LicenseDownloadUpdateMetadata,
        DataCustodyClassId::DeviceRegistrationPairingRouteMetadata,
        DataCustodyClassId::MinimalNotificationRoutingMetadata,
        DataCustodyClassId::ShortLivedReportCompilerStatus,
        DataCustodyClassId::SupportCaseMetadata,
        DataCustodyClassId::PublicWebsiteReleaseStatus,
    ]
}

pub(super) fn must_never_be_hosted_by_default() -> Vec<DataCustodyClassId> {
    vec![
        DataCustodyClassId::ParentRulesAndApprovalHistory,
        DataCustodyClassId::EvidenceJournalSegments,
        DataCustodyClassId::SqliteEvidenceReadModelDatabase,
        DataCustodyClassId::ScreenshotsAndScreenAnalysisImages,
        DataCustodyClassId::BrowserUrlHistory,
        DataCustodyClassId::NetworkAppGameEvidence,
        DataCustodyClassId::LocationTrackingEvidence,
        DataCustodyClassId::GeneratedLongTermReports,
        DataCustodyClassId::AssistantChildEvidenceContext,
        DataCustodyClassId::ParentOwnedStorageContents,
        DataCustodyClassId::ProviderSyncPayloads,
        DataCustodyClassId::SupportBundlesContainingRawChildActivity,
        DataCustodyClassId::UniversalDecryptKeys,
    ]
}
