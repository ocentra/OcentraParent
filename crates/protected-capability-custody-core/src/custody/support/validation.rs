use super::super::CustodyError;
use crate::authority::{AuthorityError, CurrentBindingPort};
use crate::binding::{Binding, BindingLocator, BINDING_VERSION};
use crate::path_security::SecuredPath;
use crate::platform::{
    record::BrokerRecord, request::TransitionRequest, PlatformAttestation, PlatformCustodyPort,
};
use crate::storage::{self, Record};

pub(super) fn resolve_current<A: CurrentBindingPort>(
    authority: &A,
    locator: &BindingLocator,
) -> Result<Binding, CustodyError> {
    let binding = authority.resolve_current(locator).map_err(map_authority)?;
    if binding.locator() != locator {
        return Err(CustodyError::WrongBinding);
    }
    Ok(binding)
}

pub(super) fn validate_current(record: &Record, binding: &Binding) -> Result<(), CustodyError> {
    if record.canonical_binding != binding.canonical_bytes()
        || record.binding_digest != binding.digest()
        || record.lookup_digest != binding.locator().lookup_digest()
    {
        return Err(CustodyError::WrongBinding);
    }
    Ok(())
}

pub(super) fn verify_broker<P: PlatformCustodyPort>(
    platform: &P,
    broker: &BrokerRecord,
    path: &SecuredPath,
) -> Result<Record, CustodyError> {
    if broker.record_namespace != crate::RECORD_NAMESPACE
        || broker.schema_version != crate::STORAGE_SCHEMA_VERSION
        || broker.binding_version != BINDING_VERSION
        || broker.database_identity != path.identity()
        || broker.anti_rollback_watermark == 0
        || broker.canonical_binding.len() > 16 * 1024
        || broker.sealed.is_empty()
        || broker.sealed.len() > 64 * 1024
    {
        return Err(CustodyError::Tampered);
    }
    platform
        .verify(broker.seal_context(), &broker.sealed)
        .map_err(super::map_platform_error)?;
    storage::from_broker(broker).map_err(super::map_storage_error)
}

pub(super) fn validate_transition<P: PlatformCustodyPort>(
    platform: &P,
    broker: &BrokerRecord,
    request: TransitionRequest<'_>,
    path: &SecuredPath,
) -> Result<Record, CustodyError> {
    let fields_match = broker.record_namespace == request.record_namespace
        && broker.schema_version == request.schema_version
        && broker.binding_version == request.binding_version
        && broker.database_identity == request.database_identity
        && broker.record_id == *request.record_id
        && broker.lookup_digest == *request.lookup_digest
        && broker.binding_digest == *request.binding_digest
        && broker.canonical_binding == request.canonical_binding
        && broker.state == request.state
        && broker.sequence == request.sequence
        && broker.key_epoch == request.key_epoch
        && broker.writer_epoch == request.writer_epoch
        && broker.anti_rollback_watermark > request.minimum_watermark;
    if !fields_match {
        return Err(CustodyError::Tampered);
    }
    verify_broker(platform, broker, path)
}

pub(super) fn validate_attestation(
    record: &Record,
    attestation: PlatformAttestation,
) -> Result<(), CustodyError> {
    if record.database_identity != attestation.database_identity {
        return Err(CustodyError::Tampered);
    }
    if record.key_epoch != attestation.key_epoch {
        return Err(CustodyError::Rotated);
    }
    if record.writer_epoch > attestation.writer_epoch
        || record.anti_rollback_watermark > attestation.watermark_floor
    {
        return Err(CustodyError::Tampered);
    }
    Ok(())
}

fn map_authority(error: AuthorityError) -> CustodyError {
    match error {
        AuthorityError::Unavailable => CustodyError::Unavailable,
        AuthorityError::Rejected => CustodyError::WrongBinding,
    }
}
