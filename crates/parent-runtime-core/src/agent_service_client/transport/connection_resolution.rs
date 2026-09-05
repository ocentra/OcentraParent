use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::mpsc;
use std::time::{Duration, Instant};

pub(super) fn resolve_socket_addrs(
    agent_addr: &str,
    timeout: Duration,
    deadline: Instant,
) -> Result<Vec<SocketAddr>, String> {
    if let Ok(socket_addr) = agent_addr.parse::<SocketAddr>() {
        return Ok(vec![socket_addr]);
    }

    let (sender, receiver) = mpsc::channel();
    let address = agent_addr.to_string();
    std::thread::Builder::new()
        .name("parent-agent-dns-resolution".to_string())
        .spawn(move || {
            let result = address
                .to_socket_addrs()
                .map(|socket_addrs| socket_addrs.collect::<Vec<_>>())
                .map_err(|error| {
                    format!("agent-service address {address} did not resolve: {error}")
                });
            let _ = sender.send(result);
        })
        .map_err(|error| {
            format!("agent-service address {agent_addr} resolution worker could not start: {error}")
        })?;

    let remaining = super::remaining_timeout(deadline).map_err(|error| {
        format!(
            "agent-service address {agent_addr} resolution timed out after {}ms: {error}",
            timeout.as_millis(),
        )
    })?;
    match receiver.recv_timeout(remaining) {
        Ok(result) => result,
        Err(mpsc::RecvTimeoutError::Timeout) => Err(format!(
            "agent-service address {agent_addr} resolution timed out after {}ms",
            timeout.as_millis()
        )),
        Err(mpsc::RecvTimeoutError::Disconnected) => Err(format!(
            "agent-service address {agent_addr} resolution worker disconnected"
        )),
    }
}
