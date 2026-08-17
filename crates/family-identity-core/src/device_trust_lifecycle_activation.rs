use rusqlite::params;

use crate::device_trust_lifecycle::{
    to_sql_generation, DeviceTrustLifecycleError, DeviceTrustLifecycleEventKind,
    DeviceTrustLifecycleRepository, DeviceTrustLifecycleState, LifecycleEventInput,
    SealingCustodyAuthorization,
};

const PENDING: &str = "pending";
const TRUSTED: &str = "trusted";

impl DeviceTrustLifecycleRepository {
    /// Mark a pending registration trusted only by consuming an opaque proof
    /// that platform custody was durably sealed for the exact current row.
    pub fn activate_after_sealing(
        &mut self,
        authorization: SealingCustodyAuthorization,
    ) -> Result<(), DeviceTrustLifecycleError> {
        let context = ActivationContext::from(authorization);
        let target_authority_generation = context
            .authority_generation
            .checked_add(1)
            .ok_or(DeviceTrustLifecycleError::InvalidGeneration)?;
        let authority_transition = self.external_authority.begin_transition(
            &self.connection,
            &context.family_id,
            &context.trust_subject,
            &context.device_ref,
            Some(context.authority_generation),
            target_authority_generation,
        )?;
        let database_result = (|| {
            let transaction = self.transaction()?;
            let row = Self::row(
                &transaction,
                &context.family_id,
                &context.trust_subject,
                &context.device_ref,
            )?
            .ok_or(DeviceTrustLifecycleError::RegistrationMissing)?;
            let lifecycle_transition = context.validate_row(row)?;
            if lifecycle_transition.authority_generation != target_authority_generation {
                return Err(DeviceTrustLifecycleError::Unavailable);
            }
            persist_transition(&transaction, &context, lifecycle_transition)?;
            authority_transition.record(&transaction)?;
            transaction
                .commit()
                .map_err(|_error| DeviceTrustLifecycleError::Unavailable)
        })();
        self.finish_authority_transition(authority_transition, database_result)
    }
}

struct ActivationContext {
    family_id: String,
    trust_subject: String,
    device_ref: String,
    installation_id: String,
    lifecycle_generation: u64,
    installation_binding_generation: u64,
    authority_generation: u64,
    correlation_id: String,
}

impl From<SealingCustodyAuthorization> for ActivationContext {
    fn from(authorization: SealingCustodyAuthorization) -> Self {
        let (family, subject, device, installation, lifecycle, binding, authority, correlation) =
            authorization.into_parts();
        Self {
            family_id: family,
            trust_subject: subject,
            device_ref: device,
            installation_id: installation,
            lifecycle_generation: lifecycle,
            installation_binding_generation: binding,
            authority_generation: authority,
            correlation_id: correlation,
        }
    }
}

impl ActivationContext {
    fn validate_row(
        &self,
        row: (String, u64, String, u64, u64),
    ) -> Result<ActivationTransition, DeviceTrustLifecycleError> {
        let (state, lifecycle, installation, binding, authority) = row;
        if state != PENDING {
            return Err(DeviceTrustLifecycleError::InvalidState);
        }
        if installation != self.installation_id
            || lifecycle != self.lifecycle_generation
            || binding != self.installation_binding_generation
            || authority != self.authority_generation
        {
            return Err(DeviceTrustLifecycleError::Unavailable);
        }
        Ok(ActivationTransition {
            lifecycle_generation: lifecycle
                .checked_add(1)
                .ok_or(DeviceTrustLifecycleError::InvalidGeneration)?,
            installation_binding_generation: binding,
            authority_generation: authority
                .checked_add(1)
                .ok_or(DeviceTrustLifecycleError::InvalidGeneration)?,
        })
    }
}

#[derive(Clone, Copy)]
struct ActivationTransition {
    lifecycle_generation: u64,
    installation_binding_generation: u64,
    authority_generation: u64,
}

fn persist_transition(
    transaction: &rusqlite::Transaction<'_>,
    context: &ActivationContext,
    transition: ActivationTransition,
) -> Result<(), DeviceTrustLifecycleError> {
    let changed = transaction
        .execute(
            "UPDATE device_trust_lifecycle
             SET lifecycle_state = ?4, lifecycle_generation = ?5, authority_generation = ?6
             WHERE family_id = ?1 AND trust_subject = ?2 AND device_ref = ?3
               AND lifecycle_state = ?7 AND lifecycle_generation = ?8
               AND installation_binding_generation = ?9 AND authority_generation = ?10
               AND installation_id = ?11",
            params![
                &context.family_id,
                &context.trust_subject,
                &context.device_ref,
                TRUSTED,
                to_sql_generation(transition.lifecycle_generation)?,
                to_sql_generation(transition.authority_generation)?,
                PENDING,
                to_sql_generation(context.lifecycle_generation)?,
                to_sql_generation(context.installation_binding_generation)?,
                to_sql_generation(context.authority_generation)?,
                &context.installation_id,
            ],
        )
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    if changed != 1 {
        return Err(DeviceTrustLifecycleError::Unavailable);
    }
    DeviceTrustLifecycleRepository::insert_event(
        transaction,
        &LifecycleEventInput {
            family_id: &context.family_id,
            trust_subject: &context.trust_subject,
            device_ref: &context.device_ref,
            correlation_id: &context.correlation_id,
            event_binding: None,
            kind: DeviceTrustLifecycleEventKind::Activated,
            state: DeviceTrustLifecycleState::Trusted,
            lifecycle_generation: transition.lifecycle_generation,
            installation_binding_generation: transition.installation_binding_generation,
        },
    )
}
