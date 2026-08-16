use ocentra_parent_agent_protocol::activity_capture::ActivityNetworkTcpState;
use ocentra_parent_agent_protocol::constants;

pub(crate) fn tcp_state_finishing(state: &str) -> Option<ActivityNetworkTcpState> {
    match state {
        constants::activity_capture::NETSTAT_STATE_FIN_WAIT_1 => {
            Some(ActivityNetworkTcpState::FinWait1)
        }
        constants::activity_capture::NETSTAT_STATE_FIN_WAIT_2 => {
            Some(ActivityNetworkTcpState::FinWait2)
        }
        constants::activity_capture::NETSTAT_STATE_CLOSE_WAIT => {
            Some(ActivityNetworkTcpState::CloseWait)
        }
        constants::activity_capture::NETSTAT_STATE_CLOSING => {
            Some(ActivityNetworkTcpState::Closing)
        }
        constants::activity_capture::NETSTAT_STATE_LAST_ACK => {
            Some(ActivityNetworkTcpState::LastAck)
        }
        constants::activity_capture::NETSTAT_STATE_TIME_WAIT => {
            Some(ActivityNetworkTcpState::TimeWait)
        }
        constants::activity_capture::NETSTAT_STATE_DELETE_TCB => {
            Some(ActivityNetworkTcpState::DeleteTcb)
        }
        _ => None,
    }
}
