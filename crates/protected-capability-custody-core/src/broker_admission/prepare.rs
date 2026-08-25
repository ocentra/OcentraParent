use crate::binding::BindingLocator;
use crate::custody::{CustodyError, RecoveryOutcome};
use ocentra_protected_capability_custody_protocol::response::{
    ObservedGenerations, ResponseStatus,
};

use super::{outcome, token, BrokerCustodyOutcome, BrokerCustodyRuntime, BrokerRuntimeError};

pub(super) fn prepare(
    runtime: &BrokerCustodyRuntime,
    locator: &BindingLocator,
    generations: ObservedGenerations,
) -> Result<BrokerCustodyOutcome, BrokerRuntimeError> {
    match runtime.store.prepare(locator) {
        Ok(prepared) => Ok(issue_outcome(runtime, prepared, generations)),
        Err(CustodyError::Conflict) => reissue(runtime, locator, generations),
        Err(error) => Ok(outcome::from_custody_error(&error, generations)),
    }
}

fn reissue(
    runtime: &BrokerCustodyRuntime,
    locator: &BindingLocator,
    generations: ObservedGenerations,
) -> Result<BrokerCustodyOutcome, BrokerRuntimeError> {
    match runtime.store.recover(locator) {
        Ok(RecoveryOutcome::Prepared(prepared)) => {
            Ok(issue_outcome(runtime, prepared, generations))
        }
        Ok(_) | Err(_) => Ok(BrokerCustodyOutcome::stateless(ResponseStatus::Rejected)),
    }
}

pub(super) fn issue_outcome(
    runtime: &BrokerCustodyRuntime,
    prepared: crate::custody::PreparedCapability,
    generations: ObservedGenerations,
) -> BrokerCustodyOutcome {
    match token::issue(&runtime.registry_id, prepared) {
        Ok(token) => {
            BrokerCustodyOutcome::stateful(ResponseStatus::Prepared, generations, Some(token))
        }
        Err(_error) => {
            BrokerCustodyOutcome::stateful(ResponseStatus::PrepareAmbiguous, generations, None)
        }
    }
}
