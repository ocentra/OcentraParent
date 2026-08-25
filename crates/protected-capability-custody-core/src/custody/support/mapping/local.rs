use super::super::super::{CustodyError, LocalReplicaFailure, TransitionPhase};

pub(super) fn replica_failure(error: CustodyError, phase: TransitionPhase) -> CustodyError {
    let failure = match error {
        CustodyError::Unavailable => LocalReplicaFailure::Unavailable,
        CustodyError::Database => LocalReplicaFailure::Database,
        CustodyError::DatabaseReplaced => LocalReplicaFailure::DatabaseReplaced,
        CustodyError::UnsafeDatabasePath => LocalReplicaFailure::UnsafeDatabasePath,
        CustodyError::Tampered => LocalReplicaFailure::Tampered,
        CustodyError::Conflict => LocalReplicaFailure::Conflict,
        other => return other,
    };
    CustodyError::LocalReplicaBehind { phase, failure }
}
