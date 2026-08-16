use super::*;

pub(super) fn agent_command_timeout() -> Duration {
    Duration::from_secs(5)
}

pub(super) fn agent_command_timeout_for(command: &AgentCommandName) -> Duration {
    match command {
        AgentCommandName::AgentLanPairingStatusGet
        | AgentCommandName::AgentLanPairingBrowserDiscoveryScan
        | AgentCommandName::AgentLanPairingAddDeviceRequest => Duration::from_secs(15),
        _ => agent_command_timeout(),
    }
}

pub(super) fn agent_health_check_timeout_ms() -> u64 {
    agent_command_timeout_for(&AgentCommandName::AgentHealthCheck).as_millis() as u64
}
