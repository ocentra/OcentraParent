#![forbid(unsafe_code)]

use ocentra_eventing::contract_registry::EventContractRegistry;
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{
    AggregateKey, EventNamespace, EventType, IdempotencyKey, SchemaVersion,
};
use ocentra_eventing::topology::EventTopologyFamilyVariant;

use super::{
    PolicyEvent, PolicyEventApplyOutcome, PolicyEventKind, PolicyEventReplayRecord,
    PolicyEventScope,
};

mod event_contract;
mod idempotency;
mod registry;
mod replay_apply;
mod sample;
mod scope;
mod scope_key;
mod scope_label;
mod scope_value;

pub(crate) fn policy_event_redacted_summary(event: &PolicyEvent) -> String {
    event_contract::policy_event_redacted_summary(event)
}

pub(crate) fn policy_event_event_type(event: &PolicyEvent) -> Result<EventType, EventingError> {
    event_contract::policy_event_event_type(event)
}

pub(crate) fn policy_event_contract(
    event: &PolicyEvent,
) -> Result<ocentra_eventing::envelope::EventContract, EventingError> {
    event_contract::policy_event_contract(event)
}

pub(crate) fn policy_event_aggregate_key(
    event: &PolicyEvent,
) -> Result<AggregateKey, EventingError> {
    event_contract::policy_event_aggregate_key(event)
}

pub(crate) fn policy_event_idempotency_key(
    event: &PolicyEvent,
) -> Result<IdempotencyKey, EventingError> {
    event_contract::policy_event_idempotency_key(event)
}

pub(crate) fn policy_event_replay_record(
    event: &PolicyEvent,
) -> Result<PolicyEventReplayRecord, EventingError> {
    event_contract::policy_event_replay_record(event)
}

pub(crate) fn policy_event_schema_version() -> Result<SchemaVersion, EventingError> {
    registry::policy_event_schema_version()
}

pub(crate) fn policy_event_family_namespace() -> Result<EventNamespace, EventingError> {
    registry::policy_event_family_namespace()
}

pub(crate) fn policy_event_family_variants(
) -> Result<Vec<EventTopologyFamilyVariant>, EventingError> {
    registry::policy_event_family_variants()
}

pub(crate) fn policy_event_contract_registry() -> Result<EventContractRegistry, EventingError> {
    registry::policy_event_contract_registry()
}

pub(crate) fn apply_policy_event_replay(
    current: &PolicyEventReplayRecord,
    next: &PolicyEvent,
) -> Result<PolicyEventApplyOutcome, EventingError> {
    replay_apply::apply_policy_event_replay(current, next)
}

pub(crate) fn policy_event_kind_reason_code_value(kind: PolicyEventKind) -> &'static str {
    sample::policy_event_kind_reason_code_value(kind)
}

pub(crate) fn policy_event_scope_family_label(scope: &PolicyEventScope) -> &'static str {
    scope_label::policy_event_scope_family_label(scope)
}

pub(crate) fn policy_event_scope_aggregate_key(
    scope: &PolicyEventScope,
) -> Result<AggregateKey, EventingError> {
    scope_key::policy_event_scope_aggregate_key(scope)
}
