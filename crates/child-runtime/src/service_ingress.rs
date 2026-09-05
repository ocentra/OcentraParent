use ocentra_eventing::envelope::EventMetadata;
use ocentra_parent_agent_protocol::child_domain_runtime::ChildDomainObservedEvent;
use ocentra_storage_custody_core::storage_custody::StorageCustodyExecutionRequest;
use tokio::sync::{mpsc, oneshot};

use super::{
    ChildAgentCommand, ChildAgentCommandResult, ChildAgentIngress, ChildAgentIngressError,
    ChildAgentServiceError, QueuedCommand,
};

impl ChildAgentIngress {
    pub(crate) async fn submit(
        &self,
        command: ChildAgentCommand,
    ) -> Result<
        crate::child_domain_runtime_flow::ChildDomainRuntimeFlowReport,
        ChildAgentIngressError,
    > {
        match self.send(command).await? {
            ChildAgentCommandResult::Domain(report) => Ok(*report),
            ChildAgentCommandResult::StorageCustody(_) => Err(ChildAgentIngressError::Service(
                Box::new(ChildAgentServiceError::Configuration(
                    "child service returned an unexpected custody response".to_owned(),
                )),
            )),
        }
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
        request: StorageCustodyExecutionRequest,
        metadata: EventMetadata,
    ) -> Result<super::storage_custody_runtime::ChildStorageCustodyOutcome, ChildAgentIngressError>
    {
        match self
            .send(ChildAgentCommand::PublishStorageCustody { request, metadata })
            .await?
        {
            ChildAgentCommandResult::StorageCustody(outcome) => Ok(outcome),
            ChildAgentCommandResult::Domain(_) => Err(ChildAgentIngressError::Service(Box::new(
                ChildAgentServiceError::Configuration(
                    "child service returned an unexpected domain response".to_owned(),
                ),
            ))),
        }
    }

    async fn send(
        &self,
        command: ChildAgentCommand,
    ) -> Result<ChildAgentCommandResult, ChildAgentIngressError> {
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
            .map_err(|_error| ChildAgentIngressError::ServiceClosed)?
            .map_err(|error: ChildAgentServiceError| {
                ChildAgentIngressError::Service(Box::new(error))
            })
    }
}
