use std::collections::BTreeSet;

use ocentra_schema::app_game_preview_source_freshness::APP_GAME_SOURCE_FRESHNESS_POLICY_CONSUMPTION_MATRIX_ID;
use ocentra_schema::app_game_preview_source_freshness_ts::{
    app_game_preview_source_freshness_data_typescript,
    app_game_preview_source_freshness_rules_typescript,
    app_game_preview_source_freshness_values_typescript,
};

fn exported_names(source: &str) -> BTreeSet<String> {
    source
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim_start();
            for prefix in [
                "export const ",
                "export function ",
                "export interface ",
                "export type ",
            ] {
                if let Some(rest) = trimmed.strip_prefix(prefix) {
                    let name = rest
                        .split(|ch: char| ch.is_whitespace() || ch == '(' || ch == '=' || ch == '{')
                        .next()
                        .unwrap_or_default();
                    return (!name.is_empty()).then(|| name.to_owned());
                }
            }

            None
        })
        .collect()
}

fn assert_exports_include(source: &str, expected: &[&str]) {
    let expected = expected.iter().map(|name| (*name).to_owned()).collect::<BTreeSet<_>>();
    let actual = exported_names(source);

    assert!(expected.is_subset(&actual));
}

fn string_const_value(source: &str, name: &str) -> Option<String> {
    let prefix = format!("export const {name} =");
    let rest = source.split_once(&prefix)?.1;
    let quoted = rest.split_once('\'')?.1;
    Some(quoted.split_once('\'')?.0.to_owned())
}

fn request_policy_ids(source: &str) -> BTreeSet<String> {
    source
        .split("policyRequestId: '")
        .skip(1)
        .filter_map(|rest| rest.split_once('\'').map(|(value, _)| value.to_owned()))
        .collect()
}

#[test]
fn generated_typescript_app_game_preview_source_freshness_values_stay_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated/app-game-preview-source-freshness-values.ts"
    );
    let generated = app_game_preview_source_freshness_values_typescript();

    assert_eq!(checked_in, generated);
    assert_exports_include(
        &generated,
        &[
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
            "AppGameFreshnessRequirementSourceKindsGenerated",
            "AppGameSourceFreshnessPreviewGateStatusGenerated",
            "AppGameSourceFreshnessPreviewGateStateGenerated",
            "AppGameSourceFreshnessPreviewGateNoRuntimeClaimFlagsGenerated",
            "AppGameSourceGatedPolicyPreviewReadModelProjectionStateGenerated",
            "AppGameSourceGatedPolicyPreviewReadModelSensitiveBoundaryGenerated",
            "RequiredAppGameSourceGatedPolicyPreviewReadModelNonClaimsGenerated",
            "AppGameSourceGatedPolicyPreviewReadModelNoClaimFlagsGenerated",
        ],
    );
    assert_eq!(
        string_const_value(&generated, "AppGameSourceFreshnessPolicyConsumptionMatrixIdGenerated"),
        Some(APP_GAME_SOURCE_FRESHNESS_POLICY_CONSUMPTION_MATRIX_ID.to_owned())
    );
}

#[test]
fn generated_typescript_app_game_preview_source_freshness_rules_stay_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated/app-game-preview-source-freshness-rules.ts"
    );
    let generated = app_game_preview_source_freshness_rules_typescript();

    assert_eq!(checked_in, generated);
    assert_exports_include(
        &generated,
        &[
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
        ],
    );
}

#[test]
fn generated_typescript_app_game_preview_source_freshness_data_stay_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated/app-game-preview-source-freshness-data.ts"
    );
    let generated = app_game_preview_source_freshness_data_typescript();

    assert_eq!(checked_in, generated);
    assert_exports_include(
        &generated,
        &["AppGameSourceFreshnessPolicyConsumptionGeneratedAtGenerated", "AppGameSourceFreshnessPolicyConsumptionFreshObservedAtGenerated", "AppGameSourceFreshnessPolicyConsumptionStaleObservedAtGenerated", "AppGameSourceFreshnessPolicyConsumptionRequestsGenerated"],
    );
    assert_eq!(
        request_policy_ids(&generated),
        BTreeSet::from([
            "source-freshness-native-app-ready-request".to_owned(),
            "source-freshness-native-game-ready-request".to_owned(),
            "source-freshness-native-game-manual-request".to_owned(),
        ])
    );
}
