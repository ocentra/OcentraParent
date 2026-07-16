use super::{constants, DnsRecordData};

pub(super) fn dns_record_data(record_type: u16, data: &[u8]) -> DnsRecordData {
    if record_type == constants::DNS_TYPE_A && data.len() == constants::IPV4_RDATA_LEN {
        return DnsRecordData::Ipv4Address(ipv4_text(data));
    }

    DnsRecordData::Raw {
        byte_len: data.len(),
    }
}

fn ipv4_text(bytes: &[u8]) -> String {
    format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3])
}
