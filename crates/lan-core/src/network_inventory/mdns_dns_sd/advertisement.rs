use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanChildMdnsAdvertisement, LanChildMdnsAdvertisementInput, LanMdnsAdvertisementLifecycleState,
    LanMdnsAdvertisementSupportState, LanMdnsTxtRecord, LanPairingTrustState,
    LanParentMdnsAdvertisement,
};

use super::{MdnsDnsSdTxtRecord, MDNS_SERVICE_TYPES};

mod state;

pub fn is_selected_service_type(service_type: &str) -> bool {
    MDNS_SERVICE_TYPES
        .iter()
        .any(|known_type| service_type.eq_ignore_ascii_case(known_type))
}

pub fn parse_parent_mdns_advertisement(
    txt_records: &[MdnsDnsSdTxtRecord],
) -> Option<LanParentMdnsAdvertisement> {
    let records = mdns_txt_records_to_contract(txt_records);
    let advertisement_id = txt_records_value(
        &records,
        constants::lan_pairing::MDNS_ADVERTISEMENT_ID_FIELD,
    )?;
    let protocol_version = txt_records_value(
        &records,
        constants::lan_pairing::MDNS_TXT_KEY_PROTOCOL_VERSION,
    )?;
    let family_hash =
        txt_records_value(&records, constants::lan_pairing::MDNS_TXT_KEY_FAMILY_HASH)?;
    let pairing_state = parse_pairing_state(
        txt_records_value(&records, constants::lan_pairing::MDNS_TXT_KEY_PAIRING_STATE)?.as_str(),
    )?;
    let lifecycle_state = parse_lifecycle_state(
        txt_records_value(
            &records,
            constants::lan_pairing::MDNS_TXT_KEY_LIFECYCLE_STATE,
        )?
        .as_str(),
    )?;
    let support_state = parse_support_state(
        txt_records_value(&records, constants::lan_pairing::MDNS_TXT_KEY_SUPPORT_STATE)?.as_str(),
    )?;

    LanParentMdnsAdvertisement::new(
        advertisement_id,
        protocol_version,
        family_hash,
        pairing_state,
        lifecycle_state,
        support_state,
    )
    .ok()
}

pub fn parse_child_mdns_advertisement(
    txt_records: &[MdnsDnsSdTxtRecord],
) -> Option<LanChildMdnsAdvertisement> {
    let records = mdns_txt_records_to_contract(txt_records);
    let advertisement_id = txt_records_value(
        &records,
        constants::lan_pairing::MDNS_ADVERTISEMENT_ID_FIELD,
    )?;
    let opaque_device_id = txt_records_value(
        &records,
        constants::lan_pairing::MDNS_TXT_KEY_OPAQUE_DEVICE_ID,
    )?;
    let protocol_version = txt_records_value(
        &records,
        constants::lan_pairing::MDNS_TXT_KEY_PROTOCOL_VERSION,
    )?;
    let family_hash =
        txt_records_value(&records, constants::lan_pairing::MDNS_TXT_KEY_FAMILY_HASH)?;
    let platform = txt_records_value(&records, constants::lan_pairing::MDNS_TXT_KEY_PLATFORM)?;
    let agent_version =
        txt_records_value(&records, constants::lan_pairing::MDNS_TXT_KEY_AGENT_VERSION)?;
    let pairing_state = parse_pairing_state(
        txt_records_value(&records, constants::lan_pairing::MDNS_TXT_KEY_PAIRING_STATE)?.as_str(),
    )?;
    let lifecycle_state = parse_lifecycle_state(
        txt_records_value(
            &records,
            constants::lan_pairing::MDNS_TXT_KEY_LIFECYCLE_STATE,
        )?
        .as_str(),
    )?;
    let support_state = parse_support_state(
        txt_records_value(&records, constants::lan_pairing::MDNS_TXT_KEY_SUPPORT_STATE)?.as_str(),
    )?;

    LanChildMdnsAdvertisement::new(LanChildMdnsAdvertisementInput {
        advertisement_id,
        opaque_device_id,
        protocol_version,
        family_hash,
        platform,
        agent_version,
        pairing_state,
        lifecycle_state,
        support_state,
    })
    .ok()
}

pub fn mdns_txt_records_to_contract(txt_records: &[MdnsDnsSdTxtRecord]) -> Vec<LanMdnsTxtRecord> {
    let mut records = Vec::new();
    for record in txt_records {
        let value = record.value.as_deref().unwrap_or("");
        if let Ok(record) = LanMdnsTxtRecord::new(record.key.clone(), value.to_string()) {
            records.push(record);
        }
    }
    records
}

pub fn txt_records_value(records: &[LanMdnsTxtRecord], key: &str) -> Option<String> {
    records
        .iter()
        .find(|record| record.key.eq_ignore_ascii_case(key))
        .map(|record| record.value.clone())
}

pub fn parse_pairing_state(value: &str) -> Option<LanPairingTrustState> {
    state::parse_pairing_state(value)
}

pub fn parse_lifecycle_state(value: &str) -> Option<LanMdnsAdvertisementLifecycleState> {
    state::parse_lifecycle_state(value)
}

pub fn parse_support_state(value: &str) -> Option<LanMdnsAdvertisementSupportState> {
    state::parse_support_state(value)
}
