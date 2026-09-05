use super::error::{ExternalProvisioningBoundary, ProvisioningError};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Stage {
    RegistryReadback,
    ScmReadback,
    CngReadback,
    AccountIssuerReadback,
    TpmReadback,
    CngRevalidated,
    AccountIssuerRevalidated,
    RegistryRevalidated,
    ScmRevalidated,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct OrderedCeremony {
    stage: Option<Stage>,
}

impl OrderedCeremony {
    pub(super) const fn new() -> Self {
        Self { stage: None }
    }

    pub(super) fn advance(&mut self, next: Stage) -> Result<(), ProvisioningError> {
        let valid = matches!(
            (self.stage, next),
            (None, Stage::RegistryReadback)
                | (Some(Stage::RegistryReadback), Stage::ScmReadback)
                | (Some(Stage::ScmReadback), Stage::CngReadback)
                | (Some(Stage::CngReadback), Stage::AccountIssuerReadback)
                | (Some(Stage::AccountIssuerReadback), Stage::TpmReadback)
                | (Some(Stage::TpmReadback), Stage::CngRevalidated)
                | (Some(Stage::CngRevalidated), Stage::AccountIssuerRevalidated)
                | (
                    Some(Stage::AccountIssuerRevalidated),
                    Stage::RegistryRevalidated
                )
                | (Some(Stage::RegistryRevalidated), Stage::ScmRevalidated)
        );
        if !valid {
            return Err(ProvisioningError::ExistingStateRejected);
        }
        self.stage = Some(next);
        Ok(())
    }

    pub(super) fn finish_read_only(self) -> Result<(), ProvisioningError> {
        if self.stage != Some(Stage::ScmRevalidated) {
            return Err(ProvisioningError::ExistingStateRejected);
        }
        Err(ProvisioningError::ExternalProvisioningRequired(
            ExternalProvisioningBoundary::OwnerManagedLifecycle,
        ))
    }
}
