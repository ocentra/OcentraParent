use thiserror::Error;

use crate::binding::{Binding, BindingLocator};

#[derive(Debug, Error)]
pub enum AuthorityError {
    #[error("current binding authority is unavailable")]
    Unavailable,
    #[error("current binding authority rejected the locator")]
    Rejected,
}

/// Resolves the dependency-owned, current binding immediately before custody
/// transitions. Implementations must not echo unverified caller input.
pub trait CurrentBindingPort: Send + Sync {
    fn resolve_current(&self, locator: &BindingLocator) -> Result<Binding, AuthorityError>;
}
