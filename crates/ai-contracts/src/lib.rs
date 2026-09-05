#![forbid(unsafe_code)]

//! Rust-owned canonical contracts for the shared AI contract family.
//!
//! This neutral leaf owns the serialized AI shapes and their validation and
//! digest bindings. Runtime, provider, policy, and enforcement crates consume
//! these contracts; none of those consumers can mint authority through this
//! crate.

pub mod ai_contracts;
pub mod ai_contracts_ts;
