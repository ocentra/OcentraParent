use std::net::TcpStream;
use std::time::{Duration, Instant};

use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use tungstenite::WebSocket;

pub(super) fn read_agent_event(
    socket: &mut WebSocket<TcpStream>,
    phase: &str,
    timeout: Duration,
    deadline: Instant,
) -> Result<AgentEventEnvelope, String> {
    super::read_impl::read_agent_event(socket, phase, timeout, deadline)
}
