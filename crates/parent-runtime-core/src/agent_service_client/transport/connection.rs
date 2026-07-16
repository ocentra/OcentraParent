use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

use super::read_impl::is_io_timeout;

pub(super) fn connect_agent_stream(
    agent_addr: &str,
    url: &str,
    timeout: Duration,
) -> Result<TcpStream, String> {
    let socket_addrs = resolve_socket_addrs(agent_addr)?;
    if socket_addrs.is_empty() {
        return Err(format!(
            "agent-service address {agent_addr} did not resolve to any socket addresses"
        ));
    }

    let mut last_error = None;
    for socket_addr in socket_addrs {
        match TcpStream::connect_timeout(&socket_addr, timeout) {
            Ok(stream) => {
                configure_socket_timeouts(&stream, timeout, url)?;
                return Ok(stream);
            }
            Err(error) => last_error = Some(error),
        }
    }

    Err(format!(
        "agent-service WebSocket connect failed at {url}: {}",
        connection_failure_detail(last_error, timeout)
    ))
}

fn resolve_socket_addrs(agent_addr: &str) -> Result<Vec<std::net::SocketAddr>, String> {
    agent_addr
        .to_socket_addrs()
        .map(|socket_addrs| socket_addrs.collect())
        .map_err(|error| format!("agent-service address {agent_addr} did not resolve: {error}"))
}

fn configure_socket_timeouts(
    stream: &TcpStream,
    timeout: Duration,
    url: &str,
) -> Result<(), String> {
    stream.set_read_timeout(Some(timeout)).map_err(|error| {
        format!("agent-service WebSocket read timeout setup failed at {url}: {error}")
    })?;
    stream.set_write_timeout(Some(timeout)).map_err(|error| {
        format!("agent-service WebSocket write timeout setup failed at {url}: {error}")
    })
}

fn connection_failure_detail(last_error: Option<std::io::Error>, timeout: Duration) -> String {
    last_error
        .map(|error| {
            if is_io_timeout(&error) {
                format!("connect timed out after {}ms", timeout.as_millis())
            } else {
                error.to_string()
            }
        })
        .unwrap_or_else(|| "no socket address was attempted".to_string())
}
