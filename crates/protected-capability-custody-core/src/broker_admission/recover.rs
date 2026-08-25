use crate::binding::BindingLocator;
use crate::custody::RecoveryOutcome;
use ocentra_protected_capability_custody_protocol::response::{
    ObservedGenerations, ResponseStatus,
};

use super::{outcome, prepare, BrokerCustodyOutcome, BrokerCustodyRuntime, BrokerRuntimeError};

pub(super) fn recover(
    runtime: &BrokerCustodyRuntime,
    locator: &BindingLocator,
    generations: ObservedGenerations,
    resolve: bool,
) -> Result<BrokerCustodyOutcome, BrokerRuntimeError> {
    let recovered = if resolve {
        runtime.store.resolve_ambiguity(locator)
    } else {
        runtime.store.recover(locator)
    };
    Ok(match recovered {
        Ok(RecoveryOutcome::Prepared(prepared)) => {
            prepare::issue_outcome(runtime, prepared, generations)
        }
        Ok(RecoveryOutcome::Committed(_committed)) => {
            BrokerCustodyOutcome::stateful(ResponseStatus::Committed, generations, None)
        }
        Ok(RecoveryOutcome::Aborted) => {
            BrokerCustodyOutcome::stateful(ResponseStatus::Aborted, generations, None)
        }
        Ok(RecoveryOutcome::CommitAmbiguous) => {
            BrokerCustodyOutcome::stateful(ResponseStatus::CommitAmbiguous, generations, None)
        }
        Ok(RecoveryOutcome::AbortAmbiguous) => {
            BrokerCustodyOutcome::stateful(ResponseStatus::AbortAmbiguous, generations, None)
        }
        Err(error) => outcome::from_custody_error(&error, generations),
    })
}
