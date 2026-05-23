use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, LanPairingIntentKind, LanPairingProof,
    LanPairingRejectionReason, LanParentIntentEnvelope, LogFieldValue, LogFields,
};

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
    command: &AgentCommandEnvelope,
) -> Result<LanParentIntentEnvelope, LanPairingRejectionReason> {
    let pairing_id = required_anonymous_string(fields, constants::field::LAN_PAIRING_ID)?;
    let proof_digest = required_anonymous_string(fields, constants::field::LAN_PROOF_DIGEST)?;
    Ok(LanParentIntentEnvelope {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        intent_id: required_string(fields, constants::field::LAN_INTENT_ID)?,
        intent_kind: LanPairingIntentKind::HealthQuery,
        target_child_device_id: command.target.device_id.clone(),
        route_id: required_string(fields, constants::field::LAN_ROUTE_ID)?,
        pairing_id,
        proof_digest,
        origin: required_string(fields, constants::field::ORIGIN)?,
        issued_at: required_string(fields, constants::field::STARTED_AT)?,
        expires_at: required_string(fields, constants::field::STALE_AT)?,
    })
}

fn required_anonymous_string(
    fields: &LogFields,
    key: &str,
) -> Result<String, LanPairingRejectionReason> {
    required_string(fields, key).map_err(|_| LanPairingRejectionReason::Anonymous)
}

fn required_string(fields: &LogFields, key: &str) -> Result<String, LanPairingRejectionReason> {
    match fields.get(key) {
        Some(LogFieldValue::String(value)) if !value.is_empty() => Ok(value.clone()),
        _ => Err(LanPairingRejectionReason::Malformed),
    }
}
