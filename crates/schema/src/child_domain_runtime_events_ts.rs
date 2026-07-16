use ocentra_parent_agent_protocol::constants::child_domain_runtime as child_domain_runtime_constants;

const CHILD_DOMAIN_RUNTIME_EVENT_TYPE_LITERALS_TOKEN: &str =
    "__CHILD_DOMAIN_RUNTIME_EVENT_TYPE_LITERALS__";
const CHILD_DOMAIN_RUNTIME_EVENTS_TYPESCRIPT_TEMPLATE: &str = concat!(
    include_str!("child_domain_runtime_events.template.txt"),
    include_str!("child_domain_runtime_events.domain-event-schemas.template.txt"),
    include_str!("child_domain_runtime_events.domain-event-exports.template.txt"),
    include_str!("child_domain_runtime_events.domain-event-helpers.template.txt"),
);

pub fn child_domain_runtime_events_typescript() -> String {
    CHILD_DOMAIN_RUNTIME_EVENTS_TYPESCRIPT_TEMPLATE.replace(
        CHILD_DOMAIN_RUNTIME_EVENT_TYPE_LITERALS_TOKEN,
        &format!(
            "export const ChildDomainRuntimeEventTypeLiteral = {{\n  AppObserved: '{}',\n  AppEvidenceRecorded: '{}',\n  AppAiAnalysisRequested: '{}',\n  AppPolicyEvaluationRequested: '{}',\n  AppGameObserved: '{}',\n  AppGameEvidenceRecorded: '{}',\n  AppGameAiAnalysisRequested: '{}',\n  AppGamePolicyEvaluationRequested: '{}',\n  BrowserObserved: '{}',\n  BrowserEvidenceRecorded: '{}',\n  BrowserAiAnalysisRequested: '{}',\n  BrowserPolicyEvaluationRequested: '{}',\n  LanObserved: '{}',\n  LanEvidenceRecorded: '{}',\n  LanAiAnalysisRequested: '{}',\n  LanPolicyEvaluationRequested: '{}',\n  NetworkObserved: '{}',\n  NetworkEvidenceRecorded: '{}',\n  NetworkAiAnalysisRequested: '{}',\n  NetworkPolicyEvaluationRequested: '{}',\n  ScreenObserved: '{}',\n  ScreenEvidenceRecorded: '{}',\n  ScreenAiAnalysisRequested: '{}',\n  ScreenPolicyEvaluationRequested: '{}',\n  ScreenLiveViewObserved: '{}',\n  ScreenLiveViewEvidenceRecorded: '{}',\n  ScreenLiveViewAiAnalysisRequested: '{}',\n  ScreenLiveViewPolicyEvaluationRequested: '{}',\n  AiAnalysisCompleted: '{}',\n  PolicyViolationDetected: '{}',\n  NotificationRequested: '{}',\n}} as const;",
            child_domain_runtime_constants::APP_OBSERVED_EVENT_TYPE,
            child_domain_runtime_constants::APP_EVIDENCE_RECORDED_EVENT_TYPE,
            child_domain_runtime_constants::APP_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
            child_domain_runtime_constants::APP_POLICY_EVALUATION_REQUESTED_EVENT_TYPE,
            child_domain_runtime_constants::APP_GAME_OBSERVED_EVENT_TYPE,
            child_domain_runtime_constants::APP_GAME_EVIDENCE_RECORDED_EVENT_TYPE,
            child_domain_runtime_constants::APP_GAME_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
            child_domain_runtime_constants::APP_GAME_POLICY_EVALUATION_REQUESTED_EVENT_TYPE,
            child_domain_runtime_constants::BROWSER_OBSERVED_EVENT_TYPE,
            child_domain_runtime_constants::BROWSER_EVIDENCE_RECORDED_EVENT_TYPE,
            child_domain_runtime_constants::BROWSER_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
            child_domain_runtime_constants::BROWSER_POLICY_EVALUATION_REQUESTED_EVENT_TYPE,
            child_domain_runtime_constants::LAN_OBSERVED_EVENT_TYPE,
            child_domain_runtime_constants::LAN_EVIDENCE_RECORDED_EVENT_TYPE,
            child_domain_runtime_constants::LAN_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
            child_domain_runtime_constants::LAN_POLICY_EVALUATION_REQUESTED_EVENT_TYPE,
            child_domain_runtime_constants::NETWORK_OBSERVED_EVENT_TYPE,
            child_domain_runtime_constants::NETWORK_EVIDENCE_RECORDED_EVENT_TYPE,
            child_domain_runtime_constants::NETWORK_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
            child_domain_runtime_constants::NETWORK_POLICY_EVALUATION_REQUESTED_EVENT_TYPE,
            child_domain_runtime_constants::SCREEN_OBSERVED_EVENT_TYPE,
            child_domain_runtime_constants::SCREEN_EVIDENCE_RECORDED_EVENT_TYPE,
            child_domain_runtime_constants::SCREEN_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
            child_domain_runtime_constants::SCREEN_POLICY_EVALUATION_REQUESTED_EVENT_TYPE,
            child_domain_runtime_constants::SCREEN_LIVE_VIEW_OBSERVED_EVENT_TYPE,
            child_domain_runtime_constants::SCREEN_LIVE_VIEW_EVIDENCE_RECORDED_EVENT_TYPE,
            child_domain_runtime_constants::SCREEN_LIVE_VIEW_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
            child_domain_runtime_constants::SCREEN_LIVE_VIEW_POLICY_EVALUATION_REQUESTED_EVENT_TYPE,
            child_domain_runtime_constants::AI_ANALYSIS_COMPLETED_EVENT_TYPE,
            child_domain_runtime_constants::POLICY_VIOLATION_DETECTED_EVENT_TYPE,
            child_domain_runtime_constants::NOTIFICATION_REQUESTED_EVENT_TYPE,
        ),
    )
}
