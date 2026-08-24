use crate::platform::PlatformError;

#[cfg(windows)]
fn enrollment_value(registry_id: &str, name: &str) -> Result<Vec<u8>, PlatformError> {
    super::registry::read_enrollment(registry_id, name)?.ok_or(PlatformError::DeploymentRequired)
}

#[cfg(windows)]
pub(crate) fn broker_pipe_sddl(registry_id: &str) -> Result<String, PlatformError> {
    let client_sid = enrollment_value(
        registry_id,
        ocentra_protected_capability_custody_protocol::constants::CLIENT_TOKEN_SID_VALUE_NAME,
    )?;
    let client_sid = String::from_utf8(client_sid).map_err(|_| PlatformError::Tampered)?;
    validate_specific_sid(&client_sid)?;
    // The broker service and the one installer-enrolled client SID are the
    // only transport principals. Administrators are intentionally absent;
    // elevation alone is not client enrollment.
    Ok(format!("D:P(A;;GRGW;;;SY)(A;;GRGW;;;{client_sid})"))
}

#[cfg(windows)]
fn validate_specific_sid(value: &str) -> Result<(), PlatformError> {
    const BROAD_PRINCIPALS: [&str; 9] = [
        "S-1-1-0",      // Everyone
        "S-1-3-0",      // Creator Owner
        "S-1-5-4",      // Interactive
        "S-1-5-6",      // Service
        "S-1-5-11",     // Authenticated Users
        "S-1-5-18",     // Local System
        "S-1-5-19",     // Local Service
        "S-1-5-20",     // Network Service
        "S-1-5-32-544", // Administrators
    ];
    if value.len() > 184 || BROAD_PRINCIPALS.contains(&value) {
        return Err(PlatformError::Tampered);
    }
    let mut parts = value.split('-');
    if parts.next() != Some("S") || parts.next() != Some("1") {
        return Err(PlatformError::Tampered);
    }
    let authority = parse_canonical_decimal(parts.next().ok_or(PlatformError::Tampered)?)?;
    if authority > 0x0000_FFFF_FFFF_FFFF {
        return Err(PlatformError::Tampered);
    }
    let mut sub_authority_count = 0_usize;
    for part in parts {
        sub_authority_count = sub_authority_count.saturating_add(1);
        if sub_authority_count > 15 {
            return Err(PlatformError::Tampered);
        }
        let sub_authority = parse_canonical_decimal(part)?;
        u32::try_from(sub_authority).map_err(|_| PlatformError::Tampered)?;
    }
    if sub_authority_count == 0 {
        return Err(PlatformError::Tampered);
    }
    Ok(())
}

#[cfg(windows)]
fn parse_canonical_decimal(value: &str) -> Result<u64, PlatformError> {
    if value.is_empty()
        || !value.bytes().all(|byte| byte.is_ascii_digit())
        || (value.len() > 1 && value.starts_with('0'))
    {
        return Err(PlatformError::Tampered);
    }
    value.parse().map_err(|_| PlatformError::Tampered)
}
