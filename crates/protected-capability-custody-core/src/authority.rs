use thiserror::Error;

use crate::binding::{Binding, BindingLocator};

mod sealed {
    /// Implement only beside the dependency-owned production authority
    /// adapter. There is intentionally no blanket implementation.
    pub(crate) trait TrustedAuthorityOwner {}
}

#[derive(Debug, Error)]
pub(crate) enum AuthorityError {
    #[error("current binding authority is unavailable")]
    Unavailable,
    #[error("current binding authority rejected the locator")]
    Rejected,
}

/// Holds the dependency owner's real cross-process transition fence until
/// dropped. The binding cannot change while this guard is alive.
pub(crate) trait CurrentBindingGuard {
    fn binding(&self) -> &Binding;
}

/// Acquires the dependency-owned transition fence and resolves the binding
/// while that fence is held. This trait is crate-private so external callers
/// cannot substitute snapshot-only or self-attesting authority.
pub(crate) trait CurrentBindingPort: sealed::TrustedAuthorityOwner + Send + Sync {
    fn lock_current<'a>(
        &'a self,
        locator: &BindingLocator,
    ) -> Result<Box<dyn CurrentBindingGuard + 'a>, AuthorityError>;
}
