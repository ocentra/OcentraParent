use std::net::{SocketAddr, TcpStream};
use std::sync::atomic::AtomicBool;
use std::time::Instant;

use super::deadline::{poll_timeout, retryable};

pub(super) fn connect_until(
    endpoint: SocketAddr,
    deadline: Instant,
    cancellation: Option<&AtomicBool>,
) -> Option<TcpStream> {
    loop {
        match TcpStream::connect_timeout(&endpoint, poll_timeout(deadline, cancellation)?) {
            Ok(stream) => return Some(stream),
            Err(error) if retryable(&error) => continue,
            Err(_) => return None,
        }
    }
}
