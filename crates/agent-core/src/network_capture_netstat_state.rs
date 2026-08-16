use ocentra_parent_agent_protocol::activity_capture::ActivityNetworkTcpState;

mod network_capture_netstat_state_established;
mod network_capture_netstat_state_finishing;
mod network_capture_netstat_state_opening;

pub(crate) fn tcp_state_from_netstat(state: &str) -> ActivityNetworkTcpState {
    network_capture_netstat_state_opening::tcp_state_opening(state)
        .or_else(|| network_capture_netstat_state_established::tcp_state_established(state))
        .or_else(|| network_capture_netstat_state_finishing::tcp_state_finishing(state))
        .unwrap_or(ActivityNetworkTcpState::Unknown)
}
