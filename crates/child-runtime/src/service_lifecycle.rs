use super::{
    ChildAgentRemovalStatus, ChildAgentService, ChildAgentServiceError, ChildAgentTamperSignalKind,
    VerifiedParentRemovalAuthorization,
};

impl ChildAgentService {
    pub fn removal(&self) -> &super::ChildAgentRemovalBoundary {
        &self.removal
    }

    pub fn revoke_with_parent_authorization(
        &mut self,
        authorization: VerifiedParentRemovalAuthorization,
    ) -> Result<ChildAgentRemovalStatus, ChildAgentServiceError> {
        let status = self
            .removal
            .revoke_with_parent_authorization(authorization)
            .map_err(ChildAgentServiceError::Storage)?;
        Ok(status)
    }

    pub fn reauthorize_with_parent_authorization(
        &mut self,
        authorization: VerifiedParentRemovalAuthorization,
    ) -> Result<ChildAgentRemovalStatus, ChildAgentServiceError> {
        let status = self
            .removal
            .reauthorize_with_parent_authorization(authorization)
            .map_err(ChildAgentServiceError::Storage)?;
        Ok(status)
    }

    /// Records local tamper evidence and blocks command dispatch until a
    /// parent/operator resolves it. The signal is evidence only; it cannot
    /// revoke or reauthorize trust without the verified parent boundary.
    pub fn record_tamper_signal(
        &mut self,
        signal_ref: impl Into<String>,
        kind: ChildAgentTamperSignalKind,
    ) -> Result<ChildAgentRemovalStatus, ChildAgentServiceError> {
        let status = self
            .removal
            .record_tamper_signal(signal_ref, kind)
            .map_err(ChildAgentServiceError::Storage)?;
        Ok(status)
    }

    pub async fn run_until_shutdown(self) -> Result<(), ChildAgentServiceError> {
        Box::pin(super::service_supervision::run_until_shutdown(self)).await
    }
}

pub async fn run_child_agent_service() -> Result<(), ChildAgentServiceError> {
    let service = ChildAgentService::initialize().await?;
    Box::pin(service.run_until_shutdown()).await
}
