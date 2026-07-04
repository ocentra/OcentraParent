use crate::adapter_capability_status::NetworkAdapterCapabilityStatusError;

pub(crate) fn normalize_portal_ref(
    value: Option<&str>,
) -> Result<String, NetworkAdapterCapabilityStatusError> {
    match value {
        Some(raw) => {
            normalize_ref(raw).ok_or(NetworkAdapterCapabilityStatusError::EmptyPortalStatusProofRef)
        }
        None => Err(NetworkAdapterCapabilityStatusError::MissingPortalStatusProofRef),
    }
}

pub(crate) fn normalize_ref(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}
