use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use crate::authority::{
    sealed::TrustedAuthorityOwner, AuthorityError, CurrentBindingGuard, CurrentBindingPort,
};
use crate::binding::{Binding, BindingLocator};
use ocentra_protected_capability_custody_protocol::request::{ExpectedGenerations, RequestKind};

mod codec;
mod resolve;
mod storage;

pub(super) struct BrokerCurrentBindingAuthority {
    pub(super) registry_id: String,
    pub(super) bindings: Mutex<HashMap<[u8; 32], Binding>>,
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
        resolve::resolve_for_request(self, locator, kind, expected)
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

fn map_poison_error<T>(_error: std::sync::PoisonError<T>) -> AuthorityError {
    AuthorityError::Unavailable
}
