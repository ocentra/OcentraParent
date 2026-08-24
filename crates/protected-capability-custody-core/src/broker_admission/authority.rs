use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use crate::authority::{
    sealed::TrustedAuthorityOwner, AuthorityError, CurrentBindingGuard, CurrentBindingPort,
};
use crate::binding::{Binding, BindingLocator, GenerationSlot, GenerationSlotName};
use ocentra_protected_capability_custody_protocol::request::{ExpectedGenerations, RequestKind};

mod codec;
mod storage;

pub(super) struct BrokerCurrentBindingAuthority {
    registry_id: String,
    bindings: Mutex<HashMap<[u8; 32], Binding>>,
}

impl BrokerCurrentBindingAuthority {
    pub(super) fn new(registry_id: String) -> Self {
        Self {
            registry_id,
            bindings: Mutex::new(HashMap::new()),
        }
    }

    pub(super) fn resolve_for_request(
        &self,
        locator: BindingLocator,
        kind: RequestKind,
        expected: ExpectedGenerations,
    ) -> Result<ExpectedGenerations, AuthorityError> {
        let lookup_digest = locator.lookup_digest();
        let (generations, created) =
            storage::load_or_create_generations(&self.registry_id, &lookup_digest, kind)?;
        if (created && expected != ExpectedGenerations::initial_binding())
            || (!created && generations != expected)
        {
            return Err(AuthorityError::Rejected);
        }
        let binding = Binding::try_new(
            locator,
            [
                generation(GenerationSlotName::Authority, generations.authority())?,
                generation(GenerationSlotName::Target, generations.target())?,
                generation(GenerationSlotName::Key, generations.key())?,
                generation(GenerationSlotName::Writer, generations.writer())?,
            ],
        )
        .map_err(map_binding_error)?;
        let mut bindings = self.bindings.lock().map_err(map_poison_error)?;
        match bindings.get(&lookup_digest) {
            Some(current) if current == &binding => {}
            Some(_) => return Err(AuthorityError::Rejected),
            None => {
                bindings.insert(lookup_digest, binding);
            }
        }
        Ok(generations)
    }
}

impl TrustedAuthorityOwner for BrokerCurrentBindingAuthority {}

impl CurrentBindingPort for BrokerCurrentBindingAuthority {
    fn lock_current<'a>(
        &'a self,
        locator: &BindingLocator,
    ) -> Result<Box<dyn CurrentBindingGuard + 'a>, AuthorityError> {
        let bindings = self.bindings.lock().map_err(map_poison_error)?;
        let binding = bindings
            .get(&locator.lookup_digest())
            .cloned()
            .ok_or(AuthorityError::Unavailable)?;
        Ok(Box::new(BrokerBindingGuard {
            _bindings: bindings,
            binding,
        }))
    }
}

struct BrokerBindingGuard<'a> {
    _bindings: MutexGuard<'a, HashMap<[u8; 32], Binding>>,
    binding: Binding,
}

impl CurrentBindingGuard for BrokerBindingGuard<'_> {
    fn binding(&self) -> &Binding {
        &self.binding
    }
}

fn generation(name: GenerationSlotName, value: u64) -> Result<GenerationSlot, AuthorityError> {
    GenerationSlot::try_new(name, value).map_err(map_binding_error)
}

fn map_binding_error(_error: crate::binding::BindingError) -> AuthorityError {
    AuthorityError::Rejected
}

fn map_poison_error<T>(_error: std::sync::PoisonError<T>) -> AuthorityError {
    AuthorityError::Unavailable
}
