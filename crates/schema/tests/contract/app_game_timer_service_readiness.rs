use std::collections::BTreeSet;

use ocentra_schema::app_game_timer_service_readiness::APP_GAME_TIMER_SERVICE_READINESS_GENERATED_MARKER;
use ocentra_schema::app_game_timer_service_readiness_ts::{
    app_game_timer_service_readiness_rules_typescript,
    app_game_timer_service_readiness_values_typescript,
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

#[test]
fn generated_typescript_app_game_timer_service_readiness_values_stay_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated/app-game-timer-service-readiness-values.ts"
    );
    let generated = app_game_timer_service_readiness_values_typescript();

    assert_eq!(checked_in, generated);
    assert_exports_include(
        &generated,
        &[
            "AppGameSourceGatedPolicyPreviewTimerHandoffStateGenerated",
            "RequiredAppGameSourceGatedPolicyPreviewTimerHandoffNonClaimsGenerated",
            "AppGameSourceGatedPolicyPreviewTimerHandoffNoClaimFlagsGenerated",
            "AppGameSourceGatedPolicyPreviewTimerStatusStateGenerated",
            "RequiredAppGameSourceGatedPolicyPreviewTimerStatusNonClaimsGenerated",
            "AppGameSourceGatedPolicyPreviewTimerStatusNoClaimFlagsGenerated",
            "AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessStateGenerated",
            "RequiredAppGameSourceGatedPolicyPreviewTimerRuntimeReadinessNonClaimsGenerated",
            "AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessNoClaimFlagsGenerated",
            "AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceStateGenerated",
            "RequiredAppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceNonClaimsGenerated",
            "AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceNoClaimFlagsGenerated",
            "AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffStateGenerated",
            "AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelStateGenerated",
            "RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelNonClaimsGenerated",
            "AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelNoClaimFlagsGenerated",
            "AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffStateGenerated",
            "AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelStateGenerated",
            "RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelNonClaimsGenerated",
            "AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelNoClaimFlagsGenerated",
        ],
    );
    assert_eq!(APP_GAME_TIMER_SERVICE_READINESS_GENERATED_MARKER, "app-game-timer-service-readiness");
}

#[test]
fn generated_typescript_app_game_timer_service_readiness_rules_stay_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated/app-game-timer-service-readiness-rules.ts"
    );
    let generated = app_game_timer_service_readiness_rules_typescript();

    assert_eq!(checked_in, generated);
    assert_exports_include(
        &generated,
        &[
            "appGameSourceGatedPolicyPreviewTimerStateMatchesProjectionGenerated",
            "appGameSourceGatedPolicyPreviewTimerHandoffStateForProjectionGenerated",
            "countAppGameSourceGatedPolicyPreviewTimerHandoffRowsGenerated",
            "appGameSourceGatedPolicyPreviewTimerHandoffCountsMatchGenerated",
            "appGameSourceGatedPolicyPreviewTimerHandoffHasNoRuntimeClaimsGenerated",
            "appGameSourceGatedPolicyPreviewTimerStatusMatchesHandoffGenerated",
            "appGameSourceGatedPolicyPreviewTimerStatusStateForHandoffGenerated",
            "countAppGameSourceGatedPolicyPreviewTimerStatusRowsGenerated",
            "appGameSourceGatedPolicyPreviewTimerStatusRequiredProofRefsGenerated",
            "appGameSourceGatedPolicyPreviewTimerStatusCountsMatchGenerated",
            "appGameSourceGatedPolicyPreviewTimerStatusHasNoRuntimeClaimsGenerated",
            "appGameSourceGatedPolicyPreviewTimerRuntimeReadinessMatchesStatusGenerated",
            "appGameSourceGatedPolicyPreviewTimerRuntimeReadinessStateForStatusGenerated",
            "countAppGameSourceGatedPolicyPreviewTimerRuntimeReadinessRowsGenerated",
            "appGameSourceGatedPolicyPreviewTimerRuntimeReadinessRequiredProofRefsGenerated",
            "appGameSourceGatedPolicyPreviewTimerRuntimeReadinessCountsMatchGenerated",
            "appGameSourceGatedPolicyPreviewTimerRuntimeReadinessHasNoRuntimeClaimsGenerated",
            "appGameSourceGatedPolicyPreviewTimerSchedulerPersistenceMatchesRuntimeReadinessGenerated",
            "appGameSourceGatedPolicyPreviewTimerSchedulerPersistenceStateForRuntimeReadinessGenerated",
            "countAppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRowsGenerated",
            "appGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRequiredProofRefsGenerated",
            "appGameSourceGatedPolicyPreviewTimerSchedulerPersistenceCountsMatchGenerated",
            "appGameSourceGatedPolicyPreviewTimerSchedulerPersistenceHasNoRuntimeClaimsGenerated",
            "appGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelMatchesHandoffGenerated",
            "appGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelStateForHandoffGenerated",
            "countAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelRowsGenerated",
            "appGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelCountsMatchGenerated",
            "appGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelHasNoRuntimeClaimsGenerated",
            "appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelMatchesHandoffGenerated",
            "appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelStateForHandoffGenerated",
            "countAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelRowsGenerated",
            "appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelCountsMatchGenerated",
            "appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelHasNoRuntimeClaimsGenerated",
        ],
    );
}
