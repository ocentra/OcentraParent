use serde::{Deserialize, Serialize};

use super::{deserialize_lan_schema_version, LanPairingText, LanPairingTrustState};
use crate::constants;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
enum LanMdnsField {
    AdvertisementId,
    OpaqueDeviceId,
    ProtocolVersion,
    FamilyHash,
    Platform,
    AgentVersion,
    TxtKey,
    TxtValue,
}

impl LanMdnsField {
    const FIELD_NAMES: [&'static str; 8] = [
        constants::lan_pairing::MDNS_ADVERTISEMENT_ID_FIELD,
        constants::lan_pairing::MDNS_OPAQUE_DEVICE_ID_FIELD,
        constants::lan_pairing::MDNS_PROTOCOL_VERSION_FIELD,
        constants::lan_pairing::MDNS_FAMILY_HASH_FIELD,
        constants::lan_pairing::MDNS_PLATFORM_FIELD,
        constants::lan_pairing::MDNS_AGENT_VERSION_FIELD,
        constants::lan_pairing::MDNS_TXT_KEY_FIELD,
        constants::lan_pairing::MDNS_TXT_VALUE_FIELD,
    ];

    fn as_str(self) -> &'static str {
        Self::FIELD_NAMES[self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
#[serde(rename_all = "kebab-case")]
pub enum LanMdnsAdvertisementLifecycleState {
    Start,
    Update,
    Stop,
    Degraded,
}

impl LanMdnsAdvertisementLifecycleState {
    const PROTOCOL_STRINGS: [&'static str; 4] = [
        constants::lan_pairing::MDNS_TXT_VALUE_START,
        constants::lan_pairing::MDNS_TXT_VALUE_UPDATE,
        constants::lan_pairing::MDNS_TXT_VALUE_STOP,
        constants::lan_pairing::MDNS_TXT_VALUE_DEGRADED,
    ];

    pub fn as_str(&self) -> LanPairingText {
        LanPairingText(Self::PROTOCOL_STRINGS[*self as usize].to_string())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
#[serde(rename_all = "kebab-case")]
pub enum LanMdnsAdvertisementSupportState {
    Supported,
    Degraded,
    UnsupportedPlatform,
}

impl LanMdnsAdvertisementSupportState {
    const PROTOCOL_STRINGS: [&'static str; 3] = [
        constants::lan_pairing::MDNS_TXT_VALUE_SUPPORTED,
        constants::lan_pairing::MDNS_TXT_VALUE_DEGRADED,
        constants::lan_pairing::MDNS_TXT_VALUE_UNSUPPORTED_PLATFORM,
    ];

    pub fn as_str(&self) -> LanPairingText {
        LanPairingText(Self::PROTOCOL_STRINGS[*self as usize].to_string())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
#[serde(rename_all = "kebab-case")]
pub enum LanMdnsAdvertisementConfirmationState {
    HintOnly,
}

impl LanMdnsAdvertisementConfirmationState {
    pub fn as_str(&self) -> LanPairingText {
        LanPairingText(constants::lan_pairing::MDNS_TXT_VALUE_HINT_ONLY.to_string())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanMdnsTxtRecord {
    pub key: String,
    pub value: String,
}

impl LanMdnsTxtRecord {
    pub fn new<K, V>(key: K, value: V) -> Result<Self, ocentra_eventing::error::EventingError>
    where
        K: Into<LanPairingText>,
        V: Into<LanPairingText>,
    {
        let key = key.into();
        let value = value.into();
        validate_mdns_atom(LanMdnsField::TxtKey, &key)?;
        validate_mdns_atom(LanMdnsField::TxtValue, &value)?;
        Ok(Self {
            key: key.0,
            value: value.0,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanParentMdnsAdvertisement {
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
    pub schema_version: u16,
    pub service_type: String,
    pub advertisement_id: String,
    pub protocol_version: String,
    pub family_hash: String,
    pub pairing_state: LanPairingTrustState,
    pub lifecycle_state: LanMdnsAdvertisementLifecycleState,
    pub support_state: LanMdnsAdvertisementSupportState,
    pub confirmation_state: LanMdnsAdvertisementConfirmationState,
    pub txt_records: Vec<LanMdnsTxtRecord>,
}

impl LanParentMdnsAdvertisement {
    pub fn new<A, P, F>(
        advertisement_id: A,
        protocol_version: P,
        family_hash: F,
        pairing_state: LanPairingTrustState,
        lifecycle_state: LanMdnsAdvertisementLifecycleState,
        support_state: LanMdnsAdvertisementSupportState,
    ) -> Result<Self, ocentra_eventing::error::EventingError>
    where
        A: Into<LanPairingText>,
        P: Into<LanPairingText>,
        F: Into<LanPairingText>,
    {
        let advertisement_id = advertisement_id.into();
        let protocol_version = protocol_version.into();
        let family_hash = family_hash.into();
        validate_mdns_atom(LanMdnsField::AdvertisementId, &advertisement_id)?;
        validate_mdns_atom(LanMdnsField::ProtocolVersion, &protocol_version)?;
        validate_mdns_atom(LanMdnsField::FamilyHash, &family_hash)?;
        let txt_records = vec![
            LanMdnsTxtRecord::new(
                constants::lan_pairing::MDNS_TXT_KEY_SCHEMA_VERSION,
                constants::lan_pairing::SCHEMA_VERSION.to_string(),
            )?,
            LanMdnsTxtRecord::new(
                constants::lan_pairing::MDNS_TXT_KEY_PROTOCOL_VERSION,
                protocol_version.clone(),
            )?,
            LanMdnsTxtRecord::new(
                constants::lan_pairing::MDNS_TXT_KEY_FAMILY_HASH,
                family_hash.clone(),
            )?,
            LanMdnsTxtRecord::new(
                constants::lan_pairing::MDNS_TXT_KEY_PAIRING_STATE,
                pairing_state.as_str(),
            )?,
            LanMdnsTxtRecord::new(
                constants::lan_pairing::MDNS_TXT_KEY_LIFECYCLE_STATE,
                lifecycle_state.as_str(),
            )?,
            LanMdnsTxtRecord::new(
                constants::lan_pairing::MDNS_TXT_KEY_SUPPORT_STATE,
                support_state.as_str(),
            )?,
            LanMdnsTxtRecord::new(
                constants::lan_pairing::MDNS_TXT_KEY_CONFIRMATION_STATE,
                LanMdnsAdvertisementConfirmationState::HintOnly.as_str(),
            )?,
        ];

        Ok(Self {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            service_type: constants::lan_pairing::MDNS_PARENT_SERVICE_TYPE.to_string(),
            advertisement_id: advertisement_id.0,
            protocol_version: protocol_version.0,
            family_hash: family_hash.0,
            pairing_state,
            lifecycle_state,
            support_state,
            confirmation_state: LanMdnsAdvertisementConfirmationState::HintOnly,
            txt_records,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanChildMdnsAdvertisement {
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
    pub schema_version: u16,
    pub service_type: String,
    pub advertisement_id: String,
    pub opaque_device_id: String,
    pub protocol_version: String,
    pub family_hash: String,
    pub platform: String,
    pub agent_version: String,
    pub pairing_state: LanPairingTrustState,
    pub lifecycle_state: LanMdnsAdvertisementLifecycleState,
    pub support_state: LanMdnsAdvertisementSupportState,
    pub confirmation_state: LanMdnsAdvertisementConfirmationState,
    pub txt_records: Vec<LanMdnsTxtRecord>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LanChildMdnsAdvertisementInput {
    pub advertisement_id: String,
    pub opaque_device_id: String,
    pub protocol_version: String,
    pub family_hash: String,
    pub platform: String,
    pub agent_version: String,
    pub pairing_state: LanPairingTrustState,
    pub lifecycle_state: LanMdnsAdvertisementLifecycleState,
    pub support_state: LanMdnsAdvertisementSupportState,
}

impl LanChildMdnsAdvertisement {
    pub fn new(
        input: LanChildMdnsAdvertisementInput,
    ) -> Result<Self, ocentra_eventing::error::EventingError> {
        validate_mdns_advertisement_input(&input)?;
        let txt_records = mdns_advertisement_txt_records(&input)?;
        let LanChildMdnsAdvertisementInput {
            advertisement_id,
            opaque_device_id,
            protocol_version,
            family_hash,
            platform,
            agent_version,
            pairing_state,
            lifecycle_state,
            support_state,
        } = input;

        Ok(Self {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            service_type: constants::lan_pairing::MDNS_CHILD_SERVICE_TYPE.to_string(),
            advertisement_id,
            opaque_device_id,
            protocol_version,
            family_hash,
            platform,
            agent_version,
            pairing_state,
            lifecycle_state,
            support_state,
            confirmation_state: LanMdnsAdvertisementConfirmationState::HintOnly,
            txt_records,
        })
    }
}

fn validate_mdns_advertisement_input(
    input: &LanChildMdnsAdvertisementInput,
) -> Result<(), ocentra_eventing::error::EventingError> {
    validate_mdns_atom(
        LanMdnsField::AdvertisementId,
        &LanPairingText(input.advertisement_id.clone()),
    )?;
    validate_mdns_atom(
        LanMdnsField::OpaqueDeviceId,
        &LanPairingText(input.opaque_device_id.clone()),
    )?;
    validate_mdns_atom(
        LanMdnsField::ProtocolVersion,
        &LanPairingText(input.protocol_version.clone()),
    )?;
    validate_mdns_atom(
        LanMdnsField::FamilyHash,
        &LanPairingText(input.family_hash.clone()),
    )?;
    validate_mdns_atom(
        LanMdnsField::Platform,
        &LanPairingText(input.platform.clone()),
    )?;
    validate_mdns_atom(
        LanMdnsField::AgentVersion,
        &LanPairingText(input.agent_version.clone()),
    )
}

fn mdns_advertisement_txt_records(
    input: &LanChildMdnsAdvertisementInput,
) -> Result<Vec<LanMdnsTxtRecord>, ocentra_eventing::error::EventingError> {
    Ok(vec![
        LanMdnsTxtRecord::new(
            constants::lan_pairing::MDNS_TXT_KEY_SCHEMA_VERSION,
            constants::lan_pairing::SCHEMA_VERSION.to_string(),
        )?,
        LanMdnsTxtRecord::new(
            constants::lan_pairing::MDNS_TXT_KEY_PROTOCOL_VERSION,
            input.protocol_version.as_str(),
        )?,
        LanMdnsTxtRecord::new(
            constants::lan_pairing::MDNS_TXT_KEY_OPAQUE_DEVICE_ID,
            input.opaque_device_id.as_str(),
        )?,
        LanMdnsTxtRecord::new(
            constants::lan_pairing::MDNS_TXT_KEY_FAMILY_HASH,
            input.family_hash.as_str(),
        )?,
        LanMdnsTxtRecord::new(
            constants::lan_pairing::MDNS_TXT_KEY_PLATFORM,
            input.platform.as_str(),
        )?,
        LanMdnsTxtRecord::new(
            constants::lan_pairing::MDNS_TXT_KEY_AGENT_VERSION,
            input.agent_version.as_str(),
        )?,
        LanMdnsTxtRecord::new(
            constants::lan_pairing::MDNS_TXT_KEY_PAIRING_STATE,
            input.pairing_state.as_str(),
        )?,
        LanMdnsTxtRecord::new(
            constants::lan_pairing::MDNS_TXT_KEY_LIFECYCLE_STATE,
            input.lifecycle_state.as_str(),
        )?,
        LanMdnsTxtRecord::new(
            constants::lan_pairing::MDNS_TXT_KEY_SUPPORT_STATE,
            input.support_state.as_str(),
        )?,
        LanMdnsTxtRecord::new(
            constants::lan_pairing::MDNS_TXT_KEY_CONFIRMATION_STATE,
            LanMdnsAdvertisementConfirmationState::HintOnly.as_str(),
        )?,
    ])
}

fn validate_mdns_atom(
    field: LanMdnsField,
    value: &LanPairingText,
) -> Result<(), ocentra_eventing::error::EventingError> {
    let field = field.as_str();
    if value.0.trim().is_empty() {
        return Err(ocentra_eventing::error::EventingError::EmptyValue { field });
    }
    if value.0.chars().all(|character| {
        character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.' | ':')
    }) {
        return Ok(());
    }
    Err(ocentra_eventing::error::EventingError::InvalidValue {
        field,
        value: value.0.clone(),
    })
}
