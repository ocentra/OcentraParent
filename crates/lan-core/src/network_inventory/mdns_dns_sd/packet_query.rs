use super::{
    MDNS_CLASS_IN, MDNS_SERVICE_ENUMERATION, MDNS_SERVICE_TYPES, MDNS_TYPE_PTR,
    MDNS_UNICAST_RESPONSE_BIT,
};

pub(super) fn encode_mdns_query(query_name: &str) -> Vec<u8> {
    let mut packet = Vec::new();
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&1_u16.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    for label in query_name.split('.') {
        packet.push(label.len() as u8);
        packet.extend_from_slice(label.as_bytes());
    }
    packet.push(0);
    packet.extend_from_slice(&MDNS_TYPE_PTR.to_be_bytes());
    packet.extend_from_slice(&(MDNS_CLASS_IN | MDNS_UNICAST_RESPONSE_BIT).to_be_bytes());
    packet
}

pub(super) fn mdns_query_names() -> Vec<&'static str> {
    let mut names = vec![MDNS_SERVICE_ENUMERATION];
    names.extend_from_slice(MDNS_SERVICE_TYPES);
    names
}
