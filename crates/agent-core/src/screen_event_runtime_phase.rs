use ocentra_parent_agent_protocol::constants;
use serde::{Deserialize, Serialize};

use ocentra_eventing::ids::RuntimeRole;

const SCREEN_RUNTIME_PHASES: [ScreenRuntimePhase; 9] = [
    ScreenRuntimePhase::CaptureObserved,
    ScreenRuntimePhase::QueueEncrypted,
    ScreenRuntimePhase::AiAnalysisRequested,
    ScreenRuntimePhase::AiAnalysisCompleted,
    ScreenRuntimePhase::SummaryCommitted,
    ScreenRuntimePhase::PolicyDecisionCompleted,
    ScreenRuntimePhase::ActionDryRunRecorded,
    ScreenRuntimePhase::DeletionCommitted,
    ScreenRuntimePhase::PortalReadModelUpdated,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenRuntimePhase {
    CaptureObserved,
    QueueEncrypted,
    AiAnalysisRequested,
    AiAnalysisCompleted,
    SummaryCommitted,
    PolicyDecisionCompleted,
    ActionDryRunRecorded,
    DeletionCommitted,
    PortalReadModelUpdated,
}

impl ScreenRuntimePhase {
    pub fn ordered_chain() -> &'static [Self] {
        &SCREEN_RUNTIME_PHASES
    }

    pub(crate) fn event_type(self) -> &'static str {
        match self {
            Self::CaptureObserved => constants::screen_flow::EVENT_SCREEN_CAPTURE_OBSERVED,
            Self::QueueEncrypted => constants::screen_flow::EVENT_SCREEN_QUEUE_ENCRYPTED,
            Self::AiAnalysisRequested => constants::screen_flow::EVENT_SCREEN_AI_ANALYSIS_REQUESTED,
            Self::AiAnalysisCompleted => constants::screen_flow::EVENT_SCREEN_AI_ANALYSIS_COMPLETED,
            Self::SummaryCommitted => constants::screen_flow::EVENT_SCREEN_SUMMARY_COMMITTED,
            Self::PolicyDecisionCompleted => {
                constants::screen_flow::EVENT_SCREEN_POLICY_DECISION_COMPLETED
            }
            Self::ActionDryRunRecorded => {
                constants::screen_flow::EVENT_SCREEN_ACTION_DRY_RUN_RECORDED
            }
            Self::DeletionCommitted => constants::screen_flow::EVENT_SCREEN_DELETION_COMMITTED,
            Self::PortalReadModelUpdated => {
                constants::screen_flow::EVENT_SCREEN_PORTAL_READ_MODEL_UPDATED
            }
        }
    }

    pub(crate) fn subscriber_id(self) -> &'static str {
        match self {
            Self::CaptureObserved => constants::screen_flow::SUBSCRIBER_SCREEN_CAPTURE_OBSERVER,
            Self::QueueEncrypted => constants::screen_flow::SUBSCRIBER_SCREEN_QUEUE_WRITER,
            Self::AiAnalysisRequested => constants::screen_flow::SUBSCRIBER_SCREEN_AI_REQUEST,
            Self::AiAnalysisCompleted => constants::screen_flow::SUBSCRIBER_SCREEN_AI_COMPLETE,
            Self::SummaryCommitted => constants::screen_flow::SUBSCRIBER_SCREEN_SUMMARY_WRITER,
            Self::PolicyDecisionCompleted => {
                constants::screen_flow::SUBSCRIBER_SCREEN_POLICY_DECISION
            }
            Self::ActionDryRunRecorded => constants::screen_flow::SUBSCRIBER_SCREEN_ACTION_DRY_RUN,
            Self::DeletionCommitted => constants::screen_flow::SUBSCRIBER_SCREEN_DELETION_WORKER,
            Self::PortalReadModelUpdated => {
                constants::screen_flow::SUBSCRIBER_SCREEN_PORTAL_READ_MODEL
            }
        }
    }

    pub(crate) fn target_handler(self) -> &'static str {
        match self {
            Self::CaptureObserved => constants::screen_flow::TARGET_SCREEN_CAPTURE_OBSERVER,
            Self::QueueEncrypted => constants::screen_flow::TARGET_SCREEN_QUEUE_WRITER,
            Self::AiAnalysisRequested | Self::AiAnalysisCompleted => {
                constants::screen_flow::TARGET_SCREEN_AI_ANALYZER
            }
            Self::SummaryCommitted => constants::screen_flow::TARGET_SCREEN_SUMMARY_WRITER,
            Self::PolicyDecisionCompleted => constants::screen_flow::TARGET_SCREEN_POLICY_ENGINE,
            Self::ActionDryRunRecorded => constants::screen_flow::TARGET_SCREEN_ACTION_DRY_RUN,
            Self::DeletionCommitted => constants::screen_flow::TARGET_SCREEN_DELETION_WORKER,
            Self::PortalReadModelUpdated => constants::screen_flow::TARGET_SCREEN_PORTAL_READ_MODEL,
        }
    }

    pub(crate) fn runtime_role(self) -> RuntimeRole {
        let value = match self {
            Self::CaptureObserved | Self::QueueEncrypted => constants::eventing_source::ROLE_AGENT,
            Self::AiAnalysisRequested | Self::AiAnalysisCompleted => {
                constants::eventing_source::ROLE_ANALYZER
            }
            Self::SummaryCommitted | Self::DeletionCommitted => {
                constants::eventing_source::ROLE_AUDIT_WRITER
            }
            Self::PolicyDecisionCompleted => constants::eventing_source::ROLE_DECISION_ENGINE,
            Self::ActionDryRunRecorded => constants::eventing_source::ROLE_SIDE_EFFECT_ADAPTER,
            Self::PortalReadModelUpdated => constants::eventing_source::ROLE_READ_MODEL,
        };
        RuntimeRole::parse(value)
            .expect(constants::eventing_source::ERROR_RUNTIME_ROLE_CONSTANT_PARSES)
    }
}
