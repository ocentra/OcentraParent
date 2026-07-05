use std::collections::BTreeSet;

use crate::support::{assert_exports_include, ContractNames};
use ocentra_schema::app_game_timer_service_readiness::APP_GAME_TIMER_SERVICE_READINESS_GENERATED_MARKER;
use ocentra_schema::app_game_timer_service_readiness_ts::{
    app_game_timer_service_readiness_rules_typescript,
    app_game_timer_service_readiness_values_typescript,
};

#[test]
fn generated_typescript_app_game_timer_service_readiness_values_stay_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated-app-game-timer-service-readiness-values.ts"
    );
    let generated = app_game_timer_service_readiness_values_typescript();

    assert_eq!(checked_in, generated);
    assert_exports_include(
        crate::contract_text!(&generated),
        ContractNames(BTreeSet::from([
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
        ]
        .map(str::to_owned))),
    );
    assert_eq!(
        APP_GAME_TIMER_SERVICE_READINESS_GENERATED_MARKER,
        "app-game-timer-service-readiness"
    );
}

#[test]
fn generated_typescript_app_game_timer_service_readiness_rules_stay_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated-app-game-timer-service-readiness-rules.ts"
    );
    let generated = app_game_timer_service_readiness_rules_typescript();

    assert_eq!(checked_in, generated);
    assert_exports_include(
        crate::contract_text!(&generated),
        ContractNames(BTreeSet::from([
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
        ]
        .map(str::to_owned))),
    );
}
