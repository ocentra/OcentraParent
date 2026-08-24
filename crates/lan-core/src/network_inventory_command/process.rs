use std::{sync::atomic::AtomicBool, time::Duration};

use ocentra_parent_agent_protocol::constants;

use super::ProtectedCommandAdapterState;

pub(super) const fn protected_adapter_state() -> ProtectedCommandAdapterState {
    ProtectedCommandAdapterState::Unavailable
}

pub(super) fn command_stdout(program: &str, args: &[&str]) -> Option<String> {
    let timeout =
        Duration::from_millis(constants::lan_pairing::LAN_NETWORK_INVENTORY_COMMAND_TIMEOUT_MS);
    command_stdout_with_timeout(program, args, timeout)
}

pub(super) fn command_stdout_with_timeout(
    _program: &str,
    _args: &[&str],
    _timeout: Duration,
) -> Option<String> {
    // External inventory commands remain unavailable until a protected
    // platform adapter can prove executable identity and owned process-tree
    // cleanup. Never resolve through ambient PATH or approximate custody.
    None
}

pub(super) fn command_succeeded_with_timeout(
    _program: &str,
    _args: &[&str],
    _timeout: Duration,
) -> bool {
    false
}

pub(super) fn command_stdout_with_timeout_and_cancellation(
    _program: &str,
    _args: &[&str],
    _timeout: Duration,
    _cancellation: &AtomicBool,
) -> Option<String> {
    None
}
