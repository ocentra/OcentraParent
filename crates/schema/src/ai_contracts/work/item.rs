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
        next_state: AiWorkState,
        sequence: u64,
        occurred_at: super::AiTimestamp,
        durability: AiDurabilityState,
        validation: AiValidationState,
        degraded_state: AiDegradedState,
        terminal_reason: Option<AiSafeText>,
    ) -> Result<Self, &'static str> {
        if !next_state.can_transition_from(Some(self.state))
            || self
                .last_transition_sequence
                .checked_add(1)
                .is_none_or(|expected| sequence != expected)
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
            attempt: self.attempt.saturating_add(u16::from(matches!(
                next_state,
                AiWorkState::Claimed | AiWorkState::Running
            ))),
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
