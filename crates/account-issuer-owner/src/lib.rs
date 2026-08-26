#![forbid(unsafe_code)]

//! Account-owned v2 current-authority issuer.
//!
//! This crate owns orchestration and fail-closed platform seams.  The family
//! crate owns the SQLite connection, currentness capability, v2 payload, key
//! registry, receipt, and outbox transaction; this crate never opens a second
//! database connection or accepts a caller-supplied authority DTO.

pub mod contract;
pub mod currentness;
pub mod delivery;
pub mod key_registry;
pub mod outbox;
pub mod recovery;
pub mod repository;
pub mod rpc;
pub mod signing;
pub mod startup;
