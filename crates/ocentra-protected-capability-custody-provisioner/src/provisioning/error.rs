use std::process::ExitCode;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ExternalProvisioningBoundary {
    EnrollmentRegistry,
    BrokerService,
    PcpSigningKey,
    FixedTpmCounter,
    OwnerManagedLifecycle,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ProvisioningError {
    UnsupportedPlatform,
    UnexpectedArguments,
    ExternalProvisioningRequired(ExternalProvisioningBoundary),
    ExistingStateRejected,
    PlatformObservationUnavailable,
}

impl ProvisioningError {
    pub(crate) fn exit_code(self) -> ExitCode {
        let code = match self {
            Self::UnsupportedPlatform => 2,
            Self::UnexpectedArguments => 10,
            Self::ExternalProvisioningRequired(boundary) => match boundary {
                ExternalProvisioningBoundary::EnrollmentRegistry => 3,
                ExternalProvisioningBoundary::BrokerService => 4,
                ExternalProvisioningBoundary::PcpSigningKey => 7,
                ExternalProvisioningBoundary::FixedTpmCounter => 8,
                ExternalProvisioningBoundary::OwnerManagedLifecycle => 9,
            },
            Self::ExistingStateRejected => 5,
            Self::PlatformObservationUnavailable => 6,
        };
        ExitCode::from(code)
    }
}
