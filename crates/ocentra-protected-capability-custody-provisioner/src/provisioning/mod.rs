#[cfg(windows)]
mod account_issuer_acl;
mod ceremony;
#[cfg(windows)]
mod cng;
#[cfg(windows)]
mod constants;
#[cfg(windows)]
mod enrollment;
mod error;
#[cfg(windows)]
mod registry;
#[cfg(windows)]
mod registry_digest;
#[cfg(windows)]
mod registry_security;
#[cfg(windows)]
mod scm;
#[cfg(windows)]
mod scm_error;
#[cfg(windows)]
mod state;
#[cfg(windows)]
mod tpm;
#[cfg(windows)]
mod tpm_error;

pub(super) fn run() -> Result<(), error::ProvisioningError> {
    ceremony::run()
}

pub(super) fn unexpected_arguments_exit_code() -> std::process::ExitCode {
    error::ProvisioningError::UnexpectedArguments.exit_code()
}
