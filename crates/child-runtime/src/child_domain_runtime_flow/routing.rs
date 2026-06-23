use ocentra_eventing::{
    envelope::EventMetadata, envelope::EventSource, error::EventingError, ids::CorrelationId,
    ids::EventCustody, ids::EventId, ids::RecordedAt, ids::RuntimeInstanceId, ids::RuntimeRole,
    ids::SourceComponent, ids::SourceService, ids::TargetHandler,
};
use ocentra_lan_core::lan_pairing;
use ocentra_network_core::network_runtime;
use ocentra_parent_agent_protocol::child_domain_runtime::{
    ChildDomainAiAnalysisRequestedEvent, ChildDomainAiRequestId, ChildDomainEvidenceRecordedEvent,
    ChildDomainEvidenceRef, ChildDomainObservationId, ChildDomainObservedAt,
    ChildDomainObservedEvent, ChildDomainPolicyEvaluationRequestedEvent,
    ChildDomainPolicyRequestId, ChildDomainPolicyViolationId, ChildRuntimeDomain,
};
use ocentra_parent_agent_protocol::constants;

pub(super) fn child_domain_evidence_recorded_event(
    event: &ChildDomainObservedEvent,
) -> Result<ChildDomainEvidenceRecordedEvent, EventingError> {
    match event.domain {
        ChildRuntimeDomain::App => Ok(ocentra_app_core::app_evidence_recorded_event(event)),
        ChildRuntimeDomain::AppGame => Ok(ocentra_app_game_core::app_game_evidence_recorded_event(
            event,
        )),
        ChildRuntimeDomain::Browser => {
            Ok(ocentra_browser_core::browser_evidence_recorded_event(event))
        }
        ChildRuntimeDomain::Lan => Ok(lan_pairing::lan_evidence_recorded_event(event)),
        ChildRuntimeDomain::Network => Ok(network_runtime::network_evidence_recorded_event(event)),
        ChildRuntimeDomain::Screen => {
            Ok(ocentra_screen_core::screen_evidence_recorded_event(event))
        }
        ChildRuntimeDomain::ScreenLiveView => {
            Ok(ocentra_screen_live_view_core::screen_live_view_evidence_recorded_event(event))
        }
    }
}

pub(super) fn child_domain_ai_analysis_requested_event(
    event: &ChildDomainEvidenceRecordedEvent,
) -> Result<Option<ChildDomainAiAnalysisRequestedEvent>, EventingError> {
    match event.domain {
        ChildRuntimeDomain::App => Ok(ocentra_app_core::app_ai_analysis_requested_event(event)),
        ChildRuntimeDomain::AppGame => Ok(
            ocentra_app_game_core::app_game_ai_analysis_requested_event(event),
        ),
        ChildRuntimeDomain::Browser => Ok(
            ocentra_browser_core::browser_ai_analysis_requested_event(event),
        ),
        ChildRuntimeDomain::Lan => Ok(lan_pairing::lan_ai_analysis_requested_event(event)),
        ChildRuntimeDomain::Network => {
            Ok(network_runtime::network_ai_analysis_requested_event(event))
        }
        ChildRuntimeDomain::Screen => Ok(ocentra_screen_core::screen_ai_analysis_requested_event(
            event,
        )),
        ChildRuntimeDomain::ScreenLiveView => {
            Ok(ocentra_screen_live_view_core::screen_live_view_ai_analysis_requested_event(event))
        }
    }
}

pub(super) fn child_domain_policy_evaluation_requested_event(
    event: &ChildDomainEvidenceRecordedEvent,
) -> Result<Option<ChildDomainPolicyEvaluationRequestedEvent>, EventingError> {
    match event.domain {
        ChildRuntimeDomain::App => Ok(ocentra_app_core::app_policy_evaluation_requested_event(
            event,
        )),
        ChildRuntimeDomain::AppGame => {
            Ok(ocentra_app_game_core::app_game_policy_evaluation_requested_event(event))
        }
        ChildRuntimeDomain::Browser => {
            Ok(ocentra_browser_core::browser_policy_evaluation_requested_event(event))
        }
        ChildRuntimeDomain::Lan => Ok(lan_pairing::lan_policy_evaluation_requested_event(event)),
        ChildRuntimeDomain::Network => Ok(
            network_runtime::network_policy_evaluation_requested_event(event),
        ),
        ChildRuntimeDomain::Screen => {
            Ok(ocentra_screen_core::screen_policy_evaluation_requested_event(event))
        }
        ChildRuntimeDomain::ScreenLiveView => Ok(
            ocentra_screen_live_view_core::screen_live_view_policy_evaluation_requested_event(
                event,
            ),
        ),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ChildDomainRuntimeHop<'a> {
    Observed(&'a ChildDomainObservationId),
    EvidenceRecorded(&'a ChildDomainEvidenceRef),
    AiAnalysisRequested(&'a ChildDomainEvidenceRef),
    AiAnalysisCompleted(&'a ChildDomainAiRequestId),
    PolicyEvaluationRequested(&'a ChildDomainEvidenceRef),
    PolicyEvaluationRequestedFromAi(&'a ChildDomainAiRequestId),
    PolicyViolationDetected(&'a ChildDomainPolicyRequestId),
    NotificationRequested(&'a ChildDomainPolicyViolationId),
}

impl<'a> ChildDomainRuntimeHop<'a> {
    fn source_component(self) -> &'static str {
        match self {
            Self::Observed(_)
            | Self::EvidenceRecorded(_)
            | Self::AiAnalysisRequested(_)
            | Self::PolicyEvaluationRequested(_)
            | Self::PolicyEvaluationRequestedFromAi(_) => {
                constants::child_domain_runtime::SOURCE_COMPONENT_CHILD_DOMAIN_RUNTIME
            }
            Self::AiAnalysisCompleted(_) => {
                constants::child_domain_runtime::SOURCE_COMPONENT_CHILD_AI_RUNTIME
            }
            Self::PolicyViolationDetected(_) => {
                constants::child_domain_runtime::SOURCE_COMPONENT_CHILD_POLICY_RUNTIME
            }
            Self::NotificationRequested(_) => {
                constants::child_domain_runtime::SOURCE_COMPONENT_CHILD_NOTIFICATION_RUNTIME
            }
        }
    }

    fn runtime_role(self) -> &'static str {
        match self {
            Self::Observed(_)
            | Self::EvidenceRecorded(_)
            | Self::AiAnalysisRequested(_)
            | Self::PolicyEvaluationRequested(_)
            | Self::PolicyEvaluationRequestedFromAi(_) => constants::eventing_source::ROLE_AGENT,
            Self::AiAnalysisCompleted(_) => constants::eventing_source::ROLE_ANALYZER,
            Self::PolicyViolationDetected(_) => constants::eventing_source::ROLE_DECISION_ENGINE,
            Self::NotificationRequested(_) => constants::eventing_source::ROLE_SIDE_EFFECT_ADAPTER,
        }
    }

    fn target_handler(self) -> &'static str {
        match self {
            Self::Observed(_) | Self::EvidenceRecorded(_) => {
                constants::child_domain_runtime::TARGET_HANDLER_DOMAIN_OBSERVER
            }
            Self::AiAnalysisRequested(_) => {
                constants::child_domain_runtime::TARGET_HANDLER_CHILD_AI_ANALYZER
            }
            Self::AiAnalysisCompleted(_) | Self::PolicyEvaluationRequested(_) => {
                constants::child_domain_runtime::TARGET_HANDLER_CHILD_POLICY_EVALUATOR
            }
            Self::PolicyEvaluationRequestedFromAi(_) => {
                constants::child_domain_runtime::TARGET_HANDLER_CHILD_POLICY_EVALUATOR
            }
            Self::PolicyViolationDetected(_) | Self::NotificationRequested(_) => {
                constants::child_domain_runtime::TARGET_HANDLER_CHILD_NOTIFICATION_BRIDGE
            }
        }
    }

    fn correlation_ref(self) -> &'a str {
        match self {
            Self::Observed(value) => value.as_str(),
            Self::EvidenceRecorded(value)
            | Self::AiAnalysisRequested(value)
            | Self::PolicyEvaluationRequested(value) => value.as_str(),
            Self::AiAnalysisCompleted(value) => value.as_str(),
            Self::PolicyEvaluationRequestedFromAi(value) => value.as_str(),
            Self::PolicyViolationDetected(value) => value.as_str(),
            Self::NotificationRequested(value) => value.as_str(),
        }
    }
}

pub(super) fn child_domain_runtime_metadata(
    hop: ChildDomainRuntimeHop<'_>,
    recorded_at: &ChildDomainObservedAt,
) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        EventId::generated(),
        CorrelationId::parse(child_domain_runtime_correlation_id(hop))?,
        EventSource::new(
            EventCustody::parse(constants::child_agent::CUSTODY_CHILD_AGENT_RUNTIME)?,
            RuntimeRole::parse(hop.runtime_role())?,
            SourceService::parse(constants::peer::LOCAL_DEV_AGENT)?,
            SourceComponent::parse(hop.source_component())?,
            RuntimeInstanceId::parse(constants::child_agent::RUNTIME_INSTANCE_LOCAL_CHILD_AGENT)?,
        ),
        RecordedAt::parse(recorded_at.as_str())?,
        Some(TargetHandler::parse(hop.target_handler())?),
    ))
}

fn child_domain_runtime_correlation_id(hop: ChildDomainRuntimeHop<'_>) -> String {
    let mut value = String::from(constants::child_domain_runtime::CORRELATION_PREFIX);
    value.push_str(hop.correlation_ref());
    value
}
