use ocentra_lan_core::lan_pairing::LanSignedChildAgentVerificationError;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason;

pub(crate) fn signed_child_agent_rejection_reason(
    reason: &LanSignedChildAgentVerificationError,
) -> LanPairingRejectionReason {
    match reason {
        LanSignedChildAgentVerificationError::Replayed => LanPairingRejectionReason::Replayed,
        LanSignedChildAgentVerificationError::Expired
        | LanSignedChildAgentVerificationError::FutureIssuedAt => {
            LanPairingRejectionReason::Expired
        }
        LanSignedChildAgentVerificationError::WrongRoute => {
            LanPairingRejectionReason::UnsupportedRoute
        }
        LanSignedChildAgentVerificationError::WrongFamily
        | LanSignedChildAgentVerificationError::WrongParentDevice
        | LanSignedChildAgentVerificationError::WrongChildDevice => {
            LanPairingRejectionReason::WrongDevice
        }
        LanSignedChildAgentVerificationError::UnsupportedSchemaVersion
        | LanSignedChildAgentVerificationError::EmptyRequiredField
        | LanSignedChildAgentVerificationError::InvalidMetadata
        | LanSignedChildAgentVerificationError::MalformedTimestamp
        | LanSignedChildAgentVerificationError::UnsupportedAlgorithm
        | LanSignedChildAgentVerificationError::InvalidPublicKey
        | LanSignedChildAgentVerificationError::PublicKeyIdMismatch
        | LanSignedChildAgentVerificationError::InvalidSignature
        | LanSignedChildAgentVerificationError::SignatureRejected
        | LanSignedChildAgentVerificationError::SerializationFailed => {
            LanPairingRejectionReason::Malformed
        }
    }
}
