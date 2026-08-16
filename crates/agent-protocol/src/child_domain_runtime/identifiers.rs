pub fn child_domain_child_device_id() -> ChildDomainChildDeviceId {
    parse_or_panic(
        ChildDomainChildDeviceId::parse(constants::child_domain_runtime::DEFAULT_CHILD_DEVICE_ID),
        constants::child_domain_runtime::DEFAULT_CHILD_DEVICE_ID,
    )
}

pub fn child_domain_child_profile_id() -> ChildDomainChildProfileId {
    parse_or_panic(
        ChildDomainChildProfileId::parse(constants::child_domain_runtime::DEFAULT_CHILD_PROFILE_ID),
        constants::child_domain_runtime::DEFAULT_CHILD_PROFILE_ID,
    )
}

pub fn child_domain_observed_at() -> ChildDomainObservedAt {
    parse_or_panic(
        ChildDomainObservedAt::parse(constants::child_domain_runtime::DEFAULT_OBSERVED_AT),
        constants::child_domain_runtime::DEFAULT_OBSERVED_AT,
    )
}

pub fn child_domain_observed_state(value: ChildDomainObservedSignal) -> ChildDomainObservedState {
    value.into_observed_state()
}

pub fn child_domain_analysis_purpose(
    value: ChildDomainAnalysisPurposeKind,
) -> ChildDomainAnalysisPurpose {
    let value = value.as_contract_text();
    parse_or_panic(ChildDomainAnalysisPurpose::parse(value), value)
}

pub fn child_domain_policy_rule_ref(value: ChildDomainPolicyRuleKind) -> ChildDomainPolicyRuleRef {
    let value = value.as_contract_text();
    parse_or_panic(ChildDomainPolicyRuleRef::parse(value), value)
}

pub fn child_domain_policy_severity(
    value: ChildDomainPolicySeverityKind,
) -> ChildDomainPolicySeverity {
    let value = value.as_contract_text();
    parse_or_panic(ChildDomainPolicySeverity::parse(value), value)
}

pub fn child_domain_notification_channel(
    value: ChildDomainNotificationChannelKind,
) -> ChildDomainNotificationChannel {
    let value = value.as_contract_text();
    parse_or_panic(ChildDomainNotificationChannel::parse(value), value)
}

pub fn child_domain_observation_id(
    domain: ChildRuntimeDomain,
    suffix: ChildDomainRefSuffix,
) -> ChildDomainObservationId {
    let suffix_text = suffix.as_contract_text();
    parse_or_panic(
        ChildDomainObservationId::parse(child_domain_ref_text(domain, suffix_text)),
        suffix_text,
    )
}

pub fn child_domain_subject_ref(
    domain: ChildRuntimeDomain,
    suffix: ChildDomainRefSuffix,
) -> ChildDomainSubjectRef {
    let suffix_text = suffix.as_contract_text();
    parse_or_panic(
        ChildDomainSubjectRef::parse(child_domain_ref_text(domain, suffix_text)),
        suffix_text,
    )
}

pub fn child_domain_evidence_ref(
    domain: ChildRuntimeDomain,
    suffix: ChildDomainRefSuffix,
) -> ChildDomainEvidenceRef {
    let suffix_text = suffix.as_contract_text();
    parse_or_panic(
        ChildDomainEvidenceRef::parse(child_domain_ref_text(domain, suffix_text)),
        suffix_text,
    )
}

pub fn child_domain_ai_request_id(
    domain: ChildRuntimeDomain,
    suffix: ChildDomainRefSuffix,
) -> ChildDomainAiRequestId {
    let suffix_text = suffix.as_contract_text();
    parse_or_panic(
        ChildDomainAiRequestId::parse(child_domain_ref_text(domain, suffix_text)),
        suffix_text,
    )
}

pub fn child_domain_policy_request_id(
    domain: ChildRuntimeDomain,
    suffix: ChildDomainRefSuffix,
) -> ChildDomainPolicyRequestId {
    let suffix_text = suffix.as_contract_text();
    parse_or_panic(
        ChildDomainPolicyRequestId::parse(child_domain_ref_text(domain, suffix_text)),
        suffix_text,
    )
}

pub fn child_domain_fact_ref_from_observation_id(
    value: &ChildDomainObservationId,
) -> ChildDomainFactRef {
    child_domain_fact_ref_text(value.as_str())
}

pub fn child_domain_observation_id_from_subject_ref(
    domain: ChildRuntimeDomain,
    subject_ref: &ChildDomainSubjectRef,
    observed_state: &ChildDomainObservedState,
) -> ChildDomainObservationId {
    let value = child_domain_derived_identifier_text(
        domain.observed_event_type().as_str(),
        &[subject_ref.as_str(), observed_state.as_str()],
    );
    parse_or_panic(
        ChildDomainObservationId::parse(value),
        constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED,
    )
}

pub fn child_domain_evidence_ref_from_observation_id(
    domain: ChildRuntimeDomain,
    observation_id: &ChildDomainObservationId,
) -> ChildDomainEvidenceRef {
    let value = child_domain_derived_identifier_text(
        domain.evidence_recorded_event_type().as_str(),
        &[observation_id.as_str()],
    );
    parse_or_panic(
        ChildDomainEvidenceRef::parse(value),
        constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED,
    )
}

pub fn child_domain_ai_request_id_from_evidence_ref(
    domain: ChildRuntimeDomain,
    evidence_ref: &ChildDomainEvidenceRef,
) -> ChildDomainAiRequestId {
    let value = child_domain_derived_identifier_text(
        domain.ai_analysis_requested_event_type().as_str(),
        &[evidence_ref.as_str()],
    );
    parse_or_panic(
        ChildDomainAiRequestId::parse(value),
        constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED,
    )
}

pub fn child_domain_fact_ref_from_ai_request_id(
    value: &ChildDomainAiRequestId,
) -> ChildDomainFactRef {
    child_domain_fact_ref_text(value.as_str())
}

pub fn child_domain_policy_request_id_from_fact_ref(
    domain: ChildRuntimeDomain,
    fact_ref: &ChildDomainFactRef,
) -> ChildDomainPolicyRequestId {
    let value = child_domain_derived_identifier_text(
        domain.policy_evaluation_requested_event_type().as_str(),
        &[fact_ref.as_str()],
    );
    parse_or_panic(
        ChildDomainPolicyRequestId::parse(value),
        constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED,
    )
}

fn child_domain_fact_ref_text(value: &str) -> ChildDomainFactRef {
    parse_or_panic(
        ChildDomainFactRef::parse(value.to_owned()),
        constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED,
    )
}

fn child_domain_derived_identifier_text(prefix: &str, segments: &[&str]) -> String {
    let mut value = String::from(prefix);
    for segment in segments {
        value.push(':');
        value.push_str(segment);
    }
    value
}

pub fn child_domain_policy_violation_id(
    domain: ChildRuntimeDomain,
    suffix: ChildDomainRefSuffix,
) -> ChildDomainPolicyViolationId {
    let suffix_text = suffix.as_contract_text();
    parse_or_panic(
        ChildDomainPolicyViolationId::parse(child_domain_ref_text(domain, suffix_text)),
        suffix_text,
    )
}

pub fn child_domain_notification_id(
    domain: ChildRuntimeDomain,
    suffix: ChildDomainRefSuffix,
) -> ChildDomainNotificationId {
    let suffix_text = suffix.as_contract_text();
    parse_or_panic(
        ChildDomainNotificationId::parse(child_domain_ref_text(domain, suffix_text)),
        suffix_text,
    )
}

pub fn child_domain_policy_violation_id_from_policy_request_id(
    policy_request_id: &ChildDomainPolicyRequestId,
) -> ChildDomainPolicyViolationId {
    let value = child_domain_derived_identifier_text(
        constants::child_domain_runtime::POLICY_VIOLATION_DETECTED_EVENT_TYPE,
        &[policy_request_id.as_str()],
    );
    parse_or_panic(
        ChildDomainPolicyViolationId::parse(value),
        constants::child_domain_runtime::POLICY_VIOLATION_DETECTED_EVENT_TYPE,
    )
}

pub fn child_domain_notification_id_from_policy_violation_id(
    policy_violation_id: &ChildDomainPolicyViolationId,
) -> ChildDomainNotificationId {
    let value = child_domain_derived_identifier_text(
        constants::child_domain_runtime::NOTIFICATION_REQUESTED_EVENT_TYPE,
        &[policy_violation_id.as_str()],
    );
    parse_or_panic(
        ChildDomainNotificationId::parse(value),
        constants::child_domain_runtime::NOTIFICATION_REQUESTED_EVENT_TYPE,
    )
}

fn child_domain_ref_text(domain: ChildRuntimeDomain, suffix: &str) -> String {
    format!(
        "{}{}{}",
        domain.as_contract_text(),
        constants::child_domain_runtime::IDEMPOTENCY_SEPARATOR,
        suffix
    )
}

fn child_domain_contract(
    event_type: &ChildDomainEventType,
) -> Result<EventContract, EventingError> {
    Ok(EventContract::new(
        EventType::parse(event_type.as_str())?,
        SchemaVersion::new(AGENT_PROTOCOL_SCHEMA_VERSION)?,
    ))
}

fn child_domain_aggregate_key(
    domain: &ChildRuntimeDomain,
    child_device_id: &str,
    child_profile_id: &str,
) -> Result<AggregateKey, EventingError> {
    AggregateKey::parse(format!(
        "{}{}{}{}{}",
        domain.as_contract_text(),
        constants::child_domain_runtime::IDEMPOTENCY_SEPARATOR,
        child_device_id,
        constants::child_domain_runtime::IDEMPOTENCY_SEPARATOR,
        child_profile_id
    ))
}

fn child_domain_idempotency_key(
    event_type: &ChildDomainEventType,
    unique_ref: &str,
) -> Result<IdempotencyKey, EventingError> {
    IdempotencyKey::parse(format!(
        "{}{}{}",
        event_type.as_str(),
        constants::child_domain_runtime::IDEMPOTENCY_SEPARATOR,
        unique_ref
    ))
}
