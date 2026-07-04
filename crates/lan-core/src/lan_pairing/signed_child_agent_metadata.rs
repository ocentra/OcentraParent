#![forbid(unsafe_code)]

use ocentra_parent_agent_protocol::lan_pairing::LanSignedChildAgentClaim;

use super::LanSignedChildAgentVerificationError;

pub(super) fn validate_signed_child_agent_metadata(
    claim: &LanSignedChildAgentClaim,
) -> Result<(), LanSignedChildAgentVerificationError> {
    if signed_child_agent_claim_metadata_is_valid(claim) {
        Ok(())
    } else {
        Err(LanSignedChildAgentVerificationError::InvalidMetadata)
    }
}

fn signed_child_agent_claim_metadata_is_valid(claim: &LanSignedChildAgentClaim) -> bool {
    signed_child_agent_required_atoms_are_valid(claim)
        && signed_child_agent_optional_atoms_are_valid(claim)
}

fn signed_child_agent_required_atoms_are_valid(claim: &LanSignedChildAgentClaim) -> bool {
    [
        claim.child_device_id.as_str(),
        claim.parent_device_id.as_str(),
        claim.install_id.as_str(),
        claim.family_hash.as_str(),
        claim.route_id.as_str(),
        claim.nonce.as_str(),
        claim.platform.as_str(),
        claim.hostname.as_str(),
        claim.agent_version.as_str(),
    ]
    .into_iter()
    .all(signed_child_agent_atom)
}

fn signed_child_agent_optional_atoms_are_valid(claim: &LanSignedChildAgentClaim) -> bool {
    claim
        .child_profile_hash
        .as_deref()
        .is_none_or(signed_child_agent_atom)
        && claim
            .local_ips
            .iter()
            .all(|value| signed_child_agent_atom(value))
        && claim
            .mac_addresses
            .iter()
            .all(|value| signed_child_agent_atom(value))
        && claim
            .capabilities
            .iter()
            .all(|value| signed_child_agent_atom(value))
}

fn signed_child_agent_atom(value: &str) -> bool {
    value.chars().all(|character| {
        character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.' | ':')
    })
}
