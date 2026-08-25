use crate::custody::CustodyError;
use ocentra_protected_capability_custody_protocol::response::{
    ObservedGenerations, ResponseStatus,
};
use ocentra_protected_capability_custody_protocol::types::OpaquePreparedToken;

use super::BrokerCustodyOutcome;

impl BrokerCustodyOutcome {
    pub(super) fn stateful(
        status: ResponseStatus,
        observed_generations: ObservedGenerations,
        opaque_token: Option<OpaquePreparedToken>,
    ) -> Self {
        Self {
            status,
            observed_generations: Some(observed_generations),
            opaque_token,
        }
    }

    pub(super) fn stateless(status: ResponseStatus) -> Self {
        Self {
            status,
            observed_generations: None,
            opaque_token: None,
        }
    }
}

pub(super) fn from_custody_error(
    error: &CustodyError,
    generations: ObservedGenerations,
) -> BrokerCustodyOutcome {
    match error {
        CustodyError::UnsupportedPlatform => {
            BrokerCustodyOutcome::stateless(ResponseStatus::UnsupportedPlatform)
        }
        CustodyError::CommitAmbiguous => {
            BrokerCustodyOutcome::stateful(ResponseStatus::CommitAmbiguous, generations, None)
        }
        CustodyError::AbortAmbiguous => {
            BrokerCustodyOutcome::stateful(ResponseStatus::AbortAmbiguous, generations, None)
        }
        CustodyError::PrepareAmbiguous => {
            BrokerCustodyOutcome::stateful(ResponseStatus::PrepareAmbiguous, generations, None)
        }
        CustodyError::WrongBinding
        | CustodyError::Rotated
        | CustodyError::Conflict
        | CustodyError::Missing
        | CustodyError::AlreadyCommitted
        | CustodyError::Aborted
        | CustodyError::BrokerRejected => BrokerCustodyOutcome::stateless(ResponseStatus::Rejected),
        CustodyError::Unavailable
        | CustodyError::Tampered
        | CustodyError::LocalReplicaBehind { .. }
        | CustodyError::BrokerBehind
        | CustodyError::UnsafeDatabasePath
        | CustodyError::DatabaseReplaced
        | CustodyError::Database => BrokerCustodyOutcome::stateless(ResponseStatus::Unavailable),
    }
}
