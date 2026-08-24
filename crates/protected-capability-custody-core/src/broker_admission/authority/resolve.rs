use crate::authority::AuthorityError;
use crate::binding::{Binding, BindingLocator, GenerationSlot, GenerationSlotName};
use ocentra_protected_capability_custody_protocol::request::{ExpectedGenerations, RequestKind};

use super::storage;
use super::BrokerCurrentBindingAuthority;

pub(super) fn resolve_for_request(
    authority: &BrokerCurrentBindingAuthority,
    locator: BindingLocator,
    kind: RequestKind,
    expected: ExpectedGenerations,
) -> Result<ExpectedGenerations, AuthorityError> {
    let lookup_digest = locator.lookup_digest();
    let mut bindings = authority.bindings.lock().map_err(map_poison_error)?;
    let Some(generations) = storage::load_generations(&authority.registry_id, &lookup_digest)?
    else {
        // First Prepare needs one protected transaction that commits generated
        // authority generations together with the custody reservation and its
        // recoverable response/token. Persisting generations first strands all
        // retries from `initial_binding` if custody fails or the broker crashes.
        // The transaction owner is not linked, so refuse without writing.
        return if kind == RequestKind::Prepare && expected == ExpectedGenerations::initial_binding()
        {
            Err(AuthorityError::Unavailable)
        } else {
            Err(AuthorityError::Rejected)
        };
    };
    if generations != expected {
        return Err(AuthorityError::Rejected);
    }
    let binding = binding_for(locator, generations)?;
    match bindings.get(&lookup_digest) {
        Some(current) if current == &binding => {}
        Some(_) => return Err(AuthorityError::Rejected),
        None => {
            if bindings.len()
                >= ocentra_protected_capability_custody_protocol::constants::MAX_ACTIVE_AUTHORITY_BINDINGS
            {
                return Err(AuthorityError::Unavailable);
            }
            bindings.insert(lookup_digest, binding);
        }
    }
    Ok(generations)
}

fn generation(name: GenerationSlotName, value: u64) -> Result<GenerationSlot, AuthorityError> {
    GenerationSlot::try_new(name, value).map_err(map_binding_error)
}

fn binding_for(
    locator: BindingLocator,
    generations: ExpectedGenerations,
) -> Result<Binding, AuthorityError> {
    Binding::try_new(
        locator,
        [
            generation(GenerationSlotName::Authority, generations.authority())?,
            generation(GenerationSlotName::Target, generations.target())?,
            generation(GenerationSlotName::Key, generations.key())?,
            generation(GenerationSlotName::Writer, generations.writer())?,
        ],
    )
    .map_err(map_binding_error)
}

fn map_binding_error(_error: crate::binding::BindingError) -> AuthorityError {
    AuthorityError::Rejected
}

fn map_poison_error<T>(_error: std::sync::PoisonError<T>) -> AuthorityError {
    AuthorityError::Unavailable
}
