use crate::support::{assert_exports_include, property_string_value, ValueOrUnreachable as _};

use ocentra_schema::app_install_purchase_approval as contracts;
use ocentra_schema::app_install_purchase_approval_ts::{
    app_install_purchase_approval_contract_rules_typescript,
    app_install_purchase_approval_contracts_typescript,
    app_install_purchase_delivery_runtime_helpers_typescript,
    app_install_purchase_external_runtime_helpers_typescript,
    app_install_purchase_platform_evidence_helpers_typescript,
    app_install_purchase_platform_provider_helpers_typescript,
    app_install_purchase_proof_helpers_typescript,
    app_install_purchase_report_status_helpers_typescript,
    app_install_purchase_runtime_proof_rules_typescript,
};
use serde_json::json;

fn timestamp(value: &str) -> contracts::ParentTimestamp {
    contracts::ParentTimestamp::parse(value).value_or_unreachable("timestamp")
}

fn claim_boundary(value: &str) -> contracts::AppInstallPurchaseApprovalClaimBoundary {
    contracts::AppInstallPurchaseApprovalClaimBoundary::parse(value)
        .value_or_unreachable("claim boundary")
}

fn proof_requirement(value: &str) -> contracts::AppInstallPurchaseApprovalProofRequirement {
    contracts::AppInstallPurchaseApprovalProofRequirement::parse(value)
        .value_or_unreachable("proof requirement")
}

fn evidence_ref(id: &str) -> contracts::ParentEvidenceReference {
    contracts::ParentEvidenceReference {
        evidence_reference_id: contracts::ParentEvidenceReferenceId::parse(id)
            .value_or_unreachable("evidence id"),
        kind: contracts::ParentEvidenceReferenceKind::ActivityEvent,
        observed_at: timestamp("2026-06-27T10:40:00Z"),
    }
}

fn audit_event(id: &str) -> contracts::AppInstallPurchaseApprovalAuditEventRef {
    contracts::AppInstallPurchaseApprovalAuditEventRef {
        audit_event_id: contracts::AppInstallPurchaseApprovalAuditEventId::parse(id)
            .value_or_unreachable("audit event id"),
        event_kind: contracts::AppInstallPurchaseApprovalAuditEventKind::RequestRecorded,
        recorded_at: timestamp("2026-06-27T10:41:00Z"),
        evidence_references: vec![evidence_ref("evidence-install-purchase-1")],
    }
}

fn state_snapshot(
    state: contracts::AppInstallPurchaseApprovalState,
) -> contracts::AppInstallPurchaseApprovalStateSnapshot {
    contracts::AppInstallPurchaseApprovalStateSnapshot {
        state,
        expiry_state: contracts::AppInstallPurchaseApprovalExpiryState::NotExpiring,
        expires_at: None,
        review_reason: None,
    }
}

fn family() -> contracts::FamilyReference {
    contracts::FamilyReference {
        family_id: contracts::FamilyId::parse("family-alpha").value_or_unreachable("family id"),
    }
}

fn child() -> contracts::ChildProfileReference {
    contracts::ChildProfileReference {
        child_profile_id: contracts::ChildProfileId::parse("child-alpha")
            .value_or_unreachable("child id"),
        display_name: contracts::ChildProfileDisplayName::parse("Child Alpha")
            .value_or_unreachable("child display name"),
    }
}

fn device() -> contracts::ParentDeviceReference {
    contracts::ParentDeviceReference {
        device_id: contracts::ParentDeviceId::parse("device-alpha")
            .value_or_unreachable("device id"),
        child_profile_id: Some(
            contracts::ChildProfileId::parse("child-alpha").value_or_unreachable("child id"),
        ),
        label: contracts::ParentDeviceLabel::parse("Pixel Tablet")
            .value_or_unreachable("device label"),
        platform: contracts::ParentPlatform::Android,
    }
}

fn store_metadata() -> contracts::AppInstallPurchaseApprovalStoreMetadata {
    contracts::AppInstallPurchaseApprovalStoreMetadata {
        store_surface: contracts::AppInstallPurchaseApprovalStoreSurface::GooglePlay,
        source_state: contracts::AppInstallPurchaseApprovalSupportState::ManualRequired,
        freshness: contracts::AppInstallPurchaseApprovalStoreMetadataFreshness::Stale,
        listing_id: Some(
            contracts::AppInstallPurchaseApprovalStoreListingId::parse("google-play-listing")
                .value_or_unreachable("listing id"),
        ),
        app_title: Some(
            contracts::AppInstallPurchaseApprovalAppTitle::parse("Game Alpha")
                .value_or_unreachable("title"),
        ),
        publisher_name: Some(
            contracts::AppInstallPurchaseApprovalPublisherName::parse("Ocentra")
                .value_or_unreachable("publisher"),
        ),
        category: Some(
            contracts::AppInstallPurchaseApprovalCategory::parse("Games")
                .value_or_unreachable("category"),
        ),
        age_rating: Some(
            contracts::AppInstallPurchaseApprovalAgeRating::parse("Everyone")
                .value_or_unreachable("age rating"),
        ),
        refreshed_at: Some(timestamp("2026-06-27T10:42:00Z")),
        stale_at: Some(timestamp("2026-06-27T11:42:00Z")),
        proof_requirement: proof_requirement("approved store metadata proof"),
    }
}

fn install_request() -> contracts::AppInstallRequest {
    contracts::AppInstallRequest {
        schema_version: contracts::APP_INSTALL_PURCHASE_APPROVAL_SCHEMA_VERSION.to_string(),
        request_id: contracts::AppInstallPurchaseApprovalRequestId::parse("install-request-1")
            .value_or_unreachable("request id"),
        request_kind: contracts::AppInstallPurchaseApprovalRequestKind::Install,
        family: family(),
        child: child(),
        device: device(),
        platform: contracts::ParentPlatform::Android,
        store_metadata: store_metadata(),
        approval_state: state_snapshot(
            contracts::AppInstallPurchaseApprovalState::PendingParentReview,
        ),
        requested_at: timestamp("2026-06-27T10:43:00Z"),
        evidence_references: vec![evidence_ref("evidence-install-purchase-1")],
        audit_event_refs: vec![audit_event("audit-install-purchase-1")],
    }
}

fn purchase_request(
    request_id: &str,
    request_kind: contracts::AppInstallPurchaseApprovalRequestKind,
    purchase_kind: contracts::AppInstallPurchaseApprovalPurchaseKind,
) -> contracts::PurchaseRequest {
    contracts::PurchaseRequest {
        schema_version: contracts::APP_INSTALL_PURCHASE_APPROVAL_SCHEMA_VERSION.to_string(),
        request_id: contracts::AppInstallPurchaseApprovalRequestId::parse(request_id)
            .value_or_unreachable("request id"),
        request_kind,
        family: family(),
        child: child(),
        device: device(),
        platform: contracts::ParentPlatform::Android,
        store_metadata: store_metadata(),
        approval_state: state_snapshot(contracts::AppInstallPurchaseApprovalState::Approved),
        requested_at: timestamp("2026-06-27T10:44:00Z"),
        evidence_references: vec![evidence_ref("evidence-install-purchase-2")],
        audit_event_refs: vec![audit_event("audit-install-purchase-2")],
        purchase_kind,
        subscription_period: Some(contracts::AppInstallPurchaseApprovalSubscriptionPeriod::Monthly),
        price_display: Some(
            contracts::AppInstallPurchaseApprovalPriceDisplay::parse("USD 4.99")
                .value_or_unreachable("price"),
        ),
        billing_entitlement_claim:
            contracts::AppInstallPurchaseApprovalBillingEntitlementClaim::NotClaimed,
    }
}

fn approval_proof() -> contracts::AppInstallPurchaseApprovalContractProof {
    contracts::AppInstallPurchaseApprovalContractProof {
        schema_version: contracts::APP_INSTALL_PURCHASE_APPROVAL_SCHEMA_VERSION.to_string(),
        install_request: install_request(),
        purchase_request: purchase_request(
            "purchase-request-1",
            contracts::AppInstallPurchaseApprovalRequestKind::Purchase,
            contracts::AppInstallPurchaseApprovalPurchaseKind::InAppPurchase,
        ),
        subscription_request: purchase_request(
            "subscription-request-1",
            contracts::AppInstallPurchaseApprovalRequestKind::Subscription,
            contracts::AppInstallPurchaseApprovalPurchaseKind::Subscription,
        ),
        approval_decisions: vec![contracts::AppInstallPurchaseApprovalDecision {
            schema_version: contracts::APP_INSTALL_PURCHASE_APPROVAL_SCHEMA_VERSION.to_string(),
            decision_id: contracts::AppInstallPurchaseApprovalDecisionId::parse("decision-1")
                .value_or_unreachable("decision id"),
            request_id: contracts::AppInstallPurchaseApprovalRequestId::parse("purchase-request-1")
                .value_or_unreachable("request id"),
            request_kind: contracts::AppInstallPurchaseApprovalRequestKind::Purchase,
            decision_action: contracts::AppInstallPurchaseApprovalDecisionAction::Approve,
            resulting_state: state_snapshot(contracts::AppInstallPurchaseApprovalState::Approved),
            parent_action: Some(contracts::ParentActionReference {
                action_reference_id: contracts::ParentActionReferenceId::parse("action-1")
                    .value_or_unreachable("action id"),
                actor: contracts::ParentActorReference {
                    actor_id: contracts::ParentActorId::parse("parent-1").value_or_unreachable("actor id"),
                    role: contracts::ParentActorRole::Parent,
                },
                policy_version: contracts::ParentPolicyVersion::parse("policy-v1")
                    .value_or_unreachable("policy version"),
                created_at: timestamp("2026-06-27T10:45:00Z"),
            }),
            decided_at: timestamp("2026-06-27T10:46:00Z"),
            audit_event_refs: vec![audit_event("audit-install-purchase-3")],
        }],
        platform_support_matrix: vec![contracts::AppInstallPurchaseApprovalPlatformSupportRow {
            platform: contracts::ParentPlatform::Android,
            store_surface: contracts::AppInstallPurchaseApprovalStoreSurface::GooglePlay,
            contract_request_state: contracts::AppInstallPurchaseApprovalSupportState::ManualRequired,
            store_metadata_state: contracts::AppInstallPurchaseApprovalSupportState::ManualRequired,
            install_interception_state: contracts::AppInstallPurchaseApprovalSupportState::Unavailable,
            purchase_interception_state:
                contracts::AppInstallPurchaseApprovalSupportState::Unavailable,
            subscription_interception_state:
                contracts::AppInstallPurchaseApprovalSupportState::Unavailable,
            child_pending_state: contracts::AppInstallPurchaseApprovalSupportState::ManualRequired,
            approval_delivery_state:
                contracts::AppInstallPurchaseApprovalSupportState::ManualRequired,
            manual_requirement: Some(
                contracts::AppInstallPurchaseApprovalManualRequirement::parse(
                    "manual parent review only",
                )
                .value_or_unreachable("manual requirement"),
            ),
            unavailable_reason: None,
            proof_requirement: proof_requirement("android platform proof"),
            claim_boundary: claim_boundary("contract proof; no platform adapter"),
        }],
        platform_source_metadata: vec![contracts::AppInstallPurchaseApprovalPlatformSourceMetadataRow {
            schema_version: contracts::APP_INSTALL_PURCHASE_APPROVAL_SCHEMA_VERSION.to_string(),
            source_row_id: contracts::AppInstallPurchaseApprovalPlatformSourceRowId::parse(
                "platform-source-android-google-play",
            )
            .value_or_unreachable("source row id"),
            platform: contracts::ParentPlatform::Android,
            store_surface: contracts::AppInstallPurchaseApprovalStoreSurface::GooglePlay,
            source_authority:
                contracts::AppInstallPurchaseApprovalPlatformSourceAuthority::GooglePlayListing,
            metadata_state:
                contracts::AppInstallPurchaseApprovalPlatformSourceMetadataState::ManualRequired,
            source_evidence_state:
                contracts::AppInstallPurchaseApprovalPlatformSourceEvidenceState::RequiresApprovedApiProof,
            fields_available_from_contract: vec![],
            fields_requiring_platform_proof: vec![
                contracts::AppInstallPurchaseApprovalPlatformSourceMetadataField::StoreListingId,
                contracts::AppInstallPurchaseApprovalPlatformSourceMetadataField::AppTitle,
            ],
            request_kind_coverage: vec![
                contracts::AppInstallPurchaseApprovalRequestKind::Install,
                contracts::AppInstallPurchaseApprovalRequestKind::Purchase,
                contracts::AppInstallPurchaseApprovalRequestKind::Subscription,
            ],
            required_artifacts: vec![
                contracts::AppInstallPurchaseApprovalPlatformSourceArtifactRequirement::parse(
                    "Google Play approved API proof",
                )
                .value_or_unreachable("required artifact"),
            ],
            limitation_reason:
                contracts::AppInstallPurchaseApprovalPlatformSourceLimitationReason::parse(
                    "play metadata needs approved proof",
                )
                .value_or_unreachable("limitation reason"),
            limitation_report_ref:
                contracts::AppInstallPurchaseApprovalPlatformSourceReportRef::parse(
                    "platform-report-1",
                )
                .value_or_unreachable("report ref"),
            parent_manual_fallback:
                contracts::AppInstallPurchaseApprovalPlatformSourceManualFallback::ContractOnlyParentReview,
            store_integration_claim:
                contracts::AppInstallPurchaseApprovalStoreIntegrationClaim::NotClaimed,
            platform_adapter_claim:
                contracts::AppInstallPurchaseApprovalPlatformAdapterClaim::NotImplemented,
            interception_claim: contracts::AppInstallPurchaseApprovalInterceptionClaim::NotClaimed,
            claim_boundary:
                contracts::AppInstallPurchaseApprovalPlatformSourceClaimBoundary::parse(
                    "contract proof; no store integration; no platform adapter",
                )
                .value_or_unreachable("claim boundary"),
            last_checked_at: timestamp("2026-06-27T10:47:00Z"),
        }],
        package_source_artifacts: vec![contracts::AppInstallPurchaseApprovalPackageSourceArtifactRow {
            schema_version: contracts::APP_INSTALL_PURCHASE_APPROVAL_SCHEMA_VERSION.to_string(),
            artifact_row_id:
                contracts::AppInstallPurchaseApprovalPackageSourceArtifactRowId::parse(
                    "package-source-android-google-play",
                )
                .value_or_unreachable("artifact row id"),
            platform: contracts::ParentPlatform::Android,
            store_surface: contracts::AppInstallPurchaseApprovalStoreSurface::GooglePlay,
            platform_source_row_id:
                contracts::AppInstallPurchaseApprovalPackageSourceMetadataRowId::parse(
                    "platform-source-android-google-play",
                )
                .value_or_unreachable("platform source row id"),
            package_source_kind:
                contracts::AppInstallPurchaseApprovalPackageSourceKind::AndroidPackageSourceRecord,
            artifact_status:
                contracts::AppInstallPurchaseApprovalPackageSourceArtifactStatus::DeviceProofRequired,
            approval_path_state:
                contracts::AppInstallPurchaseApprovalPackageSourceApprovalPathState::ManualRequired,
            package_source_fields_required: vec![
                contracts::AppInstallPurchaseApprovalPackageSourceField::PackageIdentifier,
                contracts::AppInstallPurchaseApprovalPackageSourceField::InstallerSource,
            ],
            package_source_fields_attached: vec![],
            request_kind_coverage: vec![
                contracts::AppInstallPurchaseApprovalRequestKind::Install,
                contracts::AppInstallPurchaseApprovalRequestKind::Purchase,
                contracts::AppInstallPurchaseApprovalRequestKind::Subscription,
            ],
            required_artifacts: vec![
                contracts::AppInstallPurchaseApprovalPackageSourceArtifactRequirement::parse(
                    "Android package source record",
                )
                .value_or_unreachable("required artifact"),
            ],
            artifact_evidence_claim:
                contracts::AppInstallPurchaseApprovalPackageSourceArtifactEvidenceClaim::NotAttached,
            artifact_evidence_path: None,
            artifact_captured_at: None,
            limitation_reason:
                contracts::AppInstallPurchaseApprovalPackageSourceLimitationReason::parse(
                    "device proof required",
                )
                .value_or_unreachable("limitation reason"),
            limitation_report_ref:
                contracts::AppInstallPurchaseApprovalPackageSourceReportRef::parse(
                    "package-report-1",
                )
                .value_or_unreachable("report ref"),
            store_integration_claim:
                contracts::AppInstallPurchaseApprovalStoreIntegrationClaim::NotClaimed,
            platform_adapter_claim:
                contracts::AppInstallPurchaseApprovalPlatformAdapterClaim::NotImplemented,
            interception_claim: contracts::AppInstallPurchaseApprovalInterceptionClaim::NotClaimed,
            child_data_custody:
                contracts::AppInstallPurchaseApprovalPackageSourceChildDataCustody::NoChildActivityData,
            claim_boundary:
                contracts::AppInstallPurchaseApprovalPackageSourceClaimBoundary::parse(
                    "contract proof; no store integration; no child activity data",
                )
                .value_or_unreachable("claim boundary"),
            last_checked_at: timestamp("2026-06-27T10:48:00Z"),
        }],
        child_facing_states: vec![contracts::AppInstallPurchaseApprovalChildFacingState {
            schema_version: contracts::APP_INSTALL_PURCHASE_APPROVAL_SCHEMA_VERSION.to_string(),
            child_state_id: contracts::AppInstallPurchaseApprovalChildStateId::parse("child-state-1")
                .value_or_unreachable("child state id"),
            request_id: contracts::AppInstallPurchaseApprovalRequestId::parse("purchase-request-1")
                .value_or_unreachable("request id"),
            request_kind: contracts::AppInstallPurchaseApprovalRequestKind::Purchase,
            platform: contracts::ParentPlatform::Android,
            child_visible_status:
                contracts::AppInstallPurchaseApprovalChildFacingStatus::ApprovedVisible,
            source_approval_state:
                state_snapshot(contracts::AppInstallPurchaseApprovalState::Approved),
            delivery_state: contracts::AppInstallPurchaseApprovalSupportState::ManualRequired,
            delivery_requirement: proof_requirement("parent delivery proof"),
            audit_event_refs: vec![audit_event("audit-install-purchase-4")],
            report_refs: vec![
                contracts::AppInstallPurchaseApprovalReportRef::parse("child-report-1")
                    .value_or_unreachable("report ref"),
            ],
            claim_boundary: claim_boundary("contract proof; no delivery claim"),
        }],
        audit_report_integration: vec![
            contracts::AppInstallPurchaseApprovalAuditReportIntegration {
                schema_version:
                    contracts::APP_INSTALL_PURCHASE_APPROVAL_SCHEMA_VERSION.to_string(),
                surface:
                    contracts::AppInstallPurchaseApprovalAuditReportSurface::RequestAuditHistory,
                integration_state:
                    contracts::AppInstallPurchaseApprovalProofIntegrationState::ContractOnly,
                audit_event_refs: vec![audit_event("audit-install-purchase-5")],
                report_refs: vec![
                    contracts::AppInstallPurchaseApprovalReportRef::parse("audit-report-1")
                        .value_or_unreachable("report ref"),
                ],
                proof_requirement: proof_requirement("audit report proof"),
                claim_boundary: claim_boundary("contract proof; no portal runtime"),
            },
        ],
        non_claims: vec![
            contracts::AppInstallPurchaseApprovalNonClaim::NoStoreIntegration,
            contracts::AppInstallPurchaseApprovalNonClaim::NoPortalUi,
        ],
        store_integration_claim: contracts::AppInstallPurchaseApprovalStoreIntegrationClaim::NotClaimed,
        billing_entitlement_claim:
            contracts::AppInstallPurchaseApprovalBillingEntitlementClaim::NotClaimed,
        portal_ui_claim: contracts::AppInstallPurchaseApprovalPortalUiClaim::NotImplemented,
        platform_adapter_claim:
            contracts::AppInstallPurchaseApprovalPlatformAdapterClaim::NotImplemented,
        interception_claim: contracts::AppInstallPurchaseApprovalInterceptionClaim::NotClaimed,
        runtime_blocking_separation:
            contracts::AppInstallPurchaseApprovalRuntimeBlockingSeparation::SeparateFromGenericAppBlocking,
        updated_at: timestamp("2026-06-27T10:49:00Z"),
    }
}

#[test]
fn approval_contract_round_trips_through_rust_owned_shape() {
    let proof = approval_proof();
    let encoded = serde_json::to_value(&proof).value_or_unreachable("proof serializes");

    assert_eq!(
        encoded["schemaVersion"],
        json!(contracts::APP_INSTALL_PURCHASE_APPROVAL_SCHEMA_VERSION)
    );
    assert_eq!(encoded["installRequest"]["requestKind"], json!("install"));
    assert_eq!(
        encoded["purchaseRequest"]["purchaseKind"],
        json!("in-app-purchase")
    );
    assert_eq!(
        encoded["platformSourceMetadata"][0]["sourceEvidenceState"],
        json!("requires-approved-api-proof")
    );
    assert_eq!(
        encoded["packageSourceArtifacts"][0]["artifactEvidenceClaim"],
        json!("not-attached")
    );
    assert!(encoded.get("schema_version").is_none());

    let decoded: contracts::AppInstallPurchaseApprovalContractProof =
        serde_json::from_value(encoded).value_or_unreachable("proof deserializes");
    assert_eq!(decoded, proof);
}

#[test]
fn generated_typescript_app_install_purchase_contracts_stay_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated/app-install-purchase-approval-contracts.ts"
    );
    let generated = app_install_purchase_approval_contracts_typescript();

    assert_eq!(checked_in, generated);
    assert_exports_include(
        &generated,
        &[
            "AppInstallPurchaseApprovalContractRuntime",
            "GeneratedFamilyId",
            "GeneratedChildProfileId",
            "GeneratedChildProfileDisplayName",
            "GeneratedParentDeviceId",
            "GeneratedParentDeviceLabel",
            "GeneratedParentActorId",
            "GeneratedParentPolicyVersion",
            "GeneratedParentEvidenceReferenceId",
            "GeneratedParentActionReferenceId",
            "GeneratedParentTimestamp",
            "GeneratedAppInstallPurchaseApprovalRequestId",
            "GeneratedAppInstallPurchaseApprovalDecisionId",
            "GeneratedAppInstallPurchaseApprovalAuditEventId",
            "GeneratedAppInstallPurchaseApprovalStoreListingId",
            "GeneratedAppInstallPurchaseApprovalAppTitle",
            "GeneratedAppInstallPurchaseApprovalPublisherName",
            "GeneratedAppInstallPurchaseApprovalCategory",
            "GeneratedAppInstallPurchaseApprovalAgeRating",
            "GeneratedAppInstallPurchaseApprovalReviewReason",
            "GeneratedAppInstallPurchaseApprovalProofRequirement",
            "GeneratedAppInstallPurchaseApprovalUnavailableReason",
            "GeneratedAppInstallPurchaseApprovalManualRequirement",
            "GeneratedAppInstallPurchaseApprovalClaimBoundary",
            "GeneratedAppInstallPurchaseApprovalPriceDisplay",
            "GeneratedAppInstallPurchaseApprovalChildStateId",
            "GeneratedAppInstallPurchaseApprovalReportRef",
            "GeneratedAppInstallPurchaseApprovalPlatformSourceRowId",
            "GeneratedAppInstallPurchaseApprovalPlatformSourceArtifactRequirement",
            "GeneratedAppInstallPurchaseApprovalPlatformSourceLimitationReason",
            "GeneratedAppInstallPurchaseApprovalPlatformSourceReportRef",
            "GeneratedAppInstallPurchaseApprovalPlatformSourceClaimBoundary",
            "GeneratedAppInstallPurchaseApprovalPackageSourceArtifactRowId",
            "GeneratedAppInstallPurchaseApprovalPackageSourceMetadataRowId",
            "GeneratedAppInstallPurchaseApprovalPackageSourceArtifactRequirement",
            "GeneratedAppInstallPurchaseApprovalPackageSourceLimitationReason",
            "GeneratedAppInstallPurchaseApprovalPackageSourceReportRef",
            "GeneratedAppInstallPurchaseApprovalPackageSourceClaimBoundary",
            "GeneratedParentPlatform",
            "GeneratedParentActorRole",
            "GeneratedParentEvidenceReferenceKind",
            "GeneratedAppInstallPurchaseApprovalRequestKind",
            "GeneratedAppInstallPurchaseApprovalStoreSurface",
            "GeneratedAppInstallPurchaseApprovalStoreMetadataFreshness",
            "GeneratedAppInstallPurchaseApprovalSupportState",
            "GeneratedAppInstallPurchaseApprovalDecisionAction",
            "GeneratedAppInstallPurchaseApprovalState",
            "GeneratedAppInstallPurchaseApprovalExpiryState",
            "GeneratedAppInstallPurchaseApprovalPurchaseKind",
            "GeneratedAppInstallPurchaseApprovalSubscriptionPeriod",
            "GeneratedAppInstallPurchaseApprovalChildFacingStatus",
            "GeneratedAppInstallPurchaseApprovalAuditReportSurface",
            "GeneratedAppInstallPurchaseApprovalProofIntegrationState",
            "GeneratedAppInstallPurchaseApprovalAuditEventKind",
            "GeneratedAppInstallPurchaseApprovalNonClaim",
            "GeneratedAppInstallPurchaseApprovalStoreIntegrationClaim",
            "GeneratedAppInstallPurchaseApprovalBillingEntitlementClaim",
            "GeneratedAppInstallPurchaseApprovalPortalUiClaim",
            "GeneratedAppInstallPurchaseApprovalPlatformAdapterClaim",
            "GeneratedAppInstallPurchaseApprovalInterceptionClaim",
            "GeneratedAppInstallPurchaseApprovalRuntimeBlockingSeparation",
            "GeneratedAppInstallPurchaseApprovalPlatformSourceAuthority",
            "GeneratedAppInstallPurchaseApprovalPlatformSourceMetadataState",
            "GeneratedAppInstallPurchaseApprovalPlatformSourceEvidenceState",
            "GeneratedAppInstallPurchaseApprovalPlatformSourceMetadataField",
            "GeneratedAppInstallPurchaseApprovalPlatformSourceManualFallback",
            "GeneratedAppInstallPurchaseApprovalPackageSourceArtifactStatus",
            "GeneratedAppInstallPurchaseApprovalPackageSourceApprovalPathState",
            "GeneratedAppInstallPurchaseApprovalPackageSourceArtifactEvidenceClaim",
            "GeneratedAppInstallPurchaseApprovalPackageSourceField",
            "GeneratedAppInstallPurchaseApprovalPackageSourceKind",
            "GeneratedAppInstallPurchaseApprovalPackageSourceChildDataCustody",
        ],
    );
    assert_eq!(
        property_string_value(&generated, "SchemaVersion"),
        Some(contracts::APP_INSTALL_PURCHASE_APPROVAL_SCHEMA_VERSION.to_owned())
    );
}

#[test]
fn generated_typescript_app_install_purchase_rule_helpers_stay_checked_in() {
    let approval_rules_checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated/app-install-purchase-approval-contract-rules.ts"
    );
    let approval_rules_generated = app_install_purchase_approval_contract_rules_typescript();
    let runtime_rules_checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated/app-install-purchase-runtime-proof-rules.ts"
    );
    let runtime_rules_generated = app_install_purchase_runtime_proof_rules_typescript();

    assert_eq!(approval_rules_checked_in, approval_rules_generated);
    assert_exports_include(
        &approval_rules_generated,
        &[
            "storeMetadataFreshnessIsConsistentGenerated",
            "approvalStateSnapshotIsConsistentGenerated",
            "purchaseRequestKindIsConsistentGenerated",
            "approvalDecisionIsConsistentGenerated",
            "platformSupportRowIsHonestGenerated",
            "childFacingStateIsConsistentGenerated",
            "auditReportIntegrationIsHonestGenerated",
            "appInstallPurchaseApprovalContractProofIsHonestGenerated",
        ],
    );
    assert_eq!(runtime_rules_checked_in, runtime_rules_generated);
    assert_exports_include(
        &runtime_rules_generated,
        &[
            "appInstallPurchaseRuntimeProofIsHonestGenerated",
            "appInstallPurchaseRuntimePlatformArtifactRowIsHonestGenerated",
            "appInstallPurchaseRuntimeChildDeliveryRowIsHonestGenerated",
            "appInstallPurchaseRuntimeReportIntegrationRowIsHonestGenerated",
            "appInstallPurchaseRuntimeStatusReadinessRowIsHonestGenerated",
        ],
    );
}

#[test]
fn generated_typescript_app_install_purchase_proof_helpers_stay_checked_in() {
    let helpers_checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated/app-install-purchase-proof-helpers.ts"
    );
    let helpers_generated = app_install_purchase_proof_helpers_typescript();

    assert_eq!(helpers_checked_in, helpers_generated);
    assert_exports_include(
        &helpers_generated,
        &[
            "buildAppInstallPurchaseApprovalPlatformSourceMetadataRowGenerated",
            "buildAppInstallPurchaseApprovalPackageSourceArtifactRowGenerated",
            "buildAppInstallPurchaseRuntimePlatformArtifactRowGenerated",
            "AppInstallPurchaseApprovalReportRefsGenerated",
        ],
    );
}

#[test]
fn generated_typescript_app_install_purchase_report_status_helpers_stay_checked_in() {
    let helpers_checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated/app-install-purchase-report-status-helpers.ts"
    );
    let helpers_generated = app_install_purchase_report_status_helpers_typescript();

    assert_eq!(helpers_checked_in, helpers_generated);
    assert_exports_include(
        &helpers_generated,
        &[
            "buildAppInstallPurchaseApprovalReportDomainRowGenerated",
            "buildAppInstallPurchaseReportRuntimeSurfaceRowGenerated",
            "buildAppInstallPurchaseProviderStoreReportStatusRowGenerated",
            "buildAppInstallPurchaseReportStatusReadModelRowGenerated",
            "buildAppInstallPurchaseStoreStatusHandoffRowGenerated",
            "buildAppInstallPurchaseLimitationSummaryRowGenerated",
        ],
    );
}

#[test]
fn generated_typescript_app_install_purchase_platform_provider_helpers_stay_checked_in() {
    let helpers_checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated/app-install-purchase-platform-provider-helpers.ts"
    );
    let helpers_generated = app_install_purchase_platform_provider_helpers_typescript();

    assert_eq!(helpers_checked_in, helpers_generated);
    assert_exports_include(
        &helpers_generated,
        &[
            "buildAppInstallPurchaseApprovedApiEntitlementEvidenceRowGenerated",
            "buildAppInstallPurchasePlatformStoreArtifactRowGenerated",
            "buildAppInstallPurchasePlatformProofReadinessRowGenerated",
            "buildAppInstallPurchaseStoreManualEvidenceRowGenerated",
            "buildAppInstallPurchaseProviderStoreApiExecutionRowGenerated",
            "buildAppInstallPurchaseProviderStoreExecutionReadinessRowGenerated",
        ],
    );
}

#[test]
fn generated_typescript_app_install_purchase_platform_evidence_helpers_stay_checked_in() {
    let helpers_checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated/app-install-purchase-platform-evidence-helpers.ts"
    );
    let helpers_generated = app_install_purchase_platform_evidence_helpers_typescript();

    assert_eq!(helpers_checked_in, helpers_generated);
    assert_exports_include(
        &helpers_generated,
        &[
            "buildAppInstallPurchasePlatformLimitationActionRowGenerated",
            "buildAppInstallPurchasePlatformAdapterBoundaryRowGenerated",
            "buildAppInstallPurchasePlatformAdapterEvidenceGapRowGenerated",
            "buildAppInstallPurchaseProviderStoreExecutionPreflightRowGenerated",
            "buildAppInstallPurchaseProviderStoreManualEvidencePacketRowGenerated",
            "buildAppInstallPurchaseProviderStorePlatformEvidenceRowGenerated",
            "buildAppInstallPurchaseWindowsPackageSourceAdapterEvidenceRowGenerated",
            "buildAppInstallPurchaseWindowsPackageSourceRuntimeHandoffRowGenerated",
        ],
    );
}

#[test]
fn generated_typescript_app_install_purchase_delivery_runtime_helpers_stay_checked_in() {
    let helpers_checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated/app-install-purchase-delivery-runtime-helpers.ts"
    );
    let helpers_generated = app_install_purchase_delivery_runtime_helpers_typescript();

    assert_eq!(helpers_checked_in, helpers_generated);
    assert_exports_include(
        &helpers_generated,
        &[
            "buildAppInstallPurchaseChildPackageArtifactRowGenerated",
            "buildAppInstallPurchaseChildDeviceDeliveryRuntimeWriterRowGenerated",
            "buildAppInstallPurchaseParentActionRuntimeHandoffRowGenerated",
            "buildAppInstallPurchaseParentActionDeliveryReadinessRowGenerated",
            "buildAppInstallPurchaseRuntimeWriterDeliveryRowGenerated",
            "buildAppInstallPurchaseRuntimeWriterExecutionDeliveryRowGenerated",
            "buildAppInstallPurchaseRuntimeReportWriterDeliveryRowGenerated",
            "childDeviceDeliveryReadinessStateGenerated",
        ],
    );
}

#[test]
fn generated_typescript_app_install_purchase_external_runtime_helpers_stay_checked_in() {
    let helpers_checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated/app-install-purchase-external-runtime-helpers.ts"
    );
    let helpers_generated = app_install_purchase_external_runtime_helpers_typescript();

    assert_eq!(helpers_checked_in, helpers_generated);
    assert_exports_include(
        &helpers_generated,
        &[
            "buildAppInstallPurchaseExternalRuntimeDeviceDeliveryRowGenerated",
            "buildAppInstallPurchaseExternalRuntimeDeliveryHandoffRowGenerated",
            "buildAppInstallPurchaseExternalRuntimeWriterReadinessRowGenerated",
            "buildAppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryRowGenerated",
            "buildAppInstallPurchaseExternalRuntimeWriterDeliveryBlockerRowGenerated",
            "buildAppInstallPurchaseExternalRuntimeTransportQueueRowGenerated",
            "buildAppInstallPurchaseExternalRuntimeTransportDispatchPreflightRowGenerated",
            "buildAppInstallPurchaseExternalRuntimeWriterTransportPreflightRowGenerated",
            "buildAppInstallPurchaseRuntimeDeliveryReceiptBoundaryRowGenerated",
            "buildAppInstallPurchaseRuntimeTransportDeliveryExecutionRowGenerated",
            "buildAppInstallPurchaseExternalRuntimeWriterTransportExecutionRowGenerated",
        ],
    );
}

#[test]
fn app_install_purchase_approval_adapters_stay_thin_and_generated_backed() {
    let adapter =
        include_str!("../../../../packages/schema-domain/src/app-install-purchase-approval.ts");
    let rules_adapter = include_str!(
        "../../../../packages/schema-domain/src/app-install-purchase-approval-rules.ts"
    );
    let runtime_rules_adapter = include_str!(
        "../../../../packages/schema-domain/src/app-install-purchase-runtime-proof-rules.ts"
    );
    let platform_sources_adapter = include_str!(
        "../../../../packages/schema-domain/src/app-install-purchase-approval-platform-sources.ts"
    );
    let package_sources_adapter = include_str!(
        "../../../../packages/schema-domain/src/app-install-purchase-approval-package-sources.ts"
    );
    let proof_states_adapter = include_str!(
        "../../../../packages/schema-domain/src/app-install-purchase-approval-proof-states.ts"
    );
    let proof_adapter = include_str!(
        "../../../../packages/schema-domain/src/app-install-purchase-approval-proof.ts"
    );
    let runtime_proof_adapter = include_str!(
        "../../../../packages/schema-domain/src/app-install-purchase-runtime-proof.ts"
    );
    let approval_report_domain_adapter = include_str!(
        "../../../../packages/schema-domain/src/app-install-purchase-approval-report-domain-proof.ts"
    );
    let report_runtime_adapter = include_str!(
        "../../../../packages/schema-domain/src/app-install-purchase-report-runtime-proof.ts"
    );
    let provider_store_report_status_adapter = include_str!(
        "../../../../packages/schema-domain/src/app-install-purchase-provider-store-report-status-proof.ts"
    );
    let report_status_read_model_adapter = include_str!(
        "../../../../packages/schema-domain/src/app-install-purchase-report-status-read-model-handoff-proof.ts"
    );
    let store_status_handoff_adapter = include_str!(
        "../../../../packages/schema-domain/src/app-install-purchase-store-status-handoff-proof.ts"
    );
    let limitation_summary_adapter = include_str!(
        "../../../../packages/schema-domain/src/app-install-purchase-limitation-summary-proof.ts"
    );
    let approved_api_entitlement_adapter = include_str!(
        "../../../../packages/schema-domain/src/app-install-purchase-approved-api-entitlement-proof.ts"
    );
    let platform_artifact_adapter = include_str!(
        "../../../../packages/schema-domain/src/app-install-purchase-platform-artifact-proof.ts"
    );
    let platform_proof_readiness_adapter = include_str!(
        "../../../../packages/schema-domain/src/app-install-purchase-platform-proof-readiness.ts"
    );
    let store_manual_evidence_adapter = include_str!(
        "../../../../packages/schema-domain/src/app-install-purchase-store-manual-evidence-proof.ts"
    );
    let provider_store_api_execution_adapter = include_str!(
        "../../../../packages/schema-domain/src/app-install-purchase-provider-store-api-execution-proof.ts"
    );
    let provider_store_execution_readiness_adapter = include_str!(
        "../../../../packages/schema-domain/src/app-install-purchase-provider-store-execution-readiness-proof.ts"
    );
    let platform_limitation_action_adapter = include_str!(
        "../../../../packages/schema-domain/src/app-install-purchase-platform-limitation-action-proof.ts"
    );
    let platform_adapter_boundary_adapter = include_str!(
        "../../../../packages/schema-domain/src/app-install-purchase-platform-adapter-boundary-proof.ts"
    );
    let platform_adapter_evidence_gap_adapter = include_str!(
        "../../../../packages/schema-domain/src/app-install-purchase-platform-adapter-evidence-gap-proof.ts"
    );
    let provider_store_execution_preflight_adapter = include_str!(
        "../../../../packages/schema-domain/src/app-install-purchase-provider-store-execution-preflight-proof.ts"
    );
    let provider_store_manual_evidence_packet_adapter = include_str!(
        "../../../../packages/schema-domain/src/app-install-purchase-provider-store-manual-evidence-packet-proof.ts"
    );
    let provider_store_platform_evidence_adapter = include_str!(
        "../../../../packages/schema-domain/src/app-install-purchase-provider-store-platform-evidence-proof.ts"
    );
    let windows_package_source_adapter_evidence_adapter = include_str!(
        "../../../../packages/schema-domain/src/app-install-purchase-windows-package-source-adapter-evidence.ts"
    );

    assert!(adapter
        .contains("thin adapter over Rust-generated app-install purchase approval contracts"));
    assert_import_paths_include(
        adapter,
        &["./generated/app-install-purchase-approval-contracts"],
    );
    assert!(rules_adapter
        .contains("compatibility shim over Rust-generated app-install purchase approval rules"));
    assert!(
        rules_adapter.contains("from './generated/app-install-purchase-approval-contract-rules'")
    );
    assert!(runtime_rules_adapter.contains(
        "compatibility shim over Rust-generated app-install purchase runtime proof rules"
    ));
    assert!(runtime_rules_adapter
        .contains("from './generated/app-install-purchase-runtime-proof-rules'"));
    assert!(
        platform_sources_adapter.contains("from './generated/app-install-purchase-proof-helpers'")
    );
    assert!(platform_sources_adapter
        .contains("appInstallPurchaseApprovalPlatformSourceMetadataRowsAreCompleteGenerated"));
    assert!(
        package_sources_adapter.contains("from './generated/app-install-purchase-proof-helpers'")
    );
    assert!(package_sources_adapter
        .contains("appInstallPurchaseApprovalPackageSourceArtifactRowsAreCompleteGenerated"));
    assert_import_names_include(
        proof_states_adapter,
        &["AppInstallPurchaseApprovalReportRefsGenerated"],
    );
    assert!(
        proof_states_adapter.contains("appInstallPurchaseApprovalAuditReportIntegrationGenerated")
    );
    assert!(
        proof_adapter.contains("buildAppInstallPurchaseApprovalPlatformSourceMetadataRowGenerated")
    );
    assert_import_names_include(
        proof_adapter,
        &["summarizeAppInstallPurchaseApprovalSupportStatesGenerated"],
    );
    assert!(runtime_proof_adapter
        .contains("buildAppInstallPurchaseRuntimePlatformArtifactRowGenerated"));
    assert_import_names_include(
        runtime_proof_adapter,
        &["summarizeAppInstallPurchaseRuntimeProofGenerated"],
    );
    assert!(approval_report_domain_adapter
        .contains("from './generated/app-install-purchase-report-status-helpers'"));
    assert!(approval_report_domain_adapter
        .contains("buildAppInstallPurchaseApprovalReportDomainRowGenerated"));
    assert_import_paths_include(
        report_runtime_adapter,
        &["./generated/app-install-purchase-report-status-helpers"],
    );
    assert!(
        report_runtime_adapter.contains("buildAppInstallPurchaseReportRuntimeSurfaceRowGenerated")
    );
    assert!(provider_store_report_status_adapter
        .contains("from './generated/app-install-purchase-report-status-helpers'"));
    assert!(provider_store_report_status_adapter
        .contains("buildAppInstallPurchaseProviderStoreReportStatusRowGenerated"));
    assert!(report_status_read_model_adapter
        .contains("from './generated/app-install-purchase-report-status-helpers'"));
    assert!(report_status_read_model_adapter
        .contains("buildAppInstallPurchaseReportStatusReadModelRowGenerated"));
    assert!(store_status_handoff_adapter
        .contains("from './generated/app-install-purchase-report-status-helpers'"));
    assert!(store_status_handoff_adapter
        .contains("buildAppInstallPurchaseStoreStatusHandoffRowGenerated"));
    assert!(limitation_summary_adapter
        .contains("from './generated/app-install-purchase-report-status-helpers'"));
    assert!(
        limitation_summary_adapter.contains("buildAppInstallPurchaseLimitationSummaryRowGenerated")
    );
    assert_import_paths_include(
        approved_api_entitlement_adapter,
        &["./generated/app-install-purchase-platform-provider-helpers"],
    );
    assert!(approved_api_entitlement_adapter
        .contains("buildAppInstallPurchaseApprovedApiEntitlementEvidenceRowGenerated"));
    assert!(platform_artifact_adapter
        .contains("from './generated/app-install-purchase-platform-provider-helpers'"));
    assert_import_names_include(
        platform_artifact_adapter,
        &["buildAppInstallPurchasePlatformStoreArtifactRowGenerated"],
    );
    assert!(platform_proof_readiness_adapter
        .contains("from './generated/app-install-purchase-platform-provider-helpers'"));
    assert_import_names_include(
        platform_proof_readiness_adapter,
        &["buildAppInstallPurchasePlatformProofReadinessRowGenerated"],
    );
    assert!(store_manual_evidence_adapter
        .contains("from './generated/app-install-purchase-platform-provider-helpers'"));
    assert!(store_manual_evidence_adapter
        .contains("buildAppInstallPurchaseStoreManualEvidenceRowGenerated"));
    assert!(provider_store_api_execution_adapter
        .contains("from './generated/app-install-purchase-platform-provider-helpers'"));
    assert!(provider_store_api_execution_adapter
        .contains("buildAppInstallPurchaseProviderStoreApiExecutionRowGenerated"));
    assert!(provider_store_execution_readiness_adapter
        .contains("from './generated/app-install-purchase-platform-provider-helpers'"));
    assert!(provider_store_execution_readiness_adapter
        .contains("buildAppInstallPurchaseProviderStoreExecutionReadinessRowGenerated"));
    assert!(platform_limitation_action_adapter
        .contains("from './generated/app-install-purchase-platform-evidence-helpers'"));
    assert!(platform_limitation_action_adapter
        .contains("buildAppInstallPurchasePlatformLimitationActionRowGenerated"));
    assert!(platform_adapter_boundary_adapter
        .contains("from './generated/app-install-purchase-platform-evidence-helpers'"));
    assert!(platform_adapter_boundary_adapter
        .contains("buildAppInstallPurchasePlatformAdapterBoundaryRowGenerated"));
    assert!(platform_adapter_evidence_gap_adapter
        .contains("from './generated/app-install-purchase-platform-evidence-helpers'"));
    assert!(platform_adapter_evidence_gap_adapter
        .contains("buildAppInstallPurchasePlatformAdapterEvidenceGapRowGenerated"));
    assert!(provider_store_execution_preflight_adapter
        .contains("from './generated/app-install-purchase-platform-evidence-helpers'"));
    assert!(provider_store_execution_preflight_adapter
        .contains("buildAppInstallPurchaseProviderStoreExecutionPreflightRowGenerated"));
    assert!(provider_store_manual_evidence_packet_adapter
        .contains("from './generated/app-install-purchase-platform-evidence-helpers'"));
    assert!(provider_store_manual_evidence_packet_adapter
        .contains("buildAppInstallPurchaseProviderStoreManualEvidencePacketRowGenerated"));
    assert!(provider_store_platform_evidence_adapter
        .contains("from './generated/app-install-purchase-platform-evidence-helpers'"));
    assert!(provider_store_platform_evidence_adapter
        .contains("buildAppInstallPurchaseProviderStorePlatformEvidenceRowGenerated"));
    assert!(windows_package_source_adapter_evidence_adapter
        .contains("from './generated/app-install-purchase-platform-evidence-helpers'"));
    assert!(windows_package_source_adapter_evidence_adapter
        .contains("buildAppInstallPurchaseWindowsPackageSourceAdapterEvidenceRowGenerated"));
    assert!(windows_package_source_adapter_evidence_adapter
        .contains("buildAppInstallPurchaseWindowsPackageSourceRuntimeHandoffRowGenerated"));
}
