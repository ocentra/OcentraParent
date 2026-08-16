use super::*;

pub(super) fn reject_unsupported_claims(
    input: &NetworkActionResultInput,
) -> Result<(), NetworkActionResultError> {
    if input.exact_url_claimed {
        return Err(NetworkActionResultError::ExactUrlClaimRejected);
    }
    if input.decrypted_payload_claimed {
        return Err(NetworkActionResultError::DecryptedPayloadClaimRejected);
    }
    if input.page_content_claimed {
        return Err(NetworkActionResultError::PageContentClaimRejected);
    }
    if input.host_mutation_claimed {
        return Err(NetworkActionResultError::HostMutationClaimRejected);
    }
    if input.enforcement_command_published {
        return Err(NetworkActionResultError::EnforcementCommandPublishedRejected);
    }
    Ok(())
}
