use super::super::http::sanitize_probe_text;
use super::super::{
    LanServiceIdentityProbeObservation, SERVICE_IDENTITY_PROBE_MAX_TEXT_BYTES, BER_TAG_INTEGER,
    BER_TAG_NULL, BER_TAG_OBJECT_IDENTIFIER, BER_TAG_OCTET_STRING, BER_TAG_SEQUENCE,
    SNMP_GET_RESPONSE_TAG, SNMP_SYS_DESCR_OID, SNMP_SYS_NAME_OID, SNMP_VERSION_2C,
};

pub(super) fn parse_snmp_probe_observation(
    response: &[u8],
    expected_request_id: i64,
) -> Option<LanServiceIdentityProbeObservation> {
    let (message_tag, message_body, _) = super::parse_ber_tlv(response, 0)?;
    if message_tag != BER_TAG_SEQUENCE {
        return None;
    }
    let (version_tag, version_body, mut cursor) = super::parse_ber_tlv(message_body, 0)?;
    if version_tag != BER_TAG_INTEGER || super::parse_ber_integer(version_body)? != SNMP_VERSION_2C {
        return None;
    }
    let (community_tag, _, next_cursor) = super::parse_ber_tlv(message_body, cursor)?;
    if community_tag != BER_TAG_OCTET_STRING {
        return None;
    }
    cursor = next_cursor;
    let (pdu_tag, pdu_body, _) = super::parse_ber_tlv(message_body, cursor)?;
    if pdu_tag != SNMP_GET_RESPONSE_TAG {
        return None;
    }

    let (request_id_tag, request_id_body, mut pdu_cursor) = super::parse_ber_tlv(pdu_body, 0)?;
    if request_id_tag != BER_TAG_INTEGER
        || super::parse_ber_integer(request_id_body)? != expected_request_id
    {
        return None;
    }
    let (error_status_tag, error_status_body, next_pdu_cursor) =
        super::parse_ber_tlv(pdu_body, pdu_cursor)?;
    if error_status_tag != BER_TAG_INTEGER || super::parse_ber_integer(error_status_body)? != 0 {
        return None;
    }
    pdu_cursor = next_pdu_cursor;
    let (error_index_tag, error_index_body, next_pdu_cursor) =
        super::parse_ber_tlv(pdu_body, pdu_cursor)?;
    if error_index_tag != BER_TAG_INTEGER || super::parse_ber_integer(error_index_body)? != 0 {
        return None;
    }
    pdu_cursor = next_pdu_cursor;
    let (varbind_list_tag, varbind_list_body, _) = super::parse_ber_tlv(pdu_body, pdu_cursor)?;
    if varbind_list_tag != BER_TAG_SEQUENCE {
        return None;
    }
    let (sys_descr, sys_name) = parse_snmp_probe_identity_fields(varbind_list_body)?;

    let observation = LanServiceIdentityProbeObservation {
        status_code: None,
        title: None,
        server_header: None,
        banner: None,
        redirect_location: None,
        certificate_subject: None,
        descriptor_links: Vec::new(),
        wsd_endpoint_address: None,
        wsd_types: None,
        snmp_sys_descr: sys_descr,
        snmp_sys_name: sys_name,
    };
    observation.is_meaningful().then_some(observation)
}

fn parse_snmp_probe_identity_fields(varbind_list_body: &[u8]) -> Option<(Option<String>, Option<String>)> {
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
