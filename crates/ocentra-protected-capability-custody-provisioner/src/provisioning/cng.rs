use ocentra_protected_capability_custody_windows_ffi::{
    Error as FfiError, OwnedPcpProvider, OwnedPcpSigningKey, PcpKeyObservation,
    PreparedTpmCounterRead, TpmPolicySignature, TpmPolicySignerPublic,
};

use super::enrollment::EnrollmentSnapshot;
use super::error::{ExternalProvisioningBoundary, ProvisioningError};

const NTE_BAD_KEYSET: u32 = 0x8009_0016;

pub(super) struct ExistingPcpSigner {
    key: OwnedPcpSigningKey,
    observation: PcpKeyObservation,
    modulus: [u8; 384],
    policy_public: TpmPolicySignerPublic,
}

pub(super) fn open_existing(
    enrollment: &EnrollmentSnapshot,
) -> Result<ExistingPcpSigner, ProvisioningError> {
    let provider = OwnedPcpProvider::open_machine().map_err(map_provider_error)?;
    let key = provider
        .open_fixed_signing_key()
        .map_err(map_key_open_error)?;
    let observation = key.observation().map_err(map_observation_error)?;
    let modulus = key
        .signing_public_modulus()
        .map_err(map_observation_error)?;
    let policy_public =
        TpmPolicySignerPublic::from_rsa3072_modulus(&modulus).map_err(map_observation_error)?;
    if policy_public.fixed_counter_policy_digest() != enrollment.tpm.policy_digest
        || key.observation().map_err(map_observation_error)? != observation
        || key
            .signing_public_modulus()
            .map_err(map_observation_error)?
            != modulus
    {
        return Err(ProvisioningError::ExistingStateRejected);
    }
    Ok(ExistingPcpSigner {
        key,
        observation,
        modulus,
        policy_public,
    })
}

impl ExistingPcpSigner {
    pub(super) fn policy_public(&self) -> &TpmPolicySignerPublic {
        &self.policy_public
    }

    pub(super) fn execute_read(
        &self,
        prepared: PreparedTpmCounterRead<'_>,
    ) -> Result<u64, ProvisioningError> {
        let digest = prepared.signing_digest().map_err(map_observation_error)?;
        let signature = self
            .key
            .sign_sha256_digest(&digest)
            .map_err(map_runtime_error)?;
        let policy_signature =
            TpmPolicySignature::from_rsa_pss_sha256(&signature).map_err(map_observation_error)?;
        prepared
            .execute(&policy_signature)
            .map_err(map_runtime_error)
    }

    pub(super) fn revalidate(
        &self,
        enrollment: &EnrollmentSnapshot,
    ) -> Result<(), ProvisioningError> {
        let provider = OwnedPcpProvider::open_machine().map_err(map_observation_error)?;
        let fresh_key = provider
            .open_fixed_signing_key()
            .map_err(map_revalidation_key_error)?;
        if self.key.observation().map_err(map_observation_error)? != self.observation
            || self
                .key
                .signing_public_modulus()
                .map_err(map_observation_error)?
                != self.modulus
            || fresh_key.observation().map_err(map_observation_error)? != self.observation
            || fresh_key
                .signing_public_modulus()
                .map_err(map_observation_error)?
                != self.modulus
            || self.policy_public.fixed_counter_policy_digest() != enrollment.tpm.policy_digest
        {
            return Err(ProvisioningError::ExistingStateRejected);
        }
        Ok(())
    }
}

fn map_key_open_error(error: FfiError) -> ProvisioningError {
    match error {
        FfiError::Crypto(NTE_BAD_KEYSET) => ProvisioningError::ExternalProvisioningRequired(
            ExternalProvisioningBoundary::PcpSigningKey,
        ),
        other => map_observation_error(other),
    }
}

fn map_observation_error(error: FfiError) -> ProvisioningError {
    match error {
        FfiError::UnsupportedPlatform => ProvisioningError::UnsupportedPlatform,
        FfiError::CryptoPropertyViolation
        | FfiError::InvalidInput(_)
        | FfiError::MalformedTpm
        | FfiError::BufferTooLarge => ProvisioningError::ExistingStateRejected,
        FfiError::Crypto(_) | FfiError::Win32(_) | FfiError::Tpm(_) | FfiError::Tbs(_) => {
            ProvisioningError::PlatformObservationUnavailable
        }
    }
}

fn map_runtime_error(error: FfiError) -> ProvisioningError {
    if matches!(error, FfiError::Tpm(_)) {
        ProvisioningError::ExistingStateRejected
    } else {
        map_observation_error(error)
    }
}

fn map_provider_error(error: FfiError) -> ProvisioningError {
    map_observation_error(error)
}

fn map_revalidation_key_error(error: FfiError) -> ProvisioningError {
    if matches!(error, FfiError::Crypto(NTE_BAD_KEYSET)) {
        ProvisioningError::ExistingStateRejected
    } else {
        map_observation_error(error)
    }
}
