use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingIntentKind;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason;
use ocentra_parent_agent_protocol::logging::LogFields;

use super::field_values::required_payload_text;

pub(super) fn required_intent_kind(
    fields: &LogFields,
) -> Result<LanPairingIntentKind, LanPairingRejectionReason> {
    match required_payload_text(fields, constants::field::LAN_INTENT_KIND.into())?
        .as_payload_text_ref()
        .0
    {
        constants::value::LAN_INTENT_HEALTH_QUERY => Ok(LanPairingIntentKind::HealthQuery),
        constants::value::LAN_INTENT_RULE_QUERY => Ok(LanPairingIntentKind::RuleQuery),
        constants::value::LAN_INTENT_RULE_UPDATE => Ok(LanPairingIntentKind::RuleUpdate),
        constants::value::LAN_INTENT_APPROVAL_DECISION => {
            Ok(LanPairingIntentKind::ApprovalDecision)
        }
        constants::value::LAN_INTENT_CONFIGURATION_UPDATE => {
            Ok(LanPairingIntentKind::ConfigurationUpdate)
        }
        constants::value::LAN_INTENT_CONTROLLER_LEASE_RENEW => {
            Ok(LanPairingIntentKind::ControllerLeaseRenew)
        }
        constants::value::LAN_INTENT_CONTROLLER_LEASE_RELEASE => {
            Ok(LanPairingIntentKind::ControllerLeaseRelease)
        }
        constants::value::LAN_INTENT_CONTROLLER_LEASE_TAKEOVER => {
            Ok(LanPairingIntentKind::ControllerLeaseTakeover)
        }
        constants::value::LAN_INTENT_LAN_AI_PROVIDER_STATUS => {
            Ok(LanPairingIntentKind::LanAiProviderStatus)
        }
        constants::value::LAN_INTENT_LAN_AI_JOB_SUBMIT => Ok(LanPairingIntentKind::LanAiJobSubmit),
        _ => Err(LanPairingRejectionReason::Malformed),
    }
}
