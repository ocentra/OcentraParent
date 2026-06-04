use crate::dns::{DnsObservation, DnsQueryType, NetworkEvidenceGrade};

pub fn dns_query_pcap_fixture() -> Vec<u8> {
    let dns_payload = dns_query_payload();
    let frame = ethernet_ipv4_udp_frame(&dns_payload);
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

fn ethernet_ipv4_udp_frame(dns_payload: &[u8]) -> Vec<u8> {
    let udp_len = 8 + dns_payload.len();
    let ip_len = 20 + udp_len;
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
    frame.push(17);
    frame.extend_from_slice(&0_u16.to_be_bytes());
    frame.extend_from_slice(&[192, 168, 1, 25]);
    frame.extend_from_slice(&[1, 1, 1, 1]);

    frame.extend_from_slice(&53_000_u16.to_be_bytes());
    frame.extend_from_slice(&53_u16.to_be_bytes());
    frame.extend_from_slice(&(udp_len as u16).to_be_bytes());
    frame.extend_from_slice(&0_u16.to_be_bytes());
    frame.extend_from_slice(dns_payload);

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
    push_label(&mut payload, b"video");
    push_label(&mut payload, b"example");
    push_label(&mut payload, b"test");
    payload.push(0);
    payload.extend_from_slice(&1_u16.to_be_bytes());
    payload.extend_from_slice(&1_u16.to_be_bytes());
    payload
}

fn push_label(payload: &mut Vec<u8>, label: &[u8]) {
    payload.push(label.len() as u8);
    payload.extend_from_slice(label);
}
