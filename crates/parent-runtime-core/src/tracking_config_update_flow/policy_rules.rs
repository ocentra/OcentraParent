use super::*;

pub(super) fn tracking_policy_rule_refs(
    request: &TrackingConfigUpdateRequest,
) -> Vec<TrackingPolicyRuleRef> {
    let mut rule_refs = vec![tracking_policy_rule_ref(
        constants::tracking_config_update::POLICY_RULE_LOCAL_CHILD_RUNTIME,
    )];
    if request.retention_settings.requested_remote_sync_state == TrackingRemoteSyncState::Disabled {
        rule_refs.push(tracking_policy_rule_ref(
            constants::tracking_config_update::POLICY_RULE_REMOTE_SYNC_DISABLED,
        ));
    }
    if request.retention_settings.requested_remote_ai_state == TrackingRemoteAiState::Disabled {
        rule_refs.push(tracking_policy_rule_ref(
            constants::tracking_config_update::POLICY_RULE_REMOTE_AI_DISABLED,
        ));
    }
    rule_refs
}

fn tracking_policy_rule_ref(value: &str) -> TrackingPolicyRuleRef {
    TrackingPolicyRuleRef::parse(value).unwrap_or_else(|_| std::process::abort())
}
