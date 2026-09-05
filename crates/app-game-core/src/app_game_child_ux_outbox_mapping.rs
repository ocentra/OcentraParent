use ocentra_parent_agent_protocol::schema_domain_mirrors::family::{
    ParentEvidenceReference, ParentEvidenceReferenceKind,
};
use ocentra_parent_agent_protocol::schema_domain_mirrors::notification::{
    NotificationLocalOutboxDeliveryClaimState, NotificationLocalOutboxMinimalAlertEnvelope,
    NotificationLocalOutboxPayloadPreview, NotificationLocalOutboxRecord,
    NotificationLocalOutboxReference, NotificationLocalOutboxSeverity,
    NotificationLocalOutboxState, V3NotificationRuleReasonCode,
};

use crate::app_game_child_ux_outbox_token::text_token;
use crate::app_game_child_ux_outbox_types::AppGameChildUxOutboxInput;
use crate::app_game_child_ux_types::{AppGameChildUxNotice, AppGameChildUxNoticeState};

pub(super) fn build_record(input: AppGameChildUxOutboxInput) -> NotificationLocalOutboxRecord {
    let token = text_token(&input.notice);
    let evidence_refs = input
        .notice
        .evidence_refs
        .iter()
        .map(|reference| ParentEvidenceReference {
            evidence_reference_id: reference.as_str().to_owned(),
            kind: ParentEvidenceReferenceKind::PolicyDecision,
            observed_at: input.observed_at.as_str().to_owned(),
        })
        .collect();
    let mut audit_refs = input.audit_refs.clone();
    audit_refs.extend(bound_audit_refs(&input));
    NotificationLocalOutboxRecord {
        entry_id: input.entry_id,
        state: NotificationLocalOutboxState::QueuedLocal,
        envelope: NotificationLocalOutboxMinimalAlertEnvelope {
            alert_ref: input.alert_ref,
            family: input.family,
            device: input.device,
            parent_action: input.parent_action,
            severity: severity(input.notice.state),
            reason_code: reason_code(input.notice.state),
            provider_channel: input.provider_channel,
            evidence_refs,
            policy_refs: vec![input.notice.policy_rule_ref.as_str().into()],
            audit_refs,
            payload_template_ref: format!("app-game-child-ux-template:{token}").into(),
            provider_payload_preview: NotificationLocalOutboxPayloadPreview::from(token),
            sensitive_detail_minimized: true,
            raw_child_evidence_included: false,
            raw_url_or_title_included: false,
            raw_message_text_included: false,
            screenshot_or_report_included: false,
        },
        outbox_file_ref: input.outbox_file_ref,
        local_data_path_ref: input.local_data_path_ref,
        delivery_claim_state: NotificationLocalOutboxDeliveryClaimState::LocalOutboxOnly,
        visible_after_at: None,
        retry_attempt_count: 0,
        quiet_hours_ref: None,
        retry_policy_ref: None,
        dead_letter_ref: None,
        provider_receipt_ref: None,
        manual_proof_requirements: Vec::new(),
        manual_action_required: false,
        provider_delivery_attempted: false,
        provider_delivery_observed: false,
        provider_receipt_ingested: false,
        provider_credentials_stored: false,
        cloud_routing_claimed: false,
        parent_notification_ui_claimed: false,
        sensitive_provider_metadata_stored: false,
    }
}

pub(super) fn blocked_refs(notice: &AppGameChildUxNotice) -> Vec<String> {
    notice
        .child_reason_refs
        .iter()
        .map(|reference| reference.as_str().to_owned())
        .chain(
            notice
                .child_status_refs
                .iter()
                .map(|reference| reference.as_str().to_owned()),
        )
        .collect()
}

fn bound_audit_refs(input: &AppGameChildUxOutboxInput) -> Vec<NotificationLocalOutboxReference> {
    let mut refs = vec![input.artifact.artifact_reference_id.as_str().into()];
    refs.extend(
        input
            .artifact
            .child_reason_reference_ids
            .iter()
            .map(|reference| reference.as_str().into()),
    );
    refs.extend(
        input
            .artifact
            .child_status_reference_ids
            .iter()
            .map(|reference| reference.as_str().into()),
    );
    refs
}

fn severity(state: AppGameChildUxNoticeState) -> NotificationLocalOutboxSeverity {
    match state {
        AppGameChildUxNoticeState::RequestApproved => NotificationLocalOutboxSeverity::Info,
        AppGameChildUxNoticeState::RequestDenied
        | AppGameChildUxNoticeState::RequestSubmitted
        | AppGameChildUxNoticeState::NewAppNeedsApproval
        | AppGameChildUxNoticeState::NewGameNeedsApproval => {
            NotificationLocalOutboxSeverity::Attention
        }
        _ => NotificationLocalOutboxSeverity::Urgent,
    }
}

fn reason_code(state: AppGameChildUxNoticeState) -> V3NotificationRuleReasonCode {
    match state {
        AppGameChildUxNoticeState::AppLimited
        | AppGameChildUxNoticeState::GameTimeAlmostFinished => {
            V3NotificationRuleReasonCode::PolicyViolation
        }
        _ => V3NotificationRuleReasonCode::ParentRequest,
    }
}
