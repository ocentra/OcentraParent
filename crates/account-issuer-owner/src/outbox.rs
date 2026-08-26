//! Outbox ownership is kept behind [`AccountIssuerRepository`](crate::repository::AccountIssuerRepository).
//!
//! The family crate exposes only high-level claim, failure, and receipt
//! transitions; this module intentionally contains no transaction or
//! reservation handle that could become a second mutation boundary.
