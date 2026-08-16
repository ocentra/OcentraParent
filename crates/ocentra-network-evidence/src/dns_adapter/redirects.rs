use super::{NetworkDnsAdapterAction, NetworkDnsAdapterProofError};
use crate::{normalize_domain_with_public_suffix, PublicSuffixModel};

pub(super) fn normalized_target_domain(input: &str) -> Result<String, NetworkDnsAdapterProofError> {
    normalize_domain_with_public_suffix(input, &PublicSuffixModel::ocentra_fixture())
        .map(|evidence| evidence.normalized_domain)
        .map_err(NetworkDnsAdapterProofError::InvalidTargetDomain)
}

pub(super) fn normalized_redirect_target(
    action: NetworkDnsAdapterAction,
    input: &super::NetworkDnsAdapterProofInput,
) -> Result<Option<String>, NetworkDnsAdapterProofError> {
    match (action, input.redirect_target_domain.as_deref()) {
        (NetworkDnsAdapterAction::Redirect, Some(target)) => {
            normalize_domain_with_public_suffix(target, &PublicSuffixModel::ocentra_fixture())
                .map(|evidence| Some(evidence.normalized_domain))
                .map_err(NetworkDnsAdapterProofError::InvalidRedirectTargetDomain)
        }
        (NetworkDnsAdapterAction::Redirect, None) => {
            Err(NetworkDnsAdapterProofError::MissingRedirectTargetDomain)
        }
        (NetworkDnsAdapterAction::Block, Some(target)) => {
            normalize_domain_with_public_suffix(target, &PublicSuffixModel::ocentra_fixture())
                .map(|evidence| Some(evidence.normalized_domain))
                .map_err(NetworkDnsAdapterProofError::InvalidRedirectTargetDomain)
        }
        (NetworkDnsAdapterAction::Block, None) => Ok(None),
    }
}
