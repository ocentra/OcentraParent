use std::collections::{BTreeMap, BTreeSet};

use crate::parent_presence::{
    ParentPresenceChallenge, ParentPresenceChallengeIssuanceFailureReason,
    ParentPresenceObservedAt, ParentPresenceReceiptRef, ParentPresenceVerificationAccepted,
    ParentPresenceVerificationFailureReason, ParentPresenceVerificationInput,
    ParentPresenceVerificationPort,
};
use crate::trust_bootstrap_validation::parent_presence_verification_failure_reason;

impl ParentPresenceVerificationPort {
    pub fn new() -> Self {
        Self::with_clock(
            || ParentPresenceObservedAt::from_system_time(std::time::SystemTime::now()),
        )
    }

    pub fn with_clock(clock: impl Fn() -> ParentPresenceObservedAt + 'static) -> Self {
        Self {
            clock: Box::new(clock),
            issued_challenges: BTreeMap::new(),
            consumed_challenge_refs: BTreeSet::new(),
        }
    }

    pub fn issue_challenge(
        &mut self,
        challenge: ParentPresenceChallenge,
    ) -> Result<(), ParentPresenceChallengeIssuanceFailureReason> {
        if self
            .issued_challenges
            .contains_key(&challenge.challenge_ref)
            || self
                .consumed_challenge_refs
                .contains(&challenge.challenge_ref)
        {
            return Err(ParentPresenceChallengeIssuanceFailureReason::DuplicateChallengeRef);
        }

        self.issued_challenges
            .insert(challenge.challenge_ref.clone(), challenge);
        Ok(())
    }

    pub fn verify_and_consume(
        &mut self,
        input: ParentPresenceVerificationInput,
    ) -> Result<ParentPresenceVerificationAccepted, ParentPresenceVerificationFailureReason> {
        if self.consumed_challenge_refs.contains(&input.challenge_ref) {
            return Err(ParentPresenceVerificationFailureReason::ReplayRejected);
        }

        let Some(challenge) = self.issued_challenges.get(&input.challenge_ref).cloned() else {
            return Err(ParentPresenceVerificationFailureReason::ChallengeNotIssued);
        };

        let observed_at = (self.clock)();
        if let Some(failure_reason) =
            parent_presence_verification_failure_reason(&challenge, &input.assertion, &observed_at)
        {
            return Err(failure_reason);
        }

        let challenge = self
            .issued_challenges
            .remove(&input.challenge_ref)
            .expect("challenge must still exist after a successful lookup");
        self.consumed_challenge_refs
            .insert(challenge.challenge_ref.clone());

        Ok(ParentPresenceVerificationAccepted::new(
            ParentPresenceReceiptRef::from_string(format!(
                "parent-presence-receipt:{}",
                challenge.challenge_ref
            )),
            challenge,
            input.assertion,
            observed_at,
        ))
    }
}
