use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReference;
use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReferenceKind;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingChallengeRequest;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingIntentKind;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingProof;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;
use ocentra_parent_agent_protocol::lan_pairing::LanParentIntentEnvelope;
use ocentra_parent_agent_protocol::lan_pairing::LanSignedChildAgentEnvelope;
use ocentra_parent_agent_protocol::lan_pairing_authority::LanPairingParentAuthority;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanHouseholdDeviceActionKind;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanHouseholdDeviceDecision;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;

use crate::lan_pairing::{controller_lease::LanControllerLeaseState, log_fields_contains_key};

pub(crate) fn is_challenge_request(fields: &LogFields) -> bool {
    log_fields_contains_key(fields, constants::field::LAN_PARENT_DEVICE_ID.into())
        || log_fields_contains_key(fields, constants::field::LAN_CHALLENGE_ID.into())
}

pub(crate) fn parse_challenge_request(
    fields: &LogFields,
) -> Result<LanPairingChallengeRequest, LanPairingRejectionReason> {
    Ok(LanPairingChallengeRequest {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        child_device_id: required_string(fields, constants::field::LAN_CHILD_DEVICE_ID.into())?,
        parent_device_id: required_string(fields, constants::field::LAN_PARENT_DEVICE_ID.into())?,
        route_id: required_string(fields, constants::field::LAN_ROUTE_ID.into())?,
        origin: required_string(fields, constants::field::ORIGIN.into())?,
        issued_at: required_string(fields, constants::field::STARTED_AT.into())?,
        expires_at: required_string(fields, constants::field::STALE_AT.into())?,
    })
}

pub(crate) fn parse_pairing_proof(
    fields: &LogFields,
) -> Result<LanPairingProof, LanPairingRejectionReason> {
    Ok(LanPairingProof {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        pairing_id: required_string(fields, constants::field::LAN_PAIRING_ID.into())?,
        challenge_id: required_string(fields, constants::field::LAN_CHALLENGE_ID.into())?,
        child_device_id: required_string(fields, constants::field::LAN_CHILD_DEVICE_ID.into())?,
        parent_device_id: required_string(fields, constants::field::LAN_PARENT_DEVICE_ID.into())?,
        route_id: required_string(fields, constants::field::LAN_ROUTE_ID.into())?,
        origin: required_string(fields, constants::field::ORIGIN.into())?,
        proof_digest: required_string(fields, constants::field::LAN_PROOF_DIGEST.into())?,
        issued_at: required_string(fields, constants::field::STARTED_AT.into())?,
        expires_at: required_string(fields, constants::field::STALE_AT.into())?,
    })
}

pub(crate) fn parse_signed_child_agent_envelope(
    fields: &LogFields,
) -> Result<LanSignedChildAgentEnvelope, LanPairingRejectionReason> {
    let envelope_json = required_string(
        fields,
        constants::field::LAN_SIGNED_CHILD_AGENT_ENVELOPE_JSON.into(),
    )?;
    serde_json::from_str::<LanSignedChildAgentEnvelope>(&envelope_json).map_err(|error| {
        drop(error);
        LanPairingRejectionReason::Malformed
    })
}

pub(crate) fn parse_intent(
    fields: &LogFields,
) -> Result<LanParentIntentEnvelope, LanPairingRejectionReason> {
    let pairing_id = required_anonymous_string(fields, constants::field::LAN_PAIRING_ID.into())?;
    let proof_digest =
        required_anonymous_string(fields, constants::field::LAN_PROOF_DIGEST.into())?;
    let issued_at = required_string(fields, constants::field::STARTED_AT.into())?;
    let expires_at = required_string(fields, constants::field::STALE_AT.into())?;
    let controller_lease_issued_at = required_controller_lease_string(
        fields,
        constants::field::LAN_CONTROLLER_LEASE_ISSUED_AT.into(),
    )?;
    let controller_lease = parse_controller_lease(fields)?;
    let evidence_references = parse_evidence_references(fields, issued_at.as_str());
    Ok(LanParentIntentEnvelope {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        intent_id: required_string(fields, constants::field::LAN_INTENT_ID.into())?,
        intent_kind: required_intent_kind(fields)?,
        target_child_device_id: required_string(
            fields,
            constants::field::LAN_CHILD_DEVICE_ID.into(),
        )?,
        route_id: required_string(fields, constants::field::LAN_ROUTE_ID.into())?,
        pairing_id,
        proof_digest,
        origin: required_string(fields, constants::field::ORIGIN.into())?,
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
    let decided_at = optional_string(fields, constants::field::STARTED_AT.into())
        .unwrap_or_else(|| observed_at.0.clone());
    Ok(LanHouseholdDeviceDecision {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        action_id: required_string(
            fields,
            constants::lan_pairing::HOUSEHOLD_ACTION_ID_FIELD.into(),
        )?,
        action_kind,
        canonical_device_id: required_string(
            fields,
            constants::field::LAN_CANONICAL_DEVICE_ID.into(),
        )?,
        child_profile_id: optional_string(
            fields,
            constants::lan_pairing::HOUSEHOLD_ACTION_CHILD_PROFILE_ID_FIELD.into(),
        ),
        display_name: optional_string(
            fields,
            constants::lan_pairing::HOUSEHOLD_ACTION_DISPLAY_NAME_FIELD.into(),
        ),
        device_kind: optional_household_device_kind(fields)?,
        parent_actor_id: required_string(fields, constants::field::LAN_PARENT_ACTOR_ID.into())?,
        decided_at,
        revoked_at: optional_string(
            fields,
            constants::lan_pairing::HOUSEHOLD_ACTION_REVOKED_AT_FIELD.into(),
        ),
    })
}

fn optional_household_device_kind(
    fields: &LogFields,
) -> Result<Option<String>, LanPairingRejectionReason> {
    let Some(device_kind) = optional_string(
        fields,
        constants::lan_pairing::HOUSEHOLD_ACTION_DEVICE_KIND_FIELD.into(),
    ) else {
        return Ok(None);
    };
    if constants::lan_pairing::HOUSEHOLD_DEVICE_KINDS.contains(&device_kind.as_str()) {
        return Ok(Some(device_kind));
    }
    Err(LanPairingRejectionReason::Malformed)
}

fn required_household_action_kind(
    fields: &LogFields,
) -> Result<LanHouseholdDeviceActionKind, LanPairingRejectionReason> {
    match required_string(
        fields,
        constants::lan_pairing::HOUSEHOLD_ACTION_KIND_FIELD.into(),
    )?
    .as_str()
    {
        constants::lan_pairing::HOUSEHOLD_ACTION_ASSIGN => Ok(LanHouseholdDeviceActionKind::Assign),
        constants::lan_pairing::HOUSEHOLD_ACTION_RENAME => Ok(LanHouseholdDeviceActionKind::Rename),
        constants::lan_pairing::HOUSEHOLD_ACTION_IGNORE => Ok(LanHouseholdDeviceActionKind::Ignore),
        constants::lan_pairing::HOUSEHOLD_ACTION_REVOKE => Ok(LanHouseholdDeviceActionKind::Revoke),
        constants::lan_pairing::HOUSEHOLD_ACTION_RESTORE => {
            Ok(LanHouseholdDeviceActionKind::Restore)
        }
        constants::lan_pairing::HOUSEHOLD_ACTION_TRUST => Ok(LanHouseholdDeviceActionKind::Trust),
        _ => Err(LanPairingRejectionReason::Malformed),
    }
}

fn parse_controller_lease(
    fields: &LogFields,
) -> Result<LanControllerLeaseState, LanPairingRejectionReason> {
    Ok(LanControllerLeaseState {
        controller_lease_id: required_controller_lease_string(
            fields,
            constants::field::LAN_CONTROLLER_LEASE_ID.into(),
        )?,
        controller_device_id: required_controller_lease_string(
            fields,
            constants::field::LAN_CONTROLLER_DEVICE_ID.into(),
        )?,
        parent_actor_id: required_controller_lease_string(
            fields,
            constants::field::LAN_PARENT_ACTOR_ID.into(),
        )?,
        expires_at: required_controller_lease_string(
            fields,
            constants::field::LAN_CONTROLLER_LEASE_EXPIRES_AT.into(),
        )?,
    })
}

fn parse_evidence_references(
    fields: &LogFields,
    observed_at: impl Into<LanPairingText>,
) -> Vec<ParentEvidenceReference> {
    let observed_at = observed_at.into();
    match fields.get(constants::field::LAN_EVIDENCE_REFERENCE_IDS) {
        Some(LogFieldValue::String(value)) => value
            .split(constants::delimiter::LIST)
            .filter(|evidence_id| !evidence_id.is_empty())
            .map(|evidence_id| ParentEvidenceReference {
                evidence_reference_id: evidence_id.to_string(),
                kind: ParentEvidenceReferenceKind::ActivityEvent,
                observed_at: observed_at.0.clone(),
            })
            .collect(),
        _ => Vec::new(),
    }
}

fn required_intent_kind(
    fields: &LogFields,
) -> Result<LanPairingIntentKind, LanPairingRejectionReason> {
    match required_string(fields, constants::field::LAN_INTENT_KIND.into())?.as_str() {
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
    match required_string(fields, constants::field::LAN_PARENT_AUTHORITY.into())?.as_str() {
        constants::value::LAN_PARENT_AUTHORITY_ACTIVE_CONTROLLER => {
            Ok(LanPairingParentAuthority::ActiveController)
        }
        constants::value::LAN_PARENT_AUTHORITY_OBSERVER => Ok(LanPairingParentAuthority::Observer),
        _ => Err(LanPairingRejectionReason::Malformed),
    }
}

fn required_anonymous_string(
    fields: &LogFields,
    field_name: LanPairingText,
) -> Result<String, LanPairingRejectionReason> {
    required_string(fields, field_name).map_err(|reason| {
        let _ = reason;
        LanPairingRejectionReason::Anonymous
    })
}

fn required_controller_lease_string(
    fields: &LogFields,
    field_name: LanPairingText,
) -> Result<String, LanPairingRejectionReason> {
    required_string(fields, field_name).map_err(|reason| {
        let _ = reason;
        LanPairingRejectionReason::ControllerLeaseMissing
    })
}

fn optional_string(fields: &LogFields, field_name: LanPairingText) -> Option<String> {
    match fields.get(field_name.0.as_str()) {
        Some(LogFieldValue::String(value)) if !value.is_empty() => Some(value.clone()),
        _ => None,
    }
}

fn required_string(
    fields: &LogFields,
    field_name: LanPairingText,
) -> Result<String, LanPairingRejectionReason> {
    match fields.get(field_name.0.as_str()) {
        Some(LogFieldValue::String(value)) if !value.is_empty() => Ok(value.clone()),
        _ => Err(LanPairingRejectionReason::Malformed),
    }
}
