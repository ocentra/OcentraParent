use super::super::super::{
    LanServiceIdentityProbeObservation, BER_TAG_INTEGER, BER_TAG_OCTET_STRING, BER_TAG_SEQUENCE,
    SNMP_GET_RESPONSE_TAG, SNMP_VERSION_2C,
};

pub(super) fn parse_snmp_probe_observation(
    response: &[u8],
    expected_request_id: i64,
) -> Option<LanServiceIdentityProbeObservation> {
    let pdu_body = parse_snmp_message(response)?;
    let varbind_list_body = parse_response_pdu(pdu_body, expected_request_id)?;
    let (sys_descr, sys_name) = super::parse_snmp_probe_identity_fields(varbind_list_body)?;
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

fn parse_snmp_message(response: &[u8]) -> Option<&[u8]> {
    let (message_tag, message_body, _) = super::super::parse_ber_tlv(response, 0)?;
    if message_tag != BER_TAG_SEQUENCE {
        return None;
    }
    let (version_tag, version_body, cursor) = super::super::parse_ber_tlv(message_body, 0)?;
    if version_tag != BER_TAG_INTEGER
        || super::super::parse_ber_integer(version_body)? != SNMP_VERSION_2C
    {
        return None;
    }
    let (community_tag, _, next_cursor) = super::super::parse_ber_tlv(message_body, cursor)?;
    if community_tag != BER_TAG_OCTET_STRING {
        return None;
    }
    let (pdu_tag, pdu_body, _) = super::super::parse_ber_tlv(message_body, next_cursor)?;
    (pdu_tag == SNMP_GET_RESPONSE_TAG).then_some(pdu_body)
}

fn parse_response_pdu(pdu_body: &[u8], expected_request_id: i64) -> Option<&[u8]> {
    let (request_id_tag, request_id_body, cursor) = super::super::parse_ber_tlv(pdu_body, 0)?;
    if request_id_tag != BER_TAG_INTEGER
        || super::super::parse_ber_integer(request_id_body)? != expected_request_id
    {
        return None;
    }
    let (error_status_tag, error_status_body, cursor) =
        super::super::parse_ber_tlv(pdu_body, cursor)?;
    if error_status_tag != BER_TAG_INTEGER
        || super::super::parse_ber_integer(error_status_body)? != 0
    {
        return None;
    }
    let (error_index_tag, error_index_body, cursor) =
        super::super::parse_ber_tlv(pdu_body, cursor)?;
    if error_index_tag != BER_TAG_INTEGER || super::super::parse_ber_integer(error_index_body)? != 0
    {
        return None;
    }
    let (varbind_list_tag, varbind_list_body, _) = super::super::parse_ber_tlv(pdu_body, cursor)?;
    (varbind_list_tag == BER_TAG_SEQUENCE).then_some(varbind_list_body)
}
