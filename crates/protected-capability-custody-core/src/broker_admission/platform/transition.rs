#[cfg(windows)]
use crate::platform::record::BrokerRecord;
#[cfg(windows)]
use crate::platform::request::{BrokerLookup, TransitionRequest};
#[cfg(windows)]
use crate::platform::{PlatformDatabaseGuard, PlatformError, TransitionFailure};
#[cfg(windows)]
use zeroize::Zeroizing;

#[cfg(windows)]
use super::guard::{map_poison, BrokerPlatformGuard};
#[cfg(windows)]
use super::{crypto, record, registry, state};
#[cfg(windows)]
use crate::broker_admission::record_codec;

#[cfg(windows)]
pub(super) fn reserve(
    guard: &BrokerPlatformGuard,
    next: TransitionRequest<'_>,
) -> Result<BrokerRecord, TransitionFailure> {
    guard
        .revalidate_live()
        .map_err(TransitionFailure::DefinitelyNotApplied)?;
    validate_request(guard, next).map_err(TransitionFailure::DefinitelyNotApplied)?;
    if record::read_ciphertext(&guard.registry_id, next.lookup_digest)
        .map_err(TransitionFailure::DefinitelyNotApplied)?
        .is_some()
    {
        return Err(TransitionFailure::DefinitelyNotApplied(
            PlatformError::Conflict,
        ));
    }
    persist(guard, next)
}

#[cfg(windows)]
pub(super) fn advance(
    guard: &BrokerPlatformGuard,
    prior: &BrokerRecord,
    next: TransitionRequest<'_>,
) -> Result<BrokerRecord, TransitionFailure> {
    guard
        .revalidate_live()
        .map_err(TransitionFailure::DefinitelyNotApplied)?;
    validate_request(guard, next).map_err(TransitionFailure::DefinitelyNotApplied)?;
    let current = guard
        .current(BrokerLookup {
            record_namespace: prior.record_namespace.as_slice(),
            schema_version: prior.schema_version,
            binding_version: prior.binding_version,
            database_identity: prior.database_identity,
            lookup_digest: &prior.lookup_digest,
        })
        .map_err(TransitionFailure::DefinitelyNotApplied)?
        .ok_or(TransitionFailure::DefinitelyNotApplied(
            PlatformError::Conflict,
        ))?;
    if !record::same(&current, prior)
        || prior.lookup_digest != *next.lookup_digest
        || prior.database_identity != next.database_identity
        || next.sequence != prior.sequence.saturating_add(1)
    {
        return Err(TransitionFailure::DefinitelyNotApplied(
            PlatformError::Conflict,
        ));
    }
    persist(guard, next)
}

#[cfg(windows)]
fn validate_request(
    guard: &BrokerPlatformGuard,
    request: TransitionRequest<'_>,
) -> Result<(), PlatformError> {
    let state = *guard.state.lock().map_err(map_poison)?;
    let identity = guard
        .database_identity
        .lock()
        .map_err(map_poison)?
        .ok_or(PlatformError::InvalidAttestation)?;
    if request.database_identity != identity
        || request.writer_epoch != state.writer_epoch
        || request.minimum_watermark > state.watermark
        || request.sequence == 0
    {
        return Err(PlatformError::InvalidAttestation);
    }
    if request.key_epoch != state.key_epoch {
        return Err(PlatformError::Rotated);
    }
    Ok(())
}

#[cfg(windows)]
fn persist(
    guard: &BrokerPlatformGuard,
    request: TransitionRequest<'_>,
) -> Result<BrokerRecord, TransitionFailure> {
    let mut ledger = guard
        .state
        .lock()
        .map_err(map_poison)
        .map_err(TransitionFailure::DefinitelyNotApplied)?;
    let watermark = ledger
        .watermark
        .max(request.minimum_watermark)
        .checked_add(1)
        .ok_or(TransitionFailure::DefinitelyNotApplied(
            PlatformError::AntiRollback,
        ))?;
    let plaintext = Zeroizing::new(
        record_codec::encode_transition(request, watermark)
            .map_err(TransitionFailure::DefinitelyNotApplied)?,
    );
    let sealed_record = crypto::protect_record(
        &guard.registry_id,
        request.lookup_digest,
        request.database_identity,
        plaintext.as_ref(),
    )
    .map_err(TransitionFailure::DefinitelyNotApplied)?;
    record::validate_ciphertext(&sealed_record).map_err(TransitionFailure::DefinitelyNotApplied)?;
    let next_ledger = state::LedgerState {
        watermark,
        ..*ledger
    };
    let sealed_state = state::seal(&guard.registry_id, next_ledger)
        .map_err(TransitionFailure::DefinitelyNotApplied)?;
    let record_name = record::record_name(request.lookup_digest);
    registry::write_batch(
        &guard.registry_id,
        &[
            registry::RuntimeMutation {
                name: &record_name,
                value: Some(&sealed_record),
            },
            registry::RuntimeMutation {
                name: state::STATE_VALUE_NAME,
                value: Some(&sealed_state),
            },
        ],
    )
    .map_err(map_batch_failure)?;
    *ledger = next_ledger;
    record_codec::decode_record(plaintext.as_ref(), sealed_record)
        .map_err(TransitionFailure::DefinitelyNotApplied)
}

#[cfg(windows)]
fn map_batch_failure(error: registry::RuntimeBatchFailure) -> TransitionFailure {
    match error {
        registry::RuntimeBatchFailure::DefinitelyNotApplied(error) => {
            TransitionFailure::DefinitelyNotApplied(error)
        }
        registry::RuntimeBatchFailure::OutcomeUnknown => TransitionFailure::OutcomeUnknown,
    }
}
