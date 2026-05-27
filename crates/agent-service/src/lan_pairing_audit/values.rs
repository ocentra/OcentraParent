use ocentra_parent_agent_protocol::{
    constants, LanPairingIntentKind, LanPairingParentAuthority, LanPairingRejectionReason,
};

pub(crate) fn intent_kind_value(intent_kind: &LanPairingIntentKind) -> &'static str {
    match intent_kind {
        LanPairingIntentKind::HealthQuery => constants::value::LAN_INTENT_HEALTH_QUERY,
        LanPairingIntentKind::RuleQuery => constants::value::LAN_INTENT_RULE_QUERY,
        LanPairingIntentKind::RuleUpdate => constants::value::LAN_INTENT_RULE_UPDATE,
        LanPairingIntentKind::ApprovalDecision => constants::value::LAN_INTENT_APPROVAL_DECISION,
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
}

pub(crate) fn parent_authority_value(parent_authority: &LanPairingParentAuthority) -> &'static str {
    match parent_authority {
        LanPairingParentAuthority::ActiveController => {
            constants::value::LAN_PARENT_AUTHORITY_ACTIVE_CONTROLLER
        }
        LanPairingParentAuthority::Observer => constants::value::LAN_PARENT_AUTHORITY_OBSERVER,
    }
}

pub(crate) fn reason_value(reason: &LanPairingRejectionReason) -> &'static str {
    match reason {
        LanPairingRejectionReason::Anonymous => constants::value::LAN_REASON_ANONYMOUS,
        LanPairingRejectionReason::ControllerLeaseMissing => {
            constants::value::LAN_REASON_CONTROLLER_LEASE_MISSING
        }
        LanPairingRejectionReason::ControllerLeaseExpired => {
            constants::value::LAN_REASON_CONTROLLER_LEASE_EXPIRED
        }
        LanPairingRejectionReason::WrongOrigin => constants::value::LAN_REASON_WRONG_ORIGIN,
        LanPairingRejectionReason::WrongDevice => constants::value::LAN_REASON_WRONG_DEVICE,
        LanPairingRejectionReason::WrongController => constants::value::LAN_REASON_WRONG_CONTROLLER,
        LanPairingRejectionReason::Expired => constants::value::LAN_REASON_EXPIRED,
        LanPairingRejectionReason::Replayed => constants::value::LAN_REASON_REPLAYED,
        LanPairingRejectionReason::Malformed => constants::value::LAN_REASON_MALFORMED,
        LanPairingRejectionReason::Stale => constants::value::LAN_REASON_STALE,
        LanPairingRejectionReason::Offline => constants::value::LAN_REASON_OFFLINE,
        LanPairingRejectionReason::Revoked => constants::value::LAN_REASON_REVOKED,
        LanPairingRejectionReason::LocalNetworkDisabled => {
            constants::value::LAN_REASON_UNSUPPORTED_ROUTE
        }
        LanPairingRejectionReason::UnsupportedRoute => {
            constants::value::LAN_REASON_UNSUPPORTED_ROUTE
        }
        LanPairingRejectionReason::UnselectedDevice => {
            constants::value::LAN_REASON_UNSELECTED_DEVICE
        }
        LanPairingRejectionReason::ObserverReadOnly => {
            constants::value::LAN_REASON_OBSERVER_READ_ONLY
        }
        LanPairingRejectionReason::TakeoverDenied => constants::value::LAN_REASON_TAKEOVER_DENIED,
        LanPairingRejectionReason::LanAiProviderUnavailable => {
            constants::value::LAN_REASON_LAN_AI_PROVIDER_UNAVAILABLE
        }
        LanPairingRejectionReason::LanAiJobUnauthorized => {
            constants::value::LAN_REASON_LAN_AI_JOB_UNAUTHORIZED
        }
    }
}

pub(crate) fn authentication_state_value(
    reason: Option<&LanPairingRejectionReason>,
) -> &'static str {
    match reason {
        None => constants::value::LAN_AUTH_PAIRED,
        Some(LanPairingRejectionReason::Anonymous)
        | Some(LanPairingRejectionReason::ControllerLeaseMissing)
        | Some(LanPairingRejectionReason::ControllerLeaseExpired)
        | Some(LanPairingRejectionReason::Malformed)
        | Some(LanPairingRejectionReason::ObserverReadOnly)
        | Some(LanPairingRejectionReason::TakeoverDenied)
        | Some(LanPairingRejectionReason::LanAiProviderUnavailable)
        | Some(LanPairingRejectionReason::LanAiJobUnauthorized)
        | Some(LanPairingRejectionReason::WrongController)
        | Some(LanPairingRejectionReason::WrongOrigin) => {
            constants::value::LAN_AUTH_UNAUTHENTICATED
        }
        Some(_) => constants::value::LAN_AUTH_PAIRED,
    }
}
