use super::{codecs, PassiveDhcpObservation};

mod message_type;

pub(super) fn parse_passive_dhcp_options(
    payload: &[u8],
    observation: &mut PassiveDhcpObservation,
) -> Option<()> {
    let mut cursor = 240_usize;
    while cursor < payload.len() {
        let option_code = *payload.get(cursor)?;
        cursor += 1;
        if option_code == 0 {
            continue;
        }
        if option_code == 255 {
            break;
        }
        let option_len = usize::from(*payload.get(cursor)?);
        cursor += 1;
        let option_end = cursor.checked_add(option_len)?;
        let option_value = payload.get(cursor..option_end)?;
        cursor = option_end;
        apply_passive_dhcp_option(observation, option_code, option_value);
    }
    Some(())
}

pub(super) fn apply_passive_dhcp_option(
    observation: &mut PassiveDhcpObservation,
    option_code: u8,
    option_value: &[u8],
) {
    match option_code {
        12 => observation.hostname = codecs::passive_dhcp_ascii_option(option_value),
        53 => {
            observation.message_type = option_value
                .first()
                .map(|value| message_type::label(*value))
        }
        55 => {
            observation.parameter_request_fingerprint =
                codecs::passive_dhcp_parameter_request_fingerprint(option_value)
        }
        60 => observation.vendor_class = codecs::passive_dhcp_ascii_option(option_value),
        61 => observation.client_id = codecs::passive_dhcp_client_id(option_value),
        _ => {}
    }
}

pub(super) fn dhcp_message_type_label(value: u8) -> String {
    message_type::label(value)
}
