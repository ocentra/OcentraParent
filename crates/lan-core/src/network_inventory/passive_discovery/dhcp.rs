mod codecs;
mod options;
mod summary;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PassiveDhcpObservation {
    message_type: Option<String>,
    hostname: Option<String>,
    vendor_class: Option<String>,
    client_id: Option<String>,
    parameter_request_fingerprint: Option<String>,
    client_mac: Option<String>,
}

pub fn passive_dhcp_summary(payload: &[u8]) -> Option<String> {
    summary::passive_dhcp_summary(payload)
}

pub fn passive_dhcp_device_id(payload: &[u8]) -> Option<String> {
    summary::passive_dhcp_device_id(payload)
}

pub fn parse_passive_dhcp_observation(payload: &[u8]) -> Option<PassiveDhcpObservation> {
    if payload.len() < 240 || payload.get(236..240)? != [99, 130, 83, 99] {
        return None;
    }

    let hardware_type = *payload.get(1)?;
    let hardware_address_len = usize::from(*payload.get(2)?);
    let client_mac = codecs::passive_dhcp_client_mac(payload, hardware_type, hardware_address_len);
    let mut observation = PassiveDhcpObservation {
        client_mac,
        ..PassiveDhcpObservation::default()
    };
    options::parse_passive_dhcp_options(payload, &mut observation)?;
    Some(observation)
}

pub fn parse_passive_dhcp_options(
    payload: &[u8],
    observation: &mut PassiveDhcpObservation,
) -> Option<()> {
    options::parse_passive_dhcp_options(payload, observation)
}

pub fn apply_passive_dhcp_option(
    observation: &mut PassiveDhcpObservation,
    option_code: u8,
    option_value: &[u8],
) {
    options::apply_passive_dhcp_option(observation, option_code, option_value)
}

pub fn passive_dhcp_client_mac(
    payload: &[u8],
    hardware_type: u8,
    hardware_address_len: usize,
) -> Option<String> {
    codecs::passive_dhcp_client_mac(payload, hardware_type, hardware_address_len)
}

pub fn passive_dhcp_client_id(payload: &[u8]) -> Option<String> {
    codecs::passive_dhcp_client_id(payload)
}

pub fn passive_dhcp_parameter_request_fingerprint(payload: &[u8]) -> Option<String> {
    codecs::passive_dhcp_parameter_request_fingerprint(payload)
}

pub fn passive_dhcp_ascii_option(payload: &[u8]) -> Option<String> {
    codecs::passive_dhcp_ascii_option(payload)
}

pub fn passive_dhcp_mac_bytes(payload: &[u8]) -> Option<String> {
    codecs::passive_dhcp_mac_bytes(payload)
}

pub fn passive_dhcp_hex_bytes(payload: &[u8]) -> String {
    codecs::passive_dhcp_hex_bytes(payload)
}

pub fn dhcp_message_type_label(value: u8) -> String {
    options::dhcp_message_type_label(value)
}
