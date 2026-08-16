use ocentra_parent_agent_protocol::child_domain_runtime::ChildDomainObservedEvent;
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
}
