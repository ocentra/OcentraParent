use std::collections::BTreeSet;

use crate::support::{assert_exports_include, ContractNames};
use ocentra_schema::app_game_source_freshness_policy_consumption::APP_GAME_SOURCE_FRESHNESS_POLICY_CONSUMPTION_GENERATED_MARKER;
use ocentra_schema::app_game_source_freshness_policy_consumption_ts::{
    app_game_source_freshness_policy_consumption_data_typescript,
    app_game_source_freshness_policy_consumption_typescript,
};

#[test]
fn generated_typescript_app_game_source_freshness_policy_consumption_stay_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated-app-game-source-freshness-policy-consumption.ts"
    );
    let generated = app_game_source_freshness_policy_consumption_typescript();

    assert_eq!(checked_in, generated);
    assert_exports_include(
        crate::contract_text!(&generated),
        ContractNames(BTreeSet::from(
            [
                "AppGameSourceFreshnessPolicyRequestIdSchema",
                "AppGameSourceFreshnessPolicyReadinessIdSchema",
                "AppGameSourceFreshnessTargetRefSchema",
                "AppGameSourceFreshnessEvidenceRefSchema",
                "AppGameSourceFreshnessMatrixIdSchema",
                "AppGameSourceFreshnessPolicyTargetKindSchema",
                "AppGameSourceFreshnessRequirementKindSchema",
                "AppGameSourceFreshnessSourceKindSchema",
                "AppGameSourceFreshnessReadModelStateSchema",
                "AppGameSourceFreshnessCapabilityStatusSchema",
                "AppGameSourceFreshnessRequirementStateSchema",
                "AppGameSourceFreshnessPolicyReadinessStateSchema",
                "AppGameSourceFreshnessAdapterDispatchStateSchema",
                "AppGameSourceFreshnessReasonCodeSchema",
                "AppGameSourceFreshnessStatusRowSchema",
                "AppGameSourceFreshnessPolicyTargetSchema",
                "AppGameSourceFreshnessPolicyRequestSchema",
                "AppGameSourceFreshnessRequirementResultSchema",
                "AppGameSourceFreshnessPolicyReadinessSchema",
                "AppGameSourceFreshnessPolicyConsumptionMatrixSchema",
                "evaluateAppGameSourceFreshnessPolicyReadiness",
                "decodeAppGameSourceFreshnessPolicyReadiness",
                "decodeAppGameSourceFreshnessPolicyConsumptionMatrix",
                "AppGameSourceFreshnessStatusRow",
                "AppGameSourceFreshnessPolicyTarget",
                "AppGameSourceFreshnessPolicyRequest",
                "AppGameSourceFreshnessRequirementResult",
                "AppGameSourceFreshnessPolicyReadiness",
                "AppGameSourceFreshnessPolicyConsumptionMatrix",
            ]
            .map(str::to_owned),
        )),
    );
    assert_eq!(
        APP_GAME_SOURCE_FRESHNESS_POLICY_CONSUMPTION_GENERATED_MARKER,
        "app-game-source-freshness-policy-consumption"
    );
}

#[test]
fn generated_typescript_app_game_source_freshness_policy_consumption_data_stay_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated-app-game-source-freshness-policy-consumption-data.ts"
    );
    let generated = app_game_source_freshness_policy_consumption_data_typescript();

    assert_eq!(checked_in, generated);
    assert_exports_include(
        crate::contract_text!(&generated),
        ContractNames(BTreeSet::from(
            [
                "AppGameSourceFreshnessPolicyConsumptionRequests",
                "AppGameSourceFreshnessPolicyConsumptionMatrix",
            ]
            .map(str::to_owned),
        )),
    );
}
