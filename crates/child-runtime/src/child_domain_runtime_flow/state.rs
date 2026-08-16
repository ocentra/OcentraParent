use std::sync::{Arc, Mutex};

use crate::event_flow_scaffold;
use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::child_domain_runtime::{
    ChildDomainAiAnalysisCompletedEvent, ChildDomainAiAnalysisRequestedEvent,
    ChildDomainEvidenceRecordedEvent, ChildDomainNotificationRequestedEvent,
    ChildDomainPolicyEvaluationRequestedEvent, ChildDomainPolicyViolationDetectedEvent,
};
use ocentra_parent_agent_protocol::constants;

#[derive(Clone, Debug, Default)]
pub(super) struct ChildDomainRuntimeFlowState {
    evidence_recorded: Arc<Mutex<Option<ChildDomainEvidenceRecordedEvent>>>,
    ai_analysis_requested: Arc<Mutex<Option<ChildDomainAiAnalysisRequestedEvent>>>,
    ai_analysis_completed: Arc<Mutex<Option<ChildDomainAiAnalysisCompletedEvent>>>,
    policy_evaluation_requested: Arc<Mutex<Option<ChildDomainPolicyEvaluationRequestedEvent>>>,
    policy_violation_detected: Arc<Mutex<Option<ChildDomainPolicyViolationDetectedEvent>>>,
    notification_requested: Arc<Mutex<Option<ChildDomainNotificationRequestedEvent>>>,
}

impl ChildDomainRuntimeFlowState {
    pub(super) fn record_evidence(&self, event: ChildDomainEvidenceRecordedEvent) {
        event_flow_scaffold::record_optional_event(
            &self.evidence_recorded,
            event,
            constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED,
        );
    }

    pub(super) fn record_ai_analysis_request(&self, event: ChildDomainAiAnalysisRequestedEvent) {
        event_flow_scaffold::record_optional_event(
            &self.ai_analysis_requested,
            event,
            constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED,
        );
    }

    pub(super) fn record_ai_analysis_completed(&self, event: ChildDomainAiAnalysisCompletedEvent) {
        event_flow_scaffold::record_optional_event(
            &self.ai_analysis_completed,
            event,
            constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED,
        );
    }

    pub(super) fn record_policy_evaluation_request(
        &self,
        event: ChildDomainPolicyEvaluationRequestedEvent,
    ) {
        event_flow_scaffold::record_optional_event(
            &self.policy_evaluation_requested,
            event,
            constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED,
        );
    }

    pub(super) fn record_policy_violation(&self, event: ChildDomainPolicyViolationDetectedEvent) {
        event_flow_scaffold::record_optional_event(
            &self.policy_violation_detected,
            event,
            constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED,
        );
    }

    pub(super) fn record_notification(&self, event: ChildDomainNotificationRequestedEvent) {
        event_flow_scaffold::record_optional_event(
            &self.notification_requested,
            event,
            constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED,
        );
    }

    pub(super) fn evidence_recorded(
        &self,
    ) -> Result<ChildDomainEvidenceRecordedEvent, EventingError> {
        event_flow_scaffold::required_optional_event(
            &self.evidence_recorded,
            constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED,
            constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED,
            constants::child_domain_runtime::SIGNAL_OBSERVE_ONLY,
        )
    }

    pub(super) fn ai_analysis_requested(&self) -> Option<ChildDomainAiAnalysisRequestedEvent> {
        event_flow_scaffold::optional_event(&self.ai_analysis_requested)
    }

    pub(super) fn ai_analysis_completed(&self) -> Option<ChildDomainAiAnalysisCompletedEvent> {
        event_flow_scaffold::optional_event(&self.ai_analysis_completed)
    }

    pub(super) fn policy_evaluation_requested(
        &self,
    ) -> Option<ChildDomainPolicyEvaluationRequestedEvent> {
        event_flow_scaffold::optional_event(&self.policy_evaluation_requested)
    }

    pub(super) fn policy_violation_detected(
        &self,
    ) -> Option<ChildDomainPolicyViolationDetectedEvent> {
        event_flow_scaffold::optional_event(&self.policy_violation_detected)
    }

    pub(super) fn notification_requested(&self) -> Option<ChildDomainNotificationRequestedEvent> {
        event_flow_scaffold::optional_event(&self.notification_requested)
    }
}
