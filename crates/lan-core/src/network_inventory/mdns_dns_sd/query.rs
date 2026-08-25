use std::net::{SocketAddrV4, UdpSocket};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use chrono::Utc;

use super::accumulator::parse_mdns_packets;
use super::packet::{encode_mdns_query, mdns_query_names};
use super::{MdnsDnsSdDiscovery, MDNS_IPV4_MULTICAST, MDNS_PORT, MDNS_RESPONSE_TIMEOUT_MS};

pub fn query_mdns_dns_sd() -> Option<MdnsDnsSdDiscovery> {
    query_mdns_dns_sd_with_cancellation(None, None)
}

pub fn query_mdns_dns_sd_with_cancellation(
    cancellation: Option<&AtomicBool>,
    outer_deadline: Option<Instant>,
) -> Option<MdnsDnsSdDiscovery> {
    let local_deadline = Instant::now() + Duration::from_millis(MDNS_RESPONSE_TIMEOUT_MS);
    let deadline = outer_deadline.map_or(local_deadline, |outer| outer.min(local_deadline));
    if unavailable(cancellation, deadline) {
        return None;
    }
    let socket = UdpSocket::bind(("0.0.0.0", 0)).ok()?;
    let _ = socket.set_read_timeout(Some(poll_timeout(deadline, cancellation)?));
    let _ = socket.set_write_timeout(Some(poll_timeout(deadline, cancellation)?));
    let destination = SocketAddrV4::new(MDNS_IPV4_MULTICAST, MDNS_PORT);
    for query_name in mdns_query_names() {
        if unavailable(cancellation, deadline) {
            return None;
        }
        let query = encode_mdns_query(query_name);
        let _ = socket.send_to(&query, destination);
    }

    let observed_at = Utc::now().to_rfc3339();
    let mut buffered_packets = Vec::new();
    loop {
        if cancellation.is_some_and(|value| value.load(Ordering::Acquire)) {
            return None;
        }
        if Instant::now() >= deadline {
            break;
        }
        let _ = socket.set_read_timeout(Some(poll_timeout(deadline, cancellation)?));
        let mut buffer = [0_u8; 4096];
        match socket.recv_from(&mut buffer) {
            Ok((received, _source)) => {
                buffered_packets.push(buffer[..received].to_vec());
            }
            Err(error)
                if error.kind() == std::io::ErrorKind::WouldBlock
                    || error.kind() == std::io::ErrorKind::TimedOut =>
            {
                continue;
            }
            Err(_) => return None,
        }
    }

    parse_mdns_packets(&buffered_packets, observed_at)
}

const MDNS_IO_POLL_SLICE: Duration = Duration::from_millis(50);

fn unavailable(cancellation: Option<&AtomicBool>, deadline: Instant) -> bool {
    cancellation.is_some_and(|value| value.load(Ordering::Acquire)) || Instant::now() >= deadline
}

fn poll_timeout(deadline: Instant, cancellation: Option<&AtomicBool>) -> Option<Duration> {
    if unavailable(cancellation, deadline) {
        return None;
    }
    Some(
        deadline
            .saturating_duration_since(Instant::now())
            .min(MDNS_IO_POLL_SLICE),
    )
}
