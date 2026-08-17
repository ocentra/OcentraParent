use ocentra_eventing::envelope::EventMetadata;
use ocentra_family_identity_core::household_authority_proof::VerifiedHouseholdAuthority;
use ocentra_parent_agent_protocol::child_domain_runtime::ChildDomainObservedEvent;
use ocentra_storage_custody_core::storage_custody::StorageCustodyInput;
use tokio::sync::{mpsc, oneshot};

use super::{
    ChildAgentCommand, ChildAgentIngress, ChildAgentIngressError, ChildAgentServiceError,
    QueuedCommand,
};

impl ChildAgentIngress {
    pub async fn submit(
        &self,
        command: ChildAgentCommand,
    ) -> Result<
        crate::child_domain_runtime_flow::ChildDomainRuntimeFlowReport,
        ChildAgentIngressError,
    > {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .try_send(QueuedCommand {
                command,
                response: response_sender,
            })
            .map_err(|error| match error {
                mpsc::error::TrySendError::Full(_) => ChildAgentIngressError::QueueFull,
                mpsc::error::TrySendError::Closed(_) => ChildAgentIngressError::ServiceClosed,
            })?;
        response_receiver
            .await
            .map_err(|_| ChildAgentIngressError::ServiceClosed)?
            .map_err(|error: ChildAgentServiceError| {
                ChildAgentIngressError::Service(Box::new(error))
            })
            .and_then(|result| match result {
                super::ChildAgentCommandResult::Domain(report) => Ok(report),
                super::ChildAgentCommandResult::StorageCustody(_) => {
                    Err(ChildAgentIngressError::Service(Box::new(
                        ChildAgentServiceError::Configuration(
                            "child service returned an unexpected custody response".to_owned(),
                        ),
                    )))
                }
            })
    }

    pub async fn submit_observed_event(
        &self,
        event: ChildDomainObservedEvent,
    ) -> Result<
        crate::child_domain_runtime_flow::ChildDomainRuntimeFlowReport,
        ChildAgentIngressError,
    > {
        self.submit(ChildAgentCommand::Observe(event)).await
    }

    pub async fn submit_storage_custody_action(
        &self,
        authority: VerifiedHouseholdAuthority,
        input: StorageCustodyInput,
        metadata: EventMetadata,
    ) -> Result<
        crate::runtime_gate_tombstone::ChildRuntimeTombstonePublicationOutcome,
        ChildAgentIngressError,
    > {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .try_send(QueuedCommand {
                command: ChildAgentCommand::PublishStorageCustody {
                    authority,
                    input,
                    metadata,
                },
                response: response_sender,
            })
            .map_err(|error| match error {
                mpsc::error::TrySendError::Full(_) => ChildAgentIngressError::QueueFull,
                mpsc::error::TrySendError::Closed(_) => ChildAgentIngressError::ServiceClosed,
            })?;
        match response_receiver
            .await
            .map_err(|_| ChildAgentIngressError::ServiceClosed)?
            .map_err(|error| ChildAgentIngressError::Service(Box::new(error)))?
        {
            super::ChildAgentCommandResult::StorageCustody(outcome) => Ok(outcome),
            super::ChildAgentCommandResult::Domain(_) => Err(ChildAgentIngressError::Service(
                Box::new(ChildAgentServiceError::Configuration(
                    "child service returned an unexpected domain response".to_owned(),
                )),
            )),
        }
    }
}
