use super::error::ProvisioningError;

#[cfg(windows)]
use super::{account_issuer_acl, cng, registry, scm, state, tpm};

pub(super) fn run() -> Result<(), ProvisioningError> {
    #[cfg(not(windows))]
    {
        Err(ProvisioningError::UnsupportedPlatform)
    }
    #[cfg(windows)]
    {
        let mut ceremony = state::OrderedCeremony::new();
        let enrollment = registry::readback()?;
        ceremony.advance(state::Stage::RegistryReadback)?;
        scm::readback(&enrollment)?;
        ceremony.advance(state::Stage::ScmReadback)?;
        let signer = cng::open_existing(&enrollment)?;
        ceremony.advance(state::Stage::CngReadback)?;
        account_issuer_acl::readback()?;
        ceremony.advance(state::Stage::AccountIssuerReadback)?;
        tpm::readback(&enrollment, &signer)?;
        ceremony.advance(state::Stage::TpmReadback)?;
        signer.revalidate(&enrollment)?;
        ceremony.advance(state::Stage::CngRevalidated)?;
        account_issuer_acl::revalidate()?;
        ceremony.advance(state::Stage::AccountIssuerRevalidated)?;
        registry::revalidate(&enrollment)?;
        ceremony.advance(state::Stage::RegistryRevalidated)?;
        scm::revalidate(&enrollment)?;
        ceremony.advance(state::Stage::ScmRevalidated)?;

        // This is a partial read-only preflight. The current FFI independently
        // observes the registry/SCM/PCP/AccountIssuer/NV subset, but cannot pin
        // the broker or client files or observe the enrolled client token
        // identity/session.
        // It cannot create or publish enrollment: the repository has no
        // authenticated OEM/MDM handoff or protected registry/SCM mutation
        // transaction. A later installer run consumes externally completed
        // state by repeating this fixed preflight. The envelope also has no
        // expected counter generation, so equal signed reads prove stability,
        // not owner-intended generation. Provisioned success remains forbidden
        // until the missing observations and owner completion contract exist.
        ceremony.finish_read_only()
    }
}
