use chrono::{DateTime, Utc};
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingRejectionReason, LanParentIntentEnvelope,
};

use super::{LanTrustedControllerLease, TrustedDeviceRegistry};

pub(super) struct ValidatedControllerLease {
    pub(super) candidate: LanTrustedControllerLease,
    pub(super) observed_at: DateTime<Utc>,
}

pub(super) fn validate_candidate(
    intent: &LanParentIntentEnvelope,
    observed_at: &str,
) -> Result<ValidatedControllerLease, LanPairingRejectionReason> {
    let observed_at = parse_timestamp(observed_at)?;
    let candidate = from_intent(intent)?;
    let candidate_issued_at = parse_timestamp(&candidate.issued_at)?;
    let candidate_expires_at = validate_shape(&candidate)?;
    if candidate_issued_at > observed_at {
        return Err(LanPairingRejectionReason::Malformed);
    }
    if candidate_expires_at <= observed_at {
        return Err(LanPairingRejectionReason::ControllerLeaseExpired);
    }
    Ok(ValidatedControllerLease {
        candidate,
        observed_at,
    })
}

pub(super) fn validate_state(registry: &TrustedDeviceRegistry) -> Result<(), ()> {
    registry
        .controller_lease
        .as_ref()
        .map(validate_shape)
        .transpose()
        .map(|_expires_at| ())
        .map_err(|_reason| ())
}

pub(super) fn clear_expired_or_invalid(
    registry: &mut TrustedDeviceRegistry,
    observed_at: DateTime<Utc>,
) -> Result<(), LanPairingRejectionReason> {
    let Some(active) = registry.controller_lease.as_ref() else {
        return Ok(());
    };
    let expires_at = match validate_shape(active) {
        Ok(expires_at) => expires_at,
        Err(reason) => {
            registry.controller_lease = None;
            return Err(reason);
        }
    };
    if expires_at <= observed_at {
        registry.controller_lease = None;
    }
    Ok(())
}

fn from_intent(
    intent: &LanParentIntentEnvelope,
) -> Result<LanTrustedControllerLease, LanPairingRejectionReason> {
    let lease = LanTrustedControllerLease {
        controller_lease_id: intent.controller_lease_id.clone(),
        controller_device_id: intent.controller_device_id.clone(),
        parent_actor_id: intent.parent_actor_id.clone(),
        issued_at: intent.controller_lease_issued_at.clone(),
        expires_at: intent.controller_lease_expires_at.clone(),
    };
    validate_shape(&lease)?;
    Ok(lease)
}

fn validate_shape(
    lease: &LanTrustedControllerLease,
) -> Result<DateTime<Utc>, LanPairingRejectionReason> {
    if lease.controller_lease_id.trim().is_empty()
        || lease.controller_device_id.trim().is_empty()
        || lease.parent_actor_id.trim().is_empty()
    {
        return Err(LanPairingRejectionReason::Malformed);
    }
    let issued_at = parse_timestamp(&lease.issued_at)?;
    let expires_at = parse_timestamp(&lease.expires_at)?;
    if issued_at >= expires_at {
        return Err(LanPairingRejectionReason::Malformed);
    }
    Ok(expires_at)
}

fn parse_timestamp(value: &str) -> Result<DateTime<Utc>, LanPairingRejectionReason> {
    DateTime::parse_from_rfc3339(value)
        .map(|value| value.with_timezone(&Utc))
        .map_err(|_error| LanPairingRejectionReason::Malformed)
}
