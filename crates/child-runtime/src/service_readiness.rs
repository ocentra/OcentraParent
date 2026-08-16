use ocentra_eventing::ids::CorrelationId;

use super::{
    ChildAgentHealth, ChildAgentReadiness, ChildAgentRemovalStatus, ChildAgentService,
    ChildAgentServiceError, ChildAgentTrustState,
};

impl ChildAgentService {
    pub fn health(&self) -> Result<ChildAgentHealth, ChildAgentServiceError> {
        let removal = self
            .removal
            .status()
            .map_err(ChildAgentServiceError::Storage)?;
        Ok(ChildAgentHealth {
            readiness: readiness_from_state(&removal, self.recovery_pending.as_deref()),
            domain_flow_count: self.domain_flows.len(),
            durable_root: self.paths.root().to_owned(),
            removal,
        })
    }

    pub fn readiness(&self) -> &ChildAgentReadiness {
        &self.readiness
    }

    pub fn ingress(&self) -> super::ChildAgentIngress {
        self.ingress.clone()
    }

    pub fn domain_flow_count(&self) -> usize {
        self.domain_flows.len()
    }
}

pub(super) fn readiness_from_state(
    removal: &ChildAgentRemovalStatus,
    recovery_pending: Option<&[CorrelationId]>,
) -> ChildAgentReadiness {
    if removal.trust_state == ChildAgentTrustState::Revoked {
        ChildAgentReadiness::Revoked {
            audit_ref: removal.latest_audit_ref.clone(),
        }
    } else if removal.latest_tamper_signal_ref.is_some() {
        ChildAgentReadiness::TamperManualRequired {
            signal_ref: removal.latest_tamper_signal_ref.clone(),
        }
    } else if let Some(correlation_ids) = recovery_pending {
        ChildAgentReadiness::RecoveryPending {
            correlation_ids: correlation_ids.to_vec(),
        }
    } else {
        ChildAgentReadiness::Ready
    }
}
