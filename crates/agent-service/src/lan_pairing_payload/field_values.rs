use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReference;
use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReferenceKind;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct LanPairingPayloadText(pub(super) String);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct LanPairingPayloadTextRef<'a>(pub(super) &'a str);

impl LanPairingPayloadText {
    pub(super) fn as_payload_text_ref(&self) -> LanPairingPayloadTextRef<'_> {
        LanPairingPayloadTextRef(self.0.as_str())
    }
}

pub(super) fn optional_payload_text(
    fields: &LogFields,
    field_name: LanPairingText,
) -> Option<LanPairingPayloadText> {
    let field_name = field_name.0;
    match fields.get(field_name.as_str()) {
        Some(LogFieldValue::String(value)) if !value.is_empty() => {
            Some(LanPairingPayloadText(value.clone()))
        }
        _ => None,
    }
}

pub(super) fn required_payload_text(
    fields: &LogFields,
    field_name: LanPairingText,
) -> Result<LanPairingPayloadText, LanPairingRejectionReason> {
    let field_name = field_name.0;
    match fields.get(field_name.as_str()) {
        Some(LogFieldValue::String(value)) if !value.is_empty() => {
            Ok(LanPairingPayloadText(value.clone()))
        }
        _ => Err(LanPairingRejectionReason::Malformed),
    }
}

pub(super) fn required_anonymous_payload_text(
    fields: &LogFields,
    field_name: LanPairingText,
) -> Result<LanPairingPayloadText, LanPairingRejectionReason> {
    required_payload_text(fields, field_name).map_err(|reason| {
        let _ = reason;
        LanPairingRejectionReason::Anonymous
    })
}

pub(super) fn required_controller_lease_payload_text(
    fields: &LogFields,
    field_name: LanPairingText,
) -> Result<LanPairingPayloadText, LanPairingRejectionReason> {
    required_payload_text(fields, field_name).map_err(|reason| {
        let _ = reason;
        LanPairingRejectionReason::ControllerLeaseMissing
    })
}

pub(super) fn optional_household_device_kind(
    fields: &LogFields,
) -> Result<Option<LanPairingPayloadText>, LanPairingRejectionReason> {
    let Some(device_kind) = optional_payload_text(
        fields,
        constants::lan_pairing::HOUSEHOLD_ACTION_DEVICE_KIND_FIELD.into(),
    ) else {
        return Ok(None);
    };
    if constants::lan_pairing::HOUSEHOLD_DEVICE_KINDS.contains(&device_kind.as_payload_text_ref().0)
    {
        return Ok(Some(device_kind));
    }
    Err(LanPairingRejectionReason::Malformed)
}

pub(super) fn parse_evidence_references(
    fields: &LogFields,
    observed_at: LanPairingPayloadTextRef<'_>,
) -> Vec<ParentEvidenceReference> {
    match fields.get(constants::field::LAN_EVIDENCE_REFERENCE_IDS) {
        Some(LogFieldValue::String(value)) => value
            .split(constants::delimiter::LIST)
            .filter(|evidence_id| !evidence_id.is_empty())
            .map(|evidence_id| ParentEvidenceReference {
                evidence_reference_id: evidence_id.to_string(),
                kind: ParentEvidenceReferenceKind::ActivityEvent,
                observed_at: observed_at.0.to_owned(),
            })
            .collect(),
        _ => Vec::new(),
    }
}
