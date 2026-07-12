use ocentra_browser_core::{
    browser_android_owned_shell_runtime::browser_android_owned_shell_runtime_typescript,
    browser_android_owned_shell_url_custody::browser_android_owned_shell_url_custody_typescript,
    browser_game_hidden_analysis_loader::browser_game_hidden_analysis_loader_plan_typescript,
    browser_game_policy_candidate_compiler::browser_game_policy_candidate_compiler_typescript,
    browser_game_url_shape_evaluator::browser_game_url_shape_evaluator_typescript,
    browser_hidden_analysis_loader::browser_hidden_analysis_loader_typescript,
    browser_policy_questionnaire_forest::browser_policy_questionnaire_forest_typescript,
    browser_url_intelligence::browser_url_intelligence_typescript,
    social_alert_report_local_outbox_bridge::social_alert_report_local_outbox_bridge_typescript,
    social_alert_report_preference_preflight::social_alert_report_preference_preflight_typescript,
    social_alert_report_preference_status_handoff::social_alert_report_preference_status_handoff_typescript,
    social_alert_report_provider_dispatch_execution::social_alert_report_provider_dispatch_execution_typescript,
    social_alert_report_scheduler_bridge::social_alert_report_scheduler_bridge_typescript,
    social_applied_schedule_time_budget_proof::social_applied_schedule_time_budget_proof_typescript,
    social_managed_browser_policy_execution::social_managed_browser_policy_execution_typescript,
    social_policy_candidate_compiler::social_policy_candidate_compiler_typescript,
    social_video_ai_signal_aggregate::social_video_ai_signal_aggregate_typescript,
    social_video_source_privacy::social_video_source_privacy_typescript,
};

#[test]
fn browser_generated_decision_helpers_remain_rust_owned_and_marked() {
    let generated = [
        (
            browser_hidden_analysis_loader_typescript(),
            "browserHiddenAnalysisLoaderDecisionTemplate",
        ),
        (
            browser_game_hidden_analysis_loader_plan_typescript(),
            "browserGameHiddenAnalysisDecisionTemplate",
        ),
        (
            browser_android_owned_shell_runtime_typescript(),
            "browserAndroidOwnedShellRuntimePhysicalTemplate",
        ),
        (
            browser_android_owned_shell_url_custody_typescript(),
            "browserAndroidOwnedShellUrlCustodyPhysicalEligible",
        ),
        (
            browser_game_policy_candidate_compiler_typescript(),
            "browserGamePolicyDecisionTemplate",
        ),
        (
            browser_game_url_shape_evaluator_typescript().to_string(),
            "browserGameUrlShapeParseResultTemplate",
        ),
        (
            browser_policy_questionnaire_forest_typescript().to_string(),
            "browserPolicyVisibleQuestionIdsTemplate",
        ),
        (
            browser_url_intelligence_typescript().to_string(),
            "browserUrlShapeClassificationResultTemplate",
        ),
    ];

    for (source, marker) in generated {
        assert!(source.starts_with("/* generated from crates/browser-core/src/"));
        assert_eq!(
            source
                .matches(&format!("export function {marker}("))
                .count(),
            1
        );
    }
}

#[test]
fn browser_generated_social_report_helpers_remain_rust_owned_and_marked() {
    let generated = [
        (
            social_alert_report_local_outbox_bridge_typescript(),
            "buildSocialAlertReportLocalOutboxBridgeReadModel",
        ),
        (
            social_alert_report_preference_preflight_typescript(),
            "buildSocialAlertReportPreferencePreflightReadModel",
        ),
        (
            social_alert_report_preference_status_handoff_typescript(),
            "buildSocialAlertReportPreferenceStatusHandoffReadModel",
        ),
        (
            social_alert_report_provider_dispatch_execution_typescript(),
            "buildSocialAlertReportProviderDispatchExecutionReadModel",
        ),
        (
            social_alert_report_scheduler_bridge_typescript(),
            "buildSocialAlertReportSchedulerBridgeReadModel",
        ),
        (
            social_applied_schedule_time_budget_proof_typescript(),
            "summarizeSocialAppliedScheduleTimeBudgetProof",
        ),
        (
            social_managed_browser_policy_execution_typescript(),
            "socialManagedBrowserPolicyExecutionTemplate",
        ),
        (
            social_policy_candidate_compiler_typescript(),
            "socialPolicyDecisionTemplate",
        ),
        (
            social_video_ai_signal_aggregate_typescript(),
            "buildSocialVideoAiSignalAggregate",
        ),
        (
            social_video_source_privacy_typescript(),
            "buildSocialVideoSourcePrivacySummary",
        ),
    ];

    for (source, marker) in generated {
        assert!(source.starts_with("/* generated from crates/browser-core/src/"));
        assert_eq!(
            source
                .matches(&format!("export function {marker}("))
                .count(),
            1
        );
    }
}
