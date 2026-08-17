use ocentra_child_runtime::service::{ChildAgentReadiness, ChildAgentServiceError};

pub(super) fn readiness_failure(readiness: &ChildAgentReadiness) -> Option<String> {
    match readiness {
        ChildAgentReadiness::Ready => None,
        ChildAgentReadiness::RecoveryPending { .. } => {
            Some(ChildAgentServiceError::RecoveryPending(Box::new(readiness.clone())).to_string())
        }
        ChildAgentReadiness::TrustBindingManualRequired => {
            Some(ChildAgentServiceError::TrustBindingManualRequired.to_string())
        }
        ChildAgentReadiness::TamperManualRequired { signal_ref } => Some(
            ChildAgentServiceError::TamperManualRequired {
                signal_ref: signal_ref.clone(),
            }
            .to_string(),
        ),
        ChildAgentReadiness::Revoked { audit_ref } => Some(
            ChildAgentServiceError::TrustRevoked {
                audit_ref: audit_ref.clone(),
            }
            .to_string(),
        ),
    }
}
