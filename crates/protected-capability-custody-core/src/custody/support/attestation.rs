use super::super::CustodyError;
use crate::platform::{PlatformAttestation, PlatformCustodyPort, SecurityLevel};

pub(super) fn attest<P: PlatformCustodyPort>(
    platform: &P,
) -> Result<PlatformAttestation, CustodyError> {
    let attestation = platform.attest().map_err(super::map_platform_error)?;
    if attestation.security_level != SecurityLevel::SameUserIsolated
        || attestation.key_epoch == 0
        || attestation.writer_epoch == 0
        || attestation.anti_rollback_watermark == 0
    {
        return Err(CustodyError::Unavailable);
    }
    Ok(attestation)
}
