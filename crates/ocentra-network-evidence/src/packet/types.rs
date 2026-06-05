#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedNetworkPacket {
    pub ethernet: EthernetFrameMetadata,
    pub ipv4: Option<Ipv4PacketMetadata>,
    pub transport: Option<TransportPacketMetadata>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EthernetFrameMetadata {
    pub source_mac: String,
    pub destination_mac: String,
    pub ether_type: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ipv4PacketMetadata {
    pub source_ip: String,
    pub destination_ip: String,
    pub protocol: IpProtocol,
    pub header_len: usize,
    pub total_len: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpProtocol {
    Icmp,
    Tcp,
    Udp,
    Other(u8),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransportPacketMetadata {
    Udp {
        source_port: u16,
        destination_port: u16,
        payload_len: usize,
    },
    Tcp {
        source_port: u16,
        destination_port: u16,
        header_len: usize,
        payload_len: usize,
    },
    Icmp {
        icmp_type: u8,
        code: u8,
        payload_len: usize,
    },
    Other {
        protocol: u8,
        payload_len: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UdpPayloadView<'a> {
    pub source_ip: String,
    pub destination_ip: String,
    pub source_port: u16,
    pub destination_port: u16,
    pub payload: &'a [u8],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PacketParseError {
    EthernetFrameTooShort,
    Ipv4HeaderTooShort,
    Ipv4PacketTruncated,
    UdpHeaderTooShort,
    UdpPacketTruncated,
    TcpHeaderTooShort,
    TcpSegmentTruncated,
    IcmpPacketTooShort,
}
