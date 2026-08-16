use super::{NetworkLinuxNftablesLabExecutionError, NetworkLinuxNftablesLabUnsupportedClaims};

pub(super) fn reject_unsupported_claims(
    claims: &NetworkLinuxNftablesLabUnsupportedClaims,
) -> Result<(), NetworkLinuxNftablesLabExecutionError> {
    if claims.production_enforcement_claimed {
        return Err(NetworkLinuxNftablesLabExecutionError::ProductionEnforcementClaimRejected);
    }
    if claims.persistent_rule_claimed {
        return Err(NetworkLinuxNftablesLabExecutionError::PersistentRuleClaimRejected);
    }
    if claims.generic_linux_support_claimed {
        return Err(NetworkLinuxNftablesLabExecutionError::GenericLinuxSupportClaimRejected);
    }
    if claims.service_manager_install_claimed {
        return Err(NetworkLinuxNftablesLabExecutionError::ServiceManagerInstallClaimRejected);
    }
    if claims.exact_url_claimed {
        return Err(NetworkLinuxNftablesLabExecutionError::ExactUrlClaimRejected);
    }
    if claims.decrypted_payload_claimed {
        return Err(NetworkLinuxNftablesLabExecutionError::DecryptedPayloadClaimRejected);
    }
    if claims.page_content_claimed {
        return Err(NetworkLinuxNftablesLabExecutionError::PageContentClaimRejected);
    }
    if claims.policy_engine_execution_claimed {
        return Err(NetworkLinuxNftablesLabExecutionError::PolicyEngineExecutionClaimRejected);
    }
    if claims.enforcement_command_published {
        return Err(NetworkLinuxNftablesLabExecutionError::EnforcementCommandPublishedRejected);
    }
    Ok(())
}
