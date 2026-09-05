use std::sync::MutexGuard;

use super::{support, CustodyError, CustodyStore};
use crate::authority::CurrentBindingGuard;
use crate::binding::{Binding, BindingLocator};

/// Enforces the per-transition part of the only legal lock order:
/// store-lifetime database custody guard -> store operation mutex -> Device
/// authority fence -> path/broker -> SQLite. No caller may hold a SQLite guard
/// while entering the broker. Field order below releases the inner authority
/// fence before the outer operation mutex.
pub(super) struct OperationScope<'a> {
    authority: Box<dyn CurrentBindingGuard + 'a>,
    // Rust drops fields in declaration order. Declare the inner authority
    // fence first so it is released before the outer operation mutex.
    _operation: MutexGuard<'a, ()>,
}

impl<'a> OperationScope<'a> {
    pub(super) fn acquire(
        store: &'a CustodyStore,
        locator: &BindingLocator,
    ) -> Result<Self, CustodyError> {
        let operation = store
            .operation
            .lock()
            .map_err(|_poison_error| CustodyError::Conflict)?;
        let authority = store
            .authority
            .lock_current(locator)
            .map_err(|error| support::map_authority_error(&error))?;
        if authority.binding().locator() != locator {
            return Err(CustodyError::WrongBinding);
        }
        store
            .secured_path
            .revalidate()
            .map_err(|error| support::map_path_error(&error))?;
        Ok(Self {
            authority,
            _operation: operation,
        })
    }

    pub(super) fn binding(&self) -> &Binding {
        self.authority.binding()
    }
}
