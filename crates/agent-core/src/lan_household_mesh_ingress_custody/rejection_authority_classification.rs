use ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason;

use super::LanHouseholdMeshIngressRejectionReason;

pub(super) fn for_authority_failure(
    reason: &LanPairingRejectionReason,
) -> LanHouseholdMeshIngressRejectionReason {
    match reason {
        LanPairingRejectionReason::Expired => LanHouseholdMeshIngressRejectionReason::Expired,
        LanPairingRejectionReason::WrongDevice => {
            LanHouseholdMeshIngressRejectionReason::IdentityMismatch
        }
        LanPairingRejectionReason::Malformed => {
            LanHouseholdMeshIngressRejectionReason::InvalidInput
        }
        _ => LanHouseholdMeshIngressRejectionReason::AuthorityStale,
    }
}
