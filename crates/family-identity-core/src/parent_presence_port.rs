use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Mutex, OnceLock};

use crate::parent_presence::{
    ParentPresenceChallenge, ParentPresenceChallengeIssuanceFailureReason,
    ParentPresenceObservedAt, ParentPresenceReceiptRef, ParentPresenceVerificationAccepted,
    ParentPresenceVerificationFailureReason, ParentPresenceVerificationInput,
    ParentPresenceVerificationPort,
};
use crate::trust_bootstrap_validation::parent_presence_verification_failure_reason;

#[derive(Default)]
struct ParentPresenceRegistry {
    issued_challenges: BTreeMap<String, ParentPresenceChallenge>,
    consumed_challenge_refs: BTreeSet<String>,
    next_receipt_sequence: u64,
}

fn registry_mutex() -> &'static Mutex<ParentPresenceRegistry> {
    static REGISTRY: OnceLock<Mutex<ParentPresenceRegistry>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(ParentPresenceRegistry::default()))
}

fn with_registry<R>(action: impl FnOnce(&mut ParentPresenceRegistry) -> R) -> R {
    // This adapter is intentionally process-local and partial until a durable store exists.
    let mut registry = registry_mutex()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    action(&mut registry)
}

impl ParentPresenceVerificationPort {
    pub fn new() -> Self {
        Self::with_clock(
            || ParentPresenceObservedAt::from_system_time(std::time::SystemTime::now()),
        )
    }

    fn with_clock(clock: impl Fn() -> ParentPresenceObservedAt + Send + Sync + 'static) -> Self {
        Self {
            clock: Box::new(clock),
        }
    }

    pub fn issue_challenge(
        &mut self,
        challenge: ParentPresenceChallenge,
    ) -> Result<(), ParentPresenceChallengeIssuanceFailureReason> {
        with_registry(|registry| {
            if registry
                .issued_challenges
                .contains_key(&challenge.challenge_ref)
                || registry
                    .consumed_challenge_refs
                    .contains(&challenge.challenge_ref)
            {
                return Err(ParentPresenceChallengeIssuanceFailureReason::DuplicateChallengeRef);
            }

            registry
                .issued_challenges
                .insert(challenge.challenge_ref.clone(), challenge);
            Ok(())
        })
    }

    pub fn verify_and_consume(
        &mut self,
        input: ParentPresenceVerificationInput,
    ) -> Result<ParentPresenceVerificationAccepted, ParentPresenceVerificationFailureReason> {
        with_registry(|registry| {
            if registry
                .consumed_challenge_refs
                .contains(&input.challenge_ref)
            {
                return Err(ParentPresenceVerificationFailureReason::ReplayRejected);
            }

            let Some(challenge) = registry
                .issued_challenges
                .get(&input.challenge_ref)
                .cloned()
            else {
                return Err(ParentPresenceVerificationFailureReason::ChallengeNotIssued);
            };

            let observed_at = (self.clock)();
            if let Some(failure_reason) = parent_presence_verification_failure_reason(
                &challenge,
                &input.assertion,
                &observed_at,
            ) {
                return Err(failure_reason);
            }

            let removed_challenge = registry
                .issued_challenges
                .remove(&input.challenge_ref)
                .ok_or(ParentPresenceVerificationFailureReason::ChallengeNotIssued)?;

            registry
                .consumed_challenge_refs
                .insert(removed_challenge.challenge_ref.clone());
            registry.next_receipt_sequence += 1;

            let receipt_ref = ParentPresenceReceiptRef::from_string(format!(
                "parent-presence-receipt-{}",
                registry.next_receipt_sequence
            ));

            Ok(ParentPresenceVerificationAccepted::new(
                receipt_ref,
                removed_challenge,
                input.assertion,
                observed_at,
            ))
        })
    }
}

impl Default for ParentPresenceVerificationPort {
    fn default() -> Self {
        Self::new()
    }
}
