use rusqlite::params;

use crate::{
    device_trust_lifecycle::{
        to_sql_generation, DeviceTrustLifecycleError, DeviceTrustLifecycleEventKind,
        DeviceTrustLifecycleRepository, DeviceTrustLifecycleState, LifecycleEventInput,
    },
    device_trust_lifecycle_authority_fence::AuthorityTransition,
    device_trust_signer_registration_revocation,
};

const REVOKED: &str = "revoked";
const RESET_REQUIRED: &str = "reset-required";

impl DeviceTrustLifecycleRepository {
    pub fn revoke_or_reset(
        &mut self,
        family_id: &str,
        trust_subject: &str,
        device_ref: &str,
        reset_required: bool,
        correlation_id: &str,
    ) -> Result<(), DeviceTrustLifecycleError> {
        self.validate_identifiers(family_id, trust_subject, device_ref, correlation_id)?;
        let context = RevocationContext::load(
            self,
            family_id,
            trust_subject,
            device_ref,
            correlation_id,
            reset_required,
        )?;
        let authority_transition = self.external_authority.begin_transition(
            &self.connection,
            family_id,
            trust_subject,
            device_ref,
            Some(context.lifecycle_snapshot.4),
            context.next_authority_generation,
        )?;
        let database_result = persist_revocation(self, &context, &authority_transition);
        self.finish_authority_transition(authority_transition, database_result)
    }

    /// Numeric generation alone can never restore a revoked/reset device.
    /// WP03 must replace this fail-closed boundary with a consumed opaque
    /// parent reauthorization before any repair transition is enabled.
    pub fn repair_with_new_installation(
        &mut self,
        family_id: &str,
        trust_subject: &str,
        device_ref: &str,
        installation_binding_generation: u64,
        correlation_id: &str,
    ) -> Result<(), DeviceTrustLifecycleError> {
        self.validate_identifiers(family_id, trust_subject, device_ref, correlation_id)?;
        self.require_generation(installation_binding_generation)?;
        let transaction = self.transaction()?;
        let Some((state, _generation, _installation_id, prior_installation, _authority_generation)) =
            Self::row(&transaction, family_id, trust_subject, device_ref)?
        else {
            return Err(DeviceTrustLifecycleError::RegistrationMissing);
        };
        if installation_binding_generation <= prior_installation {
            return Err(DeviceTrustLifecycleError::InvalidGeneration);
        }
        if state == REVOKED || state == RESET_REQUIRED {
            Err(DeviceTrustLifecycleError::ParentReauthorizationRequired)
        } else {
            Err(DeviceTrustLifecycleError::InvalidState)
        }
    }
}

struct RevocationContext<'a> {
    family_id: &'a str,
    trust_subject: &'a str,
    device_ref: &'a str,
    correlation_id: &'a str,
    lifecycle_snapshot: (String, u64, String, u64, u64),
    next_state: &'static str,
    kind: DeviceTrustLifecycleEventKind,
    state: DeviceTrustLifecycleState,
    next_generation: u64,
    next_authority_generation: u64,
}

impl<'a> RevocationContext<'a> {
    fn load(
        repository: &DeviceTrustLifecycleRepository,
        family_id: &'a str,
        trust_subject: &'a str,
        device_ref: &'a str,
        correlation_id: &'a str,
        reset_required: bool,
    ) -> Result<Self, DeviceTrustLifecycleError> {
        let lifecycle_snapshot = DeviceTrustLifecycleRepository::row(
            &repository.connection,
            family_id,
            trust_subject,
            device_ref,
        )?
        .ok_or(DeviceTrustLifecycleError::RegistrationMissing)?;
        if lifecycle_snapshot.0 == REVOKED {
            return Err(DeviceTrustLifecycleError::RevokedDevice);
        }
        let next_generation = lifecycle_snapshot
            .1
            .checked_add(1)
            .ok_or(DeviceTrustLifecycleError::InvalidGeneration)?;
        let next_authority_generation = lifecycle_snapshot
            .4
            .checked_add(1)
            .ok_or(DeviceTrustLifecycleError::InvalidGeneration)?;
        let (next_state, kind, state) = revocation_state(reset_required);
        Ok(Self {
            family_id,
            trust_subject,
            device_ref,
            correlation_id,
            lifecycle_snapshot,
            next_state,
            kind,
            state,
            next_generation,
            next_authority_generation,
        })
    }
}

fn persist_revocation(
    repository: &mut DeviceTrustLifecycleRepository,
    context: &RevocationContext<'_>,
    authority_transition: &AuthorityTransition,
) -> Result<(), DeviceTrustLifecycleError> {
    let transaction = repository.transaction()?;
    if DeviceTrustLifecycleRepository::row(
        &transaction,
        context.family_id,
        context.trust_subject,
        context.device_ref,
    )? != Some(context.lifecycle_snapshot.clone())
    {
        return Err(DeviceTrustLifecycleError::Unavailable);
    }
    let changed = transaction
        .execute(
            "UPDATE device_trust_lifecycle
             SET lifecycle_state = ?4, lifecycle_generation = ?5, authority_generation = ?6
             WHERE family_id = ?1 AND trust_subject = ?2 AND device_ref = ?3
               AND lifecycle_state = ?7 AND lifecycle_generation = ?8
               AND installation_id = ?9 AND installation_binding_generation = ?10
               AND authority_generation = ?11",
            params![
                context.family_id,
                context.trust_subject,
                context.device_ref,
                context.next_state,
                to_sql_generation(context.next_generation)?,
                to_sql_generation(context.next_authority_generation)?,
                &context.lifecycle_snapshot.0,
                to_sql_generation(context.lifecycle_snapshot.1)?,
                &context.lifecycle_snapshot.2,
                to_sql_generation(context.lifecycle_snapshot.3)?,
                to_sql_generation(context.lifecycle_snapshot.4)?,
            ],
        )
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    if changed != 1 {
        return Err(DeviceTrustLifecycleError::Unavailable);
    }
    let signers_revoked = device_trust_signer_registration_revocation::revoke_for_lifecycle(
        &transaction,
        context.family_id,
        context.trust_subject,
        context.device_ref,
        context.next_authority_generation,
    )?;
    insert_revocation_milestones(
        &transaction,
        LifecycleEventInput {
            family_id: context.family_id,
            trust_subject: context.trust_subject,
            device_ref: context.device_ref,
            correlation_id: context.correlation_id,
            event_binding: None,
            kind: context.kind,
            state: context.state,
            lifecycle_generation: context.next_generation,
            installation_binding_generation: context.lifecycle_snapshot.3,
        },
        signers_revoked,
    )?;
    authority_transition.record(&transaction)?;
    transaction
        .commit()
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)
}

fn revocation_state(
    reset_required: bool,
) -> (
    &'static str,
    DeviceTrustLifecycleEventKind,
    DeviceTrustLifecycleState,
) {
    if reset_required {
        (
            RESET_REQUIRED,
            DeviceTrustLifecycleEventKind::ResetRequired,
            DeviceTrustLifecycleState::ResetRequired,
        )
    } else {
        (
            REVOKED,
            DeviceTrustLifecycleEventKind::Revoked,
            DeviceTrustLifecycleState::Revoked,
        )
    }
}

fn insert_revocation_milestones(
    transaction: &rusqlite::Transaction<'_>,
    lifecycle: LifecycleEventInput<'_>,
    signer_bindings: Vec<String>,
) -> Result<(), DeviceTrustLifecycleError> {
    DeviceTrustLifecycleRepository::insert_event(transaction, &lifecycle)?;
    for signer_binding in signer_bindings {
        DeviceTrustLifecycleRepository::insert_event(
            transaction,
            &LifecycleEventInput {
                family_id: lifecycle.family_id,
                trust_subject: lifecycle.trust_subject,
                device_ref: lifecycle.device_ref,
                correlation_id: lifecycle.correlation_id,
                event_binding: Some(&signer_binding),
                kind: DeviceTrustLifecycleEventKind::SignerRevoked,
                state: lifecycle.state,
                lifecycle_generation: lifecycle.lifecycle_generation,
                installation_binding_generation: lifecycle.installation_binding_generation,
            },
        )?;
    }
    Ok(())
}
