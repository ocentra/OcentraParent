use std::collections::HashSet;

use crate::support::{extract_typescript_block, ValueOrUnreachable as _};
use ocentra_schema::data_custody_source_of_truth as contracts;
use ocentra_schema::data_custody_source_of_truth_ts::data_custody_source_of_truth_contracts_typescript;
use serde_json::json;

#[test]
fn data_custody_source_of_truth_round_trips_through_rust_owned_shape() {
    let proof = contracts::sample_data_custody_source_of_truth_contract_proof();
    let encoded = serde_json::to_value(&proof)
        .value_or_unreachable(crate::assert_context!("proof serializes"));

    assert_eq!(
        encoded["schemaVersion"],
        json!(contracts::DATA_CUSTODY_SOURCE_OF_TRUTH_SCHEMA_VERSION)
    );
    assert_eq!(
        encoded["rows"][0]["classId"],
        json!("account-identity-metadata")
    );
    assert_eq!(
        encoded["rows"][15]["sourceOfTruth"]["kind"],
        json!("derived-from-data-classes")
    );
    assert_eq!(
        encoded["rows"][27]["classId"],
        json!("universal-decrypt-keys")
    );
    assert!(encoded.get("schema_version").is_none());

    let decoded: contracts::DataCustodySourceOfTruthContractProof = serde_json::from_value(encoded)
        .value_or_unreachable(crate::assert_context!("proof deserializes"));
    assert_eq!(decoded, proof);
}

#[test]
fn data_custody_source_of_truth_has_the_exact_active_class_inventory() {
    use contracts::DataCustodyClassId::*;

    let proof = contracts::sample_data_custody_source_of_truth_contract_proof();
    let expected_classes = vec![
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
    ];
    let actual_classes: Vec<_> = proof.rows.iter().map(|row| row.class_id).collect();
    let unique_row_ids: HashSet<_> = proof
        .rows
        .iter()
        .map(|row| row.row_id.as_str())
        .collect();
    let unique_class_ids: HashSet<_> = proof.rows.iter().map(class_id_key).collect();

    assert_eq!(proof.rows.len(), 28);
    assert_eq!(actual_classes, expected_classes);
    assert_eq!(unique_row_ids.len(), proof.rows.len());
    assert_eq!(unique_class_ids.len(), proof.rows.len());
}

#[test]
fn derived_rows_reference_only_known_source_classes_and_never_become_truth() {
    let proof = contracts::sample_data_custody_source_of_truth_contract_proof();
    let known_classes: HashSet<_> = proof.rows.iter().map(class_id_key).collect();

    for row in &proof.rows {
        match row.source_of_truth.kind {
            contracts::DataCustodySourceOfTruthKind::Self_ => {
                assert!(row.source_of_truth.source_class_ids.is_empty());
                assert!(!row.derived_use_only);
            }
            contracts::DataCustodySourceOfTruthKind::DerivedFromDataClasses => {
                assert!(!row.source_of_truth.source_class_ids.is_empty());
                assert!(row.derived_use_only);

                for source_class in &row.source_of_truth.source_class_ids {
                    assert!(known_classes.contains(&class_id_key(source_class)));
                    assert_ne!(*source_class, row.class_id);
                }
            }
        }
    }
}

#[test]
fn hosting_modes_and_default_hosting_lists_match_the_declared_matrix() {
    let proof = contracts::sample_data_custody_source_of_truth_contract_proof();
    let hosted_rows: Vec<_> = proof
        .rows
        .iter()
        .filter(|row| row.ocentra_hosted_by_default)
        .map(|row| row.class_id)
        .collect();
    let never_hosted_rows: Vec<_> = proof
        .rows
        .iter()
        .filter(|row| row.must_never_be_hosted_by_default)
        .map(|row| row.class_id)
        .collect();
    let forbidden = proof
        .rows
        .iter()
        .filter(|row| {
            row.hosting_policy.ocentra_hosting_mode
                == contracts::DataCustodyOcentraHostingMode::Forbidden
        })
        .count();
    let allowed_metadata_only = proof
        .rows
        .iter()
        .filter(|row| {
            row.hosting_policy.ocentra_hosting_mode
                == contracts::DataCustodyOcentraHostingMode::AllowedMetadataOnly
        })
        .count();
    let short_lived_status_only = proof
        .rows
        .iter()
        .filter(|row| {
            row.hosting_policy.ocentra_hosting_mode
                == contracts::DataCustodyOcentraHostingMode::ShortLivedStatusOnly
        })
        .count();
    let public_release_only = proof
        .rows
        .iter()
        .filter(|row| {
            row.hosting_policy.ocentra_hosting_mode
                == contracts::DataCustodyOcentraHostingMode::PublicReleaseOnly
        })
        .count();

    assert_eq!(proof.allowed_ocentra_hosted_metadata.len(), 8);
    assert_eq!(proof.must_never_be_hosted_by_default.len(), 13);
    assert_eq!(hosted_rows, proof.allowed_ocentra_hosted_metadata);
    assert_eq!(never_hosted_rows, proof.must_never_be_hosted_by_default);
    assert_eq!(
        (
            forbidden,
            allowed_metadata_only,
            short_lived_status_only,
            public_release_only,
        ),
        (20, 6, 1, 1)
    );

    for row in &proof.rows {
        assert!(!(row.ocentra_hosted_by_default && row.must_never_be_hosted_by_default));
        if row.must_never_be_hosted_by_default {
            assert_eq!(
                row.hosting_policy.ocentra_hosting_mode,
                contracts::DataCustodyOcentraHostingMode::Forbidden
            );
        }
    }
}

#[test]
fn sensitive_raw_child_evidence_never_becomes_notification_or_default_hosting_data() {
    let proof = contracts::sample_data_custody_source_of_truth_contract_proof();
    let raw_child_rows: Vec<_> = proof
        .rows
        .iter()
        .filter(|row| row.raw_child_evidence_allowed)
        .collect();

    assert!(!raw_child_rows.is_empty());
    for row in raw_child_rows {
        assert!(row.sensitive);
        assert!(row.encrypted_before_upload);
        assert!(!row.ocentra_hosted_by_default);
        assert!(!row.may_appear_in_notifications);
        assert_eq!(
            row.notification_exposure,
            contracts::DataCustodyExposure::None
        );
        assert!(matches!(
            row.report_exposure,
            contracts::DataCustodyExposure::None
                | contracts::DataCustodyExposure::AllowedReferencesOnly
        ));
    }

    for row in &proof.rows {
        if !row.may_appear_in_notifications {
            assert_eq!(
                row.notification_exposure,
                contracts::DataCustodyExposure::None
            );
        }
    }
}

#[test]
fn custody_contract_keeps_every_no_claim_flag_fail_closed() {
    use contracts::DataCustodyNonClaim::*;

    let proof = contracts::sample_data_custody_source_of_truth_contract_proof();

    assert_eq!(
        proof.non_claims,
        vec![
            NoDefaultOcentraChildActivityStore,
            NoSqliteTruthLayer,
            NoProviderAutoApply,
            NoSupportDecryptDefault,
            NoOcentraOwnedParentRules,
            NoRawChildEvidenceInNotifications,
            NoLongLivedHostedReports,
        ]
    );
    assert_eq!(
        (
            proof.ocentra_is_default_child_data_store,
            proof.provider_auto_apply_claimed,
            proof.support_decrypt_by_default_claimed,
            proof.sqlite_as_truth_layer_claimed,
            proof.raw_child_activity_hosted_by_default_claimed,
        ),
        (false, false, false, false, false)
    );
    assert!(proof.account_control_plane_separated);
    assert!(proof.provider_owned_billing_identity_separated);
}

#[test]
fn custody_identifiers_reject_blank_values_without_normalizing_identity() {
    assert!(contracts::DataCustodySourceOfTruthRowId::parse("   ").is_none());
    assert!(contracts::DataCustodySourceOfTruthMatrixId::parse("\t").is_none());

    let row_id = contracts::DataCustodySourceOfTruthRowId::parse("custody-row-exact")
        .value_or_unreachable(crate::assert_context!("nonblank row id parses"));
    let matrix_id = contracts::DataCustodySourceOfTruthMatrixId::parse("custody-matrix-exact")
        .value_or_unreachable(crate::assert_context!("nonblank matrix id parses"));

    assert_eq!(row_id.as_str(), "custody-row-exact");
    assert_eq!(matrix_id.as_str(), "custody-matrix-exact");
}

fn class_id_key(class_id: &contracts::DataCustodyClassId) -> String {
    serde_json::to_string(class_id)
        .value_or_unreachable(crate::assert_context!("class id serializes"))
}

#[test]
fn generated_data_custody_source_of_truth_contracts_stay_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated-data-custody-source-of-truth-contracts.ts"
    );
    let generated = data_custody_source_of_truth_contracts_typescript();

    assert_eq!(
        extract_typescript_block(
            crate::contract_text!(checked_in),
            crate::text_boundary!(
                "export const GeneratedDataCustodyKnownGaps = ",
                " as const;"
            )
        ),
        extract_typescript_block(
            crate::contract_text!(&generated),
            crate::text_boundary!(
                "export const GeneratedDataCustodyKnownGaps = ",
                " as const;"
            )
        )
    );
    assert_eq!(
        extract_typescript_block(
            crate::contract_text!(checked_in),
            crate::text_boundary!(
                "export const GeneratedDataCustodySourceOfTruthContractProof =",
                " as const satisfies GeneratedDataCustodySourceOfTruthContractProof;"
            )
        ),
        extract_typescript_block(
            crate::contract_text!(&generated),
            crate::text_boundary!(
                "export const GeneratedDataCustodySourceOfTruthContractProof =",
                " as const satisfies GeneratedDataCustodySourceOfTruthContractProof;"
            )
        )
    );
    assert_eq!(
        extract_typescript_block(
            crate::contract_text!(checked_in),
            crate::text_boundary!(
                "export const GeneratedDataCustodyClassIds = [",
                "] as const satisfies readonly GeneratedDataCustodyClassId[];"
            )
        ),
        extract_typescript_block(
            crate::contract_text!(&generated),
            crate::text_boundary!(
                "export const GeneratedDataCustodyClassIds = [",
                "] as const satisfies readonly GeneratedDataCustodyClassId[];"
            )
        )
    );
    assert_generated_data_custody_source_of_truth_contracts(
        crate::contract_text!(checked_in),
        crate::contract_text!(&generated),
    );
}

fn assert_generated_data_custody_source_of_truth_contracts(
    checked_in: crate::support::ContractText<'_>,
    generated: crate::support::ContractText<'_>,
) {
    let checked_in_lines: Vec<&str> = checked_in.0.lines().collect();
    let generated_lines: Vec<&str> = generated.0.lines().collect();

    assert_eq!(
        checked_in_lines.first().copied(),
        Some("/* generated from crates/schema/src/data_custody_source_of_truth.rs */")
    );
    assert_eq!(
        generated_lines
            .iter()
            .filter(|line| **line == "export interface GeneratedDataCustodySourceOfTruthRow {")
            .count(),
        1
    );
    assert_eq!(
        generated_lines
            .iter()
            .filter(|line| **line
                == "export interface GeneratedDataCustodySourceOfTruthContractProof {")
            .count(),
        1
    );
    assert_eq!(
        generated_lines
            .iter()
            .filter(|line| **line == "export const GeneratedDataCustodyKnownGaps = [")
            .count(),
        1
    );
    let schema_version_line = format!(
        "  SchemaVersion: '{}',",
        contracts::DATA_CUSTODY_SOURCE_OF_TRUTH_SCHEMA_VERSION
    );
    assert_eq!(
        generated_lines
            .iter()
            .find(|line| **line == schema_version_line.as_str()),
        Some(&schema_version_line.as_str())
    );
    assert_eq!(
        generated_lines
            .iter()
            .filter(|line| **line == "export const GeneratedDataCustodyClassIds = [")
            .count(),
        1
    );
}
