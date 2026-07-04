use ocentra_parent_agent_protocol::activity_capture::ActivityNetworkTcpState;
use ocentra_parent_agent_protocol::constants;

pub(crate) fn tcp_state_established(state: &str) -> Option<ActivityNetworkTcpState> {
    match state {
        constants::activity_capture::NETSTAT_STATE_SYN_RECEIVED => {
            Some(ActivityNetworkTcpState::SynReceived)
        }
        constants::activity_capture::NETSTAT_STATE_ESTABLISHED => {
            Some(ActivityNetworkTcpState::Established)
        }
        _ => None,
    }
}
