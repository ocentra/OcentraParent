use std::io::{Read, Write};
use std::time::Duration;

use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use tungstenite::WebSocket;

pub(super) fn read_agent_event<S: Read + Write>(
    socket: &mut WebSocket<S>,
    phase: &str,
    timeout: Duration,
) -> Result<AgentEventEnvelope, String> {
    super::read_impl::read_agent_event(socket, phase, timeout)
}
