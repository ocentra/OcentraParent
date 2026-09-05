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
            readiness: readiness_from_state(
                &removal,
                self.recovery_pending.as_deref(),
                self.trust_binding.as_ref(),
                self.paths.trust_binding_source(),
            ),
            storage_custody: self.storage_custody.readiness(),
            domain_flow_count: self.domain_flows.len(),
            durable_root: self.paths.root().to_owned(),
            removal,
        })
    }

    pub fn readiness(&self) -> Result<ChildAgentReadiness, ChildAgentServiceError> {
        let removal = self
            .removal
            .status()
            .map_err(ChildAgentServiceError::Storage)?;
        Ok(readiness_from_state(
            &removal,
            self.recovery_pending.as_deref(),
            self.trust_binding.as_ref(),
            self.paths.trust_binding_source(),
        ))
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
    expected_binding: Option<
        &ocentra_family_identity_core::device_trust_current_binding::CurrentChildDeviceTrustBinding,
    >,
    source: Option<&dyn super::trust_binding::ChildAgentTrustBindingSource>,
) -> ChildAgentReadiness {
    if removal.trust_state == ChildAgentTrustState::Revoked {
        ChildAgentReadiness::Revoked {
            audit_ref: removal.latest_audit_ref.clone(),
        }
    } else if !trust_binding_is_current(expected_binding, source) {
        ChildAgentReadiness::TrustBindingManualRequired
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

fn trust_binding_is_current(
    expected_binding: Option<
        &ocentra_family_identity_core::device_trust_current_binding::CurrentChildDeviceTrustBinding,
    >,
    source: Option<&dyn super::trust_binding::ChildAgentTrustBindingSource>,
) -> bool {
    let (Some(expected_binding), Some(source)) = (expected_binding, source) else {
        return false;
    };
    let Ok(current_binding) = source.current_trust_binding() else {
        return false;
    };
    current_binding.state()
        == ocentra_family_identity_core::device_trust_lifecycle::DeviceTrustLifecycleState::Trusted
        && &current_binding == expected_binding
}
