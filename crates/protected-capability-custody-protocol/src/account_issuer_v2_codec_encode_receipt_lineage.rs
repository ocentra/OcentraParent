use crate::account_issuer::account_issuer_receipt_lineage::AccountIssuerReceiptLineage;
use crate::account_issuer_contract::ACCOUNT_ISSUER_MAX_FIELD_BYTES;
use crate::types::ProtocolError;

pub(super) fn append(
    wire: &mut Vec<u8>,
    lineage: &AccountIssuerReceiptLineage,
) -> Result<(), ProtocolError> {
    super::append_provider(wire, lineage.provider());
    for field in [
        lineage.provider_subject().as_str().as_bytes(),
        lineage.account_id().as_bytes(),
        lineage.household_id().as_bytes(),
        lineage.member_id().as_bytes(),
        lineage.device_id().as_bytes(),
        lineage.session_id().as_bytes(),
        lineage.service_binding_id().as_bytes(),
    ] {
        super::append_field(wire, field, ACCOUNT_ISSUER_MAX_FIELD_BYTES)?;
    }
    for generation in [
        lineage.key_generation(),
        lineage.enrollment_generation(),
        lineage.authority_generation(),
        lineage.session_generation(),
    ] {
        wire.extend_from_slice(&generation.to_be_bytes());
    }
    for field in [
        lineage.issued_at().as_bytes(),
        lineage.expires_at().as_bytes(),
    ] {
        super::append_field(wire, field, ACCOUNT_ISSUER_MAX_FIELD_BYTES)?;
    }
    Ok(())
}
