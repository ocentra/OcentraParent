use std::net::TcpStream;
use std::time::{Duration, Instant};

#[path = "connection_resolution.rs"]
mod connection_resolution;
#[path = "deadline_stream.rs"]
pub(super) mod deadline_stream;
use self::connection_resolution::resolve_socket_addrs;
use self::deadline_stream::DeadlineTcpStream;
use super::read_impl::is_io_timeout;

pub(super) fn connect_agent_stream(
    agent_addr: &str,
    url: &str,
    timeout: Duration,
    deadline: Instant,
) -> Result<DeadlineTcpStream, String> {
    let socket_addrs = resolve_socket_addrs(agent_addr, timeout, deadline)?;
    if socket_addrs.is_empty() {
        return Err(format!(
            "agent-service address {agent_addr} did not resolve to any socket addresses"
        ));
    }

    let mut last_error = None;
    for socket_addr in socket_addrs {
        let remaining = remaining_timeout(deadline).map_err(|_| {
            format!(
                "agent-service WebSocket connect timed out after {}ms at {url}",
                timeout.as_millis()
            )
        })?;
        match TcpStream::connect_timeout(&socket_addr, remaining) {
            Ok(stream) => {
                let stream = DeadlineTcpStream::new(stream, deadline);
                configure_socket_timeouts(&stream, remaining_timeout(deadline)?, url)?;
                return Ok(stream);
            }
            Err(error) => last_error = Some(error),
        }
    }

    if remaining_timeout(deadline).is_err() {
        return Err(format!(
            "agent-service WebSocket connect timed out after {}ms at {url}",
            timeout.as_millis()
        ));
    }

    Err(format!(
        "agent-service WebSocket connect failed at {url}: {}",
        connection_failure_detail(last_error, timeout)
    ))
}

pub(super) fn remaining_timeout(deadline: Instant) -> Result<Duration, String> {
    match deadline.checked_duration_since(Instant::now()) {
        Some(remaining) if !remaining.is_zero() => Ok(remaining),
        _ => Err("agent-service WebSocket overall deadline exhausted".to_string()),
    }
}

pub(super) fn configure_socket_timeouts(
    stream: &DeadlineTcpStream,
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
