use crate::binding::BindingLocator;
use crate::custody::{Decision, FinalizeOutcome};
use ocentra_protected_capability_custody_protocol::request::UntrustedRequest;
use ocentra_protected_capability_custody_protocol::response::{
    ObservedGenerations, ResponseStatus,
};

use super::BrokerRuntimeError;
use super::{error_status, outcome, token, BrokerCustodyOutcome, BrokerCustodyRuntime};

pub(super) fn finalize(
    runtime: &BrokerCustodyRuntime,
    request: &UntrustedRequest,
    locator: BindingLocator,
    generations: ObservedGenerations,
    decision: Decision,
) -> Result<BrokerCustodyOutcome, BrokerRuntimeError> {
    let digest = request
        .opaque_token_digest()
        .ok_or(BrokerRuntimeError::InvalidRequest)?;
    let lookup_digest = locator.lookup_digest();
    let prepared = token::redeem(&runtime.registry_id, digest, locator)
        .map_err(|error| error_status::token_platform(&error))?;
    match runtime.store.finalize(prepared, decision) {
        Ok(FinalizeOutcome::Committed(_committed)) => terminal(
            runtime,
            &lookup_digest,
            digest,
            generations,
            ResponseStatus::Committed,
        ),
        Ok(FinalizeOutcome::Aborted) => terminal(
            runtime,
            &lookup_digest,
            digest,
            generations,
            ResponseStatus::Aborted,
        ),
        Ok(FinalizeOutcome::CommitAmbiguous) => Ok(BrokerCustodyOutcome::stateful(
            ResponseStatus::CommitAmbiguous,
            generations,
            None,
        )),
        Ok(FinalizeOutcome::AbortAmbiguous) => Ok(BrokerCustodyOutcome::stateful(
            ResponseStatus::AbortAmbiguous,
            generations,
            None,
        )),
        Err(error) => Ok(outcome::from_custody_error(&error, generations)),
    }
}

fn terminal(
    runtime: &BrokerCustodyRuntime,
    lookup_digest: &[u8; 32],
    digest: [u8; 32],
    generations: ObservedGenerations,
    status: ResponseStatus,
) -> Result<BrokerCustodyOutcome, BrokerRuntimeError> {
    token::consume(&runtime.registry_id, lookup_digest, digest)
        .map_err(|error| error_status::token_platform(&error))?;
    Ok(BrokerCustodyOutcome::stateful(status, generations, None))
}
