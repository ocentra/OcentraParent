use crate::dns::types::{DnsObservation, DnsQueryType, NetworkEvidenceGrade};

pub mod visibility;

pub fn dns_query_pcap_fixture() -> Vec<u8> {
    let frame = dns_query_frame_fixture();
    let mut pcap = Vec::new();

    pcap.extend_from_slice(&[0xd4, 0xc3, 0xb2, 0xa1]);
    pcap.extend_from_slice(&2_u16.to_le_bytes());
    pcap.extend_from_slice(&4_u16.to_le_bytes());
    pcap.extend_from_slice(&0_i32.to_le_bytes());
    pcap.extend_from_slice(&0_u32.to_le_bytes());
    pcap.extend_from_slice(&65_535_u32.to_le_bytes());
    pcap.extend_from_slice(&1_u32.to_le_bytes());

    pcap.extend_from_slice(&1_765_000_000_u32.to_le_bytes());
    pcap.extend_from_slice(&123_000_u32.to_le_bytes());
    pcap.extend_from_slice(&(frame.len() as u32).to_le_bytes());
    pcap.extend_from_slice(&(frame.len() as u32).to_le_bytes());
    pcap.extend_from_slice(&frame);

    pcap
}

pub fn dns_query_replay_expected() -> DnsObservation {
    DnsObservation {
        transaction_id: 0x1234,
        query_name: "video.example.test".to_owned(),
        query_type: DnsQueryType::A,
        source_ip: "192.168.1.25".to_owned(),
        destination_ip: "1.1.1.1".to_owned(),
        source_port: 53_000,
        destination_port: 53,
        observed_at_micros: 1_765_000_000_123_000,
        evidence_grade: NetworkEvidenceGrade::B,
        exact_url_available: false,
        decrypted_payload_available: false,
    }
}

pub fn dns_query_frame_fixture() -> Vec<u8> {
    ethernet_ipv4_udp_frame(&dns_query_payload())
}

pub fn dns_response_payload_fixture() -> Vec<u8> {
    let mut payload = Vec::new();
    payload.extend_from_slice(&0x1234_u16.to_be_bytes());
    payload.extend_from_slice(&0x8180_u16.to_be_bytes());
    payload.extend_from_slice(&1_u16.to_be_bytes());
    payload.extend_from_slice(&1_u16.to_be_bytes());
    payload.extend_from_slice(&0_u16.to_be_bytes());
    payload.extend_from_slice(&0_u16.to_be_bytes());
    push_dns_query_name(&mut payload);
    payload.extend_from_slice(&1_u16.to_be_bytes());
    payload.extend_from_slice(&1_u16.to_be_bytes());
    payload.extend_from_slice(&[0xc0, 0x0c]);
    payload.extend_from_slice(&1_u16.to_be_bytes());
    payload.extend_from_slice(&1_u16.to_be_bytes());
    payload.extend_from_slice(&300_u32.to_be_bytes());
    payload.extend_from_slice(&4_u16.to_be_bytes());
    payload.extend_from_slice(&[203, 0, 113, 7]);
    payload
}

pub fn tcp_syn_frame_fixture() -> Vec<u8> {
    let mut tcp = Vec::new();
    tcp.extend_from_slice(&53_001_u16.to_be_bytes());
    tcp.extend_from_slice(&443_u16.to_be_bytes());
    tcp.extend_from_slice(&0_u32.to_be_bytes());
    tcp.extend_from_slice(&0_u32.to_be_bytes());
    tcp.push(0x50);
    tcp.push(0x02);
    tcp.extend_from_slice(&1024_u16.to_be_bytes());
    tcp.extend_from_slice(&0_u16.to_be_bytes());
    tcp.extend_from_slice(&0_u16.to_be_bytes());
    ethernet_ipv4_frame(6, &tcp, [192, 168, 1, 25], [203, 0, 113, 10])
}

pub fn icmp_echo_frame_fixture() -> Vec<u8> {
    let mut icmp = Vec::new();
    icmp.push(8);
    icmp.push(0);
    icmp.extend_from_slice(&0_u16.to_be_bytes());
    icmp.extend_from_slice(&7_u16.to_be_bytes());
    icmp.extend_from_slice(&1_u16.to_be_bytes());
    ethernet_ipv4_frame(1, &icmp, [192, 168, 1, 25], [198, 51, 100, 20])
}

fn ethernet_ipv4_udp_frame(dns_payload: &[u8]) -> Vec<u8> {
    let udp_len = 8 + dns_payload.len();
    let mut udp = Vec::new();
    udp.extend_from_slice(&53_000_u16.to_be_bytes());
    udp.extend_from_slice(&53_u16.to_be_bytes());
    udp.extend_from_slice(&(udp_len as u16).to_be_bytes());
    udp.extend_from_slice(&0_u16.to_be_bytes());
    udp.extend_from_slice(dns_payload);
    ethernet_ipv4_frame(17, &udp, [192, 168, 1, 25], [1, 1, 1, 1])
}

fn ethernet_ipv4_frame(
    protocol: u8,
    payload: &[u8],
    source_ip: [u8; 4],
    destination_ip: [u8; 4],
) -> Vec<u8> {
    let ip_len = 20 + payload.len();
    let mut frame = Vec::new();
    frame.extend_from_slice(&[0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff]);
    frame.extend_from_slice(&[0x10, 0x20, 0x30, 0x40, 0x50, 0x60]);
    frame.extend_from_slice(&0x0800_u16.to_be_bytes());
    frame.push(0x45);
    frame.push(0);
    frame.extend_from_slice(&(ip_len as u16).to_be_bytes());
    frame.extend_from_slice(&0_u16.to_be_bytes());
    frame.extend_from_slice(&0_u16.to_be_bytes());
    frame.push(64);
    frame.push(protocol);
    frame.extend_from_slice(&0_u16.to_be_bytes());
    frame.extend_from_slice(&source_ip);
    frame.extend_from_slice(&destination_ip);
    frame.extend_from_slice(payload);
    frame
}

fn dns_query_payload() -> Vec<u8> {
    let mut payload = Vec::new();
    payload.extend_from_slice(&0x1234_u16.to_be_bytes());
    payload.extend_from_slice(&0x0100_u16.to_be_bytes());
    payload.extend_from_slice(&1_u16.to_be_bytes());
    payload.extend_from_slice(&0_u16.to_be_bytes());
    payload.extend_from_slice(&0_u16.to_be_bytes());
    payload.extend_from_slice(&0_u16.to_be_bytes());
    push_dns_query_name(&mut payload);
    payload.extend_from_slice(&1_u16.to_be_bytes());
    payload.extend_from_slice(&1_u16.to_be_bytes());
    payload
}

fn push_dns_query_name(payload: &mut Vec<u8>) {
    push_label(payload, b"video");
    push_label(payload, b"example");
    push_label(payload, b"test");
    payload.push(0);
}

fn push_label(payload: &mut Vec<u8>, label: &[u8]) {
    payload.push(label.len() as u8);
    payload.extend_from_slice(label);
}
