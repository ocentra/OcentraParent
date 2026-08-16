#[path = "lan_pairing_payload/field_values.rs"]
mod field_values;
#[path = "lan_pairing_payload/intent_values.rs"]
mod intent_values;
#[path = "lan_pairing_payload/kind_values.rs"]
mod kind_values;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingChallengeRequest;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingProof;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;
use ocentra_parent_agent_protocol::lan_pairing::LanParentIntentEnvelope;
use ocentra_parent_agent_protocol::lan_pairing::LanSignedChildAgentEnvelope;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanHouseholdDeviceDecision;
use ocentra_parent_agent_protocol::logging::LogFields;

use crate::lan_pairing::{controller_lease::LanControllerLeaseState, log_fields_contains_key};

use self::field_values::{
    optional_household_device_kind, optional_payload_text, parse_evidence_references,
    required_anonymous_payload_text, required_controller_lease_payload_text, required_payload_text,
};
use self::intent_values::required_intent_kind;
use self::kind_values::{required_household_action_kind, required_parent_authority};

pub(crate) fn is_challenge_request(fields: &LogFields) -> bool {
    log_fields_contains_key(fields, constants::field::LAN_PARENT_DEVICE_ID.into())
        || log_fields_contains_key(fields, constants::field::LAN_CHALLENGE_ID.into())
}

pub(crate) fn parse_challenge_request(
    fields: &LogFields,
) -> Result<LanPairingChallengeRequest, LanPairingRejectionReason> {
    Ok(LanPairingChallengeRequest {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        child_device_id: required_payload_text(
            fields,
            constants::field::LAN_CHILD_DEVICE_ID.into(),
        )?
        .0,
        parent_device_id: required_payload_text(
            fields,
            constants::field::LAN_PARENT_DEVICE_ID.into(),
        )?
        .0,
        route_id: required_payload_text(fields, constants::field::LAN_ROUTE_ID.into())?.0,
        origin: required_payload_text(fields, constants::field::ORIGIN.into())?.0,
        issued_at: required_payload_text(fields, constants::field::STARTED_AT.into())?.0,
        expires_at: required_payload_text(fields, constants::field::STALE_AT.into())?.0,
    })
}

pub(crate) fn parse_pairing_proof(
    fields: &LogFields,
) -> Result<LanPairingProof, LanPairingRejectionReason> {
    Ok(LanPairingProof {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        pairing_id: required_payload_text(fields, constants::field::LAN_PAIRING_ID.into())?.0,
        challenge_id: required_payload_text(fields, constants::field::LAN_CHALLENGE_ID.into())?.0,
        child_device_id: required_payload_text(
            fields,
            constants::field::LAN_CHILD_DEVICE_ID.into(),
        )?
        .0,
        parent_device_id: required_payload_text(
            fields,
            constants::field::LAN_PARENT_DEVICE_ID.into(),
        )?
        .0,
        route_id: required_payload_text(fields, constants::field::LAN_ROUTE_ID.into())?.0,
        origin: required_payload_text(fields, constants::field::ORIGIN.into())?.0,
        proof_digest: required_payload_text(fields, constants::field::LAN_PROOF_DIGEST.into())?.0,
        issued_at: required_payload_text(fields, constants::field::STARTED_AT.into())?.0,
        expires_at: required_payload_text(fields, constants::field::STALE_AT.into())?.0,
    })
}

pub(crate) fn parse_signed_child_agent_envelope(
    fields: &LogFields,
) -> Result<LanSignedChildAgentEnvelope, LanPairingRejectionReason> {
    let envelope_json = required_payload_text(
        fields,
        constants::field::LAN_SIGNED_CHILD_AGENT_ENVELOPE_JSON.into(),
    )?;
    serde_json::from_str::<LanSignedChildAgentEnvelope>(envelope_json.as_payload_text_ref().0)
        .map_err(|error| {
            drop(error);
            LanPairingRejectionReason::Malformed
        })
}

pub(crate) fn parse_intent(
    fields: &LogFields,
) -> Result<LanParentIntentEnvelope, LanPairingRejectionReason> {
    let pairing_id =
        required_anonymous_payload_text(fields, constants::field::LAN_PAIRING_ID.into())?;
    let proof_digest =
        required_anonymous_payload_text(fields, constants::field::LAN_PROOF_DIGEST.into())?;
    let issued_at = required_payload_text(fields, constants::field::STARTED_AT.into())?;
    let expires_at = required_payload_text(fields, constants::field::STALE_AT.into())?;
    let controller_lease_issued_at = required_controller_lease_payload_text(
        fields,
        constants::field::LAN_CONTROLLER_LEASE_ISSUED_AT.into(),
    )?;
    let controller_lease = parse_controller_lease(fields)?;
    let evidence_references = parse_evidence_references(fields, issued_at.as_payload_text_ref());
    Ok(LanParentIntentEnvelope {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        intent_id: required_payload_text(fields, constants::field::LAN_INTENT_ID.into())?.0,
        intent_kind: required_intent_kind(fields)?,
        target_child_device_id: required_payload_text(
            fields,
            constants::field::LAN_CHILD_DEVICE_ID.into(),
        )?
        .0,
        route_id: required_payload_text(fields, constants::field::LAN_ROUTE_ID.into())?.0,
        pairing_id: pairing_id.0,
        proof_digest: proof_digest.0,
        origin: required_payload_text(fields, constants::field::ORIGIN.into())?.0,
        issued_at: issued_at.0,
        expires_at: expires_at.0,
        controller_lease_id: controller_lease.controller_lease_id,
        controller_device_id: controller_lease.controller_device_id,
        parent_actor_id: controller_lease.parent_actor_id,
        parent_authority: required_parent_authority(fields)?,
        controller_lease_issued_at: controller_lease_issued_at.0,
        controller_lease_expires_at: controller_lease.expires_at,
        evidence_references,
    })
}

pub(crate) fn parse_household_device_decision(
    fields: &LogFields,
    observed_at: impl Into<LanPairingText>,
) -> Option<Result<LanHouseholdDeviceDecision, LanPairingRejectionReason>> {
    if !log_fields_contains_key(
        fields,
        constants::lan_pairing::HOUSEHOLD_ACTION_KIND_FIELD.into(),
    ) {
        return None;
    }
    Some(parse_household_device_decision_fields(fields, observed_at))
}

fn parse_household_device_decision_fields(
    fields: &LogFields,
    observed_at: impl Into<LanPairingText>,
) -> Result<LanHouseholdDeviceDecision, LanPairingRejectionReason> {
    let observed_at = observed_at.into();
    let action_kind = required_household_action_kind(fields)?;
    let decided_at = optional_payload_text(fields, constants::field::STARTED_AT.into())
        .map(|value| value.0)
        .unwrap_or_else(|| observed_at.0.clone());
    Ok(LanHouseholdDeviceDecision {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        action_id: required_payload_text(
            fields,
            constants::lan_pairing::HOUSEHOLD_ACTION_ID_FIELD.into(),
        )?
        .0,
        action_kind,
        canonical_device_id: required_payload_text(
            fields,
            constants::field::LAN_CANONICAL_DEVICE_ID.into(),
        )?
        .0,
        child_profile_id: optional_payload_text(
            fields,
            constants::lan_pairing::HOUSEHOLD_ACTION_CHILD_PROFILE_ID_FIELD.into(),
        )
        .map(|value| value.0),
        display_name: optional_payload_text(
            fields,
            constants::lan_pairing::HOUSEHOLD_ACTION_DISPLAY_NAME_FIELD.into(),
        )
        .map(|value| value.0),
        device_kind: optional_household_device_kind(fields)?.map(|value| value.0),
        parent_actor_id: required_payload_text(
            fields,
            constants::field::LAN_PARENT_ACTOR_ID.into(),
        )?
        .0,
        decided_at,
        revoked_at: optional_payload_text(
            fields,
            constants::lan_pairing::HOUSEHOLD_ACTION_REVOKED_AT_FIELD.into(),
        )
        .map(|value| value.0),
    })
}

fn parse_controller_lease(
    fields: &LogFields,
) -> Result<LanControllerLeaseState, LanPairingRejectionReason> {
    Ok(LanControllerLeaseState {
        controller_lease_id: required_controller_lease_payload_text(
            fields,
            constants::field::LAN_CONTROLLER_LEASE_ID.into(),
        )?
        .0,
        controller_device_id: required_controller_lease_payload_text(
            fields,
            constants::field::LAN_CONTROLLER_DEVICE_ID.into(),
        )?
        .0,
        parent_actor_id: required_controller_lease_payload_text(
            fields,
            constants::field::LAN_PARENT_ACTOR_ID.into(),
        )?
        .0,
        expires_at: required_controller_lease_payload_text(
            fields,
            constants::field::LAN_CONTROLLER_LEASE_EXPIRES_AT.into(),
        )?
        .0,
    })
}
