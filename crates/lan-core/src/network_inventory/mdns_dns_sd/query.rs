use std::net::{SocketAddrV4, UdpSocket};
use std::time::Duration;

use chrono::Utc;

use super::accumulator::parse_mdns_packets;
use super::packet::{encode_mdns_query, mdns_query_names};
use super::{MdnsDnsSdDiscovery, MDNS_IPV4_MULTICAST, MDNS_PORT, MDNS_RESPONSE_TIMEOUT_MS};

pub fn query_mdns_dns_sd() -> Option<MdnsDnsSdDiscovery> {
    let socket = UdpSocket::bind(("0.0.0.0", 0)).ok()?;
    let _ = socket.set_read_timeout(Some(Duration::from_millis(MDNS_RESPONSE_TIMEOUT_MS)));
    let _ = socket.set_write_timeout(Some(Duration::from_millis(MDNS_RESPONSE_TIMEOUT_MS)));
    let destination = SocketAddrV4::new(MDNS_IPV4_MULTICAST, MDNS_PORT);
    for query_name in mdns_query_names() {
        let query = encode_mdns_query(query_name);
        let _ = socket.send_to(&query, destination);
    }

    let observed_at = Utc::now().to_rfc3339();
    let mut buffered_packets = Vec::new();
    loop {
        let mut buffer = [0_u8; 4096];
        match socket.recv_from(&mut buffer) {
            Ok((received, _source)) => {
                buffered_packets.push(buffer[..received].to_vec());
            }
            Err(error)
                if error.kind() == std::io::ErrorKind::WouldBlock
                    || error.kind() == std::io::ErrorKind::TimedOut =>
            {
                break;
            }
            Err(_) => return None,
        }
    }

    parse_mdns_packets(&buffered_packets, observed_at)
}
