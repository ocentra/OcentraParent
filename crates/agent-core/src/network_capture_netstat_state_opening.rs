use ocentra_parent_agent_protocol::activity_capture::ActivityNetworkTcpState;
use ocentra_parent_agent_protocol::constants;

pub(crate) fn tcp_state_opening(state: &str) -> Option<ActivityNetworkTcpState> {
    match state {
        constants::activity_capture::NETSTAT_STATE_CLOSED => Some(ActivityNetworkTcpState::Closed),
        constants::activity_capture::NETSTAT_STATE_LISTENING => {
            Some(ActivityNetworkTcpState::Listen)
        }
        constants::activity_capture::NETSTAT_STATE_SYN_SENT => {
            Some(ActivityNetworkTcpState::SynSent)
        }
        _ => None,
    }
}
