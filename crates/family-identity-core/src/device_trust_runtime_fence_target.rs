use crate::{
    device_trust_current_binding::CurrentChildDeviceTrustBinding,
    device_trust_lifecycle::{DeviceTrustLifecycleRepository, DeviceTrustLifecycleState},
    device_trust_lifecycle_authority::authority_key,
    device_trust_lifecycle_authority_lock::AuthorityReadFence,
    device_trust_lifecycle_schema::is_lower_hex,
    device_trust_signer_registration,
    device_trust_signer_registration_validation::validate_canonical_identity,
    household_authority::HouseholdAuthorityAction,
};
use rusqlite::Transaction;

use super::{action, DeviceTrustRuntimeFenceError, DeviceTrustRuntimeFenceTarget};

pub(super) fn from_binding(
    action_value: HouseholdAuthorityAction,
    binding: &CurrentChildDeviceTrustBinding,
) -> Result<DeviceTrustRuntimeFenceTarget, DeviceTrustRuntimeFenceError> {
    from_binding_code(action::code(action_value), binding)
}

pub(super) fn from_binding_code(
    action_code: i64,
    binding: &CurrentChildDeviceTrustBinding,
) -> Result<DeviceTrustRuntimeFenceTarget, DeviceTrustRuntimeFenceError> {
    if !(0..=10).contains(&action_code) {
        return Err(DeviceTrustRuntimeFenceError::InvalidTarget);
    }
    if binding.state() != DeviceTrustLifecycleState::Trusted {
        return Err(DeviceTrustRuntimeFenceError::DeviceTrustRevoked);
    }
    for identity in [
        binding.family_id(),
        binding.trust_subject(),
        binding.parent_device_id(),
        binding.child_device_id(),
        binding.installation_id(),
    ] {
        validate_canonical_identity(identity)
            .map_err(|_| DeviceTrustRuntimeFenceError::InvalidTarget)?;
    }
    if !is_lower_hex(binding.signer_key_id(), 32)
        || !is_lower_hex(binding.signer_key_sha256(), 64)
        || binding.lifecycle_generation() == 0
        || binding.installation_binding_generation() == 0
        || binding.authority_generation() == 0
    {
        return Err(DeviceTrustRuntimeFenceError::InvalidTarget);
    }
    Ok(DeviceTrustRuntimeFenceTarget {
        action_code,
        family_id: binding.family_id().to_owned(),
        trust_subject: binding.trust_subject().to_owned(),
        parent_device_id: binding.parent_device_id().to_owned(),
        child_device_id: binding.child_device_id().to_owned(),
        installation_id: binding.installation_id().to_owned(),
        signer_key_id: binding.signer_key_id().to_owned(),
        signer_key_sha256: binding.signer_key_sha256().to_owned(),
        lifecycle_generation: binding.lifecycle_generation(),
        installation_binding_generation: binding.installation_binding_generation(),
        authority_generation: binding.authority_generation(),
        state: binding.state(),
    })
}

pub(super) fn from_stored(
    action_code: i64,
    family_id: &str,
    trust_subject: &str,
    parent_device_id: &str,
    child_device_id: &str,
    installation_id: &str,
    signer_key_id: &str,
    signer_key_sha256: &str,
    lifecycle_generation: u64,
    installation_binding_generation: u64,
    authority_generation: u64,
) -> Result<DeviceTrustRuntimeFenceTarget, DeviceTrustRuntimeFenceError> {
    if !(0..=10).contains(&action_code) {
        return Err(DeviceTrustRuntimeFenceError::Unavailable);
    }
    Ok(DeviceTrustRuntimeFenceTarget {
        action_code,
        family_id: family_id.to_owned(),
        trust_subject: trust_subject.to_owned(),
        parent_device_id: parent_device_id.to_owned(),
        child_device_id: child_device_id.to_owned(),
        installation_id: installation_id.to_owned(),
        signer_key_id: signer_key_id.to_owned(),
        signer_key_sha256: signer_key_sha256.to_owned(),
        lifecycle_generation,
        installation_binding_generation,
        authority_generation,
        state: DeviceTrustLifecycleState::Trusted,
    })
}

pub(super) fn clone_target(
    target: &DeviceTrustRuntimeFenceTarget,
) -> DeviceTrustRuntimeFenceTarget {
    DeviceTrustRuntimeFenceTarget {
        action_code: target.action_code,
        family_id: target.family_id.clone(),
        trust_subject: target.trust_subject.clone(),
        parent_device_id: target.parent_device_id.clone(),
        child_device_id: target.child_device_id.clone(),
        installation_id: target.installation_id.clone(),
        signer_key_id: target.signer_key_id.clone(),
        signer_key_sha256: target.signer_key_sha256.clone(),
        lifecycle_generation: target.lifecycle_generation,
        installation_binding_generation: target.installation_binding_generation,
        authority_generation: target.authority_generation,
        state: target.state,
    }
}

pub(super) fn current_target_in_transaction(
    transaction: &Transaction<'_>,
    expected: &DeviceTrustRuntimeFenceTarget,
    fence: &AuthorityReadFence,
) -> Result<DeviceTrustRuntimeFenceTarget, DeviceTrustRuntimeFenceError> {
    let (state, lifecycle_generation, installation_id, binding_generation, authority_generation) =
        DeviceTrustLifecycleRepository::row(
            transaction,
            &expected.family_id,
            &expected.trust_subject,
            &expected.parent_device_id,
        )?
        .ok_or(DeviceTrustRuntimeFenceError::DeviceTrustUnavailable)?;
    if state != "trusted" {
        return Err(DeviceTrustRuntimeFenceError::DeviceTrustRevoked);
    }
    let authority = device_trust_signer_registration::current(
        transaction,
        &expected.family_id,
        &expected.trust_subject,
        &expected.parent_device_id,
        &expected.child_device_id,
    )
    .map_err(DeviceTrustRuntimeFenceError::from)?;
    if authority.installation_id() != installation_id
        || authority.lifecycle_generation() != lifecycle_generation
        || authority.installation_binding_generation() != binding_generation
        || authority.authority_generation() != authority_generation
    {
        return Err(DeviceTrustRuntimeFenceError::DeviceTrustUnavailable);
    }
    if !fence
        .matches(
            &authority_key(
                &expected.family_id,
                &expected.trust_subject,
                &expected.parent_device_id,
            ),
            authority_generation,
        )
        .map_err(DeviceTrustRuntimeFenceError::from)?
    {
        return Err(DeviceTrustRuntimeFenceError::DeviceTrustUnavailable);
    }
    let binding = authority.into_current_child_device_trust_binding();
    let current = from_binding_code(expected.action_code, &binding)?;
    ensure_current(expected, &current)?;
    Ok(current)
}

pub(super) fn ensure_current(
    expected: &DeviceTrustRuntimeFenceTarget,
    current: &DeviceTrustRuntimeFenceTarget,
) -> Result<(), DeviceTrustRuntimeFenceError> {
    if expected.family_id != current.family_id
        || expected.trust_subject != current.trust_subject
        || expected.parent_device_id != current.parent_device_id
        || expected.child_device_id != current.child_device_id
        || expected.installation_id != current.installation_id
        || expected.signer_key_id != current.signer_key_id
        || expected.signer_key_sha256 != current.signer_key_sha256
        || expected.action_code != current.action_code
    {
        return Err(DeviceTrustRuntimeFenceError::TargetMismatch);
    }
    if expected.state != current.state {
        return Err(DeviceTrustRuntimeFenceError::DeviceTrustRevoked);
    }
    if expected.lifecycle_generation != current.lifecycle_generation
        || expected.installation_binding_generation != current.installation_binding_generation
        || expected.authority_generation != current.authority_generation
    {
        return Err(DeviceTrustRuntimeFenceError::GenerationMismatch);
    }
    Ok(())
}
