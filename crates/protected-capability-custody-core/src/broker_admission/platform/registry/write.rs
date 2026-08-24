#[cfg(windows)]
use std::borrow::Cow;
#[cfg(windows)]
use std::io;

#[cfg(windows)]
use winreg::enums::REG_BINARY;
#[cfg(windows)]
use winreg::RegValue;

#[cfg(windows)]
use crate::platform::PlatformError;

#[cfg(windows)]
pub(super) fn one(registry_id: &str, name: &str, value: &[u8]) -> Result<(), PlatformError> {
    batch(
        registry_id,
        &[super::super::RuntimeMutation {
            name,
            value: Some(value),
        }],
    )
    .map_err(super::super::RuntimeBatchFailure::into_platform_error)
}

#[cfg(windows)]
pub(super) fn delete(registry_id: &str, name: &str) -> Result<(), PlatformError> {
    batch(
        registry_id,
        &[super::super::RuntimeMutation { name, value: None }],
    )
    .map_err(super::super::RuntimeBatchFailure::into_platform_error)
}

#[cfg(windows)]
pub(super) fn batch(
    registry_id: &str,
    mutations: &[super::super::RuntimeMutation<'_>],
) -> Result<(), super::super::RuntimeBatchFailure> {
    validate_mutations(mutations)
        .map_err(super::super::RuntimeBatchFailure::DefinitelyNotApplied)?;
    let key = super::super::open_runtime_write_key(registry_id)
        .map_err(super::super::RuntimeBatchFailure::DefinitelyNotApplied)?;
    super::super::verify_runtime_snapshot(registry_id, &key)
        .map_err(super::super::RuntimeBatchFailure::DefinitelyNotApplied)?;
    // The provider advances first. Any later failure is outcome-unknown and
    // the bound checkpoint prevents an inconsistent snapshot from reopening.
    let permit = super::super::authorize_runtime_batch(registry_id, &key, mutations)?;
    for mutation in mutations {
        apply_mutation(&key, mutation)?;
    }
    super::super::confirm_runtime_batch(registry_id, &key, permit)
        .map_err(|_| super::super::RuntimeBatchFailure::OutcomeUnknown)
}

#[cfg(windows)]
fn apply_mutation(
    key: &winreg::RegKey,
    mutation: &super::super::RuntimeMutation<'_>,
) -> Result<(), super::super::RuntimeBatchFailure> {
    match mutation.value {
        Some(value) => key
            .set_raw_value(
                mutation.name,
                &RegValue {
                    bytes: Cow::Borrowed(value),
                    vtype: REG_BINARY,
                },
            )
            .map_err(|_| super::super::RuntimeBatchFailure::OutcomeUnknown),
        None => delete_value(key, mutation.name),
    }
}

#[cfg(windows)]
fn delete_value(key: &winreg::RegKey, name: &str) -> Result<(), super::super::RuntimeBatchFailure> {
    match key.delete_value(name) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(_) => Err(super::super::RuntimeBatchFailure::OutcomeUnknown),
    }
}

#[cfg(windows)]
fn validate_mutations(
    mutations: &[super::super::RuntimeMutation<'_>],
) -> Result<(), PlatformError> {
    if mutations.is_empty() {
        return Err(PlatformError::InvalidAttestation);
    }
    for (index, mutation) in mutations.iter().enumerate() {
        validate_mutation(mutation)?;
        if mutations[..index]
            .iter()
            .any(|prior| prior.name == mutation.name)
        {
            return Err(PlatformError::InvalidAttestation);
        }
    }
    Ok(())
}

#[cfg(windows)]
fn validate_mutation(mutation: &super::super::RuntimeMutation<'_>) -> Result<(), PlatformError> {
    let too_large = mutation.value.is_some_and(|value| {
        value.len()
            > ocentra_protected_capability_custody_protocol::constants::MAX_REGISTRY_VALUE_BYTES
    });
    if mutation.name.is_empty() || too_large {
        Err(PlatformError::InvalidAttestation)
    } else {
        Ok(())
    }
}
