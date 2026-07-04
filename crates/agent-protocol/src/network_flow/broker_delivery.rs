use ocentra_eventing::{delivery::validation::EventDeliveryDecisionProof, ids::SourceComponent};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeBrokerDeliverySemantics {
    LocalIdempotencyQueueProof,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeBrokerDeliverySemanticsReport {
    pub delivery_decision: EventDeliveryDecisionProof,
    pub delivery_semantics: NetworkRuntimeBrokerDeliverySemantics,
    pub replay_plan_ref: SourceComponent,
    pub dropped_event_audit_ref: SourceComponent,
    pub adapter_action_ledger_ref: SourceComponent,
    pub queued_duplicate_rejected: bool,
    pub completed_duplicate_rejected: bool,
    pub dropped_event_dead_letter_count: usize,
    pub duplicate_stored_event_count: usize,
    pub enforcement_command_event_count: usize,
    pub adapter_action_executed_count: usize,
    pub external_transport_delivery_implemented: bool,
    pub external_relay_delivery_implemented: bool,
}
