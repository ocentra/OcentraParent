#![forbid(unsafe_code)]

mod helpers;

use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{
    AggregateKey, EventNamespace, EventType, IdempotencyKey, SchemaVersion,
};
use ocentra_eventing::topology::EventTopologyFamilyVariant;

use super::{
    PolicyEvent, PolicyEventApplyOutcome, PolicyEventKind, PolicyEventReplayRecord,
    PolicyEventScope,
};

pub(crate) fn policy_event_redacted_summary(event: &PolicyEvent) -> String {
    helpers::policy_event_redacted_summary(event)
}

pub(crate) fn policy_event_event_type(event: &PolicyEvent) -> Result<EventType, EventingError> {
    helpers::policy_event_event_type(event)
}

pub(crate) fn policy_event_contract(
    event: &PolicyEvent,
) -> Result<ocentra_eventing::envelope::EventContract, EventingError> {
    helpers::policy_event_contract(event)
}

pub(crate) fn policy_event_aggregate_key(
    event: &PolicyEvent,
) -> Result<AggregateKey, EventingError> {
    helpers::policy_event_aggregate_key(event)
}

pub(crate) fn policy_event_idempotency_key(
    event: &PolicyEvent,
) -> Result<IdempotencyKey, EventingError> {
    helpers::policy_event_idempotency_key(event)
}

pub(crate) fn policy_event_replay_record(
    event: &PolicyEvent,
) -> Result<PolicyEventReplayRecord, EventingError> {
    helpers::policy_event_replay_record(event)
}

pub(crate) fn policy_event_schema_version() -> Result<SchemaVersion, EventingError> {
    helpers::policy_event_schema_version()
}

pub(crate) fn policy_event_family_namespace() -> Result<EventNamespace, EventingError> {
    helpers::policy_event_family_namespace()
}

pub(crate) fn policy_event_family_variants(
) -> Result<Vec<EventTopologyFamilyVariant>, EventingError> {
    helpers::policy_event_family_variants()
}

pub(crate) fn policy_event_contract_registry(
) -> Result<ocentra_eventing::contract_registry::EventContractRegistry, EventingError> {
    helpers::policy_event_contract_registry()
}

pub(crate) fn apply_policy_event_replay(
    current: &PolicyEventReplayRecord,
    next: &PolicyEvent,
) -> Result<PolicyEventApplyOutcome, EventingError> {
    helpers::apply_policy_event_replay(current, next)
}

pub(crate) fn policy_event_kind_reason_code_value(kind: PolicyEventKind) -> &'static str {
    helpers::policy_event_kind_reason_code_value(kind)
}

pub(crate) fn policy_event_scope_family_label(scope: &PolicyEventScope) -> &'static str {
    helpers::policy_event_scope_family_label(scope)
}

pub(crate) fn policy_event_scope_aggregate_key(
    scope: &PolicyEventScope,
) -> Result<AggregateKey, EventingError> {
    helpers::policy_event_scope_aggregate_key(scope)
}
