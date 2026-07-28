use std::path::PathBuf;

use ocentra_parent_agent_protocol::constants;
use serde::Deserialize;

use super::trusted_delivery_error::TrustedDeliveryError;
use super::trusted_delivery_store::{create_receipt, read_record};
use super::{EnforcementDeviceRefText, EnforcementText};

const RECORD_FIELD_NAMES: [&str; 7] = [
    constants::enforcement::TRUSTED_DELIVERY_ID_FIELD,
    constants::enforcement::TRUSTED_DELIVERY_DEVICE_ID_FIELD,
    constants::enforcement::TRUSTED_DELIVERY_EVIDENCE_REFERENCES_FIELD,
    constants::enforcement::TRUSTED_DELIVERY_PROCESS_ID_FIELD,
    constants::enforcement::TRUSTED_DELIVERY_PROCESS_NAME_FIELD,
    constants::enforcement::TRUSTED_DELIVERY_POLICY_DECISION_ID_FIELD,
    constants::enforcement::TRUSTED_DELIVERY_ENFORCEMENT_INTENT_ID_FIELD,
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct TrustedDeliveryDirectory(PathBuf);

impl TrustedDeliveryDirectory {
    pub(crate) fn from_store_path(store_path: &std::path::Path) -> Self {
        Self(
            store_path.with_extension(constants::enforcement::TRUSTED_DELIVERY_DIRECTORY_EXTENSION),
        )
    }

    pub(crate) fn path(&self) -> &std::path::Path {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct TrustedDeliveryBinding {
    delivery_id: EnforcementText,
    device_id: EnforcementDeviceRefText,
    evidence_references: Vec<EnforcementText>,
    process_id: Option<u32>,
    process_name: EnforcementText,
    policy_decision_id: EnforcementText,
    enforcement_intent_id: EnforcementText,
}

impl TrustedDeliveryBinding {
    pub(crate) fn new(
        delivery_id: EnforcementText,
        device_id: EnforcementDeviceRefText,
        evidence_references: Vec<EnforcementText>,
        process_id: Option<u32>,
        process_name: EnforcementText,
        policy_decision_id: EnforcementText,
        enforcement_intent_id: EnforcementText,
    ) -> Self {
        Self {
            delivery_id,
            device_id,
            evidence_references,
            process_id,
            process_name,
            policy_decision_id,
            enforcement_intent_id,
        }
    }
}

#[derive(Deserialize, PartialEq, Eq)]
struct TrustedDeliveryRecord {
    delivery_id: String,
    device_id: String,
    evidence_references: Vec<String>,
    process_id: Option<u32>,
    process_name: String,
    policy_decision_id: String,
    enforcement_intent_id: String,
}

impl From<&TrustedDeliveryBinding> for TrustedDeliveryRecord {
    fn from(binding: &TrustedDeliveryBinding) -> Self {
        Self {
            delivery_id: binding.delivery_id.0.clone(),
            device_id: binding.device_id.0.clone(),
            evidence_references: binding
                .evidence_references
                .iter()
                .map(|reference| reference.0.clone())
                .collect(),
            process_id: binding.process_id,
            process_name: binding.process_name.0.clone(),
            policy_decision_id: binding.policy_decision_id.0.clone(),
            enforcement_intent_id: binding.enforcement_intent_id.0.clone(),
        }
    }
}

impl TrustedDeliveryRecord {
    fn decode(bytes: &[u8]) -> Result<Self, TrustedDeliveryError> {
        let value: serde_json::Value =
            serde_json::from_slice(bytes).map_err(TrustedDeliveryError::from)?;
        if RECORD_FIELD_NAMES
            .iter()
            .any(|field| value.get(field).is_none())
        {
            return Err(TrustedDeliveryError::Store);
        }
        serde_json::from_value(value).map_err(TrustedDeliveryError::from)
    }
}

pub(crate) fn consume(
    directory: &TrustedDeliveryDirectory,
    binding: &TrustedDeliveryBinding,
) -> Result<(), TrustedDeliveryError> {
    let expected = TrustedDeliveryRecord::from(binding);
    let bytes = read_record(directory, &binding.delivery_id).map_err(TrustedDeliveryError::from)?;
    let persisted = TrustedDeliveryRecord::decode(&bytes)?;
    if persisted != expected {
        return Err(TrustedDeliveryError::Mismatch);
    }
    create_receipt(directory, &binding.delivery_id).map_err(TrustedDeliveryError::from)
}
