#![forbid(unsafe_code)]

//! Rust-owned cross-boundary contracts.
//!
//! This crate is the contract authority for shapes that cross runtime,
//! process, host, and UI bridge boundaries. Domain crates own behavior;
//! this crate owns serializable DTO shape.

pub mod parent_ui_bridge;
pub mod parent_ui_bridge_ts;
