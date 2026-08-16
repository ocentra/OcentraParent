impl DomainEvent for ChildDomainObservedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        child_domain_contract(&self.event_type)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        child_domain_aggregate_key(
            &self.domain,
            self.child_device_id.as_str(),
            self.child_profile_id.as_str(),
        )
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        child_domain_idempotency_key(&self.event_type, self.observation_id.as_str())
    }
}

impl DomainEvent for ChildDomainEvidenceRecordedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        child_domain_contract(&self.event_type)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        child_domain_aggregate_key(
            &self.domain,
            self.child_device_id.as_str(),
            self.child_profile_id.as_str(),
        )
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        child_domain_idempotency_key(&self.event_type, self.evidence_ref.as_str())
    }
}

impl DomainEvent for ChildDomainAiAnalysisRequestedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        child_domain_contract(&self.event_type)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        child_domain_aggregate_key(
            &self.domain,
            self.child_device_id.as_str(),
            self.child_profile_id.as_str(),
        )
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        child_domain_idempotency_key(&self.event_type, self.ai_request_id.as_str())
    }
}

impl DomainEvent for ChildDomainAiAnalysisCompletedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        child_domain_contract(&self.event_type)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        child_domain_aggregate_key(
            &self.domain,
            self.child_device_id.as_str(),
            self.child_profile_id.as_str(),
        )
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        child_domain_idempotency_key(&self.event_type, self.source_ai_request_id.as_str())
    }
}

impl DomainEvent for ChildDomainPolicyEvaluationRequestedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        child_domain_contract(&self.event_type)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        child_domain_aggregate_key(
            &self.domain,
            self.child_device_id.as_str(),
            self.child_profile_id.as_str(),
        )
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        child_domain_idempotency_key(&self.event_type, self.policy_request_id.as_str())
    }
}

impl DomainEvent for ChildDomainPolicyViolationDetectedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        child_domain_contract(&self.event_type)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        child_domain_aggregate_key(
            &self.domain,
            self.child_device_id.as_str(),
            self.child_profile_id.as_str(),
        )
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        child_domain_idempotency_key(&self.event_type, self.violation_id.as_str())
    }
}

impl DomainEvent for ChildDomainNotificationRequestedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        child_domain_contract(&self.event_type)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        child_domain_aggregate_key(
            &self.domain,
            self.child_device_id.as_str(),
            self.child_profile_id.as_str(),
        )
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        child_domain_idempotency_key(&self.event_type, self.notification_id.as_str())
    }
}

fn canonical_child_domain_evidence_refs(
    evidence_refs: &[ChildDomainEvidenceRef],
) -> Vec<ChildDomainEvidenceRef> {
    let mut canonical = Vec::with_capacity(evidence_refs.len());
    for evidence_ref in evidence_refs {
        if !canonical.contains(evidence_ref) {
            canonical.push(evidence_ref.clone());
        }
    }
    canonical
}

pub fn child_domain_observed_event(
    profile: ChildDomainObservedEventProfile,
) -> ChildDomainObservedEvent {
    let subject_ref = child_domain_subject_ref(profile.domain, profile.subject_ref_suffix);
    let observed_state = child_domain_observed_state(profile.observed_state);
    ChildDomainObservedEvent {
        event_type: profile.domain.observed_event_type(),
        domain: profile.domain,
        child_device_id: child_domain_child_device_id(),
        child_profile_id: child_domain_child_profile_id(),
        observation_id: child_domain_observation_id_from_subject_ref(
            profile.domain,
            &subject_ref,
            &observed_state,
        ),
        subject_ref,
        observed_state,
        observed_at: child_domain_observed_at(),
        ai_analysis_requirement: profile.ai_analysis_requirement,
        policy_evaluation_requirement: profile.policy_evaluation_requirement,
    }
}

pub fn child_domain_evidence_recorded_event(
    event: &ChildDomainObservedEvent,
) -> ChildDomainEvidenceRecordedEvent {
    ChildDomainEvidenceRecordedEvent {
        event_type: event.domain.evidence_recorded_event_type(),
        domain: event.domain,
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        evidence_ref: child_domain_evidence_ref_from_observation_id(
            event.domain,
            &event.observation_id,
        ),
        source_observation_id: event.observation_id.clone(),
        source_observed_at: event.observed_at.clone(),
        signal: event.observed_state.clone(),
        ai_analysis_requirement: event.ai_analysis_requirement,
        policy_evaluation_requirement: event.policy_evaluation_requirement,
    }
}

pub fn child_domain_ai_analysis_requested_event(
    event: &ChildDomainEvidenceRecordedEvent,
) -> ChildDomainAiAnalysisRequestedEvent {
    ChildDomainAiAnalysisRequestedEvent {
        event_type: event.domain.ai_analysis_requested_event_type(),
        domain: event.domain,
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        ai_request_id: child_domain_ai_request_id_from_evidence_ref(
            event.domain,
            &event.evidence_ref,
        ),
        evidence_refs: vec![event.evidence_ref.clone()],
        source_observed_at: event.source_observed_at.clone(),
        allowed_analysis_purpose: child_domain_analysis_purpose(
            ChildDomainAnalysisPurposeKind::Classification,
        ),
        private_payload_state: PrivatePayloadState::Excluded,
        policy_evaluation_requirement: event.policy_evaluation_requirement,
    }
}

pub fn child_domain_ai_analysis_requested_event_if_required(
    event: &ChildDomainEvidenceRecordedEvent,
) -> Option<ChildDomainAiAnalysisRequestedEvent> {
    if event.ai_analysis_requirement == ChildDomainAiAnalysisRequirement::Required {
        Some(child_domain_ai_analysis_requested_event(event))
    } else {
        None
    }
}

pub fn child_domain_policy_evaluation_requested_event(
    event: &ChildDomainEvidenceRecordedEvent,
    source_fact_ref: ChildDomainFactRef,
) -> ChildDomainPolicyEvaluationRequestedEvent {
    ChildDomainPolicyEvaluationRequestedEvent {
        event_type: event.domain.policy_evaluation_requested_event_type(),
        domain: event.domain,
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        policy_request_id: child_domain_policy_request_id_from_fact_ref(
            event.domain,
            &source_fact_ref,
        ),
        evidence_refs: vec![event.evidence_ref.clone()],
        source_observed_at: event.source_observed_at.clone(),
        source_fact_ref,
    }
}

pub fn child_domain_direct_policy_evaluation_requested_event_if_required(
    event: &ChildDomainEvidenceRecordedEvent,
) -> Option<ChildDomainPolicyEvaluationRequestedEvent> {
    if event.policy_evaluation_requirement == ChildDomainPolicyEvaluationRequirement::Required
        && event.ai_analysis_requirement == ChildDomainAiAnalysisRequirement::NotRequired
    {
        Some(child_domain_policy_evaluation_requested_event(
            event,
            child_domain_fact_ref_from_observation_id(&event.source_observation_id),
        ))
    } else {
        None
    }
}

pub fn child_domain_policy_evaluation_requested_from_ai_event(
    event: &ChildDomainAiAnalysisRequestedEvent,
) -> ChildDomainPolicyEvaluationRequestedEvent {
    child_domain_policy_evaluation_requested_from_ai_result_event(
        &child_domain_ai_analysis_completed_event(event),
    )
}

pub fn child_domain_ai_analysis_completed_event(
    event: &ChildDomainAiAnalysisRequestedEvent,
) -> ChildDomainAiAnalysisCompletedEvent {
    ChildDomainAiAnalysisCompletedEvent {
        event_type: ChildDomainEventType::ai_analysis_completed(),
        domain: event.domain,
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        source_ai_request_id: event.ai_request_id.clone(),
        evidence_refs: event.evidence_refs.clone(),
        source_observed_at: event.source_observed_at.clone(),
        result_fact_ref: child_domain_fact_ref_from_ai_request_id(&event.ai_request_id),
        private_payload_state: PrivatePayloadState::Excluded,
        policy_evaluation_requirement: event.policy_evaluation_requirement,
    }
}

pub fn child_domain_policy_evaluation_requested_from_ai_result_event(
    event: &ChildDomainAiAnalysisCompletedEvent,
) -> ChildDomainPolicyEvaluationRequestedEvent {
    let source_fact_ref = event.result_fact_ref.clone();
    ChildDomainPolicyEvaluationRequestedEvent {
        event_type: event.domain.policy_evaluation_requested_event_type(),
        domain: event.domain,
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        policy_request_id: child_domain_policy_request_id_from_fact_ref(
            event.domain,
            &source_fact_ref,
        ),
        evidence_refs: event.evidence_refs.clone(),
        source_observed_at: event.source_observed_at.clone(),
        source_fact_ref,
    }
}

pub fn child_domain_policy_evaluation_requested_from_ai_event_if_required(
    event: &ChildDomainAiAnalysisRequestedEvent,
) -> Option<ChildDomainPolicyEvaluationRequestedEvent> {
    child_domain_policy_evaluation_requested_from_ai_result_event_if_required(
        &child_domain_ai_analysis_completed_event(event),
    )
}

pub fn child_domain_policy_evaluation_requested_from_ai_result_event_if_required(
    event: &ChildDomainAiAnalysisCompletedEvent,
) -> Option<ChildDomainPolicyEvaluationRequestedEvent> {
    if event.policy_evaluation_requirement == ChildDomainPolicyEvaluationRequirement::Required {
        Some(child_domain_policy_evaluation_requested_from_ai_result_event(event))
    } else {
        None
    }
}

pub fn child_domain_policy_violation_detected_event(
    event: &ChildDomainPolicyEvaluationRequestedEvent,
) -> ChildDomainPolicyViolationDetectedEvent {
    ChildDomainPolicyViolationDetectedEvent {
        event_type: ChildDomainEventType::policy_violation_detected(),
        domain: event.domain,
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        violation_id: child_domain_policy_violation_id_from_policy_request_id(
            &event.policy_request_id,
        ),
        policy_rule_ref: child_domain_policy_rule_ref(ChildDomainPolicyRuleKind::Default),
        severity: child_domain_policy_severity(ChildDomainPolicySeverityKind::Review),
        detected_at: event.source_observed_at.clone(),
        evidence_refs: canonical_child_domain_evidence_refs(&event.evidence_refs),
    }
}

pub fn child_domain_notification_requested_event(
    event: &ChildDomainPolicyViolationDetectedEvent,
) -> ChildDomainNotificationRequestedEvent {
    ChildDomainNotificationRequestedEvent {
        event_type: ChildDomainEventType::notification_requested(),
        domain: event.domain,
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        notification_id: child_domain_notification_id_from_policy_violation_id(&event.violation_id),
        source_policy_violation_id: event.violation_id.clone(),
        channel: child_domain_notification_channel(
            ChildDomainNotificationChannelKind::ParentPortal,
        ),
        requested_at: event.detected_at.clone(),
        evidence_refs: canonical_child_domain_evidence_refs(&event.evidence_refs),
    }
}
