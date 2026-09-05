use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, SchemaVersion};
use ocentra_evidence::PrivatePayloadState;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{constants, AGENT_PROTOCOL_SCHEMA_VERSION};

pub const CHILD_DOMAIN_RUNTIME_SCHEMA_VERSION: u16 = crate::AGENT_PROTOCOL_SCHEMA_VERSION;

fn parse_or_panic<T, E: std::fmt::Debug>(result: Result<T, E>, message: &'static str) -> T {
    result.expect_value(message)
}

fn parse_non_empty_text<T>(
    value: impl Into<String>,
    factory: impl FnOnce(String) -> T,
) -> Result<T, EventingError> {
    let value = value.into();
    (!value.trim().is_empty())
        .then_some(factory(value))
        .ok_or(EventingError::EmptyValue {
            field: constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED,
        })
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ChildDomainEventType(String);

impl ChildDomainEventType {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(EventingError::EmptyValue {
                field: constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED,
            });
        }
        is_known_child_domain_event_type(&value)
            .then_some(Self(value.clone()))
            .ok_or(EventingError::InvalidValue {
                field: constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED,
                value,
            })
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn ai_analysis_completed() -> Self {
        parse_or_panic(
            Self::parse(constants::child_domain_runtime::AI_ANALYSIS_COMPLETED_EVENT_TYPE),
            constants::child_domain_runtime::AI_ANALYSIS_COMPLETED_EVENT_TYPE,
        )
    }

    pub fn policy_violation_detected() -> Self {
        parse_or_panic(
            Self::parse(constants::child_domain_runtime::POLICY_VIOLATION_DETECTED_EVENT_TYPE),
            constants::child_domain_runtime::POLICY_VIOLATION_DETECTED_EVENT_TYPE,
        )
    }

    pub fn notification_requested() -> Self {
        parse_or_panic(
            Self::parse(constants::child_domain_runtime::NOTIFICATION_REQUESTED_EVENT_TYPE),
            constants::child_domain_runtime::NOTIFICATION_REQUESTED_EVENT_TYPE,
        )
    }
}

fn is_known_child_domain_event_type(value: &str) -> bool {
    constants::child_domain_runtime::CHILD_DOMAIN_EVENT_TYPES.contains(&value)
}

impl TryFrom<String> for ChildDomainEventType {
    type Error = EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<ChildDomainEventType> for String {
    fn from(value: ChildDomainEventType) -> Self {
        value.0
    }
}

impl std::fmt::Display for ChildDomainEventType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

macro_rules! child_domain_text_type {
    ($name:ident) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
                parse_non_empty_text(value, Self)
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = EventingError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::parse(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

child_domain_text_type!(ChildDomainChildDeviceId);
child_domain_text_type!(ChildDomainChildProfileId);
child_domain_text_type!(ChildDomainObservationId);
child_domain_text_type!(ChildDomainSubjectRef);
child_domain_text_type!(ChildDomainObservedState);
child_domain_text_type!(ChildDomainObservedAt);
child_domain_text_type!(ChildDomainEvidenceRef);
child_domain_text_type!(ChildDomainAiRequestId);
child_domain_text_type!(ChildDomainAnalysisPurpose);
child_domain_text_type!(ChildDomainPolicyRequestId);
child_domain_text_type!(ChildDomainFactRef);
child_domain_text_type!(ChildDomainPolicyViolationId);
child_domain_text_type!(ChildDomainPolicyRuleRef);
child_domain_text_type!(ChildDomainPolicySeverity);
child_domain_text_type!(ChildDomainNotificationId);
child_domain_text_type!(ChildDomainNotificationChannel);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(usize)]
pub enum ChildRuntimeDomain {
    App,
    AppGame,
    Browser,
    Lan,
    Network,
    Screen,
    ScreenLiveView,
}

#[derive(Clone, Copy)]
struct ChildRuntimeDomainSpec {
    contract_text: &'static str,
    observed_event_type: &'static str,
    evidence_recorded_event_type: &'static str,
    ai_analysis_requested_event_type: &'static str,
    policy_evaluation_requested_event_type: &'static str,
    observer_subscriber_id: &'static str,
    default_subject_ref_suffix: ChildDomainRefSuffix,
    default_observed_signal: ChildDomainObservedSignal,
    default_ai_analysis_requirement: ChildDomainAiAnalysisRequirement,
    default_policy_evaluation_requirement: ChildDomainPolicyEvaluationRequirement,
}

const CHILD_RUNTIME_DOMAIN_SPECS: [ChildRuntimeDomainSpec; 7] = [
    ChildRuntimeDomainSpec {
        contract_text: constants::child_domain_runtime::DOMAIN_APP,
        observed_event_type: constants::child_domain_runtime::APP_OBSERVED_EVENT_TYPE,
        evidence_recorded_event_type:
            constants::child_domain_runtime::APP_EVIDENCE_RECORDED_EVENT_TYPE,
        ai_analysis_requested_event_type:
            constants::child_domain_runtime::APP_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
        policy_evaluation_requested_event_type:
            constants::child_domain_runtime::APP_POLICY_EVALUATION_REQUESTED_EVENT_TYPE,
        observer_subscriber_id: constants::child_domain_runtime::SUBSCRIBER_APP_OBSERVER,
        default_subject_ref_suffix: ChildDomainRefSuffix::AppSubject,
        default_observed_signal: ChildDomainObservedSignal::RequiresPolicy,
        default_ai_analysis_requirement: ChildDomainAiAnalysisRequirement::NotRequired,
        default_policy_evaluation_requirement: ChildDomainPolicyEvaluationRequirement::Required,
    },
    ChildRuntimeDomainSpec {
        contract_text: constants::child_domain_runtime::DOMAIN_APP_GAME,
        observed_event_type: constants::child_domain_runtime::APP_GAME_OBSERVED_EVENT_TYPE,
        evidence_recorded_event_type:
            constants::child_domain_runtime::APP_GAME_EVIDENCE_RECORDED_EVENT_TYPE,
        ai_analysis_requested_event_type:
            constants::child_domain_runtime::APP_GAME_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
        policy_evaluation_requested_event_type:
            constants::child_domain_runtime::APP_GAME_POLICY_EVALUATION_REQUESTED_EVENT_TYPE,
        observer_subscriber_id: constants::child_domain_runtime::SUBSCRIBER_APP_GAME_OBSERVER,
        default_subject_ref_suffix: ChildDomainRefSuffix::AppGameSubject,
        default_observed_signal: ChildDomainObservedSignal::RequiresPolicy,
        default_ai_analysis_requirement: ChildDomainAiAnalysisRequirement::NotRequired,
        default_policy_evaluation_requirement: ChildDomainPolicyEvaluationRequirement::Required,
    },
    ChildRuntimeDomainSpec {
        contract_text: constants::child_domain_runtime::DOMAIN_BROWSER,
        observed_event_type: constants::child_domain_runtime::BROWSER_OBSERVED_EVENT_TYPE,
        evidence_recorded_event_type:
            constants::child_domain_runtime::BROWSER_EVIDENCE_RECORDED_EVENT_TYPE,
        ai_analysis_requested_event_type:
            constants::child_domain_runtime::BROWSER_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
        policy_evaluation_requested_event_type:
            constants::child_domain_runtime::BROWSER_POLICY_EVALUATION_REQUESTED_EVENT_TYPE,
        observer_subscriber_id: constants::child_domain_runtime::SUBSCRIBER_BROWSER_OBSERVER,
        default_subject_ref_suffix: ChildDomainRefSuffix::BrowserSubject,
        default_observed_signal: ChildDomainObservedSignal::RequiresAi,
        default_ai_analysis_requirement: ChildDomainAiAnalysisRequirement::Required,
        default_policy_evaluation_requirement: ChildDomainPolicyEvaluationRequirement::Required,
    },
    ChildRuntimeDomainSpec {
        contract_text: constants::child_domain_runtime::DOMAIN_LAN,
        observed_event_type: constants::child_domain_runtime::LAN_OBSERVED_EVENT_TYPE,
        evidence_recorded_event_type:
            constants::child_domain_runtime::LAN_EVIDENCE_RECORDED_EVENT_TYPE,
        ai_analysis_requested_event_type:
            constants::child_domain_runtime::LAN_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
        policy_evaluation_requested_event_type:
            constants::child_domain_runtime::LAN_POLICY_EVALUATION_REQUESTED_EVENT_TYPE,
        observer_subscriber_id: constants::child_domain_runtime::SUBSCRIBER_LAN_OBSERVER,
        default_subject_ref_suffix: ChildDomainRefSuffix::LanSubject,
        default_observed_signal: ChildDomainObservedSignal::RequiresPolicy,
        default_ai_analysis_requirement: ChildDomainAiAnalysisRequirement::NotRequired,
        default_policy_evaluation_requirement: ChildDomainPolicyEvaluationRequirement::Required,
    },
    ChildRuntimeDomainSpec {
        contract_text: constants::child_domain_runtime::DOMAIN_NETWORK,
        observed_event_type: constants::child_domain_runtime::NETWORK_OBSERVED_EVENT_TYPE,
        evidence_recorded_event_type:
            constants::child_domain_runtime::NETWORK_EVIDENCE_RECORDED_EVENT_TYPE,
        ai_analysis_requested_event_type:
            constants::child_domain_runtime::NETWORK_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
        policy_evaluation_requested_event_type:
            constants::child_domain_runtime::NETWORK_POLICY_EVALUATION_REQUESTED_EVENT_TYPE,
        observer_subscriber_id: constants::child_domain_runtime::SUBSCRIBER_NETWORK_OBSERVER,
        default_subject_ref_suffix: ChildDomainRefSuffix::NetworkSubject,
        default_observed_signal: ChildDomainObservedSignal::RequiresPolicy,
        default_ai_analysis_requirement: ChildDomainAiAnalysisRequirement::NotRequired,
        default_policy_evaluation_requirement: ChildDomainPolicyEvaluationRequirement::Required,
    },
    ChildRuntimeDomainSpec {
        contract_text: constants::child_domain_runtime::DOMAIN_SCREEN,
        observed_event_type: constants::child_domain_runtime::SCREEN_OBSERVED_EVENT_TYPE,
        evidence_recorded_event_type:
            constants::child_domain_runtime::SCREEN_EVIDENCE_RECORDED_EVENT_TYPE,
        ai_analysis_requested_event_type:
            constants::child_domain_runtime::SCREEN_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
        policy_evaluation_requested_event_type:
            constants::child_domain_runtime::SCREEN_POLICY_EVALUATION_REQUESTED_EVENT_TYPE,
        observer_subscriber_id: constants::child_domain_runtime::SUBSCRIBER_SCREEN_OBSERVER,
        default_subject_ref_suffix: ChildDomainRefSuffix::ScreenSubject,
        default_observed_signal: ChildDomainObservedSignal::RequiresAi,
        default_ai_analysis_requirement: ChildDomainAiAnalysisRequirement::Required,
        default_policy_evaluation_requirement: ChildDomainPolicyEvaluationRequirement::Required,
    },
    ChildRuntimeDomainSpec {
        contract_text: constants::child_domain_runtime::DOMAIN_SCREEN_LIVE_VIEW,
        observed_event_type: constants::child_domain_runtime::SCREEN_LIVE_VIEW_OBSERVED_EVENT_TYPE,
        evidence_recorded_event_type:
            constants::child_domain_runtime::SCREEN_LIVE_VIEW_EVIDENCE_RECORDED_EVENT_TYPE,
        ai_analysis_requested_event_type:
            constants::child_domain_runtime::SCREEN_LIVE_VIEW_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
        policy_evaluation_requested_event_type:
            constants::child_domain_runtime::SCREEN_LIVE_VIEW_POLICY_EVALUATION_REQUESTED_EVENT_TYPE,
        observer_subscriber_id:
            constants::child_domain_runtime::SUBSCRIBER_SCREEN_LIVE_VIEW_OBSERVER,
        default_subject_ref_suffix: ChildDomainRefSuffix::ScreenLiveViewSubject,
        default_observed_signal: ChildDomainObservedSignal::RequiresPolicy,
        default_ai_analysis_requirement: ChildDomainAiAnalysisRequirement::NotRequired,
        default_policy_evaluation_requirement: ChildDomainPolicyEvaluationRequirement::Required,
    },
];

const CHILD_RUNTIME_DOMAIN_VARIANTS: [&str; 7] = [
    constants::child_domain_runtime::DOMAIN_APP,
    constants::child_domain_runtime::DOMAIN_APP_GAME,
    constants::child_domain_runtime::DOMAIN_BROWSER,
    constants::child_domain_runtime::DOMAIN_LAN,
    constants::child_domain_runtime::DOMAIN_NETWORK,
    constants::child_domain_runtime::DOMAIN_SCREEN,
    constants::child_domain_runtime::DOMAIN_SCREEN_LIVE_VIEW,
];

impl ChildRuntimeDomain {
    fn spec(self) -> &'static ChildRuntimeDomainSpec {
        &CHILD_RUNTIME_DOMAIN_SPECS[self as usize]
    }

    fn parse_contract_text(value: &str) -> Option<Self> {
        [
            Self::App,
            Self::AppGame,
            Self::Browser,
            Self::Lan,
            Self::Network,
            Self::Screen,
            Self::ScreenLiveView,
        ]
        .into_iter()
        .find(|domain| domain.as_contract_text() == value)
    }

    pub fn as_contract_text(self) -> &'static str {
        self.spec().contract_text
    }

    pub fn observed_event_type(self) -> ChildDomainEventType {
        self.event_type(self.spec().observed_event_type)
    }

    pub fn evidence_recorded_event_type(self) -> ChildDomainEventType {
        self.event_type(self.spec().evidence_recorded_event_type)
    }

    pub fn ai_analysis_requested_event_type(self) -> ChildDomainEventType {
        self.event_type(self.spec().ai_analysis_requested_event_type)
    }

    pub fn policy_evaluation_requested_event_type(self) -> ChildDomainEventType {
        self.event_type(self.spec().policy_evaluation_requested_event_type)
    }

    pub fn observer_subscriber_id(self) -> &'static str {
        self.spec().observer_subscriber_id
    }

    pub fn default_observed_profile(self) -> ChildDomainObservedEventProfile {
        let spec = self.spec();
        self.observed_profile(
            spec.default_subject_ref_suffix,
            spec.default_observed_signal,
            spec.default_ai_analysis_requirement,
            spec.default_policy_evaluation_requirement,
        )
    }

    pub fn observed_profile(
        self,
        subject_ref_suffix: ChildDomainRefSuffix,
        observed_state: ChildDomainObservedSignal,
        ai_analysis_requirement: ChildDomainAiAnalysisRequirement,
        policy_evaluation_requirement: ChildDomainPolicyEvaluationRequirement,
    ) -> ChildDomainObservedEventProfile {
        ChildDomainObservedEventProfile {
            domain: self,
            subject_ref_suffix,
            observed_state,
            ai_analysis_requirement,
            policy_evaluation_requirement,
        }
    }

    fn event_type(self, value: &'static str) -> ChildDomainEventType {
        parse_or_panic(ChildDomainEventType::parse(value), value)
    }
}

impl Serialize for ChildRuntimeDomain {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_contract_text())
    }
}

impl<'de> Deserialize<'de> for ChildRuntimeDomain {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse_contract_text(value.as_str()).ok_or_else(|| {
            D::Error::unknown_variant(value.as_str(), &CHILD_RUNTIME_DOMAIN_VARIANTS)
        })
    }
}

impl std::fmt::Display for ChildRuntimeDomain {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_contract_text())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChildDomainAiAnalysisRequirement {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "not-required")]
    NotRequired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChildDomainPolicyEvaluationRequirement {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "not-required")]
    NotRequired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChildDomainObservedEventProfile {
    pub domain: ChildRuntimeDomain,
    pub subject_ref_suffix: ChildDomainRefSuffix,
    pub observed_state: ChildDomainObservedSignal,
    pub ai_analysis_requirement: ChildDomainAiAnalysisRequirement,
    pub policy_evaluation_requirement: ChildDomainPolicyEvaluationRequirement,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum ChildDomainObservedSignal {
    RequiresAi,
    RequiresPolicy,
    ObserveOnly,
}

impl ChildDomainObservedSignal {
    pub fn into_observed_state(self) -> ChildDomainObservedState {
        let value = self.as_contract_text();
        parse_or_panic(ChildDomainObservedState::parse(value), value)
    }

    fn as_contract_text(self) -> &'static str {
        const SIGNAL_TEXTS: [&str; 3] = [
            constants::child_domain_runtime::SIGNAL_REQUIRES_AI,
            constants::child_domain_runtime::SIGNAL_REQUIRES_POLICY,
            constants::child_domain_runtime::SIGNAL_OBSERVE_ONLY,
        ];
        SIGNAL_TEXTS[self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum ChildDomainAnalysisPurposeKind {
    Classification,
}

impl ChildDomainAnalysisPurposeKind {
    fn as_contract_text(self) -> &'static str {
        const ANALYSIS_PURPOSE_TEXTS: [&str; 1] =
            [constants::child_domain_runtime::AI_PURPOSE_CLASSIFICATION];
        ANALYSIS_PURPOSE_TEXTS[self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum ChildDomainPolicyRuleKind {
    Default,
}

impl ChildDomainPolicyRuleKind {
    fn as_contract_text(self) -> &'static str {
        const POLICY_RULE_TEXTS: [&str; 1] = [constants::child_domain_runtime::POLICY_RULE_DEFAULT];
        POLICY_RULE_TEXTS[self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum ChildDomainPolicySeverityKind {
    Review,
}

impl ChildDomainPolicySeverityKind {
    fn as_contract_text(self) -> &'static str {
        const POLICY_SEVERITY_TEXTS: [&str; 1] =
            [constants::child_domain_runtime::POLICY_SEVERITY_REVIEW];
        POLICY_SEVERITY_TEXTS[self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum ChildDomainNotificationChannelKind {
    ParentPortal,
}

impl ChildDomainNotificationChannelKind {
    fn as_contract_text(self) -> &'static str {
        const NOTIFICATION_CHANNEL_TEXTS: [&str; 1] =
            [constants::child_domain_runtime::NOTIFICATION_CHANNEL_PARENT_PORTAL];
        NOTIFICATION_CHANNEL_TEXTS[self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum ChildDomainRefSuffix {
    DefaultObservation,
    AppSubject,
    AppGameSubject,
    BrowserSubject,
    LanSubject,
    NetworkSubject,
    ScreenSubject,
    ScreenLiveViewSubject,
    DefaultEvidence,
    DefaultAiRequest,
    DefaultPolicyRequest,
    DefaultPolicyViolation,
    DefaultNotification,
}

impl ChildDomainRefSuffix {
    fn as_contract_text(self) -> &'static str {
        const REF_SUFFIX_TEXTS: [&str; 13] = [
            constants::child_domain_runtime::DEFAULT_OBSERVATION_ID_SUFFIX,
            constants::child_domain_runtime::APP_SUBJECT_REF_SUFFIX,
            constants::child_domain_runtime::APP_GAME_SUBJECT_REF_SUFFIX,
            constants::child_domain_runtime::BROWSER_SUBJECT_REF_SUFFIX,
            constants::child_domain_runtime::LAN_SUBJECT_REF_SUFFIX,
            constants::child_domain_runtime::NETWORK_SUBJECT_REF_SUFFIX,
            constants::child_domain_runtime::SCREEN_SUBJECT_REF_SUFFIX,
            constants::child_domain_runtime::SCREEN_LIVE_VIEW_SUBJECT_REF_SUFFIX,
            constants::child_domain_runtime::DEFAULT_EVIDENCE_REF_SUFFIX,
            constants::child_domain_runtime::DEFAULT_AI_REQUEST_ID_SUFFIX,
            constants::child_domain_runtime::DEFAULT_POLICY_REQUEST_ID_SUFFIX,
            constants::child_domain_runtime::DEFAULT_POLICY_VIOLATION_ID_SUFFIX,
            constants::child_domain_runtime::DEFAULT_NOTIFICATION_ID_SUFFIX,
        ];
        REF_SUFFIX_TEXTS[self as usize]
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ChildDomainObservedEvent {
    pub event_type: ChildDomainEventType,
    pub domain: ChildRuntimeDomain,
    pub child_device_id: ChildDomainChildDeviceId,
    pub child_profile_id: ChildDomainChildProfileId,
    pub observation_id: ChildDomainObservationId,
    pub subject_ref: ChildDomainSubjectRef,
    pub observed_state: ChildDomainObservedState,
    pub observed_at: ChildDomainObservedAt,
    pub ai_analysis_requirement: ChildDomainAiAnalysisRequirement,
    pub policy_evaluation_requirement: ChildDomainPolicyEvaluationRequirement,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ChildDomainEvidenceRecordedEvent {
    pub event_type: ChildDomainEventType,
    pub domain: ChildRuntimeDomain,
    pub child_device_id: ChildDomainChildDeviceId,
    pub child_profile_id: ChildDomainChildProfileId,
    pub evidence_ref: ChildDomainEvidenceRef,
    pub source_observation_id: ChildDomainObservationId,
    pub source_observed_at: ChildDomainObservedAt,
    pub signal: ChildDomainObservedState,
    pub ai_analysis_requirement: ChildDomainAiAnalysisRequirement,
    pub policy_evaluation_requirement: ChildDomainPolicyEvaluationRequirement,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ChildDomainAiAnalysisRequestedEvent {
    pub event_type: ChildDomainEventType,
    pub domain: ChildRuntimeDomain,
    pub child_device_id: ChildDomainChildDeviceId,
    pub child_profile_id: ChildDomainChildProfileId,
    pub ai_request_id: ChildDomainAiRequestId,
    pub evidence_refs: Vec<ChildDomainEvidenceRef>,
    pub source_observed_at: ChildDomainObservedAt,
    pub allowed_analysis_purpose: ChildDomainAnalysisPurpose,
    pub private_payload_state: PrivatePayloadState,
    pub policy_evaluation_requirement: ChildDomainPolicyEvaluationRequirement,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ChildDomainAiAnalysisCompletedEvent {
    pub event_type: ChildDomainEventType,
    pub domain: ChildRuntimeDomain,
    pub child_device_id: ChildDomainChildDeviceId,
    pub child_profile_id: ChildDomainChildProfileId,
    pub source_ai_request_id: ChildDomainAiRequestId,
    pub evidence_refs: Vec<ChildDomainEvidenceRef>,
    pub source_observed_at: ChildDomainObservedAt,
    pub result_fact_ref: ChildDomainFactRef,
    pub private_payload_state: PrivatePayloadState,
    pub policy_evaluation_requirement: ChildDomainPolicyEvaluationRequirement,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ChildDomainPolicyEvaluationRequestedEvent {
    pub event_type: ChildDomainEventType,
    pub domain: ChildRuntimeDomain,
    pub child_device_id: ChildDomainChildDeviceId,
    pub child_profile_id: ChildDomainChildProfileId,
    pub policy_request_id: ChildDomainPolicyRequestId,
    pub evidence_refs: Vec<ChildDomainEvidenceRef>,
    pub source_observed_at: ChildDomainObservedAt,
    pub source_fact_ref: ChildDomainFactRef,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ChildDomainPolicyViolationDetectedEvent {
    pub event_type: ChildDomainEventType,
    pub domain: ChildRuntimeDomain,
    pub child_device_id: ChildDomainChildDeviceId,
    pub child_profile_id: ChildDomainChildProfileId,
    pub violation_id: ChildDomainPolicyViolationId,
    pub policy_rule_ref: ChildDomainPolicyRuleRef,
    pub severity: ChildDomainPolicySeverity,
    pub detected_at: ChildDomainObservedAt,
    pub evidence_refs: Vec<ChildDomainEvidenceRef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ChildDomainNotificationRequestedEvent {
    pub event_type: ChildDomainEventType,
    pub domain: ChildRuntimeDomain,
    pub child_device_id: ChildDomainChildDeviceId,
    pub child_profile_id: ChildDomainChildProfileId,
    pub notification_id: ChildDomainNotificationId,
    pub source_policy_violation_id: ChildDomainPolicyViolationId,
    pub channel: ChildDomainNotificationChannel,
    pub requested_at: ChildDomainObservedAt,
    pub evidence_refs: Vec<ChildDomainEvidenceRef>,
}

include!("child_domain_runtime/event_flow.rs");
include!("child_domain_runtime/identifiers.rs");
