use super::{
    MdnsDnsSdPacket, MDNS_CLASS_IN, MDNS_HEADER_LEN, MDNS_SERVICE_ENUMERATION, MDNS_SERVICE_TYPES,
    MDNS_TYPE_PTR, MDNS_UNICAST_RESPONSE_BIT,
};

#[path = "packet_name.rs"]
mod packet_name;
#[path = "packet_parse.rs"]
mod packet_parse;
#[path = "packet_query.rs"]
mod packet_query;
#[path = "packet_records.rs"]
mod packet_records;

use packet_name::parse_dns_name as parse_dns_name_impl;

pub(super) fn parse_dns_name(payload: &[u8], offset: usize) -> Option<(String, usize)> {
    parse_dns_name_impl(payload, offset)
}

pub fn parse_mdns_packet(payload: &[u8]) -> Option<MdnsDnsSdPacket> {
    packet_parse::parse_mdns_packet(payload)
}

pub fn skip_mdns_questions(payload: &[u8], question_count: usize) -> usize {
    packet_parse::skip_mdns_questions(payload, question_count)
}

pub fn encode_mdns_query(query_name: &str) -> Vec<u8> {
    packet_query::encode_mdns_query(query_name)
}

pub fn mdns_query_names() -> Vec<&'static str> {
    packet_query::mdns_query_names()
}
