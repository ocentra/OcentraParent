use super::super::http::sanitize_probe_text;
use super::super::{
    LanServiceIdentityProbeObservation, BER_TAG_OBJECT_IDENTIFIER, BER_TAG_OCTET_STRING,
    BER_TAG_SEQUENCE, SERVICE_IDENTITY_PROBE_MAX_TEXT_BYTES, SNMP_SYS_DESCR_OID, SNMP_SYS_NAME_OID,
};

mod response;

pub(super) fn parse_snmp_probe_observation(
    response: &[u8],
    expected_request_id: i64,
) -> Option<LanServiceIdentityProbeObservation> {
    response::parse_snmp_probe_observation(response, expected_request_id)
}

fn parse_snmp_probe_identity_fields(
    varbind_list_body: &[u8],
) -> Option<(Option<String>, Option<String>)> {
    let mut sys_descr = None;
    let mut sys_name = None;
    let mut varbind_cursor = 0_usize;
    while varbind_cursor < varbind_list_body.len() {
        let (varbind_tag, varbind_body, next_varbind_cursor) =
            super::parse_ber_tlv(varbind_list_body, varbind_cursor)?;
        if varbind_tag != BER_TAG_SEQUENCE {
            return None;
        }
        varbind_cursor = next_varbind_cursor;

        let (oid_tag, oid_body, value_cursor) = super::parse_ber_tlv(varbind_body, 0)?;
        if oid_tag != BER_TAG_OBJECT_IDENTIFIER {
            return None;
        }
        let oid = super::parse_ber_oid(oid_body)?;
        let (value_tag, value_body, _) = super::parse_ber_tlv(varbind_body, value_cursor)?;
        if value_tag != BER_TAG_OCTET_STRING {
            continue;
        }
        let value = std::str::from_utf8(value_body)
            .ok()
            .and_then(|text| sanitize_probe_text(text, SERVICE_IDENTITY_PROBE_MAX_TEXT_BYTES));
        if oid == SNMP_SYS_DESCR_OID {
            sys_descr = value;
        } else if oid == SNMP_SYS_NAME_OID {
            sys_name = value;
        }
    }
    Some((sys_descr, sys_name))
}
