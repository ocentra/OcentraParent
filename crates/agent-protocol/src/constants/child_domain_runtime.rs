pub const DOMAIN_APP: &str = "app";
pub const DOMAIN_APP_GAME: &str = "app-game";
pub const DOMAIN_BROWSER: &str = "browser";
pub const DOMAIN_LAN: &str = "lan";
pub const DOMAIN_NETWORK: &str = "network";
pub const DOMAIN_SCREEN: &str = "screen";
pub const DOMAIN_SCREEN_LIVE_VIEW: &str = "screen-live-view";

pub const APP_OBSERVED_EVENT_TYPE: &str = "app.activity.observed";
pub const APP_EVIDENCE_RECORDED_EVENT_TYPE: &str = "app.evidence.recorded";
pub const APP_AI_ANALYSIS_REQUESTED_EVENT_TYPE: &str = "app.ai.analysis.requested";
pub const APP_POLICY_EVALUATION_REQUESTED_EVENT_TYPE: &str = "app.policy.evaluation.requested";

pub const APP_GAME_OBSERVED_EVENT_TYPE: &str = "app-game.activity.observed";
pub const APP_GAME_EVIDENCE_RECORDED_EVENT_TYPE: &str = "app-game.evidence.recorded";
pub const APP_GAME_AI_ANALYSIS_REQUESTED_EVENT_TYPE: &str = "app-game.ai.analysis.requested";
pub const APP_GAME_POLICY_EVALUATION_REQUESTED_EVENT_TYPE: &str =
    "app-game.policy.evaluation.requested";

pub const BROWSER_OBSERVED_EVENT_TYPE: &str = "browser.navigation.observed";
pub const BROWSER_EVIDENCE_RECORDED_EVENT_TYPE: &str = "browser.evidence.recorded";
pub const BROWSER_AI_ANALYSIS_REQUESTED_EVENT_TYPE: &str = "browser.ai.analysis.requested";
pub const BROWSER_POLICY_EVALUATION_REQUESTED_EVENT_TYPE: &str =
    "browser.policy.evaluation.requested";

pub const LAN_OBSERVED_EVENT_TYPE: &str = "lan.peer.observed";
pub const LAN_EVIDENCE_RECORDED_EVENT_TYPE: &str = "lan.evidence.recorded";
pub const LAN_AI_ANALYSIS_REQUESTED_EVENT_TYPE: &str = "lan.ai.analysis.requested";
pub const LAN_POLICY_EVALUATION_REQUESTED_EVENT_TYPE: &str = "lan.policy.evaluation.requested";

pub const NETWORK_OBSERVED_EVENT_TYPE: &str = "network.connection.observed";
pub const NETWORK_EVIDENCE_RECORDED_EVENT_TYPE: &str = "network.evidence.recorded";
pub const NETWORK_AI_ANALYSIS_REQUESTED_EVENT_TYPE: &str = "network.ai.analysis.requested";
pub const NETWORK_POLICY_EVALUATION_REQUESTED_EVENT_TYPE: &str =
    "network.policy.evaluation.requested";

pub const SCREEN_OBSERVED_EVENT_TYPE: &str = "screen.evidence.observed";
pub const SCREEN_EVIDENCE_RECORDED_EVENT_TYPE: &str = "screen.evidence.recorded";
pub const SCREEN_AI_ANALYSIS_REQUESTED_EVENT_TYPE: &str = "screen.ai.analysis.requested";
pub const SCREEN_POLICY_EVALUATION_REQUESTED_EVENT_TYPE: &str =
    "screen.policy.evaluation.requested";

pub const SCREEN_LIVE_VIEW_OBSERVED_EVENT_TYPE: &str = "screen-live-view.session.observed";
pub const SCREEN_LIVE_VIEW_EVIDENCE_RECORDED_EVENT_TYPE: &str =
    "screen-live-view.evidence.recorded";
pub const SCREEN_LIVE_VIEW_AI_ANALYSIS_REQUESTED_EVENT_TYPE: &str =
    "screen-live-view.ai.analysis.requested";
pub const SCREEN_LIVE_VIEW_POLICY_EVALUATION_REQUESTED_EVENT_TYPE: &str =
    "screen-live-view.policy.evaluation.requested";

pub const AI_ANALYSIS_COMPLETED_EVENT_TYPE: &str = "child-domain.ai.analysis.completed";
pub const POLICY_VIOLATION_DETECTED_EVENT_TYPE: &str = "child-domain.policy.violation.detected";
pub const NOTIFICATION_REQUESTED_EVENT_TYPE: &str = "child-domain.notification.requested";

pub const CHILD_DOMAIN_EVENT_TYPES: &[&str] = &[
    APP_OBSERVED_EVENT_TYPE,
    APP_EVIDENCE_RECORDED_EVENT_TYPE,
    APP_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
    APP_POLICY_EVALUATION_REQUESTED_EVENT_TYPE,
    APP_GAME_OBSERVED_EVENT_TYPE,
    APP_GAME_EVIDENCE_RECORDED_EVENT_TYPE,
    APP_GAME_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
    APP_GAME_POLICY_EVALUATION_REQUESTED_EVENT_TYPE,
    BROWSER_OBSERVED_EVENT_TYPE,
    BROWSER_EVIDENCE_RECORDED_EVENT_TYPE,
    BROWSER_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
    BROWSER_POLICY_EVALUATION_REQUESTED_EVENT_TYPE,
    LAN_OBSERVED_EVENT_TYPE,
    LAN_EVIDENCE_RECORDED_EVENT_TYPE,
    LAN_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
    LAN_POLICY_EVALUATION_REQUESTED_EVENT_TYPE,
    NETWORK_OBSERVED_EVENT_TYPE,
    NETWORK_EVIDENCE_RECORDED_EVENT_TYPE,
    NETWORK_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
    NETWORK_POLICY_EVALUATION_REQUESTED_EVENT_TYPE,
    SCREEN_OBSERVED_EVENT_TYPE,
    SCREEN_EVIDENCE_RECORDED_EVENT_TYPE,
    SCREEN_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
    SCREEN_POLICY_EVALUATION_REQUESTED_EVENT_TYPE,
    SCREEN_LIVE_VIEW_OBSERVED_EVENT_TYPE,
    SCREEN_LIVE_VIEW_EVIDENCE_RECORDED_EVENT_TYPE,
    SCREEN_LIVE_VIEW_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
    SCREEN_LIVE_VIEW_POLICY_EVALUATION_REQUESTED_EVENT_TYPE,
    AI_ANALYSIS_COMPLETED_EVENT_TYPE,
    POLICY_VIOLATION_DETECTED_EVENT_TYPE,
    NOTIFICATION_REQUESTED_EVENT_TYPE,
];

pub const SIGNAL_REQUIRES_AI: &str = "requires-ai-analysis";
pub const SIGNAL_REQUIRES_POLICY: &str = "requires-policy-evaluation";
pub const SIGNAL_OBSERVE_ONLY: &str = "observe-only";
pub const AI_PURPOSE_CLASSIFICATION: &str = "classification";
pub const POLICY_RULE_DEFAULT: &str = "child-domain.policy.default";
pub const POLICY_SEVERITY_REVIEW: &str = "review";
pub const NOTIFICATION_CHANNEL_PARENT_PORTAL: &str = "parent-portal";

pub const DEFAULT_CHILD_DEVICE_ID: &str = "child-device-default";
pub const DEFAULT_CHILD_PROFILE_ID: &str = "child-profile-default";
pub const DEFAULT_OBSERVED_AT: &str = "2026-06-12T12:10:00Z";
pub const DEFAULT_OBSERVATION_ID_SUFFIX: &str = "observation-default";
pub const APP_SUBJECT_REF_SUFFIX: &str = "foreground-app";
pub const APP_GAME_SUBJECT_REF_SUFFIX: &str = "foreground-game";
pub const BROWSER_SUBJECT_REF_SUFFIX: &str = "active-url";
pub const LAN_SUBJECT_REF_SUFFIX: &str = "peer-presence";
pub const NETWORK_SUBJECT_REF_SUFFIX: &str = "connection";
pub const SCREEN_SUBJECT_REF_SUFFIX: &str = "screen-frame";
pub const SCREEN_LIVE_VIEW_SUBJECT_REF_SUFFIX: &str = "live-view-session";
pub const DEFAULT_EVIDENCE_REF_SUFFIX: &str = "evidence-default";
pub const DEFAULT_AI_REQUEST_ID_SUFFIX: &str = "ai-request-default";
pub const DEFAULT_POLICY_REQUEST_ID_SUFFIX: &str = "policy-request-default";
pub const DEFAULT_POLICY_VIOLATION_ID_SUFFIX: &str = "policy-violation-default";
pub const DEFAULT_NOTIFICATION_ID_SUFFIX: &str = "notification-default";
pub const CORRELATION_PREFIX: &str = "child-domain-runtime:";
pub const IDEMPOTENCY_SEPARATOR: &str = ":";

pub const SOURCE_COMPONENT_CHILD_DOMAIN_RUNTIME: &str = "child-domain-runtime";
pub const SOURCE_COMPONENT_CHILD_AI_RUNTIME: &str = "child-ai-runtime";
pub const SOURCE_COMPONENT_CHILD_POLICY_RUNTIME: &str = "child-policy-runtime";
pub const SOURCE_COMPONENT_CHILD_NOTIFICATION_RUNTIME: &str = "child-notification-runtime";

pub const TARGET_HANDLER_DOMAIN_OBSERVER: &str = "target.child-domain.observer";
pub const TARGET_HANDLER_CHILD_AI_ANALYZER: &str = "target.child-ai.analyzer";
pub const TARGET_HANDLER_CHILD_POLICY_EVALUATOR: &str = "target.child-policy.evaluator";
pub const TARGET_HANDLER_CHILD_NOTIFICATION_BRIDGE: &str = "target.child-notification.bridge";

pub const SUBSCRIBER_APP_OBSERVER: &str = "subscriber.app.observer";
pub const SUBSCRIBER_APP_GAME_OBSERVER: &str = "subscriber.app-game.observer";
pub const SUBSCRIBER_BROWSER_OBSERVER: &str = "subscriber.browser.observer";
pub const SUBSCRIBER_LAN_OBSERVER: &str = "subscriber.lan.observer";
pub const SUBSCRIBER_NETWORK_OBSERVER: &str = "subscriber.network.observer";
pub const SUBSCRIBER_SCREEN_OBSERVER: &str = "subscriber.screen.observer";
pub const SUBSCRIBER_SCREEN_LIVE_VIEW_OBSERVER: &str = "subscriber.screen-live-view.observer";
pub const SUBSCRIBER_CHILD_AI_ANALYZER: &str = "subscriber.child-ai.analyzer";
pub const SUBSCRIBER_CHILD_AI_POLICY_BRIDGE: &str = "subscriber.child-ai.policy-bridge";
pub const SUBSCRIBER_CHILD_POLICY_EVALUATOR: &str = "subscriber.child-policy.evaluator";
pub const SUBSCRIBER_CHILD_NOTIFICATION_BRIDGE: &str = "subscriber.child-notification.bridge";

pub const ERROR_CHILD_DOMAIN_FLOW_RECORDED: &str = "child domain runtime flow recorded";
