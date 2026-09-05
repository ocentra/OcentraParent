use ocentra_protected_capability_custody_windows_ffi::{OwnedTbsContext, TpmNvPublicObservation};
use sha2::{Digest, Sha256};

use super::cng::ExistingPcpSigner;
use super::constants;
use super::enrollment::EnrollmentSnapshot;
use super::error::{ExternalProvisioningBoundary, ProvisioningError};
use super::tpm_error;

pub(super) fn readback(
    enrollment: &EnrollmentSnapshot,
    signer: &ExistingPcpSigner,
) -> Result<(), ProvisioningError> {
    if !OwnedTbsContext::is_tpm_present().map_err(tpm_error::tbs)? {
        return Err(ProvisioningError::ExternalProvisioningRequired(
            ExternalProvisioningBoundary::FixedTpmCounter,
        ));
    }
    let context = OwnedTbsContext::open().map_err(tpm_error::tbs)?;
    let first = context
        .observe_fixed_counter_public()
        .map_err(tpm_error::public_observation)?;
    validate(&first, enrollment, signer)?;
    let first_value = signed_read(&context, signer)?;
    let second_value = signed_read(&context, signer)?;
    if first_value != second_value {
        return Err(ProvisioningError::ExistingStateRejected);
    }
    let second = context
        .observe_fixed_counter_public()
        .map_err(tpm_error::public_revalidation)?;
    if first != second {
        return Err(ProvisioningError::ExistingStateRejected);
    }
    validate(&second, enrollment, signer)
}

fn signed_read(
    context: &OwnedTbsContext,
    signer: &ExistingPcpSigner,
) -> Result<u64, ProvisioningError> {
    let prepared = context
        .prepare_fixed_counter_read(signer.policy_public())
        .map_err(tpm_error::operation)?;
    signer.execute_read(prepared)
}

fn validate(
    observed: &TpmNvPublicObservation,
    enrollment: &EnrollmentSnapshot,
    signer: &ExistingPcpSigner,
) -> Result<(), ProvisioningError> {
    let expected = &enrollment.tpm;
    let signer_policy = signer.policy_public().fixed_counter_policy_digest();
    if observed.nv_index() != constants::TPM_NV_INDEX
        || observed.nv_index() != expected.index
        || observed.name_algorithm() != constants::TPM_ALG_SHA256
        || observed.name_algorithm() != expected.name_algorithm
        || observed.attributes() != constants::TPM_COUNTER_ATTRIBUTES
        || observed.attributes() != expected.attributes
        || observed.data_size() != constants::TPM_COUNTER_BYTES
        || observed.data_size() != expected.data_size
        || observed.auth_policy() != expected.policy_digest
        || observed.auth_policy() != signer_policy
        || observed.auth_policy().is_empty()
        || observed.name() != expected_name(observed).as_slice()
        || observed.name().is_empty()
    {
        return Err(ProvisioningError::ExistingStateRejected);
    }
    Ok(())
}

fn expected_name(observed: &TpmNvPublicObservation) -> Vec<u8> {
    let mut public = Vec::new();
    public.extend_from_slice(&observed.nv_index().to_be_bytes());
    public.extend_from_slice(&observed.name_algorithm().to_be_bytes());
    public.extend_from_slice(&observed.attributes().to_be_bytes());
    public.extend_from_slice(&(observed.auth_policy().len() as u16).to_be_bytes());
    public.extend_from_slice(observed.auth_policy());
    public.extend_from_slice(&observed.data_size().to_be_bytes());
    let digest = Sha256::digest(&public);
    let mut name = Vec::with_capacity(2 + digest.len());
    name.extend_from_slice(&observed.name_algorithm().to_be_bytes());
    name.extend_from_slice(&digest);
    name
}
