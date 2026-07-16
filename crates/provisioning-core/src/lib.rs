#![forbid(unsafe_code)]

//! Rust-owned provisioning and device-trust readiness boundary.
//!
//! This crate owns the readiness state machine, action/event contracts, and
//! family-context projection used by the device-trust bootstrap slice.

pub mod provisioning_install;
