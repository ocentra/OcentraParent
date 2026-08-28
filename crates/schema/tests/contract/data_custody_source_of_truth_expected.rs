use ocentra_schema::data_custody_source_of_truth as contracts;

pub(super) fn expected_derived_source_classes(
    class_id: contracts::DataCustodyClassId,
) -> &'static [contracts::DataCustodyClassId] {
    use contracts::DataCustodyClassId::*;

    match class_id {
        SqliteEvidenceReadModelDatabase => &[EvidenceJournalSegments],
        LocalAiAndPolicyDecisions => &[
            SqliteEvidenceReadModelDatabase,
            ParentRulesAndApprovalHistory,
        ],
        GeneratedLongTermReports => &[
            SqliteEvidenceReadModelDatabase,
            LocalAiAndPolicyDecisions,
            ChildProfile,
        ],
        ParentNotificationHistoryCache => &[MinimalNotificationRoutingMetadata],
        AssistantChildEvidenceContext => &[GeneratedLongTermReports, AuditLog],
        ParentOwnedStorageContents => &[
            ChildProfile,
            ParentRulesAndApprovalHistory,
            EvidenceJournalSegments,
            GeneratedLongTermReports,
            LocalAiAndPolicyDecisions,
        ],
        ProviderSyncPayloads => &[ParentOwnedStorageContents],
        SupportBundlesContainingRawChildActivity => &[
            ScreenshotsAndScreenAnalysisImages,
            BrowserUrlHistory,
            NetworkAppGameEvidence,
            LocationTrackingEvidence,
            GeneratedLongTermReports,
        ],
        _ => &[],
    }
}
