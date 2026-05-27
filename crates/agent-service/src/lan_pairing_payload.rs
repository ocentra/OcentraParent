use ocentra_parent_agent_protocol::{
    constants, LanPairingChallengeRequest, LanPairingIntentKind, LanPairingParentAuthority,
    LanPairingProof, LanPairingRejectionReason, LanParentIntentEnvelope, LogFieldValue, LogFields,
    ParentEvidenceReference, ParentEvidenceReferenceKind,
};

use crate::lan_pairing::controller_lease::LanControllerLeaseState;

pub(crate) fn is_challenge_request(fields: &LogFields) -> bool {
    fields.contains_key(constants::field::LAN_PARENT_DEVICE_ID)
        || fields.contains_key(constants::field::LAN_CHALLENGE_ID)
}

pub(crate) fn parse_challenge_request(
    fields: &LogFields,
) -> Result<LanPairingChallengeRequest, LanPairingRejectionReason> {
    Ok(LanPairingChallengeRequest {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        child_device_id: required_string(fields, constants::field::LAN_CHILD_DEVICE_ID)?,
        parent_device_id: required_string(fields, constants::field::LAN_PARENT_DEVICE_ID)?,
        route_id: required_string(fields, constants::field::LAN_ROUTE_ID)?,
        origin: required_string(fields, constants::field::ORIGIN)?,
        issued_at: required_string(fields, constants::field::STARTED_AT)?,
        expires_at: required_string(fields, constants::field::STALE_AT)?,
    })
}

pub(crate) fn parse_pairing_proof(
    fields: &LogFields,
) -> Result<LanPairingProof, LanPairingRejectionReason> {
    Ok(LanPairingProof {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        pairing_id: required_string(fields, constants::field::LAN_PAIRING_ID)?,
        challenge_id: required_string(fields, constants::field::LAN_CHALLENGE_ID)?,
        child_device_id: required_string(fields, constants::field::LAN_CHILD_DEVICE_ID)?,
        parent_device_id: required_string(fields, constants::field::LAN_PARENT_DEVICE_ID)?,
        route_id: required_string(fields, constants::field::LAN_ROUTE_ID)?,
        origin: required_string(fields, constants::field::ORIGIN)?,
        proof_digest: required_string(fields, constants::field::LAN_PROOF_DIGEST)?,
        issued_at: required_string(fields, constants::field::STARTED_AT)?,
        expires_at: required_string(fields, constants::field::STALE_AT)?,
    })
}

pub(crate) fn parse_intent(
    fields: &LogFields,
) -> Result<LanParentIntentEnvelope, LanPairingRejectionReason> {
    let pairing_id = required_anonymous_string(fields, constants::field::LAN_PAIRING_ID)?;
    let proof_digest = required_anonymous_string(fields, constants::field::LAN_PROOF_DIGEST)?;
    let issued_at = required_string(fields, constants::field::STARTED_AT)?;
    let expires_at = required_string(fields, constants::field::STALE_AT)?;
    let controller_lease_issued_at =
        required_controller_lease_string(fields, constants::field::LAN_CONTROLLER_LEASE_ISSUED_AT)?;
    let controller_lease = parse_controller_lease(fields)?;
    let evidence_references = parse_evidence_references(fields, &issued_at);
    Ok(LanParentIntentEnvelope {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        intent_id: required_string(fields, constants::field::LAN_INTENT_ID)?,
        intent_kind: required_intent_kind(fields)?,
        target_child_device_id: required_string(fields, constants::field::LAN_CHILD_DEVICE_ID)?,
        route_id: required_string(fields, constants::field::LAN_ROUTE_ID)?,
        pairing_id,
        proof_digest,
        origin: required_string(fields, constants::field::ORIGIN)?,
        issued_at,
        expires_at,
        controller_lease_id: controller_lease.controller_lease_id,
        controller_device_id: controller_lease.controller_device_id,
        parent_actor_id: controller_lease.parent_actor_id,
        parent_authority: required_parent_authority(fields)?,
        controller_lease_issued_at,
        controller_lease_expires_at: controller_lease.expires_at,
        evidence_references,
    })
}

fn parse_controller_lease(
    fields: &LogFields,
) -> Result<LanControllerLeaseState, LanPairingRejectionReason> {
    Ok(LanControllerLeaseState {
        controller_lease_id: required_controller_lease_string(
            fields,
            constants::field::LAN_CONTROLLER_LEASE_ID,
        )?,
        controller_device_id: required_controller_lease_string(
            fields,
            constants::field::LAN_CONTROLLER_DEVICE_ID,
        )?,
        parent_actor_id: required_controller_lease_string(
            fields,
            constants::field::LAN_PARENT_ACTOR_ID,
        )?,
        expires_at: required_controller_lease_string(
            fields,
            constants::field::LAN_CONTROLLER_LEASE_EXPIRES_AT,
        )?,
    })
}

fn parse_evidence_references(
    fields: &LogFields,
    observed_at: &str,
) -> Vec<ParentEvidenceReference> {
    match fields.get(constants::field::LAN_EVIDENCE_REFERENCE_IDS) {
        Some(LogFieldValue::String(value)) => value
            .split(constants::delimiter::LIST)
            .filter(|evidence_id| !evidence_id.is_empty())
            .map(|evidence_id| ParentEvidenceReference {
                evidence_reference_id: evidence_id.to_string(),
                kind: ParentEvidenceReferenceKind::ActivityEvent,
                observed_at: observed_at.to_string(),
            })
            .collect(),
        _ => Vec::new(),
    }
}

fn required_intent_kind(
    fields: &LogFields,
) -> Result<LanPairingIntentKind, LanPairingRejectionReason> {
    match required_string(fields, constants::field::LAN_INTENT_KIND)?.as_str() {
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

fn required_parent_authority(
    fields: &LogFields,
) -> Result<LanPairingParentAuthority, LanPairingRejectionReason> {
    match required_string(fields, constants::field::LAN_PARENT_AUTHORITY)?.as_str() {
        constants::value::LAN_PARENT_AUTHORITY_ACTIVE_CONTROLLER => {
            Ok(LanPairingParentAuthority::ActiveController)
        }
        constants::value::LAN_PARENT_AUTHORITY_OBSERVER => Ok(LanPairingParentAuthority::Observer),
        _ => Err(LanPairingRejectionReason::Malformed),
    }
}

fn required_anonymous_string(
    fields: &LogFields,
    key: &str,
) -> Result<String, LanPairingRejectionReason> {
    required_string(fields, key).map_err(|_| LanPairingRejectionReason::Anonymous)
}

fn required_controller_lease_string(
    fields: &LogFields,
    key: &str,
) -> Result<String, LanPairingRejectionReason> {
    required_string(fields, key).map_err(|_| LanPairingRejectionReason::ControllerLeaseMissing)
}

fn required_string(fields: &LogFields, key: &str) -> Result<String, LanPairingRejectionReason> {
    match fields.get(key) {
        Some(LogFieldValue::String(value)) if !value.is_empty() => Ok(value.clone()),
        _ => Err(LanPairingRejectionReason::Malformed),
    }
}
