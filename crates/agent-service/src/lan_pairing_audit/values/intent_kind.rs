use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingIntentKind;
use ocentra_parent_agent_protocol::logging::LogFieldValue;

pub(super) fn intent_kind_value(intent_kind: &LanPairingIntentKind) -> LogFieldValue {
    LogFieldValue::String(
        match intent_kind {
            LanPairingIntentKind::HealthQuery => constants::value::LAN_INTENT_HEALTH_QUERY,
            LanPairingIntentKind::RuleQuery => constants::value::LAN_INTENT_RULE_QUERY,
            LanPairingIntentKind::RuleUpdate => constants::value::LAN_INTENT_RULE_UPDATE,
            LanPairingIntentKind::ApprovalDecision => {
                constants::value::LAN_INTENT_APPROVAL_DECISION
            }
            LanPairingIntentKind::ConfigurationUpdate => {
                constants::value::LAN_INTENT_CONFIGURATION_UPDATE
            }
            LanPairingIntentKind::ControllerLeaseRenew => {
                constants::value::LAN_INTENT_CONTROLLER_LEASE_RENEW
            }
            LanPairingIntentKind::ControllerLeaseRelease => {
                constants::value::LAN_INTENT_CONTROLLER_LEASE_RELEASE
            }
            LanPairingIntentKind::ControllerLeaseTakeover => {
                constants::value::LAN_INTENT_CONTROLLER_LEASE_TAKEOVER
            }
            LanPairingIntentKind::LanAiProviderStatus => {
                constants::value::LAN_INTENT_LAN_AI_PROVIDER_STATUS
            }
            LanPairingIntentKind::LanAiJobSubmit => constants::value::LAN_INTENT_LAN_AI_JOB_SUBMIT,
        }
        .to_string(),
    )
}
