use std::time::{Duration, Instant};

use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use tungstenite::WebSocket;

use super::connection::DeadlineTcpStream;

pub(super) fn read_agent_event(
    socket: &mut WebSocket<DeadlineTcpStream>,
    phase: &str,
    timeout: Duration,
    deadline: Instant,
) -> Result<AgentEventEnvelope, String> {
    super::read_impl::read_agent_event(socket, phase, timeout, deadline)
}
