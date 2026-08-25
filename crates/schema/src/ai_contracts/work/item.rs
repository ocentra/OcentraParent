use super::{AiWorkItem, AiWorkState};
use crate::ai_contracts::{
    AiDegradedState, AiDurabilityState, AiSafeText, AiValidationState,
    AI_INITIAL_LIFECYCLE_SEQUENCE,
};

impl AiWorkItem {
    pub(crate) fn queued(request: super::AiWorkRequest) -> Result<Self, &'static str> {
        let requested_at = request.requested_at().clone();
        Ok(Self {
            last_transition_at: requested_at,
            request,
            state: AiWorkState::Queued,
            attempt: 0,
            durability: AiDurabilityState::AppendPending,
            validation: AiValidationState::ManualRequired,
            degraded_state: AiDegradedState::None,
            last_transition_sequence: AI_INITIAL_LIFECYCLE_SEQUENCE,
            terminal_reason: None,
        })
    }

    pub(crate) fn transition(
        &self,
        authority: super::AiWorkTransitionAuthority,
        next_state: AiWorkState,
        sequence: u64,
        occurred_at: super::AiTimestamp,
        durability: AiDurabilityState,
        validation: AiValidationState,
        degraded_state: AiDegradedState,
        terminal_reason: Option<AiSafeText>,
    ) -> Result<Self, &'static str> {
        let expected_sequence = self.last_transition_sequence.checked_add(1);
        let consumes_attempt = matches!(next_state, AiWorkState::Claimed);
        let next_attempt = if consumes_attempt {
            self.attempt
                .checked_add(1)
                .ok_or("AI work item attempt counter overflowed")?
        } else {
            self.attempt
        };
        if !next_state.can_transition_from(Some(self.state))
            || expected_sequence != Some(sequence)
            || !authority.permits(
                self.request.work_item_id(),
                self.request.request_id(),
                sequence,
                next_state,
                self.request.retry_policy.max_attempts(),
            )
            || next_attempt > self.request.retry_policy.max_attempts()
            || !matches!(durability, AiDurabilityState::Durable)
            || !occurred_at.is_well_formed()
            || !self.last_transition_at.precedes(&occurred_at)
            || (next_state.is_terminal() && terminal_reason.is_none())
            || (!next_state.is_terminal() && terminal_reason.is_some())
        {
            return Err("AI work item transition is illegal or not durable");
        }
        Ok(Self {
            request: self.request.clone(),
            state: next_state,
            attempt: next_attempt,
            durability,
            validation,
            degraded_state,
            last_transition_sequence: sequence,
            last_transition_at: occurred_at,
            terminal_reason,
        })
    }

    pub fn request(&self) -> &super::AiWorkRequest {
        &self.request
    }

    pub fn state(&self) -> AiWorkState {
        self.state
    }

    pub fn attempt(&self) -> u16 {
        self.attempt
    }

    pub fn durability(&self) -> AiDurabilityState {
        self.durability
    }

    pub fn last_transition_sequence(&self) -> u64 {
        self.last_transition_sequence
    }
}
