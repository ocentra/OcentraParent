use ocentra_eventing::delivery::validation::EventDeliveryDecisionProof;

#[derive(Clone, Debug)]
pub struct BrowserRuntimeDeliveryDecisionReport {
    pub chain_delivery: EventDeliveryDecisionProof,
    pub action_intent_status_delivery: EventDeliveryDecisionProof,
    pub action_intent_handoff_delivery: EventDeliveryDecisionProof,
    pub runtime_stream_report_delivery: EventDeliveryDecisionProof,
    pub social_provider_receipt_status_delivery: EventDeliveryDecisionProof,
    pub social_report_writer_delivery_status_delivery: EventDeliveryDecisionProof,
    pub social_parent_notification_delivery_status_delivery: EventDeliveryDecisionProof,
    pub social_parent_surface_status_delivery: EventDeliveryDecisionProof,
    pub external_transport_delivery: EventDeliveryDecisionProof,
    pub local_ready_route_count: usize,
    pub external_transport_manual_required: bool,
    pub external_transport_delivery_implemented: bool,
    pub external_relay_delivery_implemented: bool,
    pub adapter_dispatch_claimed: bool,
    pub browser_mutation_claimed: bool,
    pub child_intervention_execution_claimed: bool,
    pub final_policy_execution_claimed: bool,
    pub enforcement_claimed: bool,
}
