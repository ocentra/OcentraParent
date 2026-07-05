#![forbid(unsafe_code)]

use super::super::{assert_local_time, assert_utc_timestamp, PolicyContractScheduleBoundary};
use super::super::{PolicyContractScheduleBoundaryState, PolicyContractValidationResult};

pub(super) fn validate_policy_schedule_boundary_optional_sections(
    boundary: &PolicyContractScheduleBoundary,
) -> PolicyContractValidationResult {
    if let Some(dst_boundary) = &boundary.dst_boundary {
        assert_local_time(&dst_boundary.local_time, "dstBoundary.localTime")?;
    }
    if let Some(clock_skew) = &boundary.clock_skew {
        assert_utc_timestamp(&clock_skew.observed_at, "clockSkew.observedAt")?;
        if clock_skew.allowed_skew_minutes < 0 {
            return Err("clockSkew.allowedSkewMinutes must be a non-negative number".into());
        }
    }
    if let Some(exception) = &boundary.exception {
        assert_utc_timestamp(&exception.starts_at, "exception.startsAt")?;
        assert_utc_timestamp(&exception.expires_at, "exception.expiresAt")?;
        if exception.expires_at <= exception.starts_at {
            return Err("schedule exceptions must expire after they start".into());
        }
    }
    if let Some(expiry) = &boundary.expiry {
        assert_utc_timestamp(&expiry.expires_at, "expiry.expiresAt")?;
        assert_utc_timestamp(&expiry.expired_at, "expiry.expiredAt")?;
        if expiry.expired_at < expiry.expires_at {
            return Err("expiry.expiredAt must be on or after expiry.expiresAt".into());
        }
        if boundary.state != PolicyContractScheduleBoundaryState::Expired
            && boundary.evaluated_at >= expiry.expires_at
        {
            return Err("non-expired schedule boundaries cannot be evaluated after expiry".into());
        }
    }
    Ok(())
}
