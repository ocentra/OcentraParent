use crate::{
    device_trust_lifecycle::{
        DeviceTrustLifecycleError, DeviceTrustLifecycleEventKind, DeviceTrustLifecycleRepository,
        DeviceTrustLifecycleState, LifecycleEventInput,
    },
    device_trust_signer_registration::{
        self, CurrentSignerAuthority, SignerRegistrationAuthorization,
    },
};

const TRUSTED: &str = "trusted";

#[derive(Clone, PartialEq, Eq)]
struct LifecycleSnapshot {
    state: String,
    lifecycle_generation: u64,
    installation_id: String,
    installation_binding_generation: u64,
    authority_generation: u64,
}

impl From<(String, u64, String, u64, u64)> for LifecycleSnapshot {
    fn from(value: (String, u64, String, u64, u64)) -> Self {
        Self {
            state: value.0,
            lifecycle_generation: value.1,
            installation_id: value.2,
            installation_binding_generation: value.3,
            authority_generation: value.4,
        }
    }
}

impl DeviceTrustLifecycleRepository {
    /// Register a signer only with an opaque authorization issued by the
    /// family-owned parent ceremony. LAN consumers cannot call this method.
    pub(crate) fn register_signer_anchor(
        &mut self,
        authorization: SignerRegistrationAuthorization,
    ) -> Result<CurrentSignerAuthority, DeviceTrustLifecycleError> {
        let (family, subject, parent, child, installation, correlation) =
            authorization.registration_identity();
        let family_id = family.to_owned();
        let trust_subject = subject.to_owned();
        let parent_device_id = parent.to_owned();
        let child_device_id = child.to_owned();
        let installation_id = installation.to_owned();
        let correlation_id = correlation.to_owned();
        let signer_event_binding = authorization.event_binding();
        let snapshot = current_lifecycle_snapshot(
            self,
            &family_id,
            &trust_subject,
            &parent_device_id,
            &installation_id,
        )?;
        let transaction = self.transaction()?;
        if Self::row(&transaction, &family_id, &trust_subject, &parent_device_id)?
            .map(LifecycleSnapshot::from)
            != Some(snapshot.clone())
        {
            return Err(DeviceTrustLifecycleError::Unavailable);
        }
        device_trust_signer_registration::register(
            &transaction,
            authorization,
            snapshot.lifecycle_generation,
            snapshot.installation_binding_generation,
            snapshot.authority_generation,
        )?;
        Self::insert_event(
            &transaction,
            &LifecycleEventInput {
                family_id: &family_id,
                trust_subject: &trust_subject,
                device_ref: &parent_device_id,
                correlation_id: &correlation_id,
                event_binding: Some(&signer_event_binding),
                kind: DeviceTrustLifecycleEventKind::SignerRegistered,
                state: DeviceTrustLifecycleState::Trusted,
                lifecycle_generation: snapshot.lifecycle_generation,
                installation_binding_generation: snapshot.installation_binding_generation,
            },
        )?;
        transaction
            .commit()
            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        device_trust_signer_registration::current_authority(
            &self.connection,
            &self.external_authority,
            &family_id,
            &trust_subject,
            &parent_device_id,
            &child_device_id,
        )
    }
}

fn current_lifecycle_snapshot(
    repository: &mut DeviceTrustLifecycleRepository,
    family_id: &str,
    trust_subject: &str,
    parent_device_id: &str,
    installation_id: &str,
) -> Result<LifecycleSnapshot, DeviceTrustLifecycleError> {
    repository
        .external_authority
        .reconcile(&repository.connection)?;
    let snapshot = LifecycleSnapshot::from(
        DeviceTrustLifecycleRepository::row(
            &repository.connection,
            family_id,
            trust_subject,
            parent_device_id,
        )?
        .ok_or(DeviceTrustLifecycleError::RegistrationMissing)?,
    );
    if snapshot.state != TRUSTED {
        return Err(DeviceTrustLifecycleError::InvalidState);
    }
    if snapshot.installation_id != installation_id {
        return Err(DeviceTrustLifecycleError::SignerRegistrationConflict);
    }
    repository
        .external_authority
        .matches(
            family_id,
            trust_subject,
            parent_device_id,
            snapshot.authority_generation,
        )
        .then_some(snapshot)
        .ok_or(DeviceTrustLifecycleError::Unavailable)
}
