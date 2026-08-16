use super::{
    service_readiness::readiness_from_state, ChildAgentCommand, ChildAgentReadiness,
    ChildAgentService, ChildAgentServiceError,
};

impl ChildAgentService {
    pub(super) async fn dispatch(
        &self,
        command: ChildAgentCommand,
    ) -> Result<
        crate::child_domain_runtime_flow::ChildDomainRuntimeFlowReport,
        ChildAgentServiceError,
    > {
        let removal = self
            .removal
            .status()
            .map_err(ChildAgentServiceError::Storage)?;
        let readiness = readiness_from_state(&removal, self.recovery_pending.as_deref());
        validate_readiness(&readiness)?;
        match command {
            ChildAgentCommand::Observe(event) => {
                let flow = self
                    .domain_flows
                    .iter()
                    .find(|flow| flow.domain() == event.domain)
                    .ok_or(ChildAgentServiceError::UnknownDomain(event.domain))?;
                flow.publish_observed(event).await.map_err(Into::into)
            }
        }
    }
}

fn validate_readiness(readiness: &ChildAgentReadiness) -> Result<(), ChildAgentServiceError> {
    match readiness {
        ChildAgentReadiness::Ready => Ok(()),
        ChildAgentReadiness::RecoveryPending { .. } => Err(
            ChildAgentServiceError::RecoveryPending(Box::new(readiness.clone())),
        ),
        ChildAgentReadiness::TamperManualRequired { signal_ref } => {
            Err(ChildAgentServiceError::TamperManualRequired {
                signal_ref: signal_ref.clone(),
            })
        }
        ChildAgentReadiness::Revoked { audit_ref } => Err(ChildAgentServiceError::TrustRevoked {
            audit_ref: audit_ref.clone(),
        }),
    }
}
