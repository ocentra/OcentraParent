use ocentra_eventing::envelope::{DomainEvent, EventEnvelope, EventMetadata};
use ocentra_storage_custody_core::storage_custody::{
    storage_custody_action_planned_event, storage_custody_decision_recorded_event,
    StorageCustodyActionPlannedEvent, StorageCustodyAggregateId, StorageCustodyDecisionId,
    StorageCustodyEffect, StorageCustodyEffectKind, StorageCustodyExecutionRequest,
    StorageCustodyInput,
};

use super::{
    storage_custody_runtime_authority_reasons::authority_error_reason,
    storage_custody_runtime_existing::existing_outcome,
    storage_custody_runtime_idempotency::{existing_pending, prepare_record, publish_action},
    storage_custody_runtime_validation::{
        invalid_custody, local_delete_action_is_allowed, validate_effect_location,
    },
    ChildStorageCustodyOutcome, ChildStorageCustodyRuntime,
};
use crate::service::ChildAgentServiceError;

impl ChildStorageCustodyRuntime {
    pub(crate) async fn execute(
        &self,
        request: StorageCustodyExecutionRequest,
        metadata: EventMetadata,
    ) -> Result<ChildStorageCustodyOutcome, ChildAgentServiceError> {
        let effect_kind = request.effect.kind();
        if let Err(error) = self.authority.validate_for(effect_kind) {
            return Ok(manual_required(
                effect_kind,
                &metadata,
                authority_error_reason(error),
            ));
        }
        let Some(input) = self.authority.custody_input(effect_kind) else {
            return Ok(manual_required(
                effect_kind,
                &metadata,
                "current custody decision state is unavailable from its owner",
            ));
        };
        validate_effect_location(effect_kind, input.location)?;
        if let StorageCustodyEffect::DeleteLocal { relative_path } = &request.effect {
            if !self.authority.allows_local_payload(relative_path) {
                return Ok(manual_required(
                    effect_kind,
                    &metadata,
                    "local payload reference is not approved by the custody owner",
                ));
            }
        } else if effect_kind == StorageCustodyEffectKind::LocalDelete {
            return Err(invalid_custody("local delete effect shape is invalid"));
        }
        let action = build_action(&self.authority, &metadata, input)?;
        if effect_kind == StorageCustodyEffectKind::LocalDelete
            && !local_delete_action_is_allowed(&action)
        {
            return Ok(manual_required(
                effect_kind,
                &metadata,
                "current custody decision does not permit local deletion",
            ));
        }
        let operation_ref = action.action_plan_id.as_str().to_owned();
        if let Some(outcome) = existing_outcome(self, &operation_ref, effect_kind, &request.effect)?
        {
            return Ok(outcome);
        }
        if let Some(record) = existing_pending(self, &operation_ref, effect_kind, &request.effect)?
        {
            return self.resume_pending_record(&record).await;
        }
        let envelope = EventEnvelope::from_event(action.clone(), metadata.clone())
            .and_then(|event| event.store())
            .map_err(ChildAgentServiceError::Runtime)?;
        self.effects
            .prepare(prepare_record(
                &self.authority,
                operation_ref.clone(),
                effect_kind,
                request.effect.reference(),
                &request.effect,
                input,
                action.clone(),
                envelope,
            ))
            .map_err(ChildAgentServiceError::Storage)?;
        if !publish_action(&self.flow, effect_kind, action, metadata).await? {
            return Ok(ChildStorageCustodyOutcome::PendingJournalRetry {
                operation_ref,
                effect: effect_kind,
            });
        }
        self.effects
            .mark_journaled(&operation_ref)
            .map_err(ChildAgentServiceError::Storage)?;
        self.finish_record_by_ref(&operation_ref).await
    }
}

fn manual_required(
    effect: StorageCustodyEffectKind,
    metadata: &EventMetadata,
    reason: &str,
) -> ChildStorageCustodyOutcome {
    ChildStorageCustodyOutcome::ManualRequired {
        operation_ref: format!("custody-manual:{}", metadata.correlation_id.as_str()),
        effect,
        reason: reason.to_owned(),
    }
}

fn build_action(
    authority: &super::ChildStorageCustodyAuthorityHandle,
    metadata: &EventMetadata,
    input: StorageCustodyInput,
) -> Result<StorageCustodyActionPlannedEvent, ChildAgentServiceError> {
    let aggregate_id = StorageCustodyAggregateId::parse(format!(
        "child-storage-custody:{}:{}:{}",
        authority.household_id(),
        authority.child_profile_id(),
        authority.target_device_id()
    ))
    .map_err(ChildAgentServiceError::Runtime)?;
    let decision_id = StorageCustodyDecisionId::parse(format!(
        "child-storage-custody-decision:{}",
        metadata.correlation_id.as_str()
    ))
    .map_err(ChildAgentServiceError::Runtime)?;
    let decision = storage_custody_decision_recorded_event(aggregate_id, decision_id, input);
    Ok(storage_custody_action_planned_event(decision))
}
