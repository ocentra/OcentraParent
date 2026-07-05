#![forbid(unsafe_code)]

use super::super::{
    assert_utc_timestamp, PolicyContractOverrideGrant, PolicyContractValidationResult,
};

pub(crate) fn validate_policy_override_grant_state_rules(
    grant: &PolicyContractOverrideGrant,
    evaluated_at: &str,
) -> PolicyContractValidationResult {
    assert_utc_timestamp(&grant.effective_from, "override.effectiveFrom")?;
    assert_utc_timestamp(&grant.effective_until, "override.effectiveUntil")?;
    if grant.effective_until <= grant.effective_from {
        return Err("override.effectiveUntil must be after override.effectiveFrom".into());
    }

    match grant.state {
        super::super::PolicyContractOverrideState::Active => {
            if evaluated_at < grant.effective_from.as_str()
                || evaluated_at >= grant.effective_until.as_str()
            {
                return Err(
                    "active overrides require evaluatedAt within the effective window".into(),
                );
            }
        }
        super::super::PolicyContractOverrideState::Expired => {
            if evaluated_at < grant.effective_until.as_str() {
                return Err(
                    "expired overrides require evaluatedAt on or after effectiveUntil".into(),
                );
            }
        }
        super::super::PolicyContractOverrideState::Revoked => {
            if evaluated_at < grant.effective_from.as_str() {
                return Err("revoked overrides require an effectiveFrom boundary".into());
            }
        }
    }

    Ok(())
}
