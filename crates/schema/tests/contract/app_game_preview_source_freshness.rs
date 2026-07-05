use std::collections::BTreeSet;

use crate::support::{
    assert_exports_include, request_policy_ids, string_const_value, ContractNames, ContractString,
};
use ocentra_schema::app_game_preview_source_freshness::APP_GAME_SOURCE_FRESHNESS_POLICY_CONSUMPTION_MATRIX_ID;
use ocentra_schema::app_game_preview_source_freshness_ts::{
    app_game_preview_source_freshness_data_typescript,
    app_game_preview_source_freshness_rules_typescript,
    app_game_preview_source_freshness_values_typescript,
};

#[test]
fn generated_typescript_app_game_preview_source_freshness_values_stay_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated-app-game-preview-source-freshness-values.ts"
    );
    let generated = app_game_preview_source_freshness_values_typescript();

    assert_eq!(checked_in, generated);
    assert_exports_include(
        crate::contract_text!(&generated),
        ContractNames(BTreeSet::from(
            [
                "AppGamePolicyPreviewTargetDomainGenerated",
                "AppGamePolicyPreviewStatusGenerated",
                "AppGamePolicyPreviewNoRuntimeClaimStatesGenerated",
                "AppGamePolicyPreviewNoRuntimeClaimFlagsGenerated",
                "AppGameSourceFreshnessPolicyConsumptionMatrixIdGenerated",
                "AppGameSourceFreshnessPolicyTargetKindGenerated",
                "AppGameSourceFreshnessRequirementKindGenerated",
                "AppGameSourceFreshnessSourceKindGenerated",
                "AppGameSourceFreshnessReadModelStateGenerated",
                "AppGameSourceFreshnessCapabilityStatusGenerated",
                "AppGameSourceFreshnessRequirementStateGenerated",
                "AppGameSourceFreshnessPolicyReadinessStateGenerated",
                "AppGameSourceFreshnessAdapterDispatchStateGenerated",
                "AppGameSourceFreshnessReasonCodeGenerated",
                "AppGameSourceFreshnessRequirementSourceKindsGenerated",
                "AppGameSourceFreshnessPreviewGateStatusGenerated",
                "AppGameSourceFreshnessPreviewGateStateGenerated",
                "AppGameSourceFreshnessPreviewGateNoRuntimeClaimFlagsGenerated",
                "AppGameSourceGatedPolicyPreviewReadModelProjectionStateGenerated",
                "AppGameSourceGatedPolicyPreviewReadModelSensitiveBoundaryGenerated",
                "RequiredAppGameSourceGatedPolicyPreviewReadModelNonClaimsGenerated",
                "AppGameSourceGatedPolicyPreviewReadModelNoClaimFlagsGenerated",
            ]
            .map(str::to_owned),
        )),
    );
    assert_eq!(
        string_const_value(
            crate::contract_text!(&generated),
            crate::contract_text!("AppGameSourceFreshnessPolicyConsumptionMatrixIdGenerated"),
        ),
        Some(ContractString(
            APP_GAME_SOURCE_FRESHNESS_POLICY_CONSUMPTION_MATRIX_ID.to_owned(),
        ))
    );
}

#[test]
fn generated_typescript_app_game_preview_source_freshness_rules_stay_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated-app-game-preview-source-freshness-rules.ts"
    );
    let generated = app_game_preview_source_freshness_rules_typescript();

    assert_eq!(checked_in, generated);
    assert_exports_include(
        crate::contract_text!(&generated),
        ContractNames(BTreeSet::from(
            [
                "appGamePolicyPreviewTargetDomainForKindGenerated",
                "appGamePolicyPreviewStatusForOutcomeGenerated",
                "appGamePolicyPreviewStatusMatchesOutcomeGenerated",
                "appGamePolicyPreviewRowIsDryRunOnlyGenerated",
                "appGamePolicyPreviewRowHasNoRuntimeClaimsGenerated",
                "appGamePolicyPreviewRowHasProofRefsGenerated",
                "countAppGamePolicyPreviewRowsGenerated",
                "countAppGamePolicyPreviewStatusesGenerated",
                "countAppGamePolicyPreviewReadModelRowsGenerated",
                "appGamePolicyPreviewReadModelCountsMatchRowsGenerated",
                "appGameSourceFreshnessTargetAllowsNullRefGenerated",
                "appGameSourceFreshnessSourceKindSatisfiesRequirementGenerated",
                "appGameSourceFreshnessRowsForRequirementGenerated",
                "appGameSourceFreshnessRowIsFreshGenerated",
                "appGameSourceFreshnessRowHasEvidenceGenerated",
                "appGameSourceFreshnessRequirementIsSatisfiedGenerated",
                "appGameSourceFreshnessReadinessIsPolicyReadyGenerated",
                "appGameSourceFreshnessRequirementFailureGenerated",
                "evaluateAppGameSourceFreshnessPolicyReadinessGenerated",
                "appGameSourceFreshnessPreviewGateRowHasNoRuntimeClaimsGenerated",
                "appGameSourceFreshnessPreviewGateRowMatchesSourceStateGenerated",
                "appGameSourceFreshnessPreviewGateRowMatchesPreviewStateGenerated",
                "countAppGameSourceFreshnessPreviewGateRowsGenerated",
                "countAppGameSourceFreshnessPreviewGateStatusesGenerated",
                "countAppGameSourceFreshnessPreviewGateStatesGenerated",
                "countAppGameSourceFreshnessPreviewGateReadModelRowsGenerated",
                "appGameSourceFreshnessPreviewGateReadModelCountsMatchRowsGenerated",
                "appGameSourceFreshnessPreviewGateReadModelHasNoRuntimeClaimsGenerated",
                "appGameSourceGatedPolicyPreviewReadModelRowHasNoRuntimeClaimsGenerated",
                "appGameSourceGatedPolicyPreviewReadModelRowMatchesGateStateGenerated",
                "countAppGameSourceGatedPolicyPreviewRowsGenerated",
                "countAppGameSourceGatedPolicyPreviewProjectionStatesGenerated",
                "countAppGameSourceGatedPolicyPreviewReadModelRowsGenerated",
                "appGameSourceGatedPolicyPreviewReadModelCountsMatchRowsGenerated",
                "appGameSourceGatedPolicyPreviewReadModelHasNoRuntimeClaimsGenerated",
            ]
            .map(str::to_owned),
        )),
    );
}

#[test]
fn generated_typescript_app_game_preview_source_freshness_data_stay_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated-app-game-preview-source-freshness-data.ts"
    );
    let generated = app_game_preview_source_freshness_data_typescript();

    assert_eq!(checked_in, generated);
    assert_exports_include(
        crate::contract_text!(&generated),
        ContractNames(BTreeSet::from(
            [
                "AppGameSourceFreshnessPolicyConsumptionGeneratedAtGenerated",
                "AppGameSourceFreshnessPolicyConsumptionFreshObservedAtGenerated",
                "AppGameSourceFreshnessPolicyConsumptionStaleObservedAtGenerated",
                "AppGameSourceFreshnessPolicyConsumptionRequestsGenerated",
            ]
            .map(str::to_owned),
        )),
    );
    assert_eq!(
        request_policy_ids(crate::contract_text!(&generated)),
        ContractNames(BTreeSet::from([
            "source-freshness-native-app-ready-request".to_owned(),
            "source-freshness-native-game-ready-request".to_owned(),
            "source-freshness-native-game-manual-request".to_owned(),
        ]))
    );
}
