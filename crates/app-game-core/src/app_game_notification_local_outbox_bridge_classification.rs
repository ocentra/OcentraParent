use ocentra_parent_agent_protocol::app_game_notification_readiness::{
    AppGameNotificationReadinessRow, APP_GAME_NOTIFICATION_READINESS_REASON_APPROVAL_REQUEST,
    APP_GAME_NOTIFICATION_READINESS_REASON_CAPABILITY_UNAVAILABLE,
    APP_GAME_NOTIFICATION_READINESS_REASON_MANUAL_REQUIRED,
    APP_GAME_NOTIFICATION_READINESS_REASON_SUSPICIOUS_UNKNOWN,
    APP_GAME_NOTIFICATION_READINESS_REASON_TIME_LIMIT_EXCEEDED,
    APP_GAME_NOTIFICATION_READINESS_STATE_MANUAL_REQUIRED,
    APP_GAME_NOTIFICATION_READINESS_STATE_READY_FOR_LOCAL_INTENT,
    APP_GAME_NOTIFICATION_READINESS_STATE_UNAVAILABLE,
};
use ocentra_parent_agent_protocol::schema_domain_mirrors::notification::{
    NotificationLocalOutboxSeverity, V3NotificationRuleReasonCode,
};

use crate::app_game_notification_local_outbox_bridge_types::AppGameNotificationLocalOutboxBridgeStatus;

#[derive(Clone)]
pub(super) struct NotificationOutboxClassification {
    pub status: AppGameNotificationLocalOutboxBridgeStatus,
    pub severity: NotificationLocalOutboxSeverity,
    pub reason_code: V3NotificationRuleReasonCode,
}

pub(super) fn classify(
    row: &AppGameNotificationReadinessRow,
) -> Option<NotificationOutboxClassification> {
    match (row.readiness_state.as_str(), row.reason.as_str()) {
        (
            APP_GAME_NOTIFICATION_READINESS_STATE_READY_FOR_LOCAL_INTENT,
            APP_GAME_NOTIFICATION_READINESS_REASON_TIME_LIMIT_EXCEEDED,
        ) => Some(classification(
            AppGameNotificationLocalOutboxBridgeStatus::Linked,
            NotificationLocalOutboxSeverity::Urgent,
            V3NotificationRuleReasonCode::PolicyViolation,
        )),
        (
            APP_GAME_NOTIFICATION_READINESS_STATE_READY_FOR_LOCAL_INTENT,
            APP_GAME_NOTIFICATION_READINESS_REASON_SUSPICIOUS_UNKNOWN,
        ) => Some(classification(
            AppGameNotificationLocalOutboxBridgeStatus::Linked,
            NotificationLocalOutboxSeverity::Attention,
            V3NotificationRuleReasonCode::SuspiciousUnknown,
        )),
        (
            APP_GAME_NOTIFICATION_READINESS_STATE_READY_FOR_LOCAL_INTENT,
            APP_GAME_NOTIFICATION_READINESS_REASON_APPROVAL_REQUEST,
        ) => Some(classification(
            AppGameNotificationLocalOutboxBridgeStatus::Linked,
            NotificationLocalOutboxSeverity::Attention,
            V3NotificationRuleReasonCode::ParentRequest,
        )),
        (
            APP_GAME_NOTIFICATION_READINESS_STATE_MANUAL_REQUIRED,
            APP_GAME_NOTIFICATION_READINESS_REASON_MANUAL_REQUIRED,
        ) => Some(classification(
            AppGameNotificationLocalOutboxBridgeStatus::ManualRequired,
            NotificationLocalOutboxSeverity::Info,
            V3NotificationRuleReasonCode::ParentRequest,
        )),
        (
            APP_GAME_NOTIFICATION_READINESS_STATE_UNAVAILABLE,
            APP_GAME_NOTIFICATION_READINESS_REASON_CAPABILITY_UNAVAILABLE,
        ) => Some(classification(
            AppGameNotificationLocalOutboxBridgeStatus::Unavailable,
            NotificationLocalOutboxSeverity::Info,
            V3NotificationRuleReasonCode::ParentRequest,
        )),
        _ => None,
    }
}

fn classification(
    status: AppGameNotificationLocalOutboxBridgeStatus,
    severity: NotificationLocalOutboxSeverity,
    reason_code: V3NotificationRuleReasonCode,
) -> NotificationOutboxClassification {
    NotificationOutboxClassification {
        status,
        severity,
        reason_code,
    }
}
