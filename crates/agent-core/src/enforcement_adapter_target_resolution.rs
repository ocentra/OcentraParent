use std::path::Path;

use ed25519_dalek::Signature;
use ocentra_schema::{
    authenticated_delivery_grant::{
        authenticated_delivery_grant_audit_fingerprint, AuthenticatedDeliveryGrant,
    },
    authenticated_delivery_managed_process::AuthenticatedManagedProcessTargetBinding,
};

use super::{AuthenticatedDeliveryGrantTrustedIssuer, AuthenticatedOwnedProcessTerminationTarget};
use crate::activity_store::ActivityStore;

pub(super) fn resolve(
    grant: &AuthenticatedDeliveryGrant,
    binding: &AuthenticatedManagedProcessTargetBinding,
    trusted_issuer: &AuthenticatedDeliveryGrantTrustedIssuer,
    activity_store_path: impl AsRef<Path>,
) -> Result<AuthenticatedOwnedProcessTerminationTarget, ()> {
    validate_binding(grant, binding, trusted_issuer)?;
    let signature = Signature::from_slice(&binding.signature).map_err(|_error| ())?;
    trusted_issuer
        .verifying_key
        .verify_strict(&binding.signing_bytes(), &signature)
        .map_err(|_error| ())?;
    let store = ActivityStore::open(activity_store_path).map_err(|_error| ())?;
    let model = store
        .app_game_service_read_model(
            ocentra_parent_agent_protocol::constants::activity_store::DEFAULT_RECENT_LIMIT,
            ocentra_parent_agent_protocol::constants::enforcement::APP_GAME_RUNTIME_EVIDENCE_GENERATED_AT,
        )
        .map_err(|_error| ())?;
    let runtime = model
        .running_now_rows
        .iter()
        .find(|row| {
            row.process_identity == binding.managed_process_identity
                && row.launcher_ref.is_some()
                && matches!(
                    row.classification_state.as_str(),
                    ocentra_parent_agent_protocol::app_game::APP_GAME_CLASSIFICATION_KNOWN_APP
                        | ocentra_parent_agent_protocol::app_game::APP_GAME_CLASSIFICATION_KNOWN_GAME
                )
                && row.executable_path_ref.is_some()
                && row.started_at.is_some()
        })
        .ok_or(())?;
    let process_start_time =
        chrono::DateTime::parse_from_rfc3339(runtime.started_at.as_deref().ok_or(())?)
            .map_err(|_error| ())?
            .timestamp()
            .try_into()
            .map_err(|_error| ())?;
    let summary = store
        .app_game_session_summaries(
            ocentra_parent_agent_protocol::constants::activity_store::DEFAULT_RECENT_LIMIT,
        )
        .map_err(|_error| ())?
        .into_iter()
        .find(|summary| {
            summary.primary_process_identity == runtime.process_identity
                && summary.launcher_ref.is_some()
                && summary.last_observed_at >= runtime.observed_at
        })
        .ok_or(())?;
    if summary.primary_process_identity != binding.managed_process_identity {
        return Err(());
    }
    Ok(
        AuthenticatedOwnedProcessTerminationTarget::from_local_binding(
            binding,
            u32::try_from(runtime.process_id).map_err(|_error| ())?,
            runtime.process_name.clone(),
            runtime.executable_path_ref.clone().ok_or(())?,
            process_start_time,
        ),
    )
}

fn validate_binding(
    grant: &AuthenticatedDeliveryGrant,
    binding: &AuthenticatedManagedProcessTargetBinding,
    trusted_issuer: &AuthenticatedDeliveryGrantTrustedIssuer,
) -> Result<(), ()> {
    if grant.validate_shape().is_err()
        || binding.validate_shape().is_err()
        || binding.issuer_key_id != trusted_issuer.key_id
        || binding.issuer_key_id != grant.issuer_key_id
        || binding.grant_fingerprint != authenticated_delivery_grant_audit_fingerprint(grant)
        || binding.nonce != grant.nonce
        || binding.issuer_actor_id != grant.issuer_actor_id
        || binding.household_id != grant.household_id
        || binding.parent_device_id != grant.parent_device_id
        || binding.child_profile_id != grant.child_profile_id
        || binding.target_device_id != grant.target_device_id
        || binding.policy_decision_id != grant.policy_decision_id
        || binding.policy_version != grant.policy_version
        || binding.action_id != grant.action_id
        || binding.capability_id != grant.capability_id
    {
        Err(())
    } else {
        Ok(())
    }
}
